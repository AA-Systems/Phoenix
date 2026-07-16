use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub sub: uuid::Uuid, // user ID
    pub sid: uuid::Uuid, // session ID
    pub iss: String,
    pub aud: String,
    pub iat: i64,
    pub exp: i64,
}
