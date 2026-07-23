use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CancelOrderRequest {
    #[validate(length(min = 1, max = 64))]
    pub order_id: String,
}
