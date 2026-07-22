use std::collections::HashMap;

use types::{
    assets::Asset, balances::Balance, markets::Market, order::Order, orderbook::MarketBooks,
};
use uuid::Uuid;

pub struct OrderEngineState {
    pub next_order_id: u64,
    pub orders: HashMap<String, Order>,
    pub balances: HashMap<(Uuid, Uuid), Balance>,
    pub books: MarketBooks,
    pub markets: Vec<Market>,
    pub assets: Vec<Asset>,
}

impl OrderEngineState {
    pub fn new() -> Self {
        // TODO: load balances, open orders, books, markets, assets and last Kafka offset from snapshot
        Self {
            next_order_id: 1,
            orders: HashMap::new(),
            balances: HashMap::new(),
            books: HashMap::new(),
            markets: Vec::new(),
            assets: Vec::new(),
        }
    }
}

impl Default for OrderEngineState {
    fn default() -> Self {
        Self::new()
    }
}
