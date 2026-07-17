use std::time::Duration;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use axum_extra::extract::CookieJar;
use rand_core::OsRng;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use axum_limit::DynamicFixedWindowLimit;
use db::auth::register_with_session::{RegistrationWithSession, register_with_session};
use sqlx::types::chrono::Utc;
use types::auth::{auth_response::AuthResponse, register_user_request::RegisterUserRequest};
use validator::Validate;

use crate::{
    app_state::{AppState, AuthQuota},
    middlewares::rate_limit_key::ClientIpUri,
    services::refresh_token_service::RefreshTokenService,
};

pub async fn register_user(
    _: DynamicFixedWindowLimit<ClientIpUri, AuthQuota>,
    State(app_state): State<AppState>,
    jar: CookieJar,
    Json(mut body): Json<RegisterUserRequest>,
) -> Result<AuthResponse, (StatusCode, String)> {
    body.name = body.name.trim().to_string();
    body.email = body.email.trim().to_lowercase();
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let password_hash = tokio::task::spawn_blocking(move || {
        let password = body.password;
        let salt = SaltString::generate(&mut OsRng);

        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    })
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    let refresh_token = RefreshTokenService::generate();
    let session_expires_at =
        Utc::now() + Duration::from_secs(app_state.refresh_token_config.refresh_token_ttl_seconds);

    let registration = RegistrationWithSession {
        name: &body.name,
        email: &body.email,
        password_hash: &password_hash,
        refresh_token_hash: &refresh_token.token_hash,
        session_expires_at,
        user_agent: None,
        ip_address: None,
    };

    let (user, session) = register_with_session(&app_state.pool, registration)
        .await
        .map_err(|error| match error {
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
                (StatusCode::BAD_REQUEST, "Email already registered".to_string())
            }
            sqlx::Error::RowNotFound => (
                StatusCode::BAD_REQUEST,
                "Unable to complete registration".to_string(),
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
        refresh_token.raw_token,
        app_state.refresh_token_config.refresh_token_ttl_seconds,
        app_state.refresh_token_config.cookie_secure,
    );
    let jar = jar.add(refresh_cookie);

    Ok(AuthResponse::created(
        jar,
        user,
        access_token,
        app_state.token_service.access_token_ttl_seconds() as u32,
    ))
}
