use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};

const REFRESH_TOKEN_BYTES: usize = 32;

pub struct GeneratedRefreshToken {
    pub raw_token: String,
    pub token_hash: String,
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
}
