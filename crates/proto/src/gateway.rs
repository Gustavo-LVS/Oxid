//! Eventos trocados pelo WebSocket do gateway.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{message::Message, user::User};

/// Mensagens enviadas pelo servidor ao cliente.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", content = "d", rename_all = "snake_case")]
pub enum ServerEvent {
    /// Primeira mensagem após conectar.
    Hello {
        version: u32,
        heartbeat_interval_ms: u64,
    },
    /// Enviado após autenticação bem-sucedida.
    Ready { user: User, session_id: Uuid },
    /// Resposta a um heartbeat.
    HeartbeatAck,
    /// Mudança de presença de um usuário.
    PresenceUpdate {
        user_id: Uuid,
        status: PresenceStatus,
    },
    /// Nova mensagem num canal de texto de um servidor do usuário.
    MessageCreate { message: Message },
    /// Erro fatal; o servidor fecha a conexão em seguida.
    InvalidSession { reason: String },
}

/// Mensagens enviadas pelo cliente ao servidor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", content = "d", rename_all = "snake_case")]
pub enum ClientEvent {
    Identify { token: String },
    Heartbeat,
    SetPresence { status: PresenceStatus },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresenceStatus {
    Online,
    Idle,
    DoNotDisturb,
    Offline,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_event_wire_format_is_stable() {
        let json = serde_json::to_string(&ClientEvent::Identify { token: "t".into() }).unwrap();
        assert_eq!(json, r#"{"op":"identify","d":{"token":"t"}}"#);
        let back: ClientEvent = serde_json::from_str(r#"{"op":"heartbeat"}"#).unwrap();
        assert_eq!(back, ClientEvent::Heartbeat);
    }

    #[test]
    fn server_event_roundtrip() {
        let ev = ServerEvent::Hello {
            version: crate::GATEWAY_VERSION,
            heartbeat_interval_ms: 30_000,
        };
        let json = serde_json::to_string(&ev).unwrap();
        assert_eq!(serde_json::from_str::<ServerEvent>(&json).unwrap(), ev);
    }
}
