use chrono::Utc;
use types::{
    ledger_entries::{LedgerEntryType, LedgerIntent},
    markets::MarketStatus,
    order::{Order, OrderType},
    orderbook::{BookOrder, OrderBook},
};
use uuid::Uuid;

use crate::{
    commands::apply_command::ApplyError,
    helper::{match_order::match_order, quote_notional::quote_notional},
    memory::OrderEngineState,
};

pub fn create_order(
    state: &mut OrderEngineState,
    command_id: Uuid,
    user_id: Uuid,
    market_symbol: String,
    order_type: OrderType,
    price: i64,
    quantity: i64,
) -> Result<Vec<LedgerIntent>, ApplyError> {
    if price <= 0 {
        return Err(ApplyError::InvalidPrice);
    }
    if quantity <= 0 {
        return Err(ApplyError::InvalidQuantity);
    }

    let market = state
        .markets
        .iter()
        .find(|market| market.symbol == market_symbol)
        .ok_or(ApplyError::MarketNotFound)?;

    if market.status != MarketStatus::Trading {
        return Err(ApplyError::MarketNotTrading);
    }

    if price % market.price_tick_size != 0 {
        return Err(ApplyError::InvalidTickSize);
    }
    if quantity % market.quantity_step_size != 0 {
        return Err(ApplyError::InvalidQuantityStep);
    }
    if quantity < market.min_order_quantity {
        return Err(ApplyError::BelowMinQuantity);
    }

    let market_id = market.id;
    let base_asset_id = market.base_asset_id;
    let quote_asset_id = market.quote_asset_id;
    let min_order_notional = market.min_order_notional;

    let base_decimals = state
        .assets
        .iter()
        .find(|asset| asset.id == base_asset_id)
        .ok_or(ApplyError::AssetNotFound)?
        .decimals;

    let notional = quote_notional(price, quantity, base_decimals)?;
    if notional < min_order_notional {
        return Err(ApplyError::BelowMinNotional);
    }

    let (reserve_asset_id, reserve_amount) = match order_type {
        OrderType::Buy => (quote_asset_id, notional),
        OrderType::Sell => (base_asset_id, quantity),
    };

    let balance = state
        .balances
        .get_mut(&(user_id, reserve_asset_id))
        .ok_or(ApplyError::InsufficientBalance)?;

    if balance.available < reserve_amount {
        return Err(ApplyError::InsufficientBalance);
    }

    balance.available -= reserve_amount;
    balance.locked += reserve_amount;
    balance.updated_at = Utc::now();

    let mut intents = vec![LedgerIntent {
        command_id,
        sequence: 0,
        user_id,
        asset_id: reserve_asset_id,
        entry_type: LedgerEntryType::Lock,
        available_delta: -reserve_amount,
        locked_delta: reserve_amount,
        available_after: balance.available,
        locked_after: balance.locked,
        reference_id: Some(command_id),
        reference_type: Some("order".into()),
    }];
    let mut next_sequence = 1;

    let order_id = state.next_order_id.to_string();
    let order = Order::new(
        order_id.clone(),
        user_id,
        market_symbol.clone(),
        order_type,
        price,
        quantity,
    );

    state.orders.insert(order_id.clone(), order);
    state.next_order_id += 1;

    let trade_intents = match_order(
        state,
        command_id,
        &mut next_sequence,
        &order_id,
        market_id,
        &market_symbol,
        base_asset_id,
        quote_asset_id,
        base_decimals,
    )?;
    intents.extend(trade_intents);

    let remaining = state
        .orders
        .get(&order_id)
        .ok_or(ApplyError::OrderNotFound)?
        .remaining();

    if remaining > 0 {
        let book = state
            .books
            .entry(market_symbol)
            .or_insert_with(OrderBook::new);

        let book_order = BookOrder {
            user_id,
            order_id,
            quantity: remaining,
        };

        match order_type {
            OrderType::Buy => book.bids.entry(price).or_default().push_back(book_order),
            OrderType::Sell => book.asks.entry(price).or_default().push_back(book_order),
        }
    }

    Ok(intents)
}
