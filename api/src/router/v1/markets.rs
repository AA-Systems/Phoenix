use axum::{
    Router, middleware,
    routing::{get, patch, post},
};

use crate::{
    app_state::AppState,
    handlers::markets::{
        get_candles::get_candles_for_market,
        get_markets::{get_market, list_markets},
        get_order_book::get_order_book_depth,
        get_recent_trades::get_recent_trades_for_market,
        insert_markets::insert_market,
        update_market::{set_market_config, set_market_status},
    },
    middlewares::admin_auth::admin_auth,
};

pub fn markets_router(app_state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/", get(list_markets))
        .route("/book", post(get_order_book_depth))
        .route("/trades", post(get_recent_trades_for_market))
        .route("/candles", post(get_candles_for_market))
        .route("/{base}/{quote}", get(get_market));

    let admin_routes = Router::new()
        .route("/admin/insert", post(insert_market))
        .route("/admin/{id}/status", patch(set_market_status))
        .route("/admin/{id}/config", patch(set_market_config))
        .layer(middleware::from_fn_with_state(app_state, admin_auth));

    Router::new().merge(public_routes).merge(admin_routes)
}
