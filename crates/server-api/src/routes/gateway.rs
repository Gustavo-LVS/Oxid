//! WebSocket do gateway: Hello -> Identify -> Ready, com heartbeat.

use std::time::Duration;

use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use oxid_proto::{
    GATEWAY_VERSION,
    gateway::{ClientEvent, ServerEvent},
};
use uuid::Uuid;

use super::users::find_user;
use crate::state::AppState;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const IDENTIFY_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn upgrade(State(state): State<AppState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| session(socket, state))
}

fn encode(ev: &ServerEvent) -> Message {
    Message::Text(
        serde_json::to_string(ev)
            .expect("evento serializável")
            .into(),
    )
}

fn decode(msg: &Message) -> Option<ClientEvent> {
    match msg {
        Message::Text(t) => serde_json::from_str(t).ok(),
        _ => None,
    }
}

async fn session(socket: WebSocket, state: AppState) {
    let (mut tx, mut rx) = socket.split();
    let hello = ServerEvent::Hello {
        version: GATEWAY_VERSION,
        heartbeat_interval_ms: HEARTBEAT_INTERVAL.as_millis() as u64,
    };
    if tx.send(encode(&hello)).await.is_err() {
        return;
    }

    // Primeiro evento precisa ser Identify, dentro do prazo.
    let first = tokio::time::timeout(IDENTIFY_TIMEOUT, rx.next()).await;
    let token = match first {
        Ok(Some(Ok(msg))) => match decode(&msg) {
            Some(ClientEvent::Identify { token }) => token,
            _ => return reject(&mut tx, "primeiro evento deve ser identify").await,
        },
        _ => return reject(&mut tx, "identify não recebido").await,
    };
    let user = match state.tokens.verify(&token) {
        Ok(id) => match find_user(&state.db, id).await {
            Ok(Some(u)) => u,
            _ => return reject(&mut tx, "usuário não encontrado").await,
        },
        Err(_) => return reject(&mut tx, "token inválido").await,
    };
    let user_id = user.id;
    let ready = ServerEvent::Ready {
        user,
        session_id: Uuid::now_v7(),
    };
    if tx.send(encode(&ready)).await.is_err() {
        return;
    }
    tracing::info!(%user_id, "sessão de gateway pronta");

    // Cliente que some por 2 intervalos sem heartbeat é desconectado.
    while let Ok(Some(Ok(msg))) = tokio::time::timeout(HEARTBEAT_INTERVAL * 2, rx.next()).await {
        match decode(&msg) {
            Some(ClientEvent::Heartbeat) => {
                if tx.send(encode(&ServerEvent::HeartbeatAck)).await.is_err() {
                    break;
                }
            }
            Some(ClientEvent::SetPresence { status }) => {
                // TODO(passo 3/4): propagar via Redis pub/sub para quem compartilha servidores.
                tracing::debug!(%user_id, ?status, "presença atualizada");
            }
            Some(ClientEvent::Identify { .. }) | None => {
                if matches!(msg, Message::Close(_)) {
                    break;
                }
            }
        }
    }
    tracing::info!(%user_id, "sessão de gateway encerrada");
}

async fn reject<S>(tx: &mut S, reason: &str)
where
    S: futures_util::Sink<Message> + Unpin,
{
    let _ = tx
        .send(encode(&ServerEvent::InvalidSession {
            reason: reason.into(),
        }))
        .await;
    let _ = tx.send(Message::Close(None)).await;
}
