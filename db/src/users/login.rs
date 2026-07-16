use sqlx::{Executor, Postgres};
use types::auth::UserCredentials;

pub async fn find_by_email<'e, E>(executor: E, email: &str) -> Result<UserCredentials, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, UserCredentials>(include_str!("sql/find_user.sql"))
        .bind(email)
        .fetch_one(executor)
        .await
}
