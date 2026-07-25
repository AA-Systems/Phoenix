use std::collections::HashMap;
use std::time::Duration;

use redis::aio::ConnectionManager;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, RedisResult};
use sqlx::PgPool;
use tracing::{error, info, warn};
use types::command::Command;

use crate::commands::apply_command::{
    ApplyOutcome, apply_command_effects, commit_command, revert_intents,
};
use crate::events::{
    build_events::{affected_book_markets, build_exchange_events, snapshot_books},
    publish::publish_exchange_events,
};
use crate::helper::ack::ack;
use crate::helper::field_as_bytes::field_as_bytes;
use crate::memory::OrderEngineState;
use crate::memory::snapshot::{DEFAULT_CURSOR, SnapshotMeta};

pub async fn replay_since_cursors(
    conn: &mut ConnectionManager,
    pool: &PgPool,
    state: &mut OrderEngineState,
    meta: &mut SnapshotMeta,
    order_stream: &str,
    engine_stream: &str,
    events_stream: &str,
) {
    let mut applied = 0u64;
    applied += replay_stream(conn, pool, state, meta, order_stream, true, events_stream).await;
    applied += replay_stream(conn, pool, state, meta, engine_stream, false, events_stream).await;

    info!(applied, "finished command stream replay");
}

async fn replay_stream(
    conn: &mut ConnectionManager,
    pool: &PgPool,
    state: &mut OrderEngineState,
    meta: &mut SnapshotMeta,
    stream: &str,
    is_order_stream: bool,
    events_stream: &str,
) -> u64 {
    let mut cursor = if is_order_stream {
        meta.order_commands_cursor.clone()
    } else {
        meta.engine_commands_cursor.clone()
    };
    if cursor.is_empty() {
        cursor = DEFAULT_CURSOR.to_string();
    }

    let mut applied = 0u64;
    loop {
        let opts = StreamReadOptions::default().count(100);
        let reply: RedisResult<StreamReadReply> =
            conn.xread_options(&[stream], &[&cursor], &opts).await;

        let reply = match reply {
            Ok(reply) => reply,
            Err(err) => {
                error!(%err, %stream, "replay XREAD failed");
                break;
            }
        };

        if reply.keys.is_empty() {
            break;
        }

        let mut progressed = false;
        let mut stuck = false;
        for stream_key in reply.keys {
            for entry in stream_key.ids {
                progressed = true;
                match apply_durable_entry(conn, pool, state, &entry.id, &entry.map, events_stream)
                    .await
                {
                    DurableApply::Applied => {
                        applied += 1;
                        cursor = entry.id.clone();
                        if is_order_stream {
                            meta.order_commands_cursor = entry.id.clone();
                        } else {
                            meta.engine_commands_cursor = entry.id.clone();
                        }
                    }
                    DurableApply::Skip => {
                        // Poison / duplicate / reject — advance past so we do not loop forever.
                        cursor = entry.id.clone();
                        if is_order_stream {
                            meta.order_commands_cursor = entry.id.clone();
                        } else {
                            meta.engine_commands_cursor = entry.id.clone();
                        }
                    }
                    DurableApply::Retry => {
                        stuck = true;
                        break;
                    }
                }
            }
            if stuck {
                break;
            }
        }

        if !progressed || stuck {
            break;
        }
    }

    applied
}

pub async fn drain_pending(
    conn: &mut ConnectionManager,
    pool: &PgPool,
    state: &mut OrderEngineState,
    meta: &mut SnapshotMeta,
    order_stream: &str,
    engine_stream: &str,
    group: &str,
    consumer: &str,
    events_stream: &str,
) {
    let read_opts = StreamReadOptions::default()
        .group(group, consumer)
        .count(50);

    let mut total = 0u64;
    loop {
        // ID "0" = pending entries already delivered to this consumer.
        let reply: RedisResult<StreamReadReply> = conn
            .xread_options(&[order_stream, engine_stream], &["0", "0"], &read_opts)
            .await;

        let reply = match reply {
            Ok(reply) => reply,
            Err(err) => {
                error!(%err, "pending XREADGROUP failed");
                break;
            }
        };

        if reply.keys.is_empty() {
            break;
        }

        let mut batch = 0u64;
        let mut stuck = false;
        for stream_key in reply.keys {
            let is_order = stream_key.key == order_stream;
            for entry in stream_key.ids {
                batch += 1;
                match apply_durable_entry(conn, pool, state, &entry.id, &entry.map, events_stream)
                    .await
                {
                    DurableApply::Applied => {
                        total += 1;
                        if is_order {
                            meta.order_commands_cursor = entry.id.clone();
                        } else {
                            meta.engine_commands_cursor = entry.id.clone();
                        }
                        ack(conn, &stream_key.key, group, &entry.id).await;
                    }
                    DurableApply::Skip => {
                        ack(conn, &stream_key.key, group, &entry.id).await;
                    }
                    DurableApply::Retry => {
                        // Leave pending so the next boot / loop can retry ledger persist.
                        stuck = true;
                    }
                }
            }
        }

        if batch == 0 || stuck {
            break;
        }
    }

    if total > 0 {
        info!(total, "drained pending command entries");
    }
}

enum DurableApply {
    Applied,
    Skip,
    Retry,
}

async fn apply_durable_entry(
    conn: &mut ConnectionManager,
    pool: &PgPool,
    state: &mut OrderEngineState,
    entry_id: &str,
    fields: &HashMap<String, redis::Value>,
    events_stream: &str,
) -> DurableApply {
    let Some(payload) = field_as_bytes(fields, "payload") else {
        warn!(%entry_id, "missing payload");
        return DurableApply::Skip;
    };
    let Ok(command) = serde_json::from_slice::<Command>(&payload) else {
        warn!(%entry_id, "invalid command payload");
        return DurableApply::Skip;
    };

    let command_id = command.command_id();
    let trades_before = state.trades.len();
    let cancel_market_symbol = match &command {
        Command::CancelOrder { order_id, .. } => state
            .orders
            .get(order_id)
            .map(|order| order.market_symbol.clone()),
        _ => None,
    };
    let book_markets = affected_book_markets(&command, cancel_market_symbol.as_deref());
    let books_before = snapshot_books(state, &book_markets);
    let command_for_events = command.clone();

    match apply_command_effects(state, command) {
        Ok(ApplyOutcome::AlreadyProcessed) => DurableApply::Skip,
        Ok(ApplyOutcome::Applied {
            command_id,
            intents,
        }) => {
            if intents.is_empty() {
                commit_command(state, command_id);
                publish_events(
                    conn,
                    state,
                    &command_for_events,
                    &intents,
                    trades_before,
                    &books_before,
                    events_stream,
                )
                .await;
                return DurableApply::Applied;
            }

            match db::balances::persist_intents::persist_intents(pool, &intents).await {
                Ok(()) => {
                    commit_command(state, command_id);
                    publish_events(
                        conn,
                        state,
                        &command_for_events,
                        &intents,
                        trades_before,
                        &books_before,
                        events_stream,
                    )
                    .await;
                    DurableApply::Applied
                }
                Err(err) => {
                    revert_intents(state, &intents);
                    error!(%command_id, %entry_id, %err, "persist failed during recovery");
                    DurableApply::Retry
                }
            }
        }
        Err(err) => {
            warn!(%command_id, %entry_id, ?err, "apply failed during recovery");
            DurableApply::Skip
        }
    }
}

async fn publish_events(
    conn: &mut ConnectionManager,
    state: &OrderEngineState,
    command: &Command,
    intents: &[types::ledger_entries::LedgerIntent],
    trades_before: usize,
    books_before: &std::collections::HashMap<String, types::orderbook::OrderBookDepth>,
    events_stream: &str,
) {
    let events = build_exchange_events(state, command, intents, trades_before, books_before);
    if !events.is_empty() {
        publish_exchange_events(conn, events_stream, &events).await;
    }
}

pub async fn maybe_snapshot(
    pool: &PgPool,
    state: &OrderEngineState,
    meta: &SnapshotMeta,
    commands_since: &mut u32,
    force: bool,
) {
    const EVERY_N: u32 = 25;
    if !force && *commands_since < EVERY_N {
        return;
    }
    if let Err(err) = crate::memory::snapshot::persist(pool, state, meta).await {
        error!(%err, "failed to persist engine snapshot");
        return;
    }
    *commands_since = 0;
}

pub fn touch_cursor(meta: &mut SnapshotMeta, stream: &str, entry_id: &str, order_stream: &str) {
    if stream == order_stream {
        meta.order_commands_cursor = entry_id.to_string();
    } else {
        meta.engine_commands_cursor = entry_id.to_string();
    }
}

pub async fn snapshot_ticker(
    pool: PgPool,
    state: std::sync::Arc<tokio::sync::Mutex<OrderEngineState>>,
    meta: std::sync::Arc<tokio::sync::Mutex<SnapshotMeta>>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    // Skip the immediate first tick — boot already writes after replay.
    interval.tick().await;
    loop {
        interval.tick().await;
        let state_guard = state.lock().await;
        let meta_guard = meta.lock().await;
        if let Err(err) = crate::memory::snapshot::persist(&pool, &state_guard, &meta_guard).await {
            error!(%err, "periodic snapshot failed");
        }
    }
}
