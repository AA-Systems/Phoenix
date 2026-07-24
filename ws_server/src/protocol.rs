use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ClientMessage {
    Auth {
        token: String,
    },
    Subscribe {
        channel: Channel,
        #[serde(default)]
        market: Option<String>,
    },
    Unsubscribe {
        channel: Channel,
        #[serde(default)]
        market: Option<String>,
    },
    Ping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    Orderbook,
    Trades,
    Balances,
    OpenOrders,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage<'a> {
    Ready,
    Authenticated {
        user_id: Uuid,
    },
    Subscribed {
        channel: Channel,
        #[serde(skip_serializing_if = "Option::is_none")]
        market: Option<&'a str>,
    },
    Unsubscribed {
        channel: Channel,
        #[serde(skip_serializing_if = "Option::is_none")]
        market: Option<&'a str>,
    },
    Error {
        message: &'a str,
    },
    Pong,
    Event {
        event: &'a types::event::ExchangeEvent,
    },
}
