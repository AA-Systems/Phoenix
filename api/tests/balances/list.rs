use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use tower::ServiceExt;
use types::auth::auth_response::AuthBody;
use types::balances::AssetBalance;

use crate::common::{
    test_state,
    users::insert_user_req::{get_balances_req, register_user_req, unique_email},
};

const VALID_PASSWORD: &str = "StrongPassword123!";

#[tokio::test]
async fn get_balances_requires_access_token() {
    let app = build_app(test_state().await);

    let response = app.oneshot(get_balances_req(None)).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_balances_returns_empty_list_for_new_user() {
    let app = build_app(test_state().await);
    let email = unique_email();

    let request = register_user_req("Test User", &email, VALID_PASSWORD);
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let registered: AuthBody =
        serde_json::from_slice(&bytes).expect("registration should return auth JSON");

    let request = get_balances_req(Some(&registered.access_token));
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let balances: Vec<AssetBalance> =
        serde_json::from_slice(&bytes).expect("balances should return JSON array");
    assert!(balances.is_empty());
}
