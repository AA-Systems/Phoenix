use axum::{Router, middleware, routing::post};

use crate::{
    app_state::AppState, handlers::markets::insert_markets::insert_market,
    middlewares::admin_auth::admin_auth,
};

pub fn markets_admin_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/admin/insert", post(insert_market))
        .layer(middleware::from_fn_with_state(app_state, admin_auth))
}
