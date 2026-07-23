use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use tower::ServiceExt;
use types::auth::auth_response::AuthBody;
use types::ledger_entries::{LedgerEntryType, LedgerEntryView, LedgerIntent};

use crate::common::{
    insert_asset_req, test_state,
    users::insert_user_req::{get_ledger_req, register_user_req, unique_email},
};

const VALID_PASSWORD: &str = "StrongPassword123!";

#[tokio::test]
#[serial_test::serial]
async fn list_ledger_requires_access_token() {
    let app = build_app(test_state().await);
    let response = app.oneshot(get_ledger_req(None)).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[serial_test::serial]
async fn list_ledger_returns_empty_for_new_user() {
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
        .oneshot(get_ledger_req(Some(&registered.access_token)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let entries: Vec<LedgerEntryView> =
        serde_json::from_slice(&bytes).expect("ledger should return JSON array");
    assert!(entries.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn list_ledger_returns_persisted_deposit() {
    let state = test_state().await;
    let pool = state.pool.clone();
    let app = build_app(state);
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
            "LED",
            "Ledger Coin",
            6,
            true,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let asset = db::assets::get::get_by_symbol(&pool, "LED")
        .await
        .expect("asset should exist");

    let command_id = uuid::Uuid::new_v4();
    let intent = LedgerIntent {
        command_id,
        user_id: registered.user.id,
        asset_id: asset.id,
        entry_type: LedgerEntryType::Deposit,
        available_delta: 1_000_000,
        locked_delta: 0,
        available_after: 1_000_000,
        locked_after: 0,
        reference_id: Some(command_id),
        reference_type: Some("command".into()),
    };
    db::balances::persist_intents::persist_intents(&pool, &[intent])
        .await
        .expect("persist ledger");

    let response = app
        .oneshot(get_ledger_req(Some(&registered.access_token)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let entries: Vec<LedgerEntryView> =
        serde_json::from_slice(&bytes).expect("ledger should return JSON array");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].asset_symbol, "LED");
    assert_eq!(entries[0].entry_type, LedgerEntryType::Deposit);
    assert_eq!(entries[0].available_delta, 1_000_000);
    assert_eq!(entries[0].command_id, Some(command_id));
}
