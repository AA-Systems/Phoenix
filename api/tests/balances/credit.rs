use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use redis::AsyncCommands;
use tower::ServiceExt;
use types::auth::auth_response::AuthBody;
use types::balances::credit_balance_response::CreditBalanceBody;
use types::command::Command;

use crate::common::{
    TEST_ENGINE_COMMANDS_STREAM, TEST_REDIS_URL, credit_balance_req, insert_asset_req, test_state,
    users::insert_user_req::{register_user_req, unique_email},
};

const VALID_PASSWORD: &str = "StrongPassword123!";

#[tokio::test]
#[serial_test::serial]
async fn credit_balance_requires_admin_token() {
    let app = build_app(test_state().await);
    let request = credit_balance_req(uuid::Uuid::new_v4(), "USDC", 100, false);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial_test::serial]
async fn credit_balance_enqueues_command_on_redis_stream() {
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

    let request = insert_asset_req("/api/v1/assets/admin/insert", "INR", "INR COIN", 2, true);
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let request = credit_balance_req(registered.user.id, "INR", 100_000_000, true);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let credited: CreditBalanceBody =
        serde_json::from_slice(&bytes).expect("credit should return JSON");
    assert_eq!(credited.user_id, registered.user.id);
    assert_eq!(credited.asset_symbol, "INR");
    assert_eq!(credited.amount, 100_000_000);

    let client = redis::Client::open(TEST_REDIS_URL).unwrap();
    let mut conn = redis::aio::ConnectionManager::new(client).await.unwrap();
    let entries: redis::streams::StreamRangeReply = conn
        .xrange(TEST_ENGINE_COMMANDS_STREAM, "-", "+")
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
        Command::CreditBalance {
            command_id,
            user_id,
            amount,
            ..
        } => {
            assert_eq!(command_id, credited.command_id);
            assert_eq!(user_id, registered.user.id);
            assert_eq!(amount, 100_000_000);
        }
        other => panic!("expected CreditBalance, got {other:?}"),
    }
}
