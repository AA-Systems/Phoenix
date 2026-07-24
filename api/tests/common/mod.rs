pub mod assets;
pub mod balances;
pub mod markets;
pub mod orders;
pub mod users;

use std::{fs, path::Path};

pub use assets::insert_asset_req::insert_asset_req;
pub use balances::credit_balance_req::credit_balance_req;
pub use markets::insert_market_req::insert_market_req;

use api::{
    app_state::{AppState, RateLimitQuotas},
    services::{refresh_token_service::RefreshTokenConfig, token_service::TokenService},
};
use axum_limit::Quota;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, Client};
use sqlx::postgres::PgPoolOptions;
use types::query::{EngineQuery, EngineReply};

pub const ADMIN_TOKEN: &str = "test-token";
pub const TEST_DATABASE_URL: &str = "postgres://admin:supersecretpassword@localhost:5433/cex_test";
pub const TEST_REDIS_URL: &str = "redis://localhost:6379";
pub const TEST_ORDER_COMMANDS_STREAM: &str = "order-commands-test";
pub const TEST_ENGINE_COMMANDS_STREAM: &str = "engine-commands-test";
pub const TEST_ENGINE_QUERIES_STREAM: &str = "engine-queries-test";
pub const TEST_ENGINE_QUERY_TIMEOUT_SECS: f64 = 2.0;
pub const TEST_JWT_ISSUER: &str = "centralized-exchange-test";
pub const TEST_JWT_AUDIENCE: &str = "exchange-api-test";
pub const TEST_ACCESS_TOKEN_TTL_SECONDS: u64 = 900;

pub async fn test_state() -> AppState {
    test_state_with_resource_quotas(Quota::per_second(10_000), Quota::per_second(10_000)).await
}

pub async fn test_state_with_resource_quotas(market_quota: Quota, asset_quota: Quota) -> AppState {
    let pool = PgPoolOptions::new()
        .connect(TEST_DATABASE_URL)
        .await
        .unwrap();

    sqlx::query("TRUNCATE assets CASCADE")
        .execute(&pool)
        .await
        .unwrap();

    let redis_client = Client::open(TEST_REDIS_URL).expect("invalid test REDIS_URL");
    let mut redis = redis::aio::ConnectionManager::new(redis_client)
        .await
        .expect("cannot connect to test Redis");

    let _: Result<(), redis::RedisError> = redis::cmd("DEL")
        .arg(TEST_ORDER_COMMANDS_STREAM)
        .query_async(&mut redis)
        .await;
    let _: Result<(), redis::RedisError> = redis::cmd("DEL")
        .arg(TEST_ENGINE_COMMANDS_STREAM)
        .query_async(&mut redis)
        .await;
    let _: Result<(), redis::RedisError> = redis::cmd("DEL")
        .arg(TEST_ENGINE_QUERIES_STREAM)
        .query_async(&mut redis)
        .await;

    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let private_key_pem = fs::read(repository_root.join("secrets/jwt-private.pem"))
        .expect("cannot read test JWT private key");
    let public_key_pem = fs::read(repository_root.join("secrets/jwt-public.pem"))
        .expect("cannot read test JWT public key");
    let token_service = TokenService::new(
        &private_key_pem,
        &public_key_pem,
        TEST_JWT_ISSUER.to_string(),
        TEST_JWT_AUDIENCE.to_string(),
        TEST_ACCESS_TOKEN_TTL_SECONDS,
    )
    .expect("cannot initialize token service");

    let refresh_token_config = RefreshTokenConfig {
        refresh_token_ttl_seconds: 2592000,
        cookie_secure: false,
    };

    AppState::new(
        pool,
        ADMIN_TOKEN.into(),
        token_service,
        refresh_token_config,
        redis,
        TEST_ORDER_COMMANDS_STREAM.to_string(),
        TEST_ENGINE_COMMANDS_STREAM.to_string(),
        TEST_ENGINE_QUERIES_STREAM.to_string(),
        TEST_ENGINE_QUERY_TIMEOUT_SECS,
        RateLimitQuotas {
            auth: Quota::per_second(10_000),
            health: Quota::per_second(10_000),
            market: market_quota,
            asset: asset_quota,
            order: Quota::per_second(10_000),
        },
    )
}

pub async fn spawn_empty_balance_query_responder() -> tokio::task::JoinHandle<()> {
    let client = Client::open(TEST_REDIS_URL).unwrap();
    let mut conn = redis::aio::ConnectionManager::new(client).await.unwrap();

    let _: Result<(), redis::RedisError> = redis::cmd("DEL")
        .arg(TEST_ENGINE_QUERIES_STREAM)
        .query_async(&mut conn)
        .await;

    let group = "test-query-responder";
    let _: Result<(), redis::RedisError> = redis::cmd("XGROUP")
        .arg("CREATE")
        .arg(TEST_ENGINE_QUERIES_STREAM)
        .arg(group)
        .arg("0")
        .arg("MKSTREAM")
        .query_async(&mut conn)
        .await;

    tokio::spawn(async move {
        let opts = StreamReadOptions::default()
            .group(group, "responder-1")
            .count(1)
            .block(2000);

        loop {
            let reply: Result<StreamReadReply, redis::RedisError> = conn
                .xread_options(&[TEST_ENGINE_QUERIES_STREAM], &[">"], &opts)
                .await;

            let Ok(reply) = reply else {
                continue;
            };

            for stream_key in reply.keys {
                for entry in stream_key.ids {
                    let payload = match entry.map.get("payload") {
                        Some(redis::Value::BulkString(bytes)) => {
                            String::from_utf8(bytes.clone()).ok()
                        }
                        Some(redis::Value::SimpleString(text)) => Some(text.clone()),
                        _ => None,
                    };

                    if let Some(payload) = payload {
                        if let Ok(query) = serde_json::from_str::<EngineQuery>(&payload) {
                            let reply = match query {
                                EngineQuery::GetBalances { request_id, .. } => {
                                    EngineReply::GetBalances {
                                        request_id,
                                        balances: Vec::new(),
                                    }
                                }
                                EngineQuery::GetOpenOrders { request_id, .. } => {
                                    EngineReply::GetOpenOrders {
                                        request_id,
                                        orders: Vec::new(),
                                    }
                                }
                                EngineQuery::GetOrderBook {
                                    request_id,
                                    market_symbol,
                                } => EngineReply::GetOrderBook {
                                    request_id,
                                    book: Some(types::orderbook::OrderBookDepth {
                                        market_symbol,
                                        bids: Vec::new(),
                                        asks: Vec::new(),
                                    }),
                                },
                                EngineQuery::GetRecentTrades { request_id, .. } => {
                                    EngineReply::GetRecentTrades {
                                        request_id,
                                        trades: Some(Vec::new()),
                                    }
                                }
                            };
                            let body = serde_json::to_string(&reply).unwrap();
                            let key = format!("engine-reply:{}", reply.request_id());
                            let _: Result<(), redis::RedisError> = conn.lpush(&key, body).await;
                            let _: Result<(), redis::RedisError> = conn.expire(&key, 30).await;
                        }
                    }

                    let _: Result<u64, redis::RedisError> = redis::cmd("XACK")
                        .arg(TEST_ENGINE_QUERIES_STREAM)
                        .arg(group)
                        .arg(&entry.id)
                        .query_async(&mut conn)
                        .await;
                }
            }
        }
    })
}
