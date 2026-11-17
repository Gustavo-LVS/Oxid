//! Emissão de token para salas de voz no LiveKit.
//!
//! Prova de conceito (passo 2): qualquer usuário autenticado entra em qualquer
//! sala. No passo 3 a sala passa a ser um canal de voz e a permissão vem dele.

use std::time::Duration;

use axum::{Json, extract::State};
use livekit_api::access_token::{AccessToken, VideoGrants};
use oxid_proto::voice::{VoiceTokenRequest, VoiceTokenResponse};

use super::users::find_user;
use crate::{error::ApiError, extract::AuthUser, state::AppState};

const VOICE_TOKEN_TTL: Duration = Duration::from_secs(10 * 60);

fn valid_room(room: &str) -> bool {
    (1..=64).contains(&room.len())
        && room
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
}

pub async fn token(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(req): Json<VoiceTokenRequest>,
) -> Result<Json<VoiceTokenResponse>, ApiError> {
    let lk = state
        .livekit
        .as_ref()
        .ok_or(ApiError::Unavailable("servidor de voz"))?;
    if !valid_room(&req.room) {
        return Err(ApiError::Validation(
            "sala deve ter 1 a 64 caracteres: letras, números, - ou _".into(),
        ));
    }
    let user = find_user(&state.db, user_id)
        .await?
        .ok_or(ApiError::Unauthorized)?;
    let token = AccessToken::with_api_key(&lk.api_key, &lk.api_secret)
        .with_identity(&user.id.to_string())
        .with_name(user.display_name.as_deref().unwrap_or(&user.username))
        .with_ttl(VOICE_TOKEN_TTL)
        .with_grants(VideoGrants {
            room_join: true,
            room: req.room,
            can_publish: Some(true),
            can_subscribe: Some(true),
            can_publish_data: Some(true),
            ..Default::default()
        })
        .to_jwt()
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("falha ao gerar token LiveKit: {e}")))?;
    Ok(Json(VoiceTokenResponse {
        url: lk.url.clone(),
        token,
    }))
}

#[cfg(test)]
mod tests {
    use super::valid_room;

    #[test]
    fn room_names() {
        assert!(valid_room("sala-1_teste"));
        assert!(!valid_room(""));
        assert!(!valid_room("com espaço"));
        assert!(!valid_room(&"a".repeat(65)));
    }
}
