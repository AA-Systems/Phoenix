use axum::{Json, extract::State, http::StatusCode};
use db::assets::insert::insert;
use serde::Deserialize;
use types::assets::Asset;
use validator::Validate;

use crate::app_state::AppState;

#[derive(Deserialize, Validate)]
pub struct InsertAssetRequest {
    #[validate(length(min = 1, max = 16))]
    pub symbol: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(range(max = 18))]
    pub decimals: u32,
}

pub async fn insert_asset(
    State(app_state): State<AppState>,
    Json(body): Json<InsertAssetRequest>,
) -> Result<(StatusCode, Json<Asset>), (StatusCode, String)> {
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let db_response = insert(
        &app_state.pool,
        body.symbol.trim().to_uppercase(),
        body.name.trim().to_string(),
        body.decimals,
    )
    .await;

    match db_response {
        Ok(asset) => Ok((StatusCode::CREATED, Json(asset))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
