use axum::{Json, extract::State, http::StatusCode};
use axum_limit::DynamicFixedWindowLimit;
use types::trade::TradeView;
use types::trade::get_recent_trades_request::GetRecentTradesRequest;
use validator::Validate;

use crate::{
    app_state::{AppState, MarketQuota},
    middlewares::rate_limit_key::ClientIpUri,
    services::engine_query::{EngineQueryError, get_recent_trades},
};

pub async fn get_recent_trades_for_market(
    _: DynamicFixedWindowLimit<ClientIpUri, MarketQuota>,
    State(app_state): State<AppState>,
    Json(mut body): Json<GetRecentTradesRequest>,
) -> Result<Json<Vec<TradeView>>, (StatusCode, String)> {
    body.market_symbol = body.market_symbol.trim().to_uppercase();
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let mut redis = app_state.redis.clone();
    let trades = get_recent_trades(
        &mut redis,
        &app_state.engine_queries_stream,
        body.market_symbol,
        body.limit,
        app_state.engine_query_timeout_secs,
    )
    .await
    .map_err(|error| match error {
        EngineQueryError::NotFound => (StatusCode::NOT_FOUND, "Market not found".to_string()),
        EngineQueryError::Timeout => (
            StatusCode::GATEWAY_TIMEOUT,
            "Engine did not respond in time".to_string(),
        ),
        EngineQueryError::Enqueue | EngineQueryError::InvalidReply => (
            StatusCode::BAD_GATEWAY,
            "Failed to query order engine".to_string(),
        ),
    })?;

    Ok(Json(trades))
}
