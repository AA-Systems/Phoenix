use axum::{Json, extract::State, http::StatusCode};
use axum_limit::DynamicFixedWindowLimit;
use types::orderbook::OrderBookDepth;
use types::orderbook::get_order_book_request::GetOrderBookRequest;
use validator::Validate;

use crate::{
    app_state::{AppState, MarketQuota},
    middlewares::rate_limit_key::ClientIpUri,
    services::engine_query::{EngineQueryError, get_order_book},
};

pub async fn get_order_book_depth(
    _: DynamicFixedWindowLimit<ClientIpUri, MarketQuota>,
    State(app_state): State<AppState>,
    Json(mut body): Json<GetOrderBookRequest>,
) -> Result<Json<OrderBookDepth>, (StatusCode, String)> {
    body.market_symbol = body.market_symbol.trim().to_uppercase();
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let mut redis = app_state.redis.clone();
    let book = get_order_book(
        &mut redis,
        &app_state.engine_queries_stream,
        body.market_symbol,
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

    Ok(Json(book))
}
