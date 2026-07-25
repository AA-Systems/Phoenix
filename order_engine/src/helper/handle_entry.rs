use std::collections::HashMap;

use redis::{Value, aio::ConnectionManager};
use sqlx::PgPool;
use tracing::{error, info, warn};
use types::command::Command;

use crate::{
    commands::apply_command::{
        ApplyOutcome, apply_command_effects, commit_command, revert_intents,
    },
    events::{
        build_events::{affected_book_markets, build_exchange_events, snapshot_books},
        publish::publish_exchange_events,
    },
    helper::{ack::ack, field_as_bytes::field_as_bytes},
    memory::{
        OrderEngineState,
        replay::{maybe_snapshot, touch_cursor},
        snapshot::SnapshotMeta,
    },
};

pub async fn handle_entry(
    conn: &mut ConnectionManager,
    pool: &PgPool,
    state: &mut OrderEngineState,
    meta: &mut SnapshotMeta,
    commands_since: &mut u32,
    stream: &str,
    order_stream: &str,
    group: &str,
    entry_id: &str,
    fields: &HashMap<String, Value>,
    events_stream: &str,
) {
    let payload = match field_as_bytes(fields, "payload") {
        Some(bytes) => bytes,
        None => {
            warn!(%entry_id, "missing payload field; acking");
            ack(conn, stream, group, entry_id).await;
            return;
        }
    };

    let command = match serde_json::from_slice::<Command>(&payload) {
        Ok(command) => command,
        Err(err) => {
            error!(%entry_id, %err, "failed to deserialize Command");
            ack(conn, stream, group, entry_id).await;
            return;
        }
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
        Ok(ApplyOutcome::AlreadyProcessed) => {
            info!(%command_id, %entry_id, "command already processed");
            touch_cursor(meta, stream, entry_id, order_stream);
            ack(conn, stream, group, entry_id).await;
        }
        Ok(ApplyOutcome::Applied {
            command_id,
            intents,
        }) => {
            let publish = async |state: &OrderEngineState, conn: &mut ConnectionManager| {
                let events = build_exchange_events(
                    state,
                    &command_for_events,
                    &intents,
                    trades_before,
                    &books_before,
                );
                if !events.is_empty() {
                    publish_exchange_events(conn, events_stream, &events).await;
                }
            };

            if intents.is_empty() {
                commit_command(state, command_id);
                publish(state, conn).await;
                touch_cursor(meta, stream, entry_id, order_stream);
                *commands_since += 1;
                maybe_snapshot(pool, state, meta, commands_since, false).await;
                info!(%command_id, %entry_id, "applied command");
                ack(conn, stream, group, entry_id).await;
                return;
            }

            match db::balances::persist_intents::persist_intents(pool, &intents).await {
                Ok(()) => {
                    commit_command(state, command_id);
                    publish(state, conn).await;
                    touch_cursor(meta, stream, entry_id, order_stream);
                    *commands_since += 1;
                    maybe_snapshot(pool, state, meta, commands_since, false).await;
                    info!(
                        %command_id,
                        %entry_id,
                        ledger_rows = intents.len(),
                        "applied command and persisted ledger"
                    );
                    ack(conn, stream, group, entry_id).await;
                }
                Err(err) => {
                    revert_intents(state, &intents);
                    error!(
                        %command_id,
                        %entry_id,
                        %err,
                        "failed to persist ledger; requeued"
                    );
                }
            }
        }
        Err(err) => {
            error!(%command_id, %entry_id, ?err, "apply_command failed");
            ack(conn, stream, group, entry_id).await;
        }
    }
}
