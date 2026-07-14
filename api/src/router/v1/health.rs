use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::health::health_check;

pub fn health_router() -> Router<PgPool> {
    Router::new().route("/", get(health_check))
}
