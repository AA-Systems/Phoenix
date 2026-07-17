use axum::{Json, extract::State, http::StatusCode};
use db::assets::insert::insert;
use types::assets::{
    insert_asset_request::InsertAssetRequest, insert_asset_response::InsertAssetResponse,
};
use validator::Validate;

use crate::app_state::AppState;

pub async fn insert_asset(
    State(app_state): State<AppState>,
    Json(body): Json<InsertAssetRequest>,
) -> Result<InsertAssetResponse, (StatusCode, String)> {
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
        Ok(asset) => Ok(InsertAssetResponse::created(asset)),
        Err(error) => Err(match error {
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
                (StatusCode::BAD_REQUEST, "Asset already exists".to_string())
            }
            sqlx::Error::RowNotFound => (
                StatusCode::BAD_REQUEST,
                "Unable to create asset".to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        }),
    }
}
