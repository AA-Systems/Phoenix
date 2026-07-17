use axum::{Json, extract::State, http::StatusCode};
use db::assets::get::get_by_symbol;
use db::balances::credit::{CreditBalance, credit};
use db::users::find_by_id::find_by_id;
use types::balances::credit_balance_request::CreditBalanceRequest;
use types::balances::credit_balance_response::CreditBalanceResponse;
use validator::Validate;

use crate::app_state::AppState;

pub async fn credit_balance(
    State(app_state): State<AppState>,
    Json(mut body): Json<CreditBalanceRequest>,
) -> Result<CreditBalanceResponse, (StatusCode, String)> {
    body.asset_symbol = body.asset_symbol.trim().to_uppercase();
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    find_by_id(&app_state.pool, body.user_id)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => (StatusCode::BAD_REQUEST, "User not found".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        })?;

    let asset = get_by_symbol(&app_state.pool, &body.asset_symbol)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => (StatusCode::BAD_REQUEST, "Asset not found".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        })?;

    let (balance, ledger_entry) = credit(
        &app_state.pool,
        CreditBalance {
            user_id: body.user_id,
            asset_id: asset.id,
            amount: body.amount,
        },
    )
    .await
    .map_err(|error| match error {
        sqlx::Error::Database(ref db_err) if db_err.is_check_violation() => (
            StatusCode::BAD_REQUEST,
            "Invalid balance update".to_string(),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
    })?;

    Ok(CreditBalanceResponse::created(balance, ledger_entry))
}
