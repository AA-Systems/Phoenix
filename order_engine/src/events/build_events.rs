use std::collections::{HashMap, HashSet};

use types::command::Command;
use types::event::ExchangeEvent;
use types::ledger_entries::LedgerIntent;
use types::orderbook::OrderBookDepth;
use types::trade::TradeView;
use uuid::Uuid;

use crate::helper::diff_book_levels::diff_book_levels;
use crate::memory::OrderEngineState;
use crate::queries::answer_query::{balances_for_user, open_orders_for_user, order_book_depth};

pub fn affected_book_markets(command: &Command, cancel_market_symbol: Option<&str>) -> Vec<String> {
    match command {
        Command::CreateOrder { market_symbol, .. } => {
            vec![market_symbol.trim().to_uppercase()]
        }
        Command::CancelOrder { .. } => cancel_market_symbol
            .map(|symbol| vec![symbol.trim().to_uppercase()])
            .unwrap_or_default(),
        Command::CreditBalance { .. } => Vec::new(),
    }
}

pub fn snapshot_books(
    state: &OrderEngineState,
    markets: &[String],
) -> HashMap<String, OrderBookDepth> {
    markets
        .iter()
        .filter_map(|market| {
            order_book_depth(state, market).map(|book| (book.market_symbol.clone(), book))
        })
        .collect()
}

pub fn build_exchange_events(
    state: &OrderEngineState,
    command: &Command,
    intents: &[LedgerIntent],
    trades_before: usize,
    books_before: &HashMap<String, OrderBookDepth>,
) -> Vec<ExchangeEvent> {
    let mut events = Vec::new();
    let mut balance_users: HashSet<Uuid> = intents.iter().map(|intent| intent.user_id).collect();
    let mut open_order_users: HashSet<Uuid> = HashSet::new();
    let mut book_markets: HashSet<String> = books_before.keys().cloned().collect();

    match command {
        Command::CreateOrder {
            user_id,
            market_symbol,
            ..
        } => {
            balance_users.insert(*user_id);
            open_order_users.insert(*user_id);
            book_markets.insert(market_symbol.trim().to_uppercase());
        }
        Command::CancelOrder { user_id, .. } => {
            balance_users.insert(*user_id);
            open_order_users.insert(*user_id);
        }
        Command::CreditBalance { user_id, .. } => {
            balance_users.insert(*user_id);
        }
    }

    for trade in state.trades.iter().skip(trades_before) {
        let market_symbol = state
            .markets
            .iter()
            .find(|market| market.id == trade.market_id)
            .map(|market| market.symbol.clone())
            .unwrap_or_default();

        events.push(ExchangeEvent::TradeExecuted {
            trade: TradeView {
                id: trade.id,
                market_id: trade.market_id,
                market_symbol: market_symbol.clone(),
                maker_order_id: trade.maker_order_id.clone(),
                taker_order_id: trade.taker_order_id.clone(),
                price: trade.price,
                quantity: trade.quantity,
                buyer_user_id: trade.buyer_user_id,
                seller_user_id: trade.seller_user_id,
                created_at: trade.created_at,
            },
        });

        balance_users.insert(trade.buyer_user_id);
        balance_users.insert(trade.seller_user_id);
        open_order_users.insert(trade.buyer_user_id);
        open_order_users.insert(trade.seller_user_id);
        if !market_symbol.is_empty() {
            book_markets.insert(market_symbol);
        }
    }

    for market_symbol in book_markets {
        let before = books_before.get(&market_symbol);
        let after = order_book_depth(state, &market_symbol);
        let updates = diff_book_levels(before, after.as_ref());
        if !updates.is_empty() {
            events.push(ExchangeEvent::OrderBookUpdated {
                market_symbol,
                updates,
            });
        }
    }

    for user_id in open_order_users {
        events.push(ExchangeEvent::OpenOrdersUpdated {
            user_id,
            orders: open_orders_for_user(state, user_id),
        });
    }

    for user_id in balance_users {
        events.push(ExchangeEvent::BalanceUpdated {
            user_id,
            balances: balances_for_user(state, user_id),
        });
    }

    events
}
