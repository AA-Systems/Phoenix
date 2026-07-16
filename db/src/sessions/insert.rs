use chrono::{DateTime, Utc};
use sqlx::{Executor, Postgres};
use types::sessions::Session;
use uuid::Uuid;

pub async fn insert<'e, E>(
    executor: E,
    user_id: Uuid,
    refresh_token_hash: &str,
    expires_at: DateTime<Utc>,
    user_agent: Option<&str>,
    ip_address: Option<&str>,
) -> Result<Session, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, Session>(include_str!("sql/insert.sql"))
        .bind(user_id)
        .bind(refresh_token_hash)
        .bind(expires_at)
        .bind(user_agent)
        .bind(ip_address)
        .fetch_one(executor)
        .await
}
