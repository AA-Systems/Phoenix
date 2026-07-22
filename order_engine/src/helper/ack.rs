use redis::{AsyncCommands, RedisResult, aio::ConnectionManager};
use tracing::error;

pub async fn ack(conn: &mut ConnectionManager, stream: &str, group: &str, entry_id: &str) {
    let result: RedisResult<i64> = conn.xack(stream, group, &[entry_id]).await;
    if let Err(err) = result {
        error!(%entry_id, %err, "XACK failed");
    }
}
