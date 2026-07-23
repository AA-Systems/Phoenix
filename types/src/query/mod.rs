use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::balances::AssetBalance;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineQuery {
    GetBalances { request_id: Uuid, user_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineReply {
    GetBalances {
        request_id: Uuid,
        balances: Vec<AssetBalance>,
    },
}

impl EngineQuery {
    pub fn request_id(&self) -> Uuid {
        match self {
            EngineQuery::GetBalances { request_id, .. } => *request_id,
        }
    }

    pub fn reply_key(&self) -> String {
        format!("engine-reply:{}", self.request_id())
    }
}

impl EngineReply {
    pub fn request_id(&self) -> Uuid {
        match self {
            EngineReply::GetBalances { request_id, .. } => *request_id,
        }
    }
}
