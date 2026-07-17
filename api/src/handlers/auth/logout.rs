use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use axum_limit::DynamicFixedWindowLimit;
use db::sessions::revoke::revoke;

use crate::{
    app_state::{AppState, AuthQuota},
    middlewares::rate_limit_key::ClientIpUri,
    services::refresh_token_service::RefreshTokenService,
};

pub async fn logout(
    _: DynamicFixedWindowLimit<ClientIpUri, AuthQuota>,
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let raw_token = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing refresh token".to_string(),
        ))?;

    let token_hash = RefreshTokenService::hash(&raw_token);

    revoke(&app_state.pool, &token_hash)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => (
                StatusCode::UNAUTHORIZED,
                "Invalid or expired session".to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        })?;

    let jar = jar.add(RefreshTokenService::clear_refresh_token(
        app_state.refresh_token_config.cookie_secure,
    ));

    Ok((StatusCode::NO_CONTENT, jar))
}
