use std::env;

pub struct Config {
    pub port: String,
    pub database_url: String,
    pub admin_api_token: String,
    pub frontend_url: String,
    pub jwt_private_key_path: String,
    pub jwt_public_key_path: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub access_token_ttl_seconds: u64,
    pub refresh_token_ttl_seconds: u64,
    pub cookie_secure: bool,
    pub redis_url: String,
    pub order_commands_stream: String,
    pub engine_commands_stream: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT").expect("port must be set"),
            database_url: env::var("DATABASE_URL").expect("database_url must be set"),
            admin_api_token: env::var("ADMIN_API_TOKEN").expect("admin_api_token must be there"),
            frontend_url: env::var("FRONTEND_URL").expect("frontend_url must be set"),
            jwt_private_key_path: env::var("JWT_PRIVATE_KEY_PATH")
                .expect("jwt_private_key_path must be set"),
            jwt_public_key_path: env::var("JWT_PUBLIC_KEY_PATH")
                .expect("jwt_public_key_path must be set"),
            jwt_issuer: env::var("JWT_ISSUER").expect("jwt_issuer must be set"),
            jwt_audience: env::var("JWT_AUDIENCE").expect("jwt_audience must be set"),
            access_token_ttl_seconds: env::var("ACCESS_TOKEN_TTL_SECONDS")
                .expect("access_token_ttl_seconds must be set")
                .parse::<u64>()
                .expect("access_token_ttl_seconds must be a number"),
            refresh_token_ttl_seconds: env::var("REFRESH_TOKEN_TTL_SECONDS")
                .expect("refresh_token_ttl_seconds must be set")
                .parse::<u64>()
                .expect("refresh_token_ttl_seconds must be a number"),
            cookie_secure: env::var("COOKIE_SECURE")
                .expect("cookie_secure must be set")
                .parse::<bool>()
                .expect("cookie_secure must be a bool"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            order_commands_stream: env::var("REDIS_ORDER_COMMANDS_STREAM")
                .unwrap_or_else(|_| "order-commands".to_string()),
            engine_commands_stream: env::var("REDIS_ENGINE_COMMANDS_STREAM")
                .unwrap_or_else(|_| "engine-commands".to_string()),
        }
    }
}
