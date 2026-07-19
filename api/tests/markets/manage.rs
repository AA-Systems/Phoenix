use api::build_app;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use axum_limit::Quota;
use tower::ServiceExt;
use types::markets::{Market, MarketStatus, insert_market_response::InsertMarketBody};

use crate::common::{
    insert_asset_req, insert_market_req, test_state, test_state_with_resource_quotas,
};

async fn create_market() -> (api::app_state::AppState, Market) {
    let state = test_state().await;
    let app = build_app(state.clone());

    app.clone()
        .oneshot(insert_asset_req(
            "/api/v1/assets/admin/insert",
            "USDC",
            "USD Coin",
            6,
            true,
        ))
        .await
        .unwrap();
    app.clone()
        .oneshot(insert_asset_req(
            "/api/v1/assets/admin/insert",
            "SOL",
            "Solana",
            9,
            true,
        ))
        .await
        .unwrap();

    let response = app
        .oneshot(insert_market_req(
            "SOL/USDC",
            "Solana USDC market",
            "SOL",
            "USDC",
            true,
        ))
        .await
        .unwrap();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: InsertMarketBody = serde_json::from_slice(&bytes).unwrap();

    (state, body.market)
}

fn patch_request(uri: String, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("PATCH")
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, "Bearer test-token")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
#[serial_test::serial]
async fn market_endpoints_enforce_rate_limit() {
    let state =
        test_state_with_resource_quotas(Quota::per_minute(5), Quota::per_second(10_000)).await;
    let app = build_app(state);

    for _ in 0..5 {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/v1/markets?limit=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/markets?limit=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
#[serial_test::serial]
async fn public_market_endpoints_return_configuration() {
    let (state, _) = create_market().await;
    let app = build_app(state);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/markets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let markets: Vec<Market> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(markets.len(), 1);
    assert_eq!(markets[0].price_tick_size, 10_000);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/markets?limit=1&skip=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let markets: Vec<Market> = serde_json::from_slice(&bytes).unwrap();
    assert!(markets.is_empty());

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/markets?limit=101&skip=0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/markets/sol/usdc")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[serial_test::serial]
async fn market_status_and_configuration_follow_transition_rules() {
    let (state, market) = create_market().await;
    let app = build_app(state);
    let status_uri = format!("/api/v1/markets/admin/{}/status", market.id);
    let config_uri = format!("/api/v1/markets/admin/{}/config", market.id);

    let response = app
        .clone()
        .oneshot(patch_request(
            config_uri.clone(),
            serde_json::json!({
                "price_tick_size": 1_000,
                "quantity_step_size": 100_000,
                "min_order_quantity": 1_000_000,
                "min_order_notional": 2_000_000
            }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);

    let response = app
        .clone()
        .oneshot(patch_request(
            status_uri.clone(),
            serde_json::json!({ "status": "archived" }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);

    let response = app
        .clone()
        .oneshot(patch_request(
            status_uri.clone(),
            serde_json::json!({ "status": "halted" }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(patch_request(
            config_uri.clone(),
            serde_json::json!({
                "price_tick_size": 0,
                "quantity_step_size": 100_000,
                "min_order_quantity": 1_000_000,
                "min_order_notional": 2_000_000
            }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let response = app
        .clone()
        .oneshot(patch_request(
            config_uri,
            serde_json::json!({
                "price_tick_size": 1_000,
                "quantity_step_size": 100_000,
                "min_order_quantity": 1_000_000,
                "min_order_notional": 2_000_000
            }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let updated: Market = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(updated.status, MarketStatus::Halted);
    assert_eq!(updated.price_tick_size, 1_000);

    let response = app
        .clone()
        .oneshot(patch_request(
            status_uri.clone(),
            serde_json::json!({ "status": "archived" }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .oneshot(patch_request(
            status_uri,
            serde_json::json!({ "status": "trading" }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}
