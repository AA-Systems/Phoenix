use std::time::Duration;

use redis::{RedisResult, aio::ConnectionManager};
use tracing::{info, warn};

pub async fn ensure_consumer_group(conn: &mut ConnectionManager, stream: &str, group: &str) {
    loop {
        let created: RedisResult<String> = redis::cmd("XGROUP")
            .arg("CREATE")
            .arg(stream)
            .arg(group)
            .arg("0")
            .arg("MKSTREAM")
            .query_async(conn)
            .await;

        match created {
            Ok(_) => {
                info!(%stream, %group, "created consumer group");
                return;
            }
            Err(err) if err.to_string().contains("BUSYGROUP") => {
                info!(%stream, %group, "consumer group already exists");
                return;
            }
            Err(err) => {
                warn!(%err, "failed to create consumer group; retrying");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
