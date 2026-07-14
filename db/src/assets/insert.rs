use sqlx::PgPool;
use types::assets::Asset;

pub async fn insert(
    pool: &PgPool,
    symbol: String,
    name: String,
    decimals: u32,
) -> Result<Asset, sqlx::Error> {
    sqlx::query_as::<_, Asset>(include_str!("sql/insert.sql"))
        .bind(&symbol)
        .bind(&name)
        .bind(decimals as i32)
        .fetch_one(pool)
        .await
}
