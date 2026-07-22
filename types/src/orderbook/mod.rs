use std::collections::{BTreeMap, HashMap, VecDeque};

use uuid::Uuid;

pub struct BookOrder {
    pub user_id: Uuid,
    pub order_id: String,
    pub quantity: i64,
}

pub struct OrderBook {
    /// Price → resting buy orders (highest bid first via reverse iteration).
    pub bids: BTreeMap<i64, VecDeque<BookOrder>>,
    /// Price → resting sell orders (lowest ask first).
    pub asks: BTreeMap<i64, VecDeque<BookOrder>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

/// Engine holds one book per market symbol.
pub type MarketBooks = HashMap<String, OrderBook>;
