use sqlx::PgPool;
use types::sessions::Session;

pub async fn revoke(pool: &PgPool, refresh_token_hash: &str) -> Result<Session, sqlx::Error> {
    sqlx::query_as::<_, Session>(include_str!("sql/revoke.sql"))
        .bind(refresh_token_hash)
        .fetch_one(pool)
        .await
}
