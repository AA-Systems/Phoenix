use chrono::Utc;
use types::{
    balances::Balance,
    ledger_entries::{LedgerEntryType, LedgerIntent},
};
use uuid::Uuid;

use crate::{commands::apply_command::ApplyError, memory::OrderEngineState};

pub fn credit_balance(
    state: &mut OrderEngineState,
    command_id: Uuid,
    user_id: Uuid,
    asset_id: Uuid,
    amount: i64,
) -> Result<LedgerIntent, ApplyError> {
    if amount <= 0 {
        return Err(ApplyError::InvalidAmount);
    }

    if !state.assets.iter().any(|asset| asset.id == asset_id) {
        return Err(ApplyError::AssetNotFound);
    }

    let balance = state
        .balances
        .entry((user_id, asset_id))
        .or_insert_with(|| Balance {
            user_id,
            asset_id,
            available: 0,
            locked: 0,
            updated_at: Utc::now(),
        });

    balance.available = balance
        .available
        .checked_add(amount)
        .ok_or(ApplyError::Overflow)?;
    balance.updated_at = Utc::now();

    Ok(LedgerIntent {
        command_id,
        user_id,
        asset_id,
        entry_type: LedgerEntryType::Deposit,
        available_delta: amount,
        locked_delta: 0,
        available_after: balance.available,
        locked_after: balance.locked,
        reference_id: Some(command_id),
        reference_type: Some("command".into()),
    })
}
