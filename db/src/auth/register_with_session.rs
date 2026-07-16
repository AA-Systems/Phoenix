use chrono::{DateTime, Utc};
use sqlx::PgPool;
use types::{auth::User, sessions::Session};

use crate::{sessions::insert::insert, users::register::register};

pub struct RegistrationWithSession<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub refresh_token_hash: &'a str,
    pub session_expires_at: DateTime<Utc>,
    pub user_agent: Option<&'a str>,
    pub ip_address: Option<&'a str>,
}

pub async fn register_with_session(
    pool: &PgPool,
    registration: RegistrationWithSession<'_>,
) -> Result<(User, Session), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    let user = register(
        &mut *transaction,
        registration.name,
        registration.email,
        registration.password_hash,
    )
    .await?;

    let session = insert(
        &mut *transaction,
        user.id,
        registration.refresh_token_hash,
        registration.session_expires_at,
        registration.user_agent,
        registration.ip_address,
    )
    .await?;

    transaction.commit().await?;

    Ok((user, session))
}
