mod handlers;
mod router;

use axum::{Router, http::Method};
use common::config::Config;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::router::v1::v1_router;

#[tokio::main]
async fn main() {
    // Initialize environment variables
    dotenv().ok();
    let config = Config::from_env();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    // Initialize cors layer
    let frontend_url = config.frontend_url;
    let origins = [frontend_url.parse().unwrap()];
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_origin(origins);

    // Initialize db connection
    let database_url = config.database_url;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("Cannot connect to database");

    // Initialize router
    let app = Router::new()
        .nest("/api/v1", v1_router())
        .layer(ServiceBuilder::new().layer(cors_layer))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    // Start server
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await.unwrap();
}
