use std::collections::HashMap;

use redis::{Value, aio::ConnectionManager};
use sqlx::PgPool;
use tracing::{error, info, warn};
use types::command::Command;

use crate::{
    commands::apply_command::{
        ApplyOutcome, apply_command_effects, commit_command, revert_intents,
    },
    helper::{ack::ack, field_as_bytes::field_as_bytes},
    memory::OrderEngineState,
};

pub async fn handle_entry(
    conn: &mut ConnectionManager,
    pool: &PgPool,
    state: &mut OrderEngineState,
    stream: &str,
    group: &str,
    entry_id: &str,
    fields: &HashMap<String, Value>,
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
    match apply_command_effects(state, command) {
        Ok(ApplyOutcome::AlreadyProcessed) => {
            info!(%command_id, %entry_id, "command already processed");
            ack(conn, stream, group, entry_id).await;
        }
        Ok(ApplyOutcome::Applied {
            command_id,
            intents,
        }) => {
            if intents.is_empty() {
                commit_command(state, command_id);
                info!(%command_id, %entry_id, "applied command");
                ack(conn, stream, group, entry_id).await;
                return;
            }

            match db::balances::persist_intents::persist_intents(pool, &intents).await {
                Ok(()) => {
                    commit_command(state, command_id);
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
