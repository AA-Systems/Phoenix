use sqlx::PgPool;
use types::balances::AssetBalance;

pub async fn get_by_user_id(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<AssetBalance>, sqlx::Error> {
    sqlx::query_as::<_, AssetBalance>(include_str!("sql/list_by_user.sql"))
        .bind(user_id)
        .fetch_all(pool)
        .await
}
