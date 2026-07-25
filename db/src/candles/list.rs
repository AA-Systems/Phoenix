use sqlx::PgPool;
use types::candle::Candle;

pub async fn list_candles(
    pool: &PgPool,
    market_symbol: &str,
    interval: &str,
    limit: i64,
) -> Result<Vec<Candle>, sqlx::Error> {
    sqlx::query_as::<_, Candle>(include_str!("sql/list.sql"))
        .bind(market_symbol)
        .bind(interval)
        .bind(limit)
        .fetch_all(pool)
        .await
}
