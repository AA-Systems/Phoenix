use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod access_token_claims;
pub mod auth_response;
pub mod login_user_request;
pub mod register_user_request;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct UserCredentials {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl UserCredentials {
    pub fn into_user(self) -> User {
        User {
            id: self.id,
            name: self.name,
            email: self.email,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
