use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};
use axum_limit::DynamicFixedWindowLimit;
use db::balances::list_by_user;
use types::balances::AssetBalance;

use crate::{
    app_state::{AppState, AuthQuota},
    middlewares::{jwt_middleware::AuthUser, rate_limit_key::ClientIpUri},
};

pub async fn get_balance_by_user_id(
    _: DynamicFixedWindowLimit<ClientIpUri, AuthQuota>,
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<AssetBalance>>, (StatusCode, String)> {
    let balances = list_by_user::get_by_user_id(&app_state.pool, auth_user.user_id)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => (StatusCode::UNAUTHORIZED, String::from("Unauthorized")),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        })?;

    Ok(Json(balances))
}
