use axum::{Router, routing::get};

use crate::{app_state::AppState, handlers::health::health_check};

pub fn health_router() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}
