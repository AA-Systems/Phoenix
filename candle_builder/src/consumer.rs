use std::time::Duration;

use db::candles::apply_trade;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use redis::streams::{StreamReadOptions, StreamReadReply};
use sqlx::PgPool;
use tracing::{error, info, warn};
use types::event::ExchangeEvent;

use crate::ensure_group::ensure_consumer_group;

pub async fn run_consumer(
    mut conn: ConnectionManager,
    pool: PgPool,
    stream: String,
    group: String,
    consumer: String,
) {
    ensure_consumer_group(&mut conn, &stream, &group).await;
    info!(%stream, %group, %consumer, "candle builder consuming exchange events");

    let opts = StreamReadOptions::default()
        .group(&group, &consumer)
        .count(50)
        .block(5000);

    loop {
        let reply: redis::RedisResult<StreamReadReply> =
            conn.xread_options(&[&stream], &[">"], &opts).await;

        let reply = match reply {
            Ok(reply) => reply,
            Err(err) => {
                error!(%err, "xreadgroup failed; retrying");
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        for stream_key in reply.keys {
            for entry in stream_key.ids {
                let payload = entry
                    .map
                    .get("payload")
                    .and_then(|value| redis::from_redis_value::<String>(value).ok());

                let Some(payload) = payload else {
                    warn!(id = %entry.id, "missing payload field; acking");
                    let _: Result<u64, _> = redis::cmd("XACK")
                        .arg(&stream)
                        .arg(&group)
                        .arg(&entry.id)
                        .query_async(&mut conn)
                        .await;
                    continue;
                };

                match serde_json::from_str::<ExchangeEvent>(&payload) {
                    Ok(ExchangeEvent::TradeExecuted { trade }) => {
                        match apply_trade(&pool, &trade).await {
                            Ok(true) => {
                                info!(
                                    trade_id = %trade.id,
                                    market = %trade.market_symbol,
                                    "applied trade to candles"
                                );
                            }
                            Ok(false) => {
                                info!(trade_id = %trade.id, "trade already applied; skipping");
                            }
                            Err(err) => {
                                error!(
                                    %err,
                                    trade_id = %trade.id,
                                    "failed to apply trade; will retry without ack"
                                );
                                continue;
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(err) => {
                        warn!(%err, id = %entry.id, "invalid exchange event; acking");
                    }
                }

                let _: Result<u64, _> = redis::cmd("XACK")
                    .arg(&stream)
                    .arg(&group)
                    .arg(&entry.id)
                    .query_async(&mut conn)
                    .await;
            }
        }
    }
}
