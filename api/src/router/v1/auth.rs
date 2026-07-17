use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    app_state::AppState,
    handlers::auth::{
        login::login_user, logout::logout, me::me, refresh::refresh_token, register::register_user,
    },
    middlewares::jwt_middleware::jwt_middleware,
};

pub fn auth_router(app_state: AppState) -> Router<AppState> {
    let protected =
        Router::new()
            .route("/me", get(me))
            .route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                jwt_middleware,
            ));

    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
        .merge(protected)
}
