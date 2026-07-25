use std::collections::{HashMap, HashSet};

use types::{
    assets::Asset, balances::Balance, markets::Market, order::Order, orderbook::MarketBooks,
    trade::Trade,
};
use uuid::Uuid;

pub mod load_from_db;
pub mod replay;
pub mod snapshot;

pub struct OrderEngineState {
    pub next_order_id: u64,
    pub orders: HashMap<String, Order>,
    pub balances: HashMap<(Uuid, Uuid), Balance>,
    pub books: MarketBooks,
    pub markets: Vec<Market>,
    pub assets: Vec<Asset>,
    pub trades: Vec<Trade>,
    pub processed_commands: HashSet<Uuid>,
}

impl OrderEngineState {
    pub fn new() -> Self {
        Self {
            next_order_id: 1,
            orders: HashMap::new(),
            balances: HashMap::new(),
            books: HashMap::new(),
            markets: Vec::new(),
            assets: Vec::new(),
            trades: Vec::new(),
            processed_commands: HashSet::new(),
        }
    }
}

impl Default for OrderEngineState {
    fn default() -> Self {
        Self::new()
    }
}
