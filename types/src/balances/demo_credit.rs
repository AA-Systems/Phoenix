use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct DemoCreditRequest {
    /// When set, credit only this asset. When omitted, credit the full faucet pack.
    #[serde(default)]
    #[validate(length(min = 1, max = 16))]
    pub asset_symbol: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DemoCreditItem {
    pub command_id: Uuid,
    pub asset_symbol: String,
    pub amount: i64,
}

#[derive(Debug, Serialize)]
pub struct DemoCreditResponse {
    pub credits: Vec<DemoCreditItem>,
}
