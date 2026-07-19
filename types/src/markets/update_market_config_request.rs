use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdateMarketConfigRequest {
    #[validate(range(min = 1))]
    pub price_tick_size: i64,
    #[validate(range(min = 1))]
    pub quantity_step_size: i64,
    #[validate(range(min = 1))]
    pub min_order_quantity: i64,
    #[validate(range(min = 1))]
    pub min_order_notional: i64,
}
