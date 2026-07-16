use std::time::Duration;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode};
use db::{sessions::insert::insert, users::login::find_by_email};
use sqlx::types::chrono::Utc;
use types::auth::{auth_response::AuthResponse, login_user_request::LoginUserRequest};
use validator::Validate;

use crate::{app_state::AppState, services::refresh_token_service::RefreshTokenService};

pub async fn login_user(
    State(app_state): State<AppState>,
    Json(mut body): Json<LoginUserRequest>,
) -> Result<AuthResponse, (StatusCode, String)> {
    body.email = body.email.trim().to_lowercase();
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let credentials = find_by_email(&app_state.pool, &body.email)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid email or password".to_string(),
            )
        })?;

    let stored_password_hash = credentials.password_hash.clone();
    let password_valid = tokio::task::spawn_blocking(move || {
        let password = body.password;
        let parsed_hash = PasswordHash::new(&stored_password_hash)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
    })
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?
    .is_ok();

    if !password_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ));
    }

    let user = credentials.into_user();

    let refresh_token = RefreshTokenService::generate();
    let session_expires_at = Utc::now() + Duration::from_hours(720);

    let session = insert(
        &app_state.pool,
        user.id,
        &refresh_token.token_hash,
        session_expires_at,
        None,
        None,
    )
    .await
    .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?;

    let access_token = app_state
        .token_service
        .issue_access_token(user.id, session.id)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        })?;

    Ok(AuthResponse::ok(
        user,
        access_token,
        app_state.token_service.access_token_ttl_seconds() as u32,
    ))
}
