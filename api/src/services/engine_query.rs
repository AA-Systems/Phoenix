use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use types::balances::AssetBalance;
use types::order::OpenOrderView;
use types::orderbook::OrderBookDepth;
use types::query::{EngineQuery, EngineReply};
use uuid::Uuid;

#[derive(Debug)]
pub enum EngineQueryError {
    Enqueue,
    Timeout,
    InvalidReply,
    NotFound,
}

async fn request_reply(
    redis: &mut ConnectionManager,
    queries_stream: &str,
    query: EngineQuery,
    timeout_secs: f64,
) -> Result<EngineReply, EngineQueryError> {
    let request_id = query.request_id();
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

    if reply.request_id() != request_id {
        return Err(EngineQueryError::InvalidReply);
    }

    Ok(reply)
}

pub async fn get_balances(
    redis: &mut ConnectionManager,
    queries_stream: &str,
    user_id: Uuid,
    timeout_secs: f64,
) -> Result<Vec<AssetBalance>, EngineQueryError> {
    let request_id = Uuid::new_v4();
    let reply = request_reply(
        redis,
        queries_stream,
        EngineQuery::GetBalances {
            request_id,
            user_id,
        },
        timeout_secs,
    )
    .await?;

    match reply {
        EngineReply::GetBalances { balances, .. } => Ok(balances),
        _ => Err(EngineQueryError::InvalidReply),
    }
}

pub async fn get_open_orders(
    redis: &mut ConnectionManager,
    queries_stream: &str,
    user_id: Uuid,
    timeout_secs: f64,
) -> Result<Vec<OpenOrderView>, EngineQueryError> {
    let request_id = Uuid::new_v4();
    let reply = request_reply(
        redis,
        queries_stream,
        EngineQuery::GetOpenOrders {
            request_id,
            user_id,
        },
        timeout_secs,
    )
    .await?;

    match reply {
        EngineReply::GetOpenOrders { orders, .. } => Ok(orders),
        _ => Err(EngineQueryError::InvalidReply),
    }
}

pub async fn get_order_book(
    redis: &mut ConnectionManager,
    queries_stream: &str,
    market_symbol: String,
    timeout_secs: f64,
) -> Result<OrderBookDepth, EngineQueryError> {
    let request_id = Uuid::new_v4();
    let reply = request_reply(
        redis,
        queries_stream,
        EngineQuery::GetOrderBook {
            request_id,
            market_symbol,
        },
        timeout_secs,
    )
    .await?;

    match reply {
        EngineReply::GetOrderBook {
            book: Some(book), ..
        } => Ok(book),
        EngineReply::GetOrderBook { book: None, .. } => Err(EngineQueryError::NotFound),
        _ => Err(EngineQueryError::InvalidReply),
    }
}
