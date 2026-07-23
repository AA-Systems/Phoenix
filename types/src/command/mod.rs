use crate::order::OrderType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    CreateOrder {
        command_id: Uuid,
        user_id: Uuid,
        market_symbol: String,
        order_type: OrderType,
        price: i64,
        quantity: i64,
    },
    CancelOrder {
        command_id: Uuid,
        user_id: Uuid,
        order_id: String,
    },
    CreditBalance {
        command_id: Uuid,
        user_id: Uuid,
        asset_id: Uuid,
        amount: i64,
    },
}

impl Command {
    pub fn command_id(&self) -> Uuid {
        match self {
            Command::CreateOrder { command_id, .. }
            | Command::CancelOrder { command_id, .. }
            | Command::CreditBalance { command_id, .. } => *command_id,
        }
    }
}
