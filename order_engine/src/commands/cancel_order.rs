use chrono::Utc;
use types::{
    ledger_entries::{LedgerEntryType, LedgerIntent},
    order::{OrderStatus, OrderType},
};
use uuid::Uuid;

use crate::{
    commands::apply_command::ApplyError,
    helper::{quote_notional::quote_notional, remove_from_book::remove_from_book},
    memory::OrderEngineState,
};

pub fn cancel_order(
    state: &mut OrderEngineState,
    command_id: Uuid,
    user_id: Uuid,
    order_id: String,
) -> Result<Vec<LedgerIntent>, ApplyError> {
    let order = state
        .orders
        .get(&order_id)
        .ok_or(ApplyError::OrderNotFound)?;

    if order.user_id != user_id {
        return Err(ApplyError::Unauthorized);
    }

    match order.status {
        OrderStatus::Active | OrderStatus::PartiallyFilled => {}
        _ => return Err(ApplyError::OrderNotCancellable),
    }

    let remaining = order.remaining();
    if remaining <= 0 {
        return Err(ApplyError::OrderNotCancellable);
    }

    let market_symbol = order.market_symbol.clone();
    let order_type = order.order_type;
    let price = order.price;
    let order_user_id = order.user_id;

    let market = state
        .markets
        .iter()
        .find(|market| market.symbol == market_symbol)
        .ok_or(ApplyError::MarketNotFound)?;

    let base_asset = state
        .assets
        .iter()
        .find(|asset| asset.id == market.base_asset_id)
        .ok_or(ApplyError::AssetNotFound)?;

    let (unlock_asset_id, unlock_amount) = match order_type {
        OrderType::Buy => (
            market.quote_asset_id,
            quote_notional(price, remaining, base_asset.decimals)?,
        ),
        OrderType::Sell => (market.base_asset_id, remaining),
    };

    remove_from_book(
        state
            .books
            .get_mut(&market_symbol)
            .ok_or(ApplyError::OrderNotFound)?,
        order_type,
        price,
        &order_id,
    )?;

    let balance = state
        .balances
        .get_mut(&(order_user_id, unlock_asset_id))
        .ok_or(ApplyError::InsufficientBalance)?;

    if balance.locked < unlock_amount {
        return Err(ApplyError::InsufficientBalance);
    }

    balance.locked -= unlock_amount;
    balance.available += unlock_amount;
    balance.updated_at = Utc::now();

    let intent = LedgerIntent {
        command_id,
        sequence: 0,
        user_id: order_user_id,
        asset_id: unlock_asset_id,
        entry_type: LedgerEntryType::Unlock,
        available_delta: unlock_amount,
        locked_delta: -unlock_amount,
        available_after: balance.available,
        locked_after: balance.locked,
        reference_id: Some(command_id),
        reference_type: Some("order".into()),
    };

    let order = state
        .orders
        .get_mut(&order_id)
        .ok_or(ApplyError::OrderNotFound)?;
    order.status = OrderStatus::Cancelled;
    order.cancelled_at = Some(Utc::now());

    Ok(vec![intent])
}
