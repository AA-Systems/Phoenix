use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "market_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    Trading,
    Halted,
    Archived,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Market {
    pub id: uuid::Uuid,
    pub symbol: String,
    pub name: String,
    pub base_asset_id: uuid::Uuid,
    pub quote_asset_id: uuid::Uuid,
    pub status: MarketStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
