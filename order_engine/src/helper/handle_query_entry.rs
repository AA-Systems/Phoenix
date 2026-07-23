use std::collections::HashMap;
use std::sync::Arc;

use redis::{Value, aio::ConnectionManager, AsyncCommands};
use tokio::sync::Mutex;
use tracing::{error, info, warn};
use types::query::EngineQuery;

use crate::{
    helper::{ack::ack, field_as_bytes::field_as_bytes},
    memory::OrderEngineState,
    queries::answer_query::answer_query,
};

pub async fn handle_query_entry(
    conn: &mut ConnectionManager,
    state: &Arc<Mutex<OrderEngineState>>,
    stream: &str,
    group: &str,
    entry_id: &str,
    fields: &HashMap<String, Value>,
) {
    let payload = match field_as_bytes(fields, "payload") {
        Some(bytes) => bytes,
        None => {
            warn!(%entry_id, "missing query payload; acking");
            ack(conn, stream, group, entry_id).await;
            return;
        }
    };

    let query = match serde_json::from_slice::<EngineQuery>(&payload) {
        Ok(query) => query,
        Err(err) => {
            error!(%entry_id, %err, "failed to deserialize EngineQuery");
            ack(conn, stream, group, entry_id).await;
            return;
        }
    };

    let request_id = query.request_id();
    let reply_key = query.reply_key();
    let reply = {
        let guard = state.lock().await;
        answer_query(&guard, query)
    };

    let body = match serde_json::to_string(&reply) {
        Ok(body) => body,
        Err(err) => {
            error!(%request_id, %err, "failed to serialize EngineReply");
            ack(conn, stream, group, entry_id).await;
            return;
        }
    };

    let push: Result<(), redis::RedisError> = conn.lpush(&reply_key, &body).await;
    if let Err(err) = push {
        error!(%request_id, %reply_key, %err, "failed to push engine reply");
        return;
    }

    let _: Result<(), redis::RedisError> = conn.expire(&reply_key, 30).await;
    info!(%request_id, %entry_id, "answered engine query");
    ack(conn, stream, group, entry_id).await;
}
