use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct InsertAssetRequest {
    #[validate(length(min = 1, max = 16))]
    pub symbol: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(range(max = 18))]
    pub decimals: u32,
}
