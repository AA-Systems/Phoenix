use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod get_recent_trades_request;

pub struct Trade {
    pub id: Uuid,
    pub market_id: Uuid,
    pub maker_order_id: String,
    pub taker_order_id: String,
    pub price: i64,
    pub quantity: i64,
    pub buyer_user_id: Uuid,
    pub seller_user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeView {
    pub id: Uuid,
    pub market_id: Uuid,
    pub market_symbol: String,
    pub maker_order_id: String,
    pub taker_order_id: String,
    pub price: i64,
    pub quantity: i64,
    pub buyer_user_id: Uuid,
    pub seller_user_id: Uuid,
    pub created_at: DateTime<Utc>,
}
