use sqlx::PgPool;
use types::balances::Balance;

pub async fn list_all(pool: &PgPool) -> Result<Vec<Balance>, sqlx::Error> {
    sqlx::query_as::<_, Balance>(include_str!("sql/list_all.sql"))
        .fetch_all(pool)
        .await
}
