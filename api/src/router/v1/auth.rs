use axum::{Router, routing::post};

use crate::{
    app_state::AppState,
    handlers::auth::{login::login_user, register::register_user},
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}
