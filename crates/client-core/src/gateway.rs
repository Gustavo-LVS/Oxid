//! Conexão com o gateway WebSocket: identifica, mantém heartbeat e entrega os
//! eventos do servidor num canal.

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use oxid_proto::gateway::{ClientEvent, ServerEvent};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("falha no WebSocket: {0}")]
    Ws(Box<tokio_tungstenite::tungstenite::Error>),
    #[error("protocolo inesperado: {0}")]
    Protocol(String),
    #[error("sessão recusada: {0}")]
    InvalidSession(String),
}

// Box: o erro do tungstenite é grande e deixaria todo Result pesado (clippy::result_large_err).
impl From<tokio_tungstenite::tungstenite::Error> for GatewayError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::Ws(Box::new(e))
    }
}

/// Conexão ativa. Ao ser descartada, encerra a tarefa de fundo.
pub struct Gateway {
    pub events: mpsc::Receiver<ServerEvent>,
    task: JoinHandle<()>,
}

impl Drop for Gateway {
    fn drop(&mut self) {
        self.task.abort();
    }
}

fn ws_url(api_base: &str) -> String {
    let base = api_base
        .replacen("https://", "wss://", 1)
        .replacen("http://", "ws://", 1);
    format!("{}/v1/gateway", base.trim_end_matches('/'))
}

fn text(ev: &ClientEvent) -> Message {
    Message::Text(
        serde_json::to_string(ev)
            .expect("evento serializável")
            .into(),
    )
}

fn parse(msg: Message) -> Option<ServerEvent> {
    match msg {
        Message::Text(t) => serde_json::from_str(&t).ok(),
        _ => None,
    }
}

/// Conecta, envia Identify e espera o Ready (que é o primeiro evento entregue).
pub async fn connect(api_base: &str, token: &str) -> Result<Gateway, GatewayError> {
    let (ws, _) = tokio_tungstenite::connect_async(ws_url(api_base)).await?;
    let (mut tx, mut rx) = ws.split();

    let interval = match rx.next().await.transpose()?.and_then(parse) {
        Some(ServerEvent::Hello {
            heartbeat_interval_ms,
            ..
        }) => Duration::from_millis(heartbeat_interval_ms),
        other => {
            return Err(GatewayError::Protocol(format!(
                "esperava Hello, veio {other:?}"
            )));
        }
    };
    tx.send(text(&ClientEvent::Identify {
        token: token.into(),
    }))
    .await?;
    let ready = match rx.next().await.transpose()?.and_then(parse) {
        Some(ev @ ServerEvent::Ready { .. }) => ev,
        Some(ServerEvent::InvalidSession { reason }) => {
            return Err(GatewayError::InvalidSession(reason));
        }
        other => {
            return Err(GatewayError::Protocol(format!(
                "esperava Ready, veio {other:?}"
            )));
        }
    };

    let (events_tx, events) = mpsc::channel(256);
    let _ = events_tx.send(ready).await;
    let task = tokio::spawn(async move {
        let mut beat = tokio::time::interval(interval);
        beat.tick().await;
        loop {
            tokio::select! {
                _ = beat.tick() => {
                    if tx.send(text(&ClientEvent::Heartbeat)).await.is_err() {
                        break;
                    }
                }
                incoming = rx.next() => {
                    let Some(Ok(msg)) = incoming else { break };
                    if let Some(ev) = parse(msg) {
                        if matches!(ev, ServerEvent::HeartbeatAck) {
                            continue;
                        }
                        if events_tx.send(ev).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
        tracing::info!("gateway desconectado");
    });
    Ok(Gateway { events, task })
}

#[cfg(test)]
mod tests {
    #[test]
    fn ws_url_from_http() {
        assert_eq!(
            super::ws_url("http://localhost:8080/"),
            "ws://localhost:8080/v1/gateway"
        );
        assert_eq!(
            super::ws_url("https://api.oxid.app"),
            "wss://api.oxid.app/v1/gateway"
        );
    }
}
