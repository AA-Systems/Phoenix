use sqlx::PgPool;
use types::markets::Market;

use crate::assets::get::get_by_symbols;

pub async fn insert(
    pool: &PgPool,
    symbol: String,
    name: String,
    base_asset_symbol: String,
    quote_asset_symbol: String,
) -> Result<Market, sqlx::Error> {
    if base_asset_symbol == quote_asset_symbol {
        return Err(sqlx::Error::Protocol(
            "base and quote assets must be different".into(),
        ));
    }

    let assets = get_by_symbols(
        pool,
        &[base_asset_symbol.clone(), quote_asset_symbol.clone()],
    )
    .await?;

    let base_asset = assets
        .iter()
        .find(|a| a.symbol == base_asset_symbol)
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    let quote_asset = assets
        .iter()
        .find(|a| a.symbol == quote_asset_symbol)
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    sqlx::query_as::<_, Market>(include_str!("sql/insert.sql"))
        .bind(&symbol)
        .bind(&name)
        .bind(base_asset.id)
        .bind(quote_asset.id)
        .fetch_one(pool)
        .await
}
