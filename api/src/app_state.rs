use axum::extract::FromRef;
use axum_limit::{FixedWindowPolicy, LimitState, Quota};
use sqlx::PgPool;

use crate::middlewares::rate_limit_key::ClientIpUri;
use crate::services::{refresh_token_service::RefreshTokenConfig, token_service::TokenService};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub admin_api_token: String,
    pub token_service: TokenService,
    pub refresh_token_config: RefreshTokenConfig,
    limits: LimitState<ClientIpUri, FixedWindowPolicy>,
    auth_quota: Quota,
    health_quota: Quota,
}

impl AppState {
    pub fn new(
        pool: PgPool,
        admin_api_token: String,
        token_service: TokenService,
        refresh_token_config: RefreshTokenConfig,
        auth_quota: Quota,
        health_quota: Quota,
    ) -> Self {
        Self {
            pool,
            admin_api_token,
            token_service,
            refresh_token_config,
            limits: LimitState::default(),
            auth_quota,
            health_quota,
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

impl FromRef<AppState> for AuthQuota {
    fn from_ref(s: &AppState) -> Self {
        AuthQuota(s.auth_quota)
    }
}

impl FromRef<AppState> for HealthQuota {
    fn from_ref(s: &AppState) -> Self {
        HealthQuota(s.health_quota)
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
