use api::{
    app_state::{AppState, RateLimitQuotas},
    build_app,
    services::{refresh_token_service::RefreshTokenConfig, token_service::TokenService},
};
use axum::http::{
    Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use axum_limit::Quota;
use common::config::Config;
use dotenv::dotenv;
use redis::Client;
use sqlx::postgres::PgPoolOptions;
use std::{fs, time::Duration};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
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

    let redis_client = Client::open(config.redis_url.as_str()).expect("invalid REDIS_URL");
    let redis = redis::aio::ConnectionManager::new(redis_client)
        .await
        .expect("cannot connect to Redis");

    let private_key_pem =
        fs::read(&config.jwt_private_key_path).expect("cannot read JWT private key");
    let public_key_pem = fs::read(&config.jwt_public_key_path).expect("cannot read JWT public key");
    let token_service = TokenService::new(
        &private_key_pem,
        &public_key_pem,
        config.jwt_issuer,
        config.jwt_audience,
        config.access_token_ttl_seconds,
    )
    .expect("cannot initialize token service");

    let refresh_token_config = RefreshTokenConfig {
        refresh_token_ttl_seconds: config.refresh_token_ttl_seconds,
        cookie_secure: config.cookie_secure,
    };

    let app_state = AppState::new(
        pool,
        config.admin_api_token,
        token_service,
        refresh_token_config,
        redis,
        config.order_commands_stream,
        config.engine_commands_stream,
        config.engine_queries_stream,
        config.engine_query_timeout_secs,
        RateLimitQuotas {
            auth: Quota::per_minute(10),
            health: Quota::per_minute(5),
            market: Quota::per_minute(5),
            asset: Quota::per_minute(5),
            order: Quota::per_minute(30),
        },
    );

    // Initialize router
    let app = build_app(app_state).layer(ServiceBuilder::new().layer(cors_layer));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    // Start server
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
