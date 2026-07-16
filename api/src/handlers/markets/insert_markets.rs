use axum::{Json, extract::State, http::StatusCode};
use db::markets::insert::insert;
use types::markets::{
    insert_market_request::InsertMarketRequest, insert_market_response::InsertMarketResponse,
};
use validator::Validate;

use crate::app_state::AppState;

pub async fn insert_market(
    State(app_state): State<AppState>,
    body: Json<InsertMarketRequest>,
) -> Result<InsertMarketResponse, (StatusCode, String)> {
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
        Ok(market) => Ok(InsertMarketResponse::created(market)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
