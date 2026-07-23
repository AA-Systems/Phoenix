use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use types::balances::AssetBalance;
use types::query::{EngineQuery, EngineReply};
use uuid::Uuid;

#[derive(Debug)]
pub enum EngineQueryError {
    Enqueue,
    Timeout,
    InvalidReply,
}

pub async fn get_balances(
    redis: &mut ConnectionManager,
    queries_stream: &str,
    user_id: Uuid,
    timeout_secs: f64,
) -> Result<Vec<AssetBalance>, EngineQueryError> {
    let request_id = Uuid::new_v4();
    let query = EngineQuery::GetBalances {
        request_id,
        user_id,
    };
    let reply_key = query.reply_key();

    let payload = serde_json::to_string(&query).map_err(|_| EngineQueryError::Enqueue)?;

    redis
        .xadd::<_, _, _, _, String>(queries_stream, "*", &[("payload", payload.as_str())])
        .await
        .map_err(|_| EngineQueryError::Enqueue)?;

    let popped: Option<(String, String)> = redis
        .blpop(&reply_key, timeout_secs)
        .await
        .map_err(|_| EngineQueryError::Timeout)?;

    let Some((_key, body)) = popped else {
        return Err(EngineQueryError::Timeout);
    };

    let reply: EngineReply =
        serde_json::from_str(&body).map_err(|_| EngineQueryError::InvalidReply)?;

    match reply {
        EngineReply::GetBalances {
            request_id: replied_id,
            balances,
        } if replied_id == request_id => Ok(balances),
        _ => Err(EngineQueryError::InvalidReply),
    }
}
