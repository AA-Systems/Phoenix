use types::{command::Command, ledger_entries::LedgerIntent};
use uuid::Uuid;

use crate::{
    commands::{
        cancel_order::cancel_order, create_order::create_order, credit_balance::credit_balance,
    },
    memory::OrderEngineState,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ApplyError {
    MarketNotFound,
    MarketNotTrading,
    InvalidPrice,
    InvalidQuantity,
    InvalidAmount,
    InvalidTickSize,
    InvalidQuantityStep,
    BelowMinQuantity,
    BelowMinNotional,
    AssetNotFound,
    Overflow,
    InsufficientBalance,
    OrderNotFound,
    Unauthorized,
    OrderNotCancellable,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ApplyOutcome {
    AlreadyProcessed,
    Applied {
        command_id: Uuid,
        intents: Vec<LedgerIntent>,
    },
}

/// Apply command effects without marking `command_id` processed.
/// Caller should persist intents, then call [`commit_command`].
pub fn apply_command_effects(
    state: &mut OrderEngineState,
    command: Command,
) -> Result<ApplyOutcome, ApplyError> {
    let command_id = command.command_id();
    if state.processed_commands.contains(&command_id) {
        return Ok(ApplyOutcome::AlreadyProcessed);
    }

    let intents = match command {
        Command::CreateOrder {
            user_id,
            market_symbol,
            order_type,
            price,
            quantity,
            ..
        } => {
            create_order(state, user_id, market_symbol, order_type, price, quantity)?;
            Vec::new()
        }
        Command::CancelOrder {
            user_id, order_id, ..
        } => {
            cancel_order(state, user_id, order_id)?;
            Vec::new()
        }
        Command::CreditBalance {
            command_id,
            user_id,
            asset_id,
            amount,
        } => {
            let intent = credit_balance(state, command_id, user_id, asset_id, amount)?;
            vec![intent]
        }
    };

    Ok(ApplyOutcome::Applied {
        command_id,
        intents,
    })
}

pub fn commit_command(state: &mut OrderEngineState, command_id: Uuid) {
    state.processed_commands.insert(command_id);
}

/// Apply + mark processed (unit tests / callers that do not persist).
pub fn apply_command(state: &mut OrderEngineState, command: Command) -> Result<(), ApplyError> {
    match apply_command_effects(state, command)? {
        ApplyOutcome::AlreadyProcessed => Ok(()),
        ApplyOutcome::Applied { command_id, .. } => {
            commit_command(state, command_id);
            Ok(())
        }
    }
}

pub fn revert_intents(state: &mut OrderEngineState, intents: &[LedgerIntent]) {
    for intent in intents {
        let Some(balance) = state.balances.get_mut(&(intent.user_id, intent.asset_id)) else {
            continue;
        };
        balance.available -= intent.available_delta;
        balance.locked -= intent.locked_delta;
    }
}
