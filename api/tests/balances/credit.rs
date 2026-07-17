use api::build_app;
use axum::{body::to_bytes, http::StatusCode};
use tower::ServiceExt;
use types::auth::auth_response::AuthBody;
use types::balances::Balance;
use types::balances::credit_balance_response::CreditBalanceBody;
use types::ledger_entries::LedgerEntryType;

use crate::common::{
    credit_balance_req, insert_asset_req, test_state,
    users::insert_user_req::{get_balances_req, register_user_req, unique_email},
};

const VALID_PASSWORD: &str = "StrongPassword123!";

#[tokio::test]
async fn credit_balance_requires_admin_token() {
    let app = build_app(test_state().await);
    let request = credit_balance_req(uuid::Uuid::new_v4(), "USDC", 100, false);
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn credit_balance_then_list_balances() {
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
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let credited: CreditBalanceBody =
        serde_json::from_slice(&bytes).expect("credit should return balance JSON");
    assert_eq!(credited.balance.user_id, registered.user.id);
    assert_eq!(credited.balance.available, 100_000_000);
    assert_eq!(credited.balance.locked, 0);
    assert_eq!(credited.ledger_entry.entry_type, LedgerEntryType::Deposit);
    assert_eq!(credited.ledger_entry.available_delta, 100_000_000);
    assert_eq!(credited.ledger_entry.available_after, 100_000_000);

    let request = get_balances_req(Some(&registered.access_token));
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let balances: Vec<Balance> =
        serde_json::from_slice(&bytes).expect("balances should return JSON array");
    assert_eq!(balances.len(), 1);
    assert_eq!(balances[0].available, 100_000_000);
    assert_eq!(balances[0].locked, 0);
}
