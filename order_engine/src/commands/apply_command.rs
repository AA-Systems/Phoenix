use types::command::Command;

use crate::{
    commands::{cancel_order::cancel_order, create_order::create_order},
    memory::OrderEngineState,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ApplyError {
    MarketNotFound,
    MarketNotTrading,
    InvalidPrice,
    InvalidQuantity,
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

pub fn apply_command(state: &mut OrderEngineState, command: Command) -> Result<(), ApplyError> {
    match command {
        Command::CreateOrder(user_id, market_symbol, order_type, price, quantity) => {
            create_order(state, user_id, market_symbol, order_type, price, quantity)
        }
        Command::CancelOrder(user_id, order_id) => cancel_order(state, user_id, order_id),
    }
}
