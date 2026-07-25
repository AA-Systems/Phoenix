use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod insert_market_request;
pub mod insert_market_response;
pub mod list_markets_query;
pub mod update_market_config_request;
pub mod update_market_status_request;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "market_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    Trading,
    Halted,
    Archived,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Market {
    pub id: uuid::Uuid,
    pub symbol: String,
    pub name: String,
    pub base_asset_id: uuid::Uuid,
    pub quote_asset_id: uuid::Uuid,
    pub status: MarketStatus,
    pub price_tick_size: i64,
    pub quantity_step_size: i64,
    pub min_order_quantity: i64,
    pub min_order_notional: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
