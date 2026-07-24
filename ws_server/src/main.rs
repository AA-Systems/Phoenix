use std::env;
use std::fs;

use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use redis::Client;
use redis::aio::ConnectionManager;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::auth::JwtVerifier;
use crate::hub::Hub;
use crate::redis_consumer::run_redis_consumer;
use crate::ws_handler::{WsState, ws_handler};

mod auth;
mod hub;
mod protocol;
mod redis_consumer;
mod ws_handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("WS_PORT").unwrap_or_else(|_| "3002".to_string());
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let events_stream =
        env::var("REDIS_EXCHANGE_EVENTS_STREAM").unwrap_or_else(|_| "exchange-events".to_string());
    let public_key_path = env::var("JWT_PUBLIC_KEY_PATH").expect("JWT_PUBLIC_KEY_PATH must be set");
    let issuer = env::var("JWT_ISSUER").expect("JWT_ISSUER must be set");
    let audience = env::var("JWT_AUDIENCE").expect("JWT_AUDIENCE must be set");
    let frontend_url = env::var("FRONTEND_URL").ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    let public_key_pem = fs::read(&public_key_path).expect("cannot read JWT public key");
    let jwt =
        JwtVerifier::from_pem(&public_key_pem, issuer, audience).expect("invalid JWT public key");

    let hub = Hub::new();
    let client = Client::open(redis_url.as_str()).expect("invalid REDIS_URL");
    let redis_conn = ConnectionManager::new(client)
        .await
        .expect("failed to connect to Redis");

    let consumer_hub = hub.clone();
    tokio::spawn(async move {
        run_redis_consumer(redis_conn, events_stream, consumer_hub).await;
    });

    let cors = match frontend_url {
        Some(origin) => CorsLayer::new()
            .allow_origin(
                origin
                    .parse::<axum::http::HeaderValue>()
                    .expect("FRONTEND_URL must be a valid origin"),
            )
            .allow_methods(Any)
            .allow_headers(Any),
        None => CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    };

    let state = WsState { hub, jwt };
    let router = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/ws", get(ws_handler))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, router).await;
}
