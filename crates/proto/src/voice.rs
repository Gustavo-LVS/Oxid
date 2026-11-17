use serde::{Deserialize, Serialize};

/// Pedido de entrada numa sala de voz.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceTokenRequest {
    pub room: String,
}

/// Credenciais para conectar no SFU (LiveKit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceTokenResponse {
    /// URL de sinalização do SFU, ex.: `ws://localhost:7880`.
    pub url: String,
    pub token: String,
}
