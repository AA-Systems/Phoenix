use types::balances::AssetBalance;
use types::order::{OpenOrderView, OrderStatus};
use types::orderbook::{OrderBookDepth, PriceLevel};
use types::query::{EngineQuery, EngineReply};
use uuid::Uuid;

use crate::memory::OrderEngineState;

pub fn answer_query(state: &OrderEngineState, query: EngineQuery) -> EngineReply {
    match query {
        EngineQuery::GetBalances {
            request_id,
            user_id,
        } => EngineReply::GetBalances {
            request_id,
            balances: balances_for_user(state, user_id),
        },
        EngineQuery::GetOpenOrders {
            request_id,
            user_id,
        } => EngineReply::GetOpenOrders {
            request_id,
            orders: open_orders_for_user(state, user_id),
        },
        EngineQuery::GetOrderBook {
            request_id,
            market_symbol,
        } => EngineReply::GetOrderBook {
            request_id,
            book: order_book_depth(state, &market_symbol),
        },
    }
}

pub fn balances_for_user(state: &OrderEngineState, user_id: Uuid) -> Vec<AssetBalance> {
    let mut balances: Vec<AssetBalance> = state
        .balances
        .iter()
        .filter(|((uid, _), _)| *uid == user_id)
        .filter_map(|((_, asset_id), balance)| {
            let asset = state.assets.iter().find(|asset| asset.id == *asset_id)?;
            Some(AssetBalance {
                asset_id: *asset_id,
                symbol: asset.symbol.clone(),
                name: asset.name.clone(),
                decimals: asset.decimals,
                available: balance.available,
                locked: balance.locked,
                updated_at: balance.updated_at,
            })
        })
        .collect();

    balances.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    balances
}

pub fn open_orders_for_user(state: &OrderEngineState, user_id: Uuid) -> Vec<OpenOrderView> {
    let mut orders: Vec<OpenOrderView> = state
        .orders
        .values()
        .filter(|order| order.user_id == user_id)
        .filter(|order| {
            matches!(
                order.status,
                OrderStatus::Active | OrderStatus::PartiallyFilled
            )
        })
        .map(|order| order.to_open_view())
        .collect();

    orders.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    orders
}

pub fn order_book_depth(state: &OrderEngineState, market_symbol: &str) -> Option<OrderBookDepth> {
    let symbol = market_symbol.trim().to_uppercase();
    if !state.markets.iter().any(|market| market.symbol == symbol) {
        return None;
    }

    let book = state.books.get(&symbol);
    let bids = book
        .map(|book| {
            book.bids
                .iter()
                .rev()
                .map(|(price, queue)| PriceLevel {
                    price: *price,
                    quantity: queue.iter().map(|order| order.quantity).sum(),
                    order_count: queue.len() as u32,
                })
                .collect()
        })
        .unwrap_or_default();

    let asks = book
        .map(|book| {
            book.asks
                .iter()
                .map(|(price, queue)| PriceLevel {
                    price: *price,
                    quantity: queue.iter().map(|order| order.quantity).sum(),
                    order_count: queue.len() as u32,
                })
                .collect()
        })
        .unwrap_or_default();

    Some(OrderBookDepth {
        market_symbol: symbol,
        bids,
        asks,
    })
}
