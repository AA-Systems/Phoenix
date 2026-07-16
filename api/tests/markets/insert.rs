use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use tower::ServiceExt;
use types::markets::Market;

use crate::common::{insert_asset_req, insert_market_req, test_state};

#[tokio::test]
async fn test_insert_market_without_token() {
    let app = build_app(test_state().await);

    let request = insert_market_req("SOL/USDC", "Solana USDC market", "SOL", "USDC", false);
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
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
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let market: Market = serde_json::from_slice(&bytes).expect("response should be Market JSON");

    assert_eq!(market.symbol, "SOL/USDC");
    assert_eq!(market.name, "Solana USDC market");

    let request = insert_market_req("SOL/USDC", "Solana USDC market", "SOL", "USDC", true);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
