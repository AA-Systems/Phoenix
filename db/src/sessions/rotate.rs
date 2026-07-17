use sqlx::PgPool;
use types::sessions::Session;

pub async fn rotate(
    pool: &PgPool,
    old_token_hash: &str,
    new_token_hash: &str,
) -> Result<Session, sqlx::Error> {
    sqlx::query_as::<_, Session>(include_str!("sql/rotate.sql"))
        .bind(old_token_hash)
        .bind(new_token_hash)
        .fetch_one(pool)
        .await
}
