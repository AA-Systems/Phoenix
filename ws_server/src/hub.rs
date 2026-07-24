use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::{Mutex, mpsc};
use tracing::warn;
use types::event::ExchangeEvent;
use uuid::Uuid;

use crate::protocol::{Channel, ServerMessage};

pub type ClientId = u64;
pub type ClientTx = mpsc::UnboundedSender<String>;

#[derive(Default)]
struct ClientState {
    user_id: Option<Uuid>,
    orderbooks: HashSet<String>,
    trades: HashSet<String>,
    balances: bool,
    open_orders: bool,
}

struct HubInner {
    next_id: ClientId,
    clients: HashMap<ClientId, ClientTx>,
    state: HashMap<ClientId, ClientState>,
}

impl Default for HubInner {
    fn default() -> Self {
        Self {
            next_id: 1,
            clients: HashMap::new(),
            state: HashMap::new(),
        }
    }
}

#[derive(Clone, Default)]
pub struct Hub {
    inner: Arc<Mutex<HubInner>>,
}

impl Hub {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn register(&self, tx: ClientTx) -> ClientId {
        let mut inner = self.inner.lock().await;
        let id = inner.next_id;
        inner.next_id += 1;
        inner.clients.insert(id, tx);
        inner.state.insert(id, ClientState::default());
        id
    }

    pub async fn unregister(&self, id: ClientId) {
        let mut inner = self.inner.lock().await;
        inner.clients.remove(&id);
        inner.state.remove(&id);
    }

    pub async fn set_user(&self, id: ClientId, user_id: Uuid) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.user_id = Some(user_id);
        }
    }

    pub async fn user_id(&self, id: ClientId) -> Option<Uuid> {
        let inner = self.inner.lock().await;
        inner.state.get(&id).and_then(|s| s.user_id)
    }

    pub async fn subscribe_orderbook(&self, id: ClientId, market: String) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.orderbooks.insert(market);
        }
    }

    pub async fn unsubscribe_orderbook(&self, id: ClientId, market: &str) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.orderbooks.remove(market);
        }
    }

    pub async fn subscribe_trades(&self, id: ClientId, market: String) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.trades.insert(market);
        }
    }

    pub async fn unsubscribe_trades(&self, id: ClientId, market: &str) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.trades.remove(market);
        }
    }

    pub async fn subscribe_balances(&self, id: ClientId) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.balances = true;
        }
    }

    pub async fn unsubscribe_balances(&self, id: ClientId) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.balances = false;
        }
    }

    pub async fn subscribe_open_orders(&self, id: ClientId) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.open_orders = true;
        }
    }

    pub async fn unsubscribe_open_orders(&self, id: ClientId) {
        let mut inner = self.inner.lock().await;
        if let Some(state) = inner.state.get_mut(&id) {
            state.open_orders = false;
        }
    }

    pub async fn send_raw(&self, id: ClientId, payload: String) {
        let mut inner = self.inner.lock().await;
        if let Some(tx) = inner.clients.get(&id)
            && tx.send(payload).is_err()
        {
            inner.clients.remove(&id);
            inner.state.remove(&id);
        }
    }

    pub async fn publish(&self, event: &ExchangeEvent) {
        let payload = match serde_json::to_string(&ServerMessage::Event { event }) {
            Ok(payload) => payload,
            Err(err) => {
                warn!(%err, "failed to serialize exchange event for ws");
                return;
            }
        };

        let mut inner = self.inner.lock().await;
        let mut dead = Vec::new();

        for (&id, state) in &inner.state {
            let deliver = match event {
                ExchangeEvent::OrderBookUpdated { market_symbol, .. } => {
                    state.orderbooks.contains(&normalize_market(market_symbol))
                }
                ExchangeEvent::TradeExecuted { trade } => state
                    .trades
                    .contains(&normalize_market(&trade.market_symbol)),
                ExchangeEvent::BalanceUpdated { user_id, .. } => {
                    state.balances && state.user_id == Some(*user_id)
                }
                ExchangeEvent::OpenOrdersUpdated { user_id, .. } => {
                    state.open_orders && state.user_id == Some(*user_id)
                }
            };

            if !deliver {
                continue;
            }

            if let Some(tx) = inner.clients.get(&id)
                && tx.send(payload.clone()).is_err()
            {
                dead.push(id);
            }
        }

        for id in dead {
            inner.clients.remove(&id);
            inner.state.remove(&id);
        }
    }
}

pub fn normalize_market(market: &str) -> String {
    market.trim().to_uppercase()
}

pub fn require_market(market: Option<&str>) -> Result<String, &'static str> {
    market
        .map(normalize_market)
        .filter(|m| !m.is_empty())
        .ok_or("market is required")
}

pub fn channel_needs_auth(channel: Channel) -> bool {
    matches!(channel, Channel::Balances | Channel::OpenOrders)
}
