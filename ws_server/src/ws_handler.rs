use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::auth::JwtVerifier;
use crate::hub::{Hub, channel_needs_auth, require_market};
use crate::protocol::{Channel, ClientMessage, ServerMessage};

#[derive(Clone)]
pub struct WsState {
    pub hub: Hub,
    pub jwt: JwtVerifier,
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<WsState>) -> impl IntoResponse {
    let hub = state.hub.clone();
    let jwt = state.jwt.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, hub, jwt))
}

async fn handle_socket(socket: WebSocket, hub: Hub, jwt: JwtVerifier) {
    let (mut sink, mut stream) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let client_id = hub.register(tx).await;

    info!(%client_id, "websocket connected");

    if let Ok(payload) = serde_json::to_string(&ServerMessage::Ready) {
        let _ = sink.send(Message::Text(payload.into())).await;
    }

    let write = async {
        while let Some(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    };

    let read = async {
        while let Some(Ok(msg)) = stream.next().await {
            match msg {
                Message::Text(text) => {
                    if let Err(err) = handle_client_text(&hub, &jwt, client_id, text.as_str()).await
                    {
                        warn!(%client_id, %err, "client message error");
                        send_error(&hub, client_id, &err).await;
                    }
                }
                Message::Ping(_) | Message::Pong(_) => {}
                Message::Close(_) => break,
                Message::Binary(_) => {
                    send_error(&hub, client_id, "binary frames are not supported").await;
                }
            }
        }
    };

    tokio::select! {
        _ = write => {},
        _ = read => {},
    }

    hub.unregister(client_id).await;
    info!(%client_id, "websocket disconnected");
}

async fn send_json(hub: &Hub, client_id: u64, message: &ServerMessage<'_>) {
    if let Ok(payload) = serde_json::to_string(message) {
        hub.send_raw(client_id, payload).await;
    }
}

async fn send_error(hub: &Hub, client_id: u64, message: &str) {
    send_json(hub, client_id, &ServerMessage::Error { message }).await;
}

async fn handle_client_text(
    hub: &Hub,
    jwt: &JwtVerifier,
    client_id: u64,
    text: &str,
) -> Result<(), String> {
    let msg: ClientMessage =
        serde_json::from_str(text).map_err(|err| format!("invalid message: {err}"))?;

    match msg {
        ClientMessage::Ping => {
            send_json(hub, client_id, &ServerMessage::Pong).await;
        }
        ClientMessage::Auth { token } => {
            let user_id = jwt
                .verify(&token)
                .map_err(|err| format!("auth failed: {err}"))?;
            hub.set_user(client_id, user_id).await;
            send_json(hub, client_id, &ServerMessage::Authenticated { user_id }).await;
        }
        ClientMessage::Subscribe { channel, market } => {
            let market = apply_subscribe(hub, client_id, channel, market.as_deref()).await?;
            send_json(
                hub,
                client_id,
                &ServerMessage::Subscribed {
                    channel,
                    market: market.as_deref(),
                },
            )
            .await;
        }
        ClientMessage::Unsubscribe { channel, market } => {
            let market = apply_unsubscribe(hub, client_id, channel, market.as_deref()).await?;
            send_json(
                hub,
                client_id,
                &ServerMessage::Unsubscribed {
                    channel,
                    market: market.as_deref(),
                },
            )
            .await;
        }
    }

    Ok(())
}

async fn apply_subscribe(
    hub: &Hub,
    client_id: u64,
    channel: Channel,
    market: Option<&str>,
) -> Result<Option<String>, String> {
    if channel_needs_auth(channel) && hub.user_id(client_id).await.is_none() {
        return Err("authenticate before subscribing to private channels".into());
    }

    match channel {
        Channel::Orderbook => {
            let market = require_market(market)?;
            hub.subscribe_orderbook(client_id, market.clone()).await;
            Ok(Some(market))
        }
        Channel::Trades => {
            let market = require_market(market)?;
            hub.subscribe_trades(client_id, market.clone()).await;
            Ok(Some(market))
        }
        Channel::Balances => {
            hub.subscribe_balances(client_id).await;
            Ok(None)
        }
        Channel::OpenOrders => {
            hub.subscribe_open_orders(client_id).await;
            Ok(None)
        }
    }
}

async fn apply_unsubscribe(
    hub: &Hub,
    client_id: u64,
    channel: Channel,
    market: Option<&str>,
) -> Result<Option<String>, String> {
    match channel {
        Channel::Orderbook => {
            let market = require_market(market)?;
            hub.unsubscribe_orderbook(client_id, &market).await;
            Ok(Some(market))
        }
        Channel::Trades => {
            let market = require_market(market)?;
            hub.unsubscribe_trades(client_id, &market).await;
            Ok(Some(market))
        }
        Channel::Balances => {
            hub.unsubscribe_balances(client_id).await;
            Ok(None)
        }
        Channel::OpenOrders => {
            hub.unsubscribe_open_orders(client_id).await;
            Ok(None)
        }
    }
}
