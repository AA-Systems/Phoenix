use axum::Router;

use crate::{
    app_state::AppState,
    router::v1::{
        assets::assets_admin_router, auth::auth_router, balances::balances_router,
        health::health_router, markets::markets_router, orders::orders_router,
    },
};

pub mod assets;
pub mod auth;
pub mod balances;
pub mod health;
pub mod markets;
pub mod orders;

pub fn v1_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/health", health_router())
        .nest("/auth", auth_router(app_state.clone()))
        .nest("/assets", assets_admin_router(app_state.clone()))
        .nest("/markets", markets_router(app_state.clone()))
        .nest("/balances", balances_router(app_state.clone()))
        .nest("/orders", orders_router(app_state))
}
