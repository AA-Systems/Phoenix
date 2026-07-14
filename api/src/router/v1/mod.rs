use axum::Router;
use sqlx::PgPool;

use crate::router::v1::health::health_router;

pub mod health;

pub fn v1_router() -> Router<PgPool> {
    Router::new().nest("/health", health_router())
}
