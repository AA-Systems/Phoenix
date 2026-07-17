use axum::{Router, routing::post};

use crate::{
    app_state::AppState,
    handlers::auth::{login::login_user, refresh::refresh_token, register::register_user},
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/refresh", post(refresh_token))
}
