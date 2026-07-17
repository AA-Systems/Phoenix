use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};
use time::Duration;

const REFRESH_TOKEN_BYTES: usize = 32;

pub struct GeneratedRefreshToken {
    pub raw_token: String,
    pub token_hash: String,
}

#[derive(Clone)]
pub struct RefreshTokenConfig {
    pub refresh_token_ttl_seconds: u64,
    pub cookie_secure: bool,
}

pub struct RefreshTokenService;

impl RefreshTokenService {
    pub fn generate() -> GeneratedRefreshToken {
        let mut random_bytes = [0_u8; REFRESH_TOKEN_BYTES];
        OsRng.fill_bytes(&mut random_bytes);

        let raw_token = URL_SAFE_NO_PAD.encode(random_bytes);
        let token_hash = Self::hash(&raw_token);

        GeneratedRefreshToken {
            raw_token,
            token_hash,
        }
    }

    pub fn hash(raw_token: &str) -> String {
        let digest = Sha256::digest(raw_token.as_bytes());
        hex::encode(digest)
    }

    pub fn build_refresh_token(
        raw_token: String,
        ttl_seconds: u64,
        secure: bool,
    ) -> Cookie<'static> {
        Cookie::build(("refresh_token", raw_token))
            .http_only(true)
            .secure(secure)
            .same_site(SameSite::Lax)
            .path("/api/v1/auth")
            .max_age(Duration::seconds(
                ttl_seconds
                    .try_into()
                    .expect("refresh token TTL is too large"),
            ))
            .build()
    }

    pub fn clear_refresh_token(secure: bool) -> Cookie<'static> {
        Cookie::build(("refresh_token", ""))
            .http_only(true)
            .secure(secure)
            .same_site(SameSite::Lax)
            .path("/api/v1/auth")
            .max_age(Duration::seconds(0))
            .build()
    }
}
