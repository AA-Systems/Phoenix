use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod insert_asset_request;
pub mod insert_asset_response;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "assets_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AssetStatus {
    Active,
    Archived,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Asset {
    pub id: uuid::Uuid,
    pub symbol: String,
    pub name: String,
    pub decimals: i32,
    pub status: AssetStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
