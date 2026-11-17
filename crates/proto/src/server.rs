//! Servidores (comunidades), canais e convites.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelKind {
    Text,
    Voice,
}

impl ChannelKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Voice => "voice",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "text" => Some(Self::Text),
            "voice" => Some(Self::Voice),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub kind: ChannelKind,
    pub position: i32,
}

/// Servidor com seus canais, ordenados por `position`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetail {
    pub server: Server,
    pub channels: Vec<Channel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub kind: ChannelKind,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInviteRequest {
    /// `None` = ilimitado.
    pub max_uses: Option<i32>,
    /// `None` = não expira.
    pub expires_in_secs: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invite {
    pub code: String,
    pub server_id: Uuid,
    pub max_uses: Option<i32>,
    pub uses: i32,
    pub expires_at: Option<DateTime<Utc>>,
}

/// O que aparece antes de aceitar um convite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitePreview {
    pub code: String,
    pub server: Server,
    pub member_count: i64,
}
