use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod credit_balance_request;
pub mod credit_balance_response;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Balance {
    pub user_id: uuid::Uuid,
    pub asset_id: uuid::Uuid,
    pub available: i64,
    pub locked: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AssetBalance {
    pub asset_id: uuid::Uuid,
    pub symbol: String,
    pub name: String,
    pub decimals: i32,
    pub available: i64,
    pub locked: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
