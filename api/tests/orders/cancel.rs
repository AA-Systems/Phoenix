use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use redis::AsyncCommands;
use tower::ServiceExt;
use types::auth::auth_response::AuthBody;
use types::command::Command;
use types::order::cancel_order_response::CancelOrderBody;

use crate::common::{
    TEST_ORDER_COMMANDS_STREAM, TEST_REDIS_URL,
    orders::cancel_order_req::cancel_order_req,
    test_state,
    users::insert_user_req::{register_user_req, unique_email},
};

const VALID_PASSWORD: &str = "StrongPassword123!";

#[tokio::test]
#[serial_test::serial]
async fn cancel_order_requires_access_token() {
    let app = build_app(test_state().await);
    let response = app.oneshot(cancel_order_req(None, "1")).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial_test::serial]
async fn cancel_order_enqueues_command_on_redis_stream() {
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
        .oneshot(cancel_order_req(Some(&registered.access_token), "1"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: CancelOrderBody =
        serde_json::from_slice(&bytes).expect("cancel order should return JSON");
    assert_eq!(body.order_id, "1");

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
        Command::CancelOrder {
            command_id,
            user_id,
            order_id,
        } => {
            assert_eq!(command_id, body.command_id);
            assert_eq!(user_id, registered.user.id);
            assert_eq!(order_id, "1");
        }
        _ => panic!("expected CancelOrder command"),
    }
}
