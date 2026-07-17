use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreditBalanceRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 16))]
    pub asset_symbol: String,
    #[validate(range(min = 1))]
    pub amount: i64,
}
