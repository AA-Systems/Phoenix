use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};
use axum_limit::DynamicFixedWindowLimit;
use types::order::OpenOrderView;

use crate::{
    app_state::{AppState, OrderQuota},
    middlewares::{jwt_middleware::AuthUser, rate_limit_key::ClientIpUri},
    services::engine_query::{EngineQueryError, get_open_orders},
};

pub async fn list_open_orders(
    _: DynamicFixedWindowLimit<ClientIpUri, OrderQuota>,
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<OpenOrderView>>, (StatusCode, String)> {
    let mut redis = app_state.redis.clone();
    let orders = get_open_orders(
        &mut redis,
        &app_state.engine_queries_stream,
        auth_user.user_id,
        app_state.engine_query_timeout_secs,
    )
    .await
    .map_err(|error| match error {
        EngineQueryError::Timeout => (
            StatusCode::GATEWAY_TIMEOUT,
            "Engine did not respond in time".to_string(),
        ),
        EngineQueryError::Enqueue | EngineQueryError::InvalidReply | EngineQueryError::NotFound => {
            (
                StatusCode::BAD_GATEWAY,
                "Failed to query order engine".to_string(),
            )
        }
    })?;

    Ok(Json(orders))
}
