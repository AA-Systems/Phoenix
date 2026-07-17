use sqlx::PgPool;
use types::balances::Balance;
use types::ledger_entries::{LedgerEntry, LedgerEntryType};
use uuid::Uuid;

pub struct CreditBalance {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub amount: i64,
}

pub async fn credit(
    pool: &PgPool,
    input: CreditBalance,
) -> Result<(Balance, LedgerEntry), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    let balance = sqlx::query_as::<_, Balance>(include_str!("sql/credit.sql"))
        .bind(input.user_id)
        .bind(input.asset_id)
        .bind(input.amount)
        .fetch_one(&mut *transaction)
        .await?;

    let ledger_entry = sqlx::query_as::<_, LedgerEntry>(include_str!("sql/insert_ledger_entry.sql"))
        .bind(input.user_id)
        .bind(input.asset_id)
        .bind(LedgerEntryType::Deposit)
        .bind(input.amount)
        .bind(0_i64)
        .bind(balance.available)
        .bind(balance.locked)
        .bind(None::<Uuid>)
        .bind(None::<String>)
        .fetch_one(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok((balance, ledger_entry))
}
