use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetOrderBookRequest {
    #[validate(length(min = 1, max = 32))]
    pub market_symbol: String,
}
