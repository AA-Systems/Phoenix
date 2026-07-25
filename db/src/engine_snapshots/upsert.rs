use serde_json::Value;
use sqlx::PgPool;

pub async fn upsert_latest(
    pool: &PgPool,
    next_order_id: i64,
    order_commands_cursor: &str,
    engine_commands_cursor: &str,
    state: &Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(include_str!("sql/upsert.sql"))
        .bind(next_order_id)
        .bind(order_commands_cursor)
        .bind(engine_commands_cursor)
        .bind(state)
        .execute(pool)
        .await?;
    Ok(())
}
