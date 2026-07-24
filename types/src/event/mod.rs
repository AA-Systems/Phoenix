use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::balances::AssetBalance;
use crate::order::OpenOrderView;
use crate::orderbook::OrderBookLevelDelta;
use crate::trade::TradeView;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExchangeEvent {
    BalanceUpdated {
        user_id: Uuid,
        balances: Vec<AssetBalance>,
    },
    OrderBookUpdated {
        market_symbol: String,
        updates: Vec<OrderBookLevelDelta>,
    },
    OpenOrdersUpdated {
        user_id: Uuid,
        orders: Vec<OpenOrderView>,
    },
    TradeExecuted {
        trade: TradeView,
    },
}
