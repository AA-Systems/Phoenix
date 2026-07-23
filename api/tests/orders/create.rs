use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use redis::AsyncCommands;
use tower::ServiceExt;
use types::auth::auth_response::AuthBody;
use types::command::Command;
use types::order::OrderType;
use types::order::create_order_response::CreateOrderBody;

use crate::common::{
    TEST_ORDER_COMMANDS_STREAM, TEST_REDIS_URL, insert_asset_req, insert_market_req,
    orders::create_order_req::create_order_req,
    test_state,
    users::insert_user_req::{register_user_req, unique_email},
};

const VALID_PASSWORD: &str = "StrongPassword123!";

#[tokio::test]
#[serial_test::serial]
async fn create_order_requires_access_token() {
    let app = build_app(test_state().await);
    let response = app
        .oneshot(create_order_req(
            None,
            "SOL_USDC",
            "buy",
            150_000_000,
            100_000_000,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial_test::serial]
async fn create_order_enqueues_command_on_redis_stream() {
    let app = build_app(test_state().await);
    let email = unique_email();

    let response = app
        .clone()
        .oneshot(register_user_req("Test User", &email, VALID_PASSWORD))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let registered: AuthBody =
        serde_json::from_slice(&bytes).expect("registration should return auth JSON");

    let response = app
        .clone()
        .oneshot(insert_asset_req(
            "/api/v1/assets/admin/insert",
            "SOL",
            "Solana",
            9,
            true,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .clone()
        .oneshot(insert_asset_req(
            "/api/v1/assets/admin/insert",
            "USDC",
            "USD Coin",
            6,
            true,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .clone()
        .oneshot(insert_market_req(
            "SOL_USDC",
            "SOL / USDC",
            "SOL",
            "USDC",
            true,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .oneshot(create_order_req(
            Some(&registered.access_token),
            "SOL_USDC",
            "buy",
            150_000_000,
            100_000_000,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: CreateOrderBody =
        serde_json::from_slice(&bytes).expect("create order should return JSON");
    assert_eq!(body.market_symbol, "SOL_USDC");
    assert_eq!(body.order_type, OrderType::Buy);
    assert_eq!(body.price, 150_000_000);
    assert_eq!(body.quantity, 100_000_000);

    let client = redis::Client::open(TEST_REDIS_URL).unwrap();
    let mut conn = redis::aio::ConnectionManager::new(client).await.unwrap();
    let entries: redis::streams::StreamRangeReply = conn
        .xrange(TEST_ORDER_COMMANDS_STREAM, "-", "+")
        .await
        .unwrap();
    assert!(!entries.ids.is_empty());

    let last = entries.ids.last().unwrap();
    let payload = match last.map.get("payload").unwrap() {
        redis::Value::BulkString(bytes) => String::from_utf8(bytes.clone()).unwrap(),
        redis::Value::SimpleString(text) => text.clone(),
        other => panic!("unexpected payload value: {other:?}"),
    };
    let command: Command = serde_json::from_str(&payload).unwrap();
    match command {
        Command::CreateOrder {
            command_id,
            user_id,
            market_symbol,
            order_type,
            price,
            quantity,
        } => {
            assert_eq!(command_id, body.command_id);
            assert_eq!(user_id, registered.user.id);
            assert_eq!(market_symbol, "SOL_USDC");
            assert_eq!(order_type, OrderType::Buy);
            assert_eq!(price, 150_000_000);
            assert_eq!(quantity, 100_000_000);
        }
        _ => panic!("expected CreateOrder command"),
    }
}
