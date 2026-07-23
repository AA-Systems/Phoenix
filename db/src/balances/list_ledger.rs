use sqlx::PgPool;
use types::ledger_entries::LedgerEntryView;
use uuid::Uuid;

pub async fn list_by_user(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
) -> Result<Vec<LedgerEntryView>, sqlx::Error> {
    sqlx::query_as::<_, LedgerEntryView>(include_str!("sql/list_ledger_by_user.sql"))
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn list_command_ids(pool: &PgPool) -> Result<Vec<Uuid>, sqlx::Error> {
    let rows = sqlx::query_scalar::<_, Uuid>(include_str!("sql/list_ledger_command_ids.sql"))
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
