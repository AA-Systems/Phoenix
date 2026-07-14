use axum::{Router, middleware, routing::post};

use crate::{
    app_state::AppState, handlers::assets::insert_asset::insert_asset,
    middlewares::admin_auth::admin_auth,
};

pub fn assets_admin_router(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/admin/insert", post(insert_asset))
        .layer(middleware::from_fn_with_state(app_state, admin_auth))
}
