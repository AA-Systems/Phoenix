use chrono::{DateTime, Utc};
use uuid::Uuid;

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
