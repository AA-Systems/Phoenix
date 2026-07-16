use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct InsertMarketRequest {
    #[validate(length(min = 1, max = 32))]
    pub symbol: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(length(min = 1, max = 16))]
    pub base_asset_symbol: String,
    #[validate(length(min = 1, max = 16))]
    pub quote_asset_symbol: String,
}
