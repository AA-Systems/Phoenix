use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
    errors::Result as JwtResult, get_current_timestamp,
};
use types::auth::access_token_claims::AccessTokenClaims;
use uuid::Uuid;

#[derive(Clone)]
pub struct TokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: String,
    audience: String,
    access_token_ttl_seconds: u64,
}

impl TokenService {
    pub fn new(
        private_key_pem: &[u8],
        public_key_pem: &[u8],
        issuer: String,
        audience: String,
        access_token_ttl_seconds: u64,
    ) -> JwtResult<Self> {
        Ok(Self {
            encoding_key: EncodingKey::from_ed_pem(private_key_pem)?,
            decoding_key: DecodingKey::from_ed_pem(public_key_pem)?,
            issuer,
            audience,
            access_token_ttl_seconds,
        })
    }

    pub fn issue_access_token(&self, user_id: Uuid, session_id: Uuid) -> JwtResult<String> {
        let issued_at = get_current_timestamp();
        let expires_at = issued_at.saturating_add(self.access_token_ttl_seconds);

        let claims = AccessTokenClaims {
            sub: user_id,
            sid: session_id,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            iat: issued_at as i64,
            exp: expires_at as i64,
        };

        encode(&Header::new(Algorithm::EdDSA), &claims, &self.encoding_key)
    }

    pub fn verify_access_token(&self, token: &str) -> JwtResult<AccessTokenClaims> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_issuer(&[self.issuer.as_str()]);
        validation.set_audience(&[self.audience.as_str()]);

        decode::<AccessTokenClaims>(token, &self.decoding_key, &validation)
            .map(|token_data| token_data.claims)
    }

    pub fn access_token_ttl_seconds(&self) -> u64 {
        self.access_token_ttl_seconds
    }
}
