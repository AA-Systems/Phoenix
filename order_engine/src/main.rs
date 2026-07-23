use std::time::Duration;

use order_engine::helper::ensure_consumer_group::ensure_consumer_group;
use order_engine::helper::handle_entry::handle_entry;
use order_engine::memory::load_from_db::load_from_db;
use redis::aio::ConnectionManager;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, Client, RedisResult};
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let order_stream = std::env::var("REDIS_ORDER_COMMANDS_STREAM")
        .unwrap_or_else(|_| "order-commands".to_string());
    let engine_stream = std::env::var("REDIS_ENGINE_COMMANDS_STREAM")
        .unwrap_or_else(|_| "engine-commands".to_string());
    let group =
        std::env::var("REDIS_ORDER_ENGINE_GROUP").unwrap_or_else(|_| "order-engine".to_string());
    let consumer = std::env::var("REDIS_ORDER_ENGINE_CONSUMER")
        .unwrap_or_else(|_| "order-engine-1".to_string());

    info!(
        %redis_url,
        %order_stream,
        %engine_stream,
        %group,
        %consumer,
        "starting order engine consumer"
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("cannot connect to database");

    let mut state = load_from_db(&pool)
        .await
        .expect("failed to load engine state from database");

    let client = Client::open(redis_url.as_str()).expect("invalid REDIS_URL");
    let mut conn = ConnectionManager::new(client)
        .await
        .expect("failed to connect to Redis");

    ensure_consumer_group(&mut conn, &order_stream, &group).await;
    ensure_consumer_group(&mut conn, &engine_stream, &group).await;

    let read_opts = StreamReadOptions::default()
        .group(&group, &consumer)
        .count(1)
        .block(5000);

    info!("consuming order and engine commands from Redis streams");

    loop {
        let reply: RedisResult<StreamReadReply> = conn
            .xread_options(&[&order_stream, &engine_stream], &[">", ">"], &read_opts)
            .await;

        match reply {
            Ok(reply) => {
                for stream_key in reply.keys {
                    for entry in stream_key.ids {
                        handle_entry(
                            &mut conn,
                            &pool,
                            &mut state,
                            &stream_key.key,
                            &group,
                            &entry.id,
                            &entry.map,
                        )
                        .await;
                    }
                }
            }
            Err(err) => {
                error!(%err, "Redis XREADGROUP failed");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
