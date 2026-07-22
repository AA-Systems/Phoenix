use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Active,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

pub struct Order {
    pub id: String,
    pub user_id: Uuid,
    pub market_symbol: String,
    pub order_type: OrderType,
    pub price: i64,
    pub quantity: i64,
    pub filled_quantity: i64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub filled_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

impl Order {
    pub fn new(
        id: String,
        user_id: Uuid,
        market_symbol: String,
        order_type: OrderType,
        price: i64,
        quantity: i64,
    ) -> Self {
        Self {
            id,
            user_id,
            market_symbol,
            order_type,
            price,
            quantity,
            filled_quantity: 0,
            status: OrderStatus::Active,
            created_at: Utc::now(),
            filled_at: None,
            cancelled_at: None,
        }
    }

    pub fn remaining(&self) -> i64 {
        self.quantity - self.filled_quantity
    }
}
