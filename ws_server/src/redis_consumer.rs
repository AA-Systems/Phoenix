use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use redis::streams::{StreamReadOptions, StreamReadReply};
use tracing::{error, info, warn};
use types::event::ExchangeEvent;

use crate::hub::Hub;

pub async fn run_redis_consumer(mut conn: ConnectionManager, stream: String, hub: Hub) {
    let mut last_id = "$".to_string();
    let opts = StreamReadOptions::default().count(100).block(5000);

    info!(%stream, "consuming exchange events");

    loop {
        let reply: Result<StreamReadReply, redis::RedisError> =
            conn.xread_options(&[&stream], &[&last_id], &opts).await;

        match reply {
            Ok(reply) => {
                for key in reply.keys {
                    for entry in key.ids {
                        last_id = entry.id.clone();

                        let Some(payload) = entry.map.get("payload") else {
                            warn!(id = %entry.id, "exchange event missing payload");
                            continue;
                        };

                        let bytes = match payload {
                            redis::Value::BulkString(bytes) => bytes.as_slice(),
                            redis::Value::SimpleString(s) => s.as_bytes(),
                            _ => {
                                warn!(id = %entry.id, "unexpected payload type");
                                continue;
                            }
                        };

                        match serde_json::from_slice::<ExchangeEvent>(bytes) {
                            Ok(event) => hub.publish(&event).await,
                            Err(err) => {
                                error!(%err, id = %entry.id, "failed to decode ExchangeEvent")
                            }
                        }
                    }
                }
            }
            Err(err) => {
                error!(%err, "Redis XREAD failed");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }
}
