pub mod app_state;
pub mod handlers;
pub mod middlewares;
pub mod router;
pub mod services;

use crate::app_state::AppState;
use crate::router::v1::v1_router;
use axum::Router;

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", v1_router(state.clone()))
        .with_state(state)
}
