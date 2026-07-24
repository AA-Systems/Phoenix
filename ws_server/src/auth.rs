use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use types::auth::access_token_claims::AccessTokenClaims;
use uuid::Uuid;

#[derive(Clone)]
pub struct JwtVerifier {
    decoding_key: DecodingKey,
    issuer: String,
    audience: String,
}

impl JwtVerifier {
    pub fn from_pem(
        public_key_pem: &[u8],
        issuer: String,
        audience: String,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        Ok(Self {
            decoding_key: DecodingKey::from_ed_pem(public_key_pem)?,
            issuer,
            audience,
        })
    }

    pub fn verify(&self, token: &str) -> Result<Uuid, String> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_issuer(&[self.issuer.as_str()]);
        validation.set_audience(&[self.audience.as_str()]);

        decode::<AccessTokenClaims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims.sub)
            .map_err(|err| err.to_string())
    }
}
