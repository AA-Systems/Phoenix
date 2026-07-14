use std::env;

pub struct Config {
    pub port: String,
    pub database_url: String,
    pub admin_api_token: String,
    pub frontend_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT").expect("port must be set"),
            database_url: env::var("DATABASE_URL").expect("database_url must be set"),
            admin_api_token: env::var("ADMIN_API_TOKEN").expect("admin_api_token must be there"),
            frontend_url: env::var("FRONTEND_URL").expect("frontend_url must be set"),
        }
    }
}
