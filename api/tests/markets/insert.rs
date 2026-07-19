use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use tower::ServiceExt;
use types::markets::insert_market_response::InsertMarketBody;

use crate::common::{insert_asset_req, insert_market_req, test_state};

#[tokio::test]
#[serial_test::serial]
async fn test_insert_market_without_token() {
    let app = build_app(test_state().await);

    let request = insert_market_req("SOL/USDC", "Solana USDC market", "SOL", "USDC", false);
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial_test::serial]
async fn test_insert_market_then_duplicate() {
    let state = test_state().await;
    let app = build_app(state.clone());

    // Insert assets
    let request = insert_asset_req("/api/v1/assets/admin/insert", "USDC", "USD Coin", 6, true);
    app.clone().oneshot(request).await.unwrap();
    let request = insert_asset_req("/api/v1/assets/admin/insert", "SOL", "Solana", 9, true);
    app.clone().oneshot(request).await.unwrap();

    let request = insert_market_req("SOL/USDC", "Solana USDC market", "SOL", "USDC", true);
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: InsertMarketBody =
        serde_json::from_slice(&bytes).expect("response should contain market JSON");

    assert_eq!(body.market.symbol, "SOL/USDC");
    assert_eq!(body.market.name, "Solana USDC market");

    let request = insert_market_req("SOL/USDC", "Solana USDC market", "SOL", "USDC", true);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
