use chrono::Utc;
use types::{
    balances::Balance,
    ledger_entries::{LedgerEntryType, LedgerIntent},
    order::{OrderStatus, OrderType},
    trade::Trade,
};
use uuid::Uuid;

use crate::{
    commands::apply_command::ApplyError, helper::quote_notional::quote_notional,
    memory::OrderEngineState,
};

struct Fill {
    maker_order_id: String,
    maker_user_id: Uuid,
    maker_price: i64,
    fill_qty: i64,
}

pub fn match_order(
    state: &mut OrderEngineState,
    command_id: Uuid,
    next_sequence: &mut i32,
    taker_order_id: &str,
    market_id: Uuid,
    market_symbol: &str,
    base_asset_id: Uuid,
    quote_asset_id: Uuid,
    base_decimals: i32,
) -> Result<Vec<LedgerIntent>, ApplyError> {
    let mut intents = Vec::new();

    loop {
        let Some(fill) = take_next_fill(state, taker_order_id, market_symbol)? else {
            break;
        };

        let taker = state
            .orders
            .get(taker_order_id)
            .ok_or(ApplyError::OrderNotFound)?;

        let (taker_type, taker_user_id, taker_price) =
            (taker.order_type, taker.user_id, taker.price);

        let (buyer_user_id, seller_user_id, buyer_order_price) = match taker_type {
            OrderType::Buy => (taker_user_id, fill.maker_user_id, taker_price),
            OrderType::Sell => {
                let maker = state
                    .orders
                    .get(&fill.maker_order_id)
                    .ok_or(ApplyError::OrderNotFound)?;
                (fill.maker_user_id, taker_user_id, maker.price)
            }
        };

        let trade_id = Uuid::new_v4();
        let fill_intents = settle_fill(
            state,
            command_id,
            next_sequence,
            trade_id,
            buyer_user_id,
            seller_user_id,
            buyer_order_price,
            fill.maker_price,
            fill.fill_qty,
            base_asset_id,
            quote_asset_id,
            base_decimals,
        )?;
        intents.extend(fill_intents);

        apply_fill(state, &fill.maker_order_id, fill.fill_qty)?;
        apply_fill(state, taker_order_id, fill.fill_qty)?;

        state.trades.push(Trade {
            id: trade_id,
            market_id,
            maker_order_id: fill.maker_order_id,
            taker_order_id: taker_order_id.to_string(),
            price: fill.maker_price,
            quantity: fill.fill_qty,
            buyer_user_id,
            seller_user_id,
            created_at: Utc::now(),
        });
    }

    Ok(intents)
}

fn take_next_fill(
    state: &mut OrderEngineState,
    taker_order_id: &str,
    market_symbol: &str,
) -> Result<Option<Fill>, ApplyError> {
    loop {
        let taker = state
            .orders
            .get(taker_order_id)
            .ok_or(ApplyError::OrderNotFound)?;

        let (taker_type, taker_price, taker_remaining, taker_user_id) = (
            taker.order_type,
            taker.price,
            taker.remaining(),
            taker.user_id,
        );

        if taker_remaining <= 0 {
            return Ok(None);
        }

        let book = state.books.entry(market_symbol.to_string()).or_default();

        let maker_price = match taker_type {
            OrderType::Buy => book.asks.keys().next().copied(),
            OrderType::Sell => book.bids.keys().next_back().copied(),
        };

        let Some(maker_price) = maker_price else {
            return Ok(None);
        };

        let crosses = match taker_type {
            OrderType::Buy => maker_price <= taker_price,
            OrderType::Sell => maker_price >= taker_price,
        };
        if !crosses {
            return Ok(None);
        }

        let queue_empty = match taker_type {
            OrderType::Buy => book
                .asks
                .get(&maker_price)
                .map(|queue| queue.is_empty())
                .unwrap_or(true),
            OrderType::Sell => book
                .bids
                .get(&maker_price)
                .map(|queue| queue.is_empty())
                .unwrap_or(true),
        };

        if queue_empty {
            match taker_type {
                OrderType::Buy => {
                    book.asks.remove(&maker_price);
                }
                OrderType::Sell => {
                    book.bids.remove(&maker_price);
                }
            }
            continue;
        }

        let fill = {
            let queue = match taker_type {
                OrderType::Buy => book.asks.get_mut(&maker_price),
                OrderType::Sell => book.bids.get_mut(&maker_price),
            }
            .ok_or(ApplyError::OrderNotFound)?;

            let front = queue.front().ok_or(ApplyError::OrderNotFound)?;

            // Self-trade prevention: stop at own resting order (do not trade through).
            if front.user_id == taker_user_id {
                return Ok(None);
            }

            let maker_order_id = front.order_id.clone();
            let maker_user_id = front.user_id;
            let fill_qty = taker_remaining.min(front.quantity);

            if fill_qty == front.quantity {
                queue.pop_front();
            } else {
                queue.front_mut().ok_or(ApplyError::OrderNotFound)?.quantity -= fill_qty;
            }

            let level_empty = queue.is_empty();
            (maker_order_id, maker_user_id, fill_qty, level_empty)
        };

        let (maker_order_id, maker_user_id, fill_qty, level_empty) = fill;
        if level_empty {
            match taker_type {
                OrderType::Buy => {
                    book.asks.remove(&maker_price);
                }
                OrderType::Sell => {
                    book.bids.remove(&maker_price);
                }
            }
        }

        return Ok(Some(Fill {
            maker_order_id,
            maker_user_id,
            maker_price,
            fill_qty,
        }));
    }
}

fn apply_fill(
    state: &mut OrderEngineState,
    order_id: &str,
    fill_qty: i64,
) -> Result<(), ApplyError> {
    let order = state
        .orders
        .get_mut(order_id)
        .ok_or(ApplyError::OrderNotFound)?;
    order.filled_quantity += fill_qty;

    if order.remaining() == 0 {
        order.status = OrderStatus::Filled;
        order.filled_at = Some(Utc::now());
    } else {
        order.status = OrderStatus::PartiallyFilled;
    }

    Ok(())
}

fn settle_fill(
    state: &mut OrderEngineState,
    command_id: Uuid,
    next_sequence: &mut i32,
    trade_id: Uuid,
    buyer_user_id: Uuid,
    seller_user_id: Uuid,
    buyer_order_price: i64,
    trade_price: i64,
    fill_qty: i64,
    base_asset_id: Uuid,
    quote_asset_id: Uuid,
    base_decimals: i32,
) -> Result<Vec<LedgerIntent>, ApplyError> {
    let cost = quote_notional(trade_price, fill_qty, base_decimals)?;
    let buyer_reserved = quote_notional(buyer_order_price, fill_qty, base_decimals)?;
    if buyer_reserved < cost {
        return Err(ApplyError::Overflow);
    }
    let refund = buyer_reserved - cost;

    let mut intents = Vec::new();

    debit_locked(state, buyer_user_id, quote_asset_id, buyer_reserved)?;
    if refund > 0 {
        credit_available(state, buyer_user_id, quote_asset_id, refund);
    }
    intents.push(trade_intent(
        state,
        command_id,
        next_sequence,
        trade_id,
        buyer_user_id,
        quote_asset_id,
        refund,
        -buyer_reserved,
    )?);

    credit_available(state, buyer_user_id, base_asset_id, fill_qty);
    intents.push(trade_intent(
        state,
        command_id,
        next_sequence,
        trade_id,
        buyer_user_id,
        base_asset_id,
        fill_qty,
        0,
    )?);

    debit_locked(state, seller_user_id, base_asset_id, fill_qty)?;
    intents.push(trade_intent(
        state,
        command_id,
        next_sequence,
        trade_id,
        seller_user_id,
        base_asset_id,
        0,
        -fill_qty,
    )?);

    credit_available(state, seller_user_id, quote_asset_id, cost);
    intents.push(trade_intent(
        state,
        command_id,
        next_sequence,
        trade_id,
        seller_user_id,
        quote_asset_id,
        cost,
        0,
    )?);

    Ok(intents)
}

fn trade_intent(
    state: &OrderEngineState,
    command_id: Uuid,
    next_sequence: &mut i32,
    trade_id: Uuid,
    user_id: Uuid,
    asset_id: Uuid,
    available_delta: i64,
    locked_delta: i64,
) -> Result<LedgerIntent, ApplyError> {
    let balance = state
        .balances
        .get(&(user_id, asset_id))
        .ok_or(ApplyError::InsufficientBalance)?;
    let sequence = *next_sequence;
    *next_sequence += 1;
    Ok(LedgerIntent {
        command_id,
        sequence,
        user_id,
        asset_id,
        entry_type: LedgerEntryType::Trade,
        available_delta,
        locked_delta,
        available_after: balance.available,
        locked_after: balance.locked,
        reference_id: Some(trade_id),
        reference_type: Some("trade".into()),
    })
}

fn debit_locked(
    state: &mut OrderEngineState,
    user_id: Uuid,
    asset_id: Uuid,
    amount: i64,
) -> Result<(), ApplyError> {
    let balance = state
        .balances
        .get_mut(&(user_id, asset_id))
        .ok_or(ApplyError::InsufficientBalance)?;
    if balance.locked < amount {
        return Err(ApplyError::InsufficientBalance);
    }
    balance.locked -= amount;
    balance.updated_at = Utc::now();
    Ok(())
}

fn credit_available(state: &mut OrderEngineState, user_id: Uuid, asset_id: Uuid, amount: i64) {
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
    balance.available += amount;
    balance.updated_at = Utc::now();
}
