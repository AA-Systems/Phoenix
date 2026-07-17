use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};
use axum_limit::DynamicFixedWindowLimit;
use db::users::find_by_id::find_by_id;
use types::auth::User;

use crate::{
    app_state::{AppState, AuthQuota},
    middlewares::{jwt_middleware::AuthUser, rate_limit_key::ClientIpUri},
};

pub async fn me(
    _: DynamicFixedWindowLimit<ClientIpUri, AuthQuota>,
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = find_by_id(&app_state.pool, auth_user.user_id)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => (StatusCode::UNAUTHORIZED, String::from("Unauthorized")),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        })?;

    Ok(Json(user))
}
