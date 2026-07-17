use api::build_app;
use axum::{
    body::to_bytes,
    http::{StatusCode, header},
};
use tower::ServiceExt;
use types::auth::auth_response::AuthBody;

use crate::common::{
    test_state,
    users::insert_user_req::{login_user_req, refresh_token_req, register_user_req, unique_email},
};

const VALID_PASSWORD: &str = "StrongPassword123!";

#[tokio::test]
async fn rejects_a_weak_password() {
    let app = build_app(test_state().await);
    let request = register_user_req("Test User", &unique_email(), "weak-password");

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn registers_user_and_session_then_rejects_duplicate_email() {
    let state = test_state().await;
    let app = build_app(state.clone());
    let email = unique_email();

    let request = register_user_req(" Test User ", &email.to_uppercase(), VALID_PASSWORD);
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let set_cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .expect("response should set refresh cookie")
        .to_str()
        .expect("Set-Cookie should be valid text")
        .to_owned();

    let attributes: Vec<&str> = set_cookie.split("; ").collect();
    assert!(
        attributes
            .iter()
            .any(|value| value.starts_with("refresh_token="))
    );
    assert!(attributes.contains(&"HttpOnly"));
    assert!(attributes.contains(&"SameSite=Lax"));
    assert!(attributes.contains(&"Path=/api/v1/auth"));
    assert!(attributes.contains(&"Max-Age=2592000"));
    assert!(!attributes.contains(&"Secure"));

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: AuthBody = serde_json::from_slice(&bytes).expect("response should contain auth JSON");

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body.user.name, "Test User");
    assert_eq!(body.user.email, email);
    assert_eq!(body.expires_in, 900);
    assert!(!body.access_token.is_empty());

    let claims = state
        .token_service
        .verify_access_token(&body.access_token)
        .expect("access token should be valid");
    assert_eq!(claims.sub, body.user.id);

    let request = register_user_req("Test User", &email, VALID_PASSWORD);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn logs_in_registered_user_and_rejects_wrong_password() {
    let state = test_state().await;
    let app = build_app(state.clone());
    let email = unique_email();

    let request = register_user_req("Test User", &email, VALID_PASSWORD);
    let response = app.clone().oneshot(request).await.unwrap();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let registered: AuthBody =
        serde_json::from_slice(&bytes).expect("registration should return auth JSON");

    let request = login_user_req(&email.to_uppercase(), VALID_PASSWORD);
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let set_cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .expect("response should set refresh cookie")
        .to_str()
        .expect("Set-Cookie should be valid text")
        .to_owned();

    let attributes: Vec<&str> = set_cookie.split("; ").collect();
    assert!(
        attributes
            .iter()
            .any(|value| value.starts_with("refresh_token="))
    );
    assert!(attributes.contains(&"HttpOnly"));
    assert!(attributes.contains(&"SameSite=Lax"));
    assert!(attributes.contains(&"Path=/api/v1/auth"));
    assert!(attributes.contains(&"Max-Age=2592000"));
    assert!(!attributes.contains(&"Secure"));

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let logged_in: AuthBody =
        serde_json::from_slice(&bytes).expect("login should return auth JSON");

    assert_eq!(status, StatusCode::OK);
    assert_eq!(logged_in.user.id, registered.user.id);
    assert_eq!(logged_in.user.email, email);
    assert!(!logged_in.access_token.is_empty());

    let claims = state
        .token_service
        .verify_access_token(&logged_in.access_token)
        .expect("login access token should be valid");
    assert_eq!(claims.sub, logged_in.user.id);

    let request = login_user_req(&email, "WrongPassword123!");
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn refreshes_access_token_and_rejects_reuse_of_old_cookie() {
    let state = test_state().await;
    let app = build_app(state.clone());
    let email = unique_email();

    let request = register_user_req("Test User", &email, VALID_PASSWORD);
    let response = app.clone().oneshot(request).await.unwrap();
    let set_cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .expect("registration should set refresh cookie")
        .to_str()
        .expect("Set-Cookie should be valid text")
        .to_owned();
    let refresh_cookie = set_cookie
        .split(';')
        .next()
        .expect("Set-Cookie should include refresh_token")
        .to_owned();

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let registered: AuthBody =
        serde_json::from_slice(&bytes).expect("registration should return auth JSON");

    let request = refresh_token_req(&refresh_cookie);
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let new_set_cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .expect("refresh should set a new refresh cookie")
        .to_str()
        .expect("Set-Cookie should be valid text")
        .to_owned();

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let refreshed: AuthBody =
        serde_json::from_slice(&bytes).expect("refresh should return auth JSON");

    assert_eq!(status, StatusCode::OK);
    assert_eq!(refreshed.user.id, registered.user.id);
    assert_eq!(refreshed.user.email, email);
    assert!(!refreshed.access_token.is_empty());
    assert_ne!(new_set_cookie, set_cookie);

    let claims = state
        .token_service
        .verify_access_token(&refreshed.access_token)
        .expect("refreshed access token should be valid");
    assert_eq!(claims.sub, refreshed.user.id);

    let request = refresh_token_req(&refresh_cookie);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
