use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Debug, FromRow)]
pub struct EngineSnapshotRow {
    pub id: i16,
    pub created_at: DateTime<Utc>,
    pub next_order_id: i64,
    pub order_commands_cursor: String,
    pub engine_commands_cursor: String,
    pub state: Value,
}

pub async fn load_latest(pool: &PgPool) -> Result<Option<EngineSnapshotRow>, sqlx::Error> {
    sqlx::query_as::<_, EngineSnapshotRow>(include_str!("sql/get_latest.sql"))
        .fetch_optional(pool)
        .await
}
