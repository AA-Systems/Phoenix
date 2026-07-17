use sqlx::{Executor, Postgres};
use types::auth::User;
use uuid::Uuid;

pub async fn find_by_id<'e, E>(executor: E, user_id: Uuid) -> Result<User, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>,
{
    sqlx::query_as::<_, User>(include_str!("sql/find_by_id.sql"))
        .bind(user_id)
        .fetch_one(executor)
        .await
}
