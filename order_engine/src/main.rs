use std::sync::Arc;
use std::time::Duration;

use order_engine::helper::ensure_consumer_group::ensure_consumer_group;
use order_engine::helper::handle_entry::handle_entry;
use order_engine::helper::handle_query_entry::handle_query_entry;
use order_engine::memory::OrderEngineState;
use order_engine::memory::load_from_db::load_from_db;
use redis::aio::ConnectionManager;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, Client, RedisResult};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;
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
    let query_stream = std::env::var("REDIS_ENGINE_QUERIES_STREAM")
        .unwrap_or_else(|_| "engine-queries".to_string());
    let events_stream = std::env::var("REDIS_EXCHANGE_EVENTS_STREAM")
        .unwrap_or_else(|_| "exchange-events".to_string());
    let group =
        std::env::var("REDIS_ORDER_ENGINE_GROUP").unwrap_or_else(|_| "order-engine".to_string());
    let consumer = std::env::var("REDIS_ORDER_ENGINE_CONSUMER")
        .unwrap_or_else(|_| "order-engine-1".to_string());
    let query_group = std::env::var("REDIS_ENGINE_QUERY_GROUP")
        .unwrap_or_else(|_| "order-engine-queries".to_string());
    let query_consumer = std::env::var("REDIS_ENGINE_QUERY_CONSUMER")
        .unwrap_or_else(|_| "order-engine-query-1".to_string());

    info!(
        %redis_url,
        %order_stream,
        %engine_stream,
        %query_stream,
        %events_stream,
        %group,
        %consumer,
        "starting order engine"
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("cannot connect to database");

    let state = load_from_db(&pool)
        .await
        .expect("failed to load engine state from database");
    let state = Arc::new(Mutex::new(state));

    let client = Client::open(redis_url.as_str()).expect("invalid REDIS_URL");

    let mut cmd_conn = ConnectionManager::new(client.clone())
        .await
        .expect("failed to connect to Redis (commands)");
    let mut query_conn = ConnectionManager::new(client)
        .await
        .expect("failed to connect to Redis (queries)");

    ensure_consumer_group(&mut cmd_conn, &order_stream, &group).await;
    ensure_consumer_group(&mut cmd_conn, &engine_stream, &group).await;
    ensure_consumer_group(&mut query_conn, &query_stream, &query_group).await;

    let command_state = Arc::clone(&state);
    let command_pool = pool.clone();
    let order_stream_cmd = order_stream.clone();
    let engine_stream_cmd = engine_stream.clone();
    let group_cmd = group.clone();
    let consumer_cmd = consumer.clone();

    let commands = tokio::spawn(async move {
        run_command_loop(
            cmd_conn,
            command_pool,
            command_state,
            order_stream_cmd,
            engine_stream_cmd,
            group_cmd,
            consumer_cmd,
            events_stream,
        )
        .await;
    });

    let queries = tokio::spawn(async move {
        run_query_loop(query_conn, state, query_stream, query_group, query_consumer).await;
    });

    info!("command and query consumers running");
    let _ = tokio::join!(commands, queries);
}

async fn run_command_loop(
    mut conn: ConnectionManager,
    pool: sqlx::PgPool,
    state: Arc<Mutex<OrderEngineState>>,
    order_stream: String,
    engine_stream: String,
    group: String,
    consumer: String,
    events_stream: String,
) {
    let read_opts = StreamReadOptions::default()
        .group(&group, &consumer)
        .count(1)
        .block(5000);

    loop {
        let reply: RedisResult<StreamReadReply> = conn
            .xread_options(&[&order_stream, &engine_stream], &[">", ">"], &read_opts)
            .await;

        match reply {
            Ok(reply) => {
                for stream_key in reply.keys {
                    for entry in stream_key.ids {
                        let mut guard = state.lock().await;
                        handle_entry(
                            &mut conn,
                            &pool,
                            &mut guard,
                            &stream_key.key,
                            &group,
                            &entry.id,
                            &entry.map,
                            &events_stream,
                        )
                        .await;
                    }
                }
            }
            Err(err) => {
                error!(%err, "Redis command XREADGROUP failed");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn run_query_loop(
    mut conn: ConnectionManager,
    state: Arc<Mutex<OrderEngineState>>,
    query_stream: String,
    group: String,
    consumer: String,
) {
    let read_opts = StreamReadOptions::default()
        .group(&group, &consumer)
        .count(10)
        .block(2000);

    info!(%query_stream, %group, %consumer, "consuming engine queries");

    loop {
        let reply: RedisResult<StreamReadReply> = conn
            .xread_options(&[&query_stream], &[">"], &read_opts)
            .await;

        match reply {
            Ok(reply) => {
                for stream_key in reply.keys {
                    for entry in stream_key.ids {
                        handle_query_entry(
                            &mut conn,
                            &state,
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
                error!(%err, "Redis query XREADGROUP failed");
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
