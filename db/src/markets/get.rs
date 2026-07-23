use sqlx::PgPool;
use types::markets::Market;
use uuid::Uuid;

pub async fn list_all(pool: &PgPool) -> Result<Vec<Market>, sqlx::Error> {
    sqlx::query_as::<_, Market>(include_str!("sql/list_all.sql"))
        .fetch_all(pool)
        .await
}

pub async fn list(pool: &PgPool, limit: i64, skip: i64) -> Result<Vec<Market>, sqlx::Error> {
    sqlx::query_as::<_, Market>(include_str!("sql/list.sql"))
        .bind(limit)
        .bind(skip)
        .fetch_all(pool)
        .await
}

pub async fn get_by_symbol(pool: &PgPool, symbol: &str) -> Result<Market, sqlx::Error> {
    sqlx::query_as::<_, Market>(include_str!("sql/get_by_symbol.sql"))
        .bind(symbol)
        .fetch_one(pool)
        .await
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Market, sqlx::Error> {
    sqlx::query_as::<_, Market>(include_str!("sql/get_by_id.sql"))
        .bind(id)
        .fetch_one(pool)
        .await
}
