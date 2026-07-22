use order_engine::helper::ensure_consumer_group::ensure_consumer_group;
use order_engine::helper::handle_entry::handle_entry;
use order_engine::memory::OrderEngineState;
use redis::aio::ConnectionManager;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, Client, RedisResult};
use std::time::Duration;
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

    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let stream = std::env::var("REDIS_ORDER_COMMANDS_STREAM")
        .unwrap_or_else(|_| "order-commands".to_string());
    let group =
        std::env::var("REDIS_ORDER_ENGINE_GROUP").unwrap_or_else(|_| "order-engine".to_string());
    let consumer = std::env::var("REDIS_ORDER_ENGINE_CONSUMER")
        .unwrap_or_else(|_| "order-engine-1".to_string());

    info!(%redis_url, %stream, %group, %consumer, "starting order engine consumer");

    let client = Client::open(redis_url.as_str()).expect("invalid REDIS_URL");
    let mut conn = ConnectionManager::new(client)
        .await
        .expect("failed to connect to Redis");

    ensure_consumer_group(&mut conn, &stream, &group).await;

    let mut state = OrderEngineState::new();
    let read_opts = StreamReadOptions::default()
        .group(&group, &consumer)
        .count(1)
        .block(5000);

    info!("consuming order commands from Redis stream");

    loop {
        let reply: RedisResult<StreamReadReply> =
            conn.xread_options(&[&stream], &[">"], &read_opts).await;

        match reply {
            Ok(reply) => {
                for stream_key in reply.keys {
                    for entry in stream_key.ids {
                        handle_entry(
                            &mut conn, &mut state, &stream, &group, &entry.id, &entry.map,
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
