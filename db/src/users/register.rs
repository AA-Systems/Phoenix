use sqlx::{Executor, Postgres};
use types::auth::User;

pub async fn register<'e, E>(
    executor: E,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, User>(include_str!("sql/register.sql"))
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .fetch_one(executor)
        .await
}
