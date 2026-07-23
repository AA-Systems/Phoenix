use axum::{Router, middleware, routing::post};

use crate::{
    app_state::AppState,
    handlers::orders::{
        cancel_order::cancel_order, create_order::create_order, list_open_orders::list_open_orders,
    },
    middlewares::jwt_middleware::jwt_middleware,
};

pub fn orders_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/create", post(create_order))
        .route("/cancel", post(cancel_order))
        .route("/open", post(list_open_orders))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_middleware,
        ))
}
