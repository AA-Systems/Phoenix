use axum::{Json, extract::State, http::StatusCode};
use db::markets::insert::insert;
use serde::Deserialize;
use types::markets::Market;
use validator::Validate;

use crate::app_state::AppState;

#[derive(Deserialize, Validate)]
pub struct InsertMarketRequest {
    #[validate(length(min = 1, max = 32))]
    symbol: String,
    #[validate(length(min = 1, max = 64))]
    name: String,
    #[validate(length(min = 1, max = 16))]
    base_asset_symbol: String,
    #[validate(length(min = 1, max = 16))]
    quote_asset_symbol: String,
}

pub async fn insert_market(
    State(app_state): State<AppState>,
    body: Json<InsertMarketRequest>,
) -> Result<(StatusCode, Json<Market>), (StatusCode, String)> {
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let db_response = insert(
        &app_state.pool,
        body.symbol.trim().to_uppercase(),
        body.name.trim().to_string(),
        body.base_asset_symbol.trim().to_uppercase(),
        body.quote_asset_symbol.trim().to_uppercase(),
    )
    .await;

    match db_response {
        Ok(market) => Ok((StatusCode::CREATED, Json(market))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
