use axum::{Extension, Json, extract::State, http::StatusCode};
use redis::AsyncCommands;
use types::command::Command;
use types::order::cancel_order_request::CancelOrderRequest;
use types::order::cancel_order_response::CancelOrderResponse;
use uuid::Uuid;
use validator::Validate;

use crate::{app_state::AppState, middlewares::jwt_middleware::AuthUser};

pub async fn cancel_order(
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(mut body): Json<CancelOrderRequest>,
) -> Result<CancelOrderResponse, (StatusCode, String)> {
    body.order_id = body.order_id.trim().to_string();
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let command_id = Uuid::new_v4();
    let command = Command::CancelOrder {
        command_id,
        user_id: auth_user.user_id,
        order_id: body.order_id.clone(),
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
            &app_state.order_commands_stream,
            "*",
            &[("payload", payload.as_str())],
        )
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to enqueue cancel command".to_string(),
            )
        })?;

    Ok(CancelOrderResponse::accepted(command_id, body.order_id))
}
