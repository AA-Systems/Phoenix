use sqlx::PgPool;

use crate::services::token_service::TokenService;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub admin_api_token: String,
    pub token_service: TokenService,
}
