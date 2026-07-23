use axum::{Json, extract::State, http::StatusCode};
use db::assets::get::get_by_symbol;
use db::users::find_by_id::find_by_id;
use redis::AsyncCommands;
use types::balances::credit_balance_request::CreditBalanceRequest;
use types::balances::credit_balance_response::CreditBalanceResponse;
use types::command::Command;
use uuid::Uuid;
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

    let command_id = Uuid::new_v4();
    let command = Command::CreditBalance {
        command_id,
        user_id: body.user_id,
        asset_id: asset.id,
        amount: body.amount,
    };

    let payload = serde_json::to_string(&command).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    let mut redis = app_state.redis.clone();
    redis
        .xadd::<_, _, _, _, String>(
            &app_state.engine_commands_stream,
            "*",
            &[("payload", payload.as_str())],
        )
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to enqueue credit command".to_string(),
            )
        })?;

    Ok(CreditBalanceResponse::accepted(
        command_id,
        body.user_id,
        body.asset_symbol,
        body.amount,
    ))
}
