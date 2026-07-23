use api::{
    app_state::{AppState, RateLimitQuotas},
    build_app,
    services::{refresh_token_service::RefreshTokenConfig, token_service::TokenService},
};
use axum::http::{Request, StatusCode};
use axum_limit::Quota;
use redis::Client;
use sqlx::postgres::PgPoolOptions;
use std::{fs, path::Path};
use tower::ServiceExt;

use crate::common::{
    ADMIN_TOKEN, TEST_ACCESS_TOKEN_TTL_SECONDS, TEST_DATABASE_URL, TEST_ENGINE_COMMANDS_STREAM,
    TEST_ENGINE_QUERIES_STREAM, TEST_ENGINE_QUERY_TIMEOUT_SECS, TEST_JWT_AUDIENCE, TEST_JWT_ISSUER,
    TEST_ORDER_COMMANDS_STREAM, TEST_REDIS_URL, test_state,
};

#[tokio::test]
#[serial_test::serial]
async fn test_health_endpoint() {
    let app = build_app(test_state().await);
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/health")
        .body(axum::body::Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[serial_test::serial]
async fn health_endpoint_enforces_rate_limit() {
    let pool = PgPoolOptions::new()
        .connect(TEST_DATABASE_URL)
        .await
        .unwrap();

    let redis_client = Client::open(TEST_REDIS_URL).unwrap();
    let redis = redis::aio::ConnectionManager::new(redis_client)
        .await
        .unwrap();

    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let private_key_pem = fs::read(repository_root.join("secrets/jwt-private.pem")).unwrap();
    let public_key_pem = fs::read(repository_root.join("secrets/jwt-public.pem")).unwrap();
    let token_service = TokenService::new(
        &private_key_pem,
        &public_key_pem,
        TEST_JWT_ISSUER.to_string(),
        TEST_JWT_AUDIENCE.to_string(),
        TEST_ACCESS_TOKEN_TTL_SECONDS,
    )
    .unwrap();

    let state = AppState::new(
        pool,
        ADMIN_TOKEN.into(),
        token_service,
        RefreshTokenConfig {
            refresh_token_ttl_seconds: 2592000,
            cookie_secure: false,
        },
        redis,
        TEST_ORDER_COMMANDS_STREAM.to_string(),
        TEST_ENGINE_COMMANDS_STREAM.to_string(),
        TEST_ENGINE_QUERIES_STREAM.to_string(),
        TEST_ENGINE_QUERY_TIMEOUT_SECS,
        RateLimitQuotas {
            auth: Quota::per_minute(10),
            health: Quota::per_minute(5),
            market: Quota::per_minute(5),
            asset: Quota::per_minute(5),
            order: Quota::per_minute(30),
        },
    );
    let app = build_app(state);

    for i in 0..5 {
        let request = Request::builder()
            .method("GET")
            .uri("/api/v1/health")
            .body(axum::body::Body::empty())
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "request {i} should succeed"
        );
    }

    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/health")
        .body(axum::body::Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
}
