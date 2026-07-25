use serde::Deserialize;
use validator::Validate;

fn default_limit() -> u32 {
    200
}

#[derive(Deserialize, Validate)]
pub struct GetCandlesRequest {
    #[validate(length(min = 1, max = 32))]
    pub market_symbol: String,

    #[validate(length(min = 1, max = 8))]
    pub interval: String,

    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 500))]
    pub limit: u32,
}
