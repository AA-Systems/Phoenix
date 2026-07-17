pub mod assets;
pub mod balances;
pub mod markets;
pub mod users;

use std::{fs, path::Path};

pub use assets::insert_asset_req::insert_asset_req;
pub use balances::credit_balance_req::credit_balance_req;
pub use markets::insert_market_req::insert_market_req;

use api::{
    app_state::AppState,
    services::{refresh_token_service::RefreshTokenConfig, token_service::TokenService},
};
use axum_limit::Quota;
use sqlx::postgres::PgPoolOptions;

pub const ADMIN_TOKEN: &str = "test-token";
pub const TEST_DATABASE_URL: &str = "postgres://admin:supersecretpassword@localhost:5433/cex_test";
pub const TEST_JWT_ISSUER: &str = "centralized-exchange-test";
pub const TEST_JWT_AUDIENCE: &str = "exchange-api-test";
pub const TEST_ACCESS_TOKEN_TTL_SECONDS: u64 = 900;

pub async fn test_state() -> AppState {
    let pool = PgPoolOptions::new()
        .connect(TEST_DATABASE_URL)
        .await
        .unwrap();

    sqlx::query("TRUNCATE assets CASCADE")
        .execute(&pool)
        .await
        .unwrap();

    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let private_key_pem = fs::read(repository_root.join("secrets/jwt-private.pem"))
        .expect("cannot read test JWT private key");
    let public_key_pem = fs::read(repository_root.join("secrets/jwt-public.pem"))
        .expect("cannot read test JWT public key");
    let token_service = TokenService::new(
        &private_key_pem,
        &public_key_pem,
        TEST_JWT_ISSUER.to_string(),
        TEST_JWT_AUDIENCE.to_string(),
        TEST_ACCESS_TOKEN_TTL_SECONDS,
    )
    .expect("cannot initialize token service");

    let refresh_token_config = RefreshTokenConfig {
        refresh_token_ttl_seconds: 2592000,
        cookie_secure: false,
    };

    AppState::new(
        pool,
        ADMIN_TOKEN.into(),
        token_service,
        refresh_token_config,
        // High limits so integration tests do not trip rate limiting.
        Quota::per_second(10_000),
        Quota::per_second(10_000),
    )
}
