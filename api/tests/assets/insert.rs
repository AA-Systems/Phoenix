use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use tower::ServiceExt;
use types::assets::insert_asset_response::InsertAssetBody;

use crate::common::{insert_asset_req, test_state};

#[tokio::test]
async fn test_insert_asset_enpoint_without_token() {
    let app = build_app(test_state().await);

    let request = insert_asset_req("/api/v1/assets/admin/insert", "SOL", "Solana", 9, false);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED); // no token → 401
}

#[tokio::test]
async fn test_insert_asset_then_duplicate() {
    let state = test_state().await;
    let app = build_app(state.clone());

    let request = insert_asset_req("/api/v1/assets/admin/insert", "USD", "USD Coin", 6, true);
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: InsertAssetBody =
        serde_json::from_slice(&bytes).expect("response should contain asset JSON");

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body.asset.symbol, "USD");
    assert_eq!(body.asset.name, "USD Coin");
    assert_eq!(body.asset.decimals, 6);
    assert_eq!(body.asset.status, types::assets::AssetStatus::Active);

    let request = insert_asset_req("/api/v1/assets/admin/insert", "USD", "USD Coin", 6, true);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
