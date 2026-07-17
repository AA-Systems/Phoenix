use axum::{extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use axum_limit::DynamicFixedWindowLimit;
use db::{sessions::rotate::rotate, users::find_by_id::find_by_id};
use types::auth::auth_response::AuthResponse;

use crate::{
    app_state::{AppState, AuthQuota},
    middlewares::rate_limit_key::ClientIpUri,
    services::refresh_token_service::RefreshTokenService,
};

pub async fn refresh_token(
    _: DynamicFixedWindowLimit<ClientIpUri, AuthQuota>,
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> Result<AuthResponse, (StatusCode, String)> {
    let raw_token = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing refresh token".to_string(),
        ))?;

    let old_token_hash = RefreshTokenService::hash(&raw_token);
    let new_refresh_token = RefreshTokenService::generate();

    let session = rotate(
        &app_state.pool,
        &old_token_hash,
        &new_refresh_token.token_hash,
    )
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

    let user = find_by_id(&app_state.pool, session.user_id)
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

    let access_token = app_state
        .token_service
        .issue_access_token(user.id, session.id)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        })?;

    let refresh_cookie = RefreshTokenService::build_refresh_token(
        new_refresh_token.raw_token,
        app_state.refresh_token_config.refresh_token_ttl_seconds,
        app_state.refresh_token_config.cookie_secure,
    );
    let jar = jar.add(refresh_cookie);

    Ok(AuthResponse::ok(
        jar,
        user,
        access_token,
        app_state.token_service.access_token_ttl_seconds() as u32,
    ))
}
