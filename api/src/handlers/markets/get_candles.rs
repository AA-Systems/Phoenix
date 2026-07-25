use axum::{Json, extract::State, http::StatusCode};
use axum_limit::DynamicFixedWindowLimit;
use types::candle::get_candles_request::GetCandlesRequest;
use types::candle::{Candle, is_valid_interval};
use validator::Validate;

use crate::{
    app_state::{AppState, MarketQuota},
    middlewares::rate_limit_key::ClientIpUri,
};

pub async fn get_candles_for_market(
    _: DynamicFixedWindowLimit<ClientIpUri, MarketQuota>,
    State(app_state): State<AppState>,
    Json(mut body): Json<GetCandlesRequest>,
) -> Result<Json<Vec<Candle>>, (StatusCode, String)> {
    body.market_symbol = body.market_symbol.trim().to_uppercase();
    body.interval = body.interval.trim().to_lowercase();
    body.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    if !is_valid_interval(&body.interval) {
        return Err((
            StatusCode::BAD_REQUEST,
            "interval must be one of: 1m, 5m, 15m, 1h".to_string(),
        ));
    }

    match db::markets::get::get_by_symbol(&app_state.pool, &body.market_symbol).await {
        Ok(_) => {}
        Err(sqlx::Error::RowNotFound) => {
            return Err((StatusCode::NOT_FOUND, "Market not found".to_string()));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to look up market".to_string(),
            ));
        }
    }

    let mut candles = db::candles::list_candles(
        &app_state.pool,
        &body.market_symbol,
        &body.interval,
        body.limit as i64,
    )
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to load candles".to_string(),
        )
    })?;

    candles.reverse();
    Ok(Json(candles))
}
