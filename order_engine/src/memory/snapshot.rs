use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use types::order::{Order, OrderStatus};
use types::orderbook::MarketBooks;
use types::trade::Trade;
use uuid::Uuid;

use crate::memory::OrderEngineState;

pub const DEFAULT_CURSOR: &str = "0-0";
const MAX_SNAPSHOT_TRADES: usize = 500;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSnapshotPayload {
    pub next_order_id: u64,
    pub orders: HashMap<String, Order>,
    pub books: MarketBooks,
    pub trades: Vec<Trade>,
    #[serde(default)]
    pub processed_commands: HashSet<Uuid>,
}

#[derive(Debug, Clone)]
pub struct SnapshotMeta {
    pub order_commands_cursor: String,
    pub engine_commands_cursor: String,
}

impl Default for SnapshotMeta {
    fn default() -> Self {
        Self {
            order_commands_cursor: DEFAULT_CURSOR.to_string(),
            engine_commands_cursor: DEFAULT_CURSOR.to_string(),
        }
    }
}

pub fn build_payload(state: &OrderEngineState) -> EngineSnapshotPayload {
    let orders = state
        .orders
        .iter()
        .filter(|(_, order)| {
            matches!(
                order.status,
                OrderStatus::Active | OrderStatus::PartiallyFilled
            )
        })
        .map(|(id, order)| (id.clone(), order.clone()))
        .collect();

    let trade_start = state.trades.len().saturating_sub(MAX_SNAPSHOT_TRADES);
    let trades = state.trades[trade_start..].to_vec();

    EngineSnapshotPayload {
        next_order_id: state.next_order_id,
        orders,
        books: state.books.clone(),
        trades,
        processed_commands: state.processed_commands.clone(),
    }
}

pub fn apply_payload(state: &mut OrderEngineState, payload: EngineSnapshotPayload) {
    state.next_order_id = payload.next_order_id.max(1);
    state.orders = payload.orders;
    state.books = payload.books;
    state.trades = payload.trades;
    state.processed_commands.extend(payload.processed_commands);
}

pub async fn load_into_state(
    pool: &sqlx::PgPool,
    state: &mut OrderEngineState,
) -> Result<SnapshotMeta, sqlx::Error> {
    let Some(row) = db::engine_snapshots::load_latest(pool).await? else {
        info!("no engine snapshot found; starting with empty books/orders");
        return Ok(SnapshotMeta::default());
    };

    match serde_json::from_value::<EngineSnapshotPayload>(row.state) {
        Ok(payload) => {
            let open_orders = payload.orders.len();
            let books = payload.books.len();
            let trades = payload.trades.len();
            apply_payload(state, payload);
            info!(
                next_order_id = state.next_order_id,
                open_orders,
                books,
                trades,
                order_cursor = %row.order_commands_cursor,
                engine_cursor = %row.engine_commands_cursor,
                "restored engine snapshot"
            );
            Ok(SnapshotMeta {
                order_commands_cursor: row.order_commands_cursor,
                engine_commands_cursor: row.engine_commands_cursor,
            })
        }
        Err(err) => {
            warn!(%err, "failed to decode engine snapshot; ignoring");
            Ok(SnapshotMeta::default())
        }
    }
}

pub async fn persist(
    pool: &sqlx::PgPool,
    state: &OrderEngineState,
    meta: &SnapshotMeta,
) -> Result<(), sqlx::Error> {
    let payload = build_payload(state);
    let value = serde_json::to_value(&payload)
        .map_err(|err| sqlx::Error::Protocol(format!("serialize engine snapshot failed: {err}")))?;

    db::engine_snapshots::upsert_latest(
        pool,
        state.next_order_id as i64,
        &meta.order_commands_cursor,
        &meta.engine_commands_cursor,
        &value,
    )
    .await?;

    info!(
        next_order_id = state.next_order_id,
        open_orders = payload.orders.len(),
        books = payload.books.len(),
        trades = payload.trades.len(),
        order_cursor = %meta.order_commands_cursor,
        engine_cursor = %meta.engine_commands_cursor,
        "wrote engine snapshot"
    );
    Ok(())
}
