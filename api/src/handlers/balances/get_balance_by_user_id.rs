use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
};
use axum_limit::DynamicFixedWindowLimit;
use types::balances::AssetBalance;

use crate::{
    app_state::{AppState, AuthQuota},
    middlewares::{jwt_middleware::AuthUser, rate_limit_key::ClientIpUri},
    services::engine_query::{EngineQueryError, get_balances},
};

pub async fn get_balance_by_user_id(
    _: DynamicFixedWindowLimit<ClientIpUri, AuthQuota>,
    State(app_state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Vec<AssetBalance>>, (StatusCode, String)> {
    let mut redis = app_state.redis.clone();
    let balances = get_balances(
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
        EngineQueryError::Enqueue | EngineQueryError::InvalidReply => (
            StatusCode::BAD_GATEWAY,
            "Failed to query order engine".to_string(),
        ),
    })?;

    Ok(Json(balances))
}
