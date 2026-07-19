use axum::{Json, extract::State, http::StatusCode};
use axum_limit::DynamicFixedWindowLimit;
use db::markets::insert::{InsertMarketParams, insert};
use types::markets::{
    insert_market_request::InsertMarketRequest, insert_market_response::InsertMarketResponse,
};
use validator::Validate;

use crate::{
    app_state::{AppState, MarketQuota},
    middlewares::rate_limit_key::ClientIpUri,
};

pub async fn insert_market(
    _: DynamicFixedWindowLimit<ClientIpUri, MarketQuota>,
    State(app_state): State<AppState>,
    body: Json<InsertMarketRequest>,
) -> Result<InsertMarketResponse, (StatusCode, String)> {
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let insert_market_params = InsertMarketParams {
        symbol: body.symbol.trim().to_uppercase(),
        name: body.name.trim().to_string(),
        base_asset_symbol: body.base_asset_symbol.trim().to_uppercase(),
        quote_asset_symbol: body.quote_asset_symbol.trim().to_uppercase(),
        price_tick_size: body.price_tick_size,
        quantity_step_size: body.quantity_step_size,
        min_order_quantity: body.min_order_quantity,
        min_order_notional: body.min_order_notional,
    };

    let db_response = insert(&app_state.pool, insert_market_params).await;

    match db_response {
        Ok(market) => Ok(InsertMarketResponse::created(market)),
        Err(error) => Err(match error {
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
                (StatusCode::BAD_REQUEST, "Market already exists".to_string())
            }
            sqlx::Error::RowNotFound => (
                StatusCode::BAD_REQUEST,
                "Unable to create market".to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        }),
    }
}
