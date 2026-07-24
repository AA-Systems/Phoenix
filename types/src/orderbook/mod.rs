use std::collections::{BTreeMap, HashMap, VecDeque};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod get_order_book_request;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: i64,
    pub quantity: i64,
    pub order_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookDepth {
    pub market_symbol: String,
    /// Highest bid first.
    pub bids: Vec<PriceLevel>,
    /// Lowest ask first.
    pub asks: Vec<PriceLevel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookSide {
    Bid,
    Ask,
}

/// Incremental book update. `quantity == 0` means the level was removed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevelDelta {
    pub side: BookSide,
    pub price: i64,
    pub quantity: i64,
    pub order_count: u32,
}
