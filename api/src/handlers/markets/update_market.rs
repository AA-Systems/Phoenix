use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_limit::DynamicFixedWindowLimit;
use db::markets::{
    get::get_by_id,
    update::{update_config, update_status},
};
use types::markets::{
    Market, MarketStatus, update_market_config_request::UpdateMarketConfigRequest,
    update_market_status_request::UpdateMarketStatusRequest,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::{AppState, MarketQuota},
    middlewares::rate_limit_key::ClientIpUri,
};

pub async fn set_market_status(
    _: DynamicFixedWindowLimit<ClientIpUri, MarketQuota>,
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    body: Json<UpdateMarketStatusRequest>,
) -> Result<Json<Market>, (StatusCode, String)> {
    let market = find_market(&app_state, id).await?;

    if !valid_transition(&market.status, &body.status) {
        return Err((
            StatusCode::CONFLICT,
            "Invalid market status transition".to_string(),
        ));
    }

    update_status(&app_state.pool, id, body.status.clone())
        .await
        .map(Json)
        .map_err(internal_error)
}

pub async fn set_market_config(
    _: DynamicFixedWindowLimit<ClientIpUri, MarketQuota>,
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    body: Json<UpdateMarketConfigRequest>,
) -> Result<Json<Market>, (StatusCode, String)> {
    body.validate()
        .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?;

    let market = find_market(&app_state, id).await?;
    if market.status != MarketStatus::Halted {
        return Err((
            StatusCode::CONFLICT,
            "Market must be halted before changing configuration".to_string(),
        ));
    }

    update_config(
        &app_state.pool,
        id,
        body.price_tick_size,
        body.quantity_step_size,
        body.min_order_quantity,
        body.min_order_notional,
    )
    .await
    .map(Json)
    .map_err(|error| match error {
        sqlx::Error::RowNotFound => (
            StatusCode::CONFLICT,
            "Market must remain halted while changing configuration".to_string(),
        ),
        _ => internal_error(error),
    })
}

async fn find_market(app_state: &AppState, id: Uuid) -> Result<Market, (StatusCode, String)> {
    get_by_id(&app_state.pool, id)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Market not found".to_string()),
            _ => internal_error(error),
        })
}

fn valid_transition(current: &MarketStatus, target: &MarketStatus) -> bool {
    current == target
        || matches!(
            (current, target),
            (MarketStatus::Trading, MarketStatus::Halted)
                | (MarketStatus::Halted, MarketStatus::Trading)
                | (MarketStatus::Halted, MarketStatus::Archived)
        )
}

fn internal_error(_: sqlx::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal server error".to_string(),
    )
}
