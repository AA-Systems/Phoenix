use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ledger_entries::LedgerEntryType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerIntent {
    pub command_id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub entry_type: LedgerEntryType,
    pub available_delta: i64,
    pub locked_delta: i64,
    pub available_after: i64,
    pub locked_after: i64,
    pub reference_id: Option<Uuid>,
    pub reference_type: Option<String>,
}
