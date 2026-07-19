use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use axum_limit::DynamicFixedWindowLimit;
use db::markets::get::{get_by_symbol, list};
use types::markets::{Market, list_markets_query::ListMarketsQuery};
use validator::Validate;

use crate::{
    app_state::{AppState, MarketQuota},
    middlewares::rate_limit_key::ClientIpUri,
};

pub async fn list_markets(
    _: DynamicFixedWindowLimit<ClientIpUri, MarketQuota>,
    State(app_state): State<AppState>,
    Query(query): Query<ListMarketsQuery>,
) -> Result<Json<Vec<Market>>, (StatusCode, String)> {
    query
        .validate()
        .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?;

    list(&app_state.pool, query.limit, query.skip)
        .await
        .map(Json)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        })
}

pub async fn get_market(
    _: DynamicFixedWindowLimit<ClientIpUri, MarketQuota>,
    State(app_state): State<AppState>,
    axum::extract::Path((base, quote)): axum::extract::Path<(String, String)>,
) -> Result<Json<Market>, (StatusCode, String)> {
    let symbol = format!("{}/{}", base.trim(), quote.trim()).to_uppercase();

    get_by_symbol(&app_state.pool, &symbol)
        .await
        .map(Json)
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Market not found".to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        })
}
