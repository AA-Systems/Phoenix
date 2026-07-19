use serde::Deserialize;
use validator::Validate;

fn default_limit() -> i64 {
    50
}

#[derive(Deserialize, Validate)]
pub struct ListMarketsQuery {
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 100))]
    pub limit: i64,
    #[serde(default)]
    #[validate(range(min = 0))]
    pub skip: i64,
}
