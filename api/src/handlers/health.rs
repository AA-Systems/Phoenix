use axum::extract::State;
use axum_limit::DynamicFixedWindowLimit;

use crate::app_state::{AppState, HealthQuota};
use crate::middlewares::rate_limit_key::ClientIpUri;

pub async fn health_check(
    _: DynamicFixedWindowLimit<ClientIpUri, HealthQuota>,
    State(_app_state): State<AppState>,
) -> String {
    String::from("Server is healthy")
}
