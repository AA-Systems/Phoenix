use serde::Deserialize;

use crate::markets::MarketStatus;

#[derive(Deserialize)]
pub struct UpdateMarketStatusRequest {
    pub status: MarketStatus,
}
