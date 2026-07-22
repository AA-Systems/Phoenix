use crate::order::OrderType;
use uuid::Uuid;

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
}

impl Command {
    pub fn command_id(&self) -> Uuid {
        match self {
            Command::CreateOrder { command_id, .. } | Command::CancelOrder { command_id, .. } => {
                *command_id
            }
        }
    }
}
