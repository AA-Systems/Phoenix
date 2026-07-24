use axum::{Json, extract::State, http::StatusCode};
use axum_limit::DynamicFixedWindowLimit;
use db::assets::get::list_all;
use types::assets::Asset;

use crate::{
    app_state::{AppState, AssetQuota},
    middlewares::rate_limit_key::ClientIpUri,
};

pub async fn list_assets(
    _: DynamicFixedWindowLimit<ClientIpUri, AssetQuota>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Asset>>, (StatusCode, String)> {
    list_all(&app_state.pool).await.map(Json).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })
}
