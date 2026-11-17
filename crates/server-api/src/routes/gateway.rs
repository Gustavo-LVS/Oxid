//! WebSocket do gateway: Hello -> Identify -> Ready, com heartbeat.

use std::{collections::HashSet, time::Duration};

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

use tokio::sync::broadcast;

use super::users::find_user;
use crate::{hub::HubEvent, state::AppState};

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
    // Assina o hub antes de ler as associações, para não perder eventos no meio.
    let mut hub = state.hub.subscribe();
    let mut servers: HashSet<Uuid> = match sqlx::query_scalar::<_, Uuid>(
        "SELECT server_id FROM server_members WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
    {
        Ok(ids) => ids.into_iter().collect(),
        Err(e) => {
            tracing::error!(error = ?e, "falha ao carregar servidores do usuário");
            return reject(&mut tx, "erro interno").await;
        }
    };
    let ready = ServerEvent::Ready {
        user,
        session_id: Uuid::now_v7(),
    };
    if tx.send(encode(&ready)).await.is_err() {
        return;
    }
    tracing::info!(%user_id, "sessão de gateway pronta");

    let idle = tokio::time::sleep(HEARTBEAT_INTERVAL * 2);
    tokio::pin!(idle);
    loop {
        tokio::select! {
            // Cliente que some por 2 intervalos sem heartbeat é desconectado.
            () = &mut idle => break,
            incoming = rx.next() => {
                let Some(Ok(msg)) = incoming else { break };
                idle.as_mut().reset(tokio::time::Instant::now() + HEARTBEAT_INTERVAL * 2);
                match decode(&msg) {
                    Some(ClientEvent::Heartbeat) => {
                        if tx.send(encode(&ServerEvent::HeartbeatAck)).await.is_err() {
                            break;
                        }
                    }
                    Some(ClientEvent::SetPresence { status }) => {
                        // TODO: propagar presença aos membros dos mesmos servidores.
                        tracing::debug!(%user_id, ?status, "presença atualizada");
                    }
                    Some(ClientEvent::Identify { .. }) | None => {
                        if matches!(msg, Message::Close(_)) {
                            break;
                        }
                    }
                }
            }
            hub_event = hub.recv() => match hub_event {
                Ok(HubEvent::MemberJoined { server_id, user_id: joined }) if joined == user_id => {
                    servers.insert(server_id);
                }
                Ok(HubEvent::ToServer { server_id, event }) if servers.contains(&server_id) => {
                    if tx.send(encode(&event)).await.is_err() {
                        break;
                    }
                }
                Ok(_) => {}
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    // Cliente lento demais: desconecta para ele reconectar e recarregar o histórico.
                    tracing::warn!(%user_id, perdidos = n, "sessão atrasada, desconectando");
                    break;
                }
                Err(broadcast::error::RecvError::Closed) => break,
            },
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
