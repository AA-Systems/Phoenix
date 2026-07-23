use serde::Deserialize;
use validator::Validate;

use crate::order::OrderType;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(length(min = 1, max = 32))]
    pub market_symbol: String,
    pub order_type: OrderType,
    #[validate(range(min = 1))]
    pub price: i64,
    #[validate(range(min = 1))]
    pub quantity: i64,
}
