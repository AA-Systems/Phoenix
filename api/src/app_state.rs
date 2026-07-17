use sqlx::PgPool;

use crate::services::{refresh_token_service::RefreshTokenConfig, token_service::TokenService};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub admin_api_token: String,
    pub token_service: TokenService,
    pub refresh_token_config: RefreshTokenConfig,
}
