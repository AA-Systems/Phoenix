use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    app_state::AppState,
    handlers::assets::{insert_asset::insert_asset, list_assets::list_assets},
    middlewares::admin_auth::admin_auth,
};

pub fn assets_router(app_state: AppState) -> Router<AppState> {
    Router::new().route("/", get(list_assets)).merge(
        Router::new()
            .route("/admin/insert", post(insert_asset))
            .layer(middleware::from_fn_with_state(
                app_state.clone(),
                admin_auth,
            )),
    )
}
