mod consumer;
mod ensure_group;

use std::time::Duration;

use common::config::Config;
use redis::Client;
use redis::aio::ConnectionManager;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::consumer::run_consumer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env();

    info!(
        redis_url = %config.redis_url,
        events_stream = %config.exchange_events_stream,
        group = %config.candle_builder_group,
        consumer = %config.candle_builder_consumer,
        "starting candle builder"
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
        .expect("cannot connect to database");

    let client = Client::open(config.redis_url.as_str()).expect("invalid REDIS_URL");
    let conn = ConnectionManager::new(client)
        .await
        .expect("failed to connect to Redis");

    run_consumer(
        conn,
        pool,
        config.exchange_events_stream,
        config.candle_builder_group,
        config.candle_builder_consumer,
    )
    .await;
}
