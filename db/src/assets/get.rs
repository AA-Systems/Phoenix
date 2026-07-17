use sqlx::PgPool;
use types::assets::Asset;

pub async fn get_by_symbols(pool: &PgPool, symbols: &[String]) -> Result<Vec<Asset>, sqlx::Error> {
    sqlx::query_as::<_, Asset>(include_str!("sql/get_by_symbols.sql"))
        .bind(symbols)
        .fetch_all(pool)
        .await
}

pub async fn get_by_symbol(pool: &PgPool, symbol: &str) -> Result<Asset, sqlx::Error> {
    sqlx::query_as::<_, Asset>(include_str!("sql/get_by_symbol.sql"))
        .bind(symbol)
        .fetch_one(pool)
        .await
}
