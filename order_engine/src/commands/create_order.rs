use chrono::Utc;
use types::{
    markets::MarketStatus,
    order::{Order, OrderType},
    orderbook::{BookOrder, OrderBook},
};
use uuid::Uuid;

use crate::{
    commands::apply_command::ApplyError, helper::quote_notional::quote_notional,
    memory::OrderEngineState,
};

pub fn create_order(
    state: &mut OrderEngineState,
    user_id: Uuid,
    market_symbol: String,
    order_type: OrderType,
    price: i64,
    quantity: i64,
) -> Result<(), ApplyError> {
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

    let base_asset = state
        .assets
        .iter()
        .find(|asset| asset.id == market.base_asset_id)
        .ok_or(ApplyError::AssetNotFound)?;

    let notional = quote_notional(price, quantity, base_asset.decimals)?;
    if notional < market.min_order_notional {
        return Err(ApplyError::BelowMinNotional);
    }

    let (reserve_asset_id, reserve_amount) = match order_type {
        OrderType::Buy => (market.quote_asset_id, notional),
        OrderType::Sell => (market.base_asset_id, quantity),
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

    let order_id = state.next_order_id.to_string();
    let order = Order::new(
        order_id.clone(),
        user_id,
        market_symbol.clone(),
        order_type,
        price,
        quantity,
    );

    let book = state
        .books
        .entry(market_symbol)
        .or_insert_with(OrderBook::new);

    let book_order = BookOrder {
        user_id,
        order_id: order_id.clone(),
        quantity,
    };

    match order_type {
        OrderType::Buy => book.bids.entry(price).or_default().push_back(book_order),
        OrderType::Sell => book.asks.entry(price).or_default().push_back(book_order),
    }

    state.orders.insert(order_id, order);
    state.next_order_id += 1;

    Ok(())
}
