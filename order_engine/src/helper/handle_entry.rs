use std::collections::HashMap;

use redis::{Value, aio::ConnectionManager};
use tracing::{error, info, warn};
use types::command::Command;

use crate::{
    commands::apply_command::apply_command,
    helper::{ack::ack, field_as_bytes::field_as_bytes},
    memory::OrderEngineState,
};

pub async fn handle_entry(
    conn: &mut ConnectionManager,
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

    match serde_json::from_slice::<Command>(&payload) {
        Ok(command) => {
            let command_id = command.command_id();
            match apply_command(state, command) {
                Ok(()) => info!(%command_id, %entry_id, "applied command"),
                Err(err) => error!(%command_id, %entry_id, ?err, "apply_command failed"),
            }
        }
        Err(err) => error!(%entry_id, %err, "failed to deserialize Command"),
    }

    ack(conn, stream, group, entry_id).await;
}
