use sqlx::PgPool;
use types::balances::Balance;
use types::ledger_entries::LedgerIntent;

pub async fn persist_intents(pool: &PgPool, intents: &[LedgerIntent]) -> Result<(), sqlx::Error> {
    if intents.is_empty() {
        return Ok(());
    }

    let mut transaction = pool.begin().await?;

    for intent in intents {
        let _balance = sqlx::query_as::<_, Balance>(include_str!("sql/set_balance.sql"))
            .bind(intent.user_id)
            .bind(intent.asset_id)
            .bind(intent.available_after)
            .bind(intent.locked_after)
            .fetch_one(&mut *transaction)
            .await?;

        let _ = sqlx::query(include_str!("sql/insert_ledger_intent.sql"))
            .bind(intent.user_id)
            .bind(intent.asset_id)
            .bind(intent.entry_type.clone())
            .bind(intent.available_delta)
            .bind(intent.locked_delta)
            .bind(intent.available_after)
            .bind(intent.locked_after)
            .bind(intent.reference_id)
            .bind(intent.reference_type.as_deref())
            .bind(intent.command_id)
            .bind(intent.sequence)
            .execute(&mut *transaction)
            .await?;
    }

    transaction.commit().await?;
    Ok(())
}
