use axum::extract::FromRef;
use axum_limit::{FixedWindowPolicy, LimitState, Quota};
use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::middlewares::rate_limit_key::ClientIpUri;
use crate::services::{refresh_token_service::RefreshTokenConfig, token_service::TokenService};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub admin_api_token: String,
    pub token_service: TokenService,
    pub refresh_token_config: RefreshTokenConfig,
    pub redis: ConnectionManager,
    pub order_commands_stream: String,
    pub engine_commands_stream: String,
    pub engine_queries_stream: String,
    pub engine_query_timeout_secs: f64,
    limits: LimitState<ClientIpUri, FixedWindowPolicy>,
    quotas: RateLimitQuotas,
}

#[derive(Clone)]
pub struct RateLimitQuotas {
    pub auth: Quota,
    pub health: Quota,
    pub market: Quota,
    pub asset: Quota,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        admin_api_token: String,
        token_service: TokenService,
        refresh_token_config: RefreshTokenConfig,
        redis: ConnectionManager,
        order_commands_stream: String,
        engine_commands_stream: String,
        engine_queries_stream: String,
        engine_query_timeout_secs: f64,
        quotas: RateLimitQuotas,
    ) -> Self {
        Self {
            pool,
            admin_api_token,
            token_service,
            refresh_token_config,
            redis,
            order_commands_stream,
            engine_commands_stream,
            engine_queries_stream,
            engine_query_timeout_secs,
            limits: LimitState::default(),
            quotas,
        }
    }
}

impl FromRef<AppState> for LimitState<ClientIpUri, FixedWindowPolicy> {
    fn from_ref(s: &AppState) -> Self {
        s.limits.clone()
    }
}

#[derive(Clone, Copy)]
pub struct AuthQuota(Quota);

#[derive(Clone, Copy)]
pub struct HealthQuota(Quota);

#[derive(Clone, Copy)]
pub struct MarketQuota(Quota);

#[derive(Clone, Copy)]
pub struct AssetQuota(Quota);

impl FromRef<AppState> for AuthQuota {
    fn from_ref(s: &AppState) -> Self {
        AuthQuota(s.quotas.auth)
    }
}

impl FromRef<AppState> for HealthQuota {
    fn from_ref(s: &AppState) -> Self {
        HealthQuota(s.quotas.health)
    }
}

impl FromRef<AppState> for MarketQuota {
    fn from_ref(s: &AppState) -> Self {
        MarketQuota(s.quotas.market)
    }
}

impl FromRef<AppState> for AssetQuota {
    fn from_ref(s: &AppState) -> Self {
        AssetQuota(s.quotas.asset)
    }
}

impl From<AuthQuota> for Quota {
    fn from(v: AuthQuota) -> Self {
        v.0
    }
}

impl From<HealthQuota> for Quota {
    fn from(v: HealthQuota) -> Self {
        v.0
    }
}

impl From<MarketQuota> for Quota {
    fn from(v: MarketQuota) -> Self {
        v.0
    }
}

impl From<AssetQuota> for Quota {
    fn from(v: AssetQuota) -> Self {
        v.0
    }
}
