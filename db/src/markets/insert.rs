use sqlx::PgPool;
use types::markets::Market;

use crate::assets::get::get_by_symbols;

pub struct InsertMarketParams {
    pub symbol: String,
    pub name: String,
    pub base_asset_symbol: String,
    pub quote_asset_symbol: String,
    pub price_tick_size: i64,
    pub quantity_step_size: i64,
    pub min_order_quantity: i64,
    pub min_order_notional: i64,
}

pub async fn insert(pool: &PgPool, input: InsertMarketParams) -> Result<Market, sqlx::Error> {
    if input.base_asset_symbol == input.quote_asset_symbol {
        return Err(sqlx::Error::Protocol(
            "base and quote assets must be different".into(),
        ));
    }

    let assets = get_by_symbols(
        pool,
        &[
            input.base_asset_symbol.clone(),
            input.quote_asset_symbol.clone(),
        ],
    )
    .await?;

    let base_asset = assets
        .iter()
        .find(|asset| asset.symbol == input.base_asset_symbol)
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let quote_asset = assets
        .iter()
        .find(|asset| asset.symbol == input.quote_asset_symbol)
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    sqlx::query_as::<_, Market>(include_str!("sql/insert.sql"))
        .bind(&input.symbol)
        .bind(&input.name)
        .bind(base_asset.id)
        .bind(quote_asset.id)
        .bind(input.price_tick_size)
        .bind(input.quantity_step_size)
        .bind(input.min_order_quantity)
        .bind(input.min_order_notional)
        .fetch_one(pool)
        .await
}
