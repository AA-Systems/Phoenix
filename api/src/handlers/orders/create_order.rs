use axum::{Extension, Json, extract::State, http::StatusCode};
use axum_limit::DynamicFixedWindowLimit;
use db::markets::get::get_by_symbol;
use redis::AsyncCommands;
use types::command::Command;
use types::order::create_order_request::CreateOrderRequest;
use types::order::create_order_response::CreateOrderResponse;
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::{AppState, OrderQuota},
    middlewares::{jwt_middleware::AuthUser, rate_limit_key::ClientIpUri},
};

pub async fn create_order(
    _: DynamicFixedWindowLimit<ClientIpUri, OrderQuota>,
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(mut body): Json<CreateOrderRequest>,
) -> Result<CreateOrderResponse, (StatusCode, String)> {
    body.market_symbol = body.market_symbol.trim().to_uppercase();
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    get_by_symbol(&app_state.pool, &body.market_symbol)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => (StatusCode::BAD_REQUEST, "Market not found".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        })?;

    let command_id = Uuid::new_v4();
    let command = Command::CreateOrder {
        command_id,
        user_id: auth_user.user_id,
        market_symbol: body.market_symbol.clone(),
        order_type: body.order_type,
        price: body.price,
        quantity: body.quantity,
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
                "Failed to enqueue order command".to_string(),
            )
        })?;

    Ok(CreateOrderResponse::accepted(
        command_id,
        body.market_symbol,
        body.order_type,
        body.price,
        body.quantity,
    ))
}
