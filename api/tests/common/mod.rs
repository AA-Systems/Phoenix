pub mod assets;
pub mod markets;

pub use assets::insert_asset_req::insert_asset_req;
pub use markets::insert_market_req::insert_market_req;

use api::app_state::AppState;
use sqlx::postgres::PgPoolOptions;

pub const ADMIN_TOKEN: &str = "test-token";
pub const TEST_DATABASE_URL: &str = "postgres://admin:supersecretpassword@localhost:5433/cex_test";

pub async fn test_state() -> AppState {
    let pool = PgPoolOptions::new()
        .connect(TEST_DATABASE_URL)
        .await
        .unwrap();

    sqlx::query("TRUNCATE assets CASCADE")
        .execute(&pool)
        .await
        .unwrap();

    AppState {
        pool,
        admin_api_token: ADMIN_TOKEN.into(),
    }
}
