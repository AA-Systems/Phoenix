use axum::Router;

use crate::{
    app_state::AppState,
    router::v1::{
        assets::assets_admin_router, health::health_router, markets::markets_admin_router,
    },
};

pub mod assets;
pub mod health;
pub mod markets;

pub fn v1_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/health", health_router())
        .nest("/assets", assets_admin_router(app_state.clone()))
        .nest("/markets", markets_admin_router(app_state.clone()))
}
