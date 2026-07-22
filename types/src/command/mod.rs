use crate::order::OrderType;
use uuid::Uuid;

pub enum Command {
    CreateOrder(Uuid, String, OrderType, i64, i64),
    CancelOrder(Uuid, String),
}
