use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Pedido de entrada num canal de voz.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceTokenRequest {
    pub channel_id: Uuid,
}

/// Credenciais para conectar no SFU (LiveKit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceTokenResponse {
    /// URL de sinalização do SFU, ex.: `ws://localhost:7880`.
    pub url: String,
    pub token: String,
    /// Nome da sala no SFU (igual ao id do canal).
    pub room: String,
}
