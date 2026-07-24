use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use tracing::{error, info};
use types::event::ExchangeEvent;

pub async fn publish_exchange_events(
    conn: &mut ConnectionManager,
    stream: &str,
    events: &[ExchangeEvent],
) {
    for event in events {
        let payload = match serde_json::to_string(event) {
            Ok(payload) => payload,
            Err(err) => {
                error!(%err, "failed to serialize ExchangeEvent");
                continue;
            }
        };

        let result: Result<String, redis::RedisError> = conn
            .xadd(stream, "*", &[("payload", payload.as_str())])
            .await;

        match result {
            Ok(_) => info!(stream = %stream, "published exchange event"),
            Err(err) => error!(%err, stream = %stream, "failed to publish exchange event"),
        }
    }
}
