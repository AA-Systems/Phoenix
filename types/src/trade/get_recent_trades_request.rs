use serde::Deserialize;
use validator::Validate;

fn default_limit() -> u32 {
    50
}

#[derive(Deserialize, Validate)]
pub struct GetRecentTradesRequest {
    #[validate(length(min = 1, max = 32))]
    pub market_symbol: String,

    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 100))]
    pub limit: u32,
}
