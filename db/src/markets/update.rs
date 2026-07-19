use sqlx::PgPool;
use types::markets::{Market, MarketStatus};
use uuid::Uuid;

pub async fn update_status(
    pool: &PgPool,
    id: Uuid,
    status: MarketStatus,
) -> Result<Market, sqlx::Error> {
    sqlx::query_as::<_, Market>(include_str!("sql/update_status.sql"))
        .bind(id)
        .bind(status)
        .fetch_one(pool)
        .await
}

pub async fn update_config(
    pool: &PgPool,
    id: Uuid,
    price_tick_size: i64,
    quantity_step_size: i64,
    min_order_quantity: i64,
    min_order_notional: i64,
) -> Result<Market, sqlx::Error> {
    sqlx::query_as::<_, Market>(include_str!("sql/update_config.sql"))
        .bind(id)
        .bind(price_tick_size)
        .bind(quantity_step_size)
        .bind(min_order_quantity)
        .bind(min_order_notional)
        .fetch_one(pool)
        .await
}
