use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::balances::AssetBalance;
use crate::order::OpenOrderView;
use crate::orderbook::OrderBookDepth;
use crate::trade::TradeView;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineQuery {
    GetBalances {
        request_id: Uuid,
        user_id: Uuid,
    },
    GetOpenOrders {
        request_id: Uuid,
        user_id: Uuid,
    },
    GetOrderBook {
        request_id: Uuid,
        market_symbol: String,
    },
    GetRecentTrades {
        request_id: Uuid,
        market_symbol: String,
        limit: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineReply {
    GetBalances {
        request_id: Uuid,
        balances: Vec<AssetBalance>,
    },
    GetOpenOrders {
        request_id: Uuid,
        orders: Vec<OpenOrderView>,
    },
    GetOrderBook {
        request_id: Uuid,
        book: Option<OrderBookDepth>,
    },
    GetRecentTrades {
        request_id: Uuid,
        trades: Option<Vec<TradeView>>,
    },
}

impl EngineQuery {
    pub fn request_id(&self) -> Uuid {
        match self {
            EngineQuery::GetBalances { request_id, .. }
            | EngineQuery::GetOpenOrders { request_id, .. }
            | EngineQuery::GetOrderBook { request_id, .. }
            | EngineQuery::GetRecentTrades { request_id, .. } => *request_id,
        }
    }

    pub fn reply_key(&self) -> String {
        format!("engine-reply:{}", self.request_id())
    }
}

impl EngineReply {
    pub fn request_id(&self) -> Uuid {
        match self {
            EngineReply::GetBalances { request_id, .. }
            | EngineReply::GetOpenOrders { request_id, .. }
            | EngineReply::GetOrderBook { request_id, .. }
            | EngineReply::GetRecentTrades { request_id, .. } => *request_id,
        }
    }
}
