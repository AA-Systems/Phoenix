use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod intent;

pub use intent::LedgerIntent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ledger_entry_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LedgerEntryType {
    Deposit,
    Withdrawal,
    Lock,
    Unlock,
    Trade,
    Fee,
    Adjustment,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub asset_id: uuid::Uuid,
    pub entry_type: LedgerEntryType,
    pub available_delta: i64,
    pub locked_delta: i64,
    pub available_after: i64,
    pub locked_after: i64,
    pub reference_id: Option<uuid::Uuid>,
    pub reference_type: Option<String>,
    pub command_id: Option<uuid::Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LedgerEntryView {
    pub id: uuid::Uuid,
    pub asset_id: uuid::Uuid,
    pub asset_symbol: String,
    pub asset_decimals: i32,
    pub entry_type: LedgerEntryType,
    pub available_delta: i64,
    pub locked_delta: i64,
    pub available_after: i64,
    pub locked_after: i64,
    pub reference_id: Option<uuid::Uuid>,
    pub reference_type: Option<String>,
    pub command_id: Option<uuid::Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
