use axum::{Json, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::auth::User;

#[derive(Serialize)]
pub struct AuthResponse {
    #[serde(skip)]
    pub status_code: StatusCode,
    #[serde(skip)]
    pub jar: CookieJar,
    pub user: User,
    pub access_token: String,
    pub expires_in: u32,
}

impl AuthResponse {
    pub fn created(jar: CookieJar, user: User, access_token: String, expires_in: u32) -> Self {
        Self {
            status_code: StatusCode::CREATED,
            jar,
            user,
            access_token,
            expires_in,
        }
    }
    pub fn ok(jar: CookieJar, user: User, access_token: String, expires_in: u32) -> Self {
        Self {
            status_code: StatusCode::OK,
            jar,
            user,
            access_token,
            expires_in,
        }
    }
}

impl IntoResponse for AuthResponse {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code;
        let jar = self.jar.clone();
        (status, jar, Json(self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthBody {
    pub user: User,
    pub access_token: String,
    pub expires_in: u64,
}
