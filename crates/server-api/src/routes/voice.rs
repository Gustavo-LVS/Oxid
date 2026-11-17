//! Emissão de token para salas de voz no LiveKit.
//!
//! A sala no SFU é o id do canal de voz; só membros do servidor entram.

use std::time::Duration;

use axum::{Json, extract::State};
use livekit_api::access_token::{AccessToken, VideoGrants};
use oxid_proto::{
    server::ChannelKind,
    voice::{VoiceTokenRequest, VoiceTokenResponse},
};

use super::{
    servers::{find_channel, is_member},
    users::find_user,
};
use crate::{error::ApiError, extract::AuthUser, state::AppState};

const VOICE_TOKEN_TTL: Duration = Duration::from_secs(10 * 60);

pub async fn token(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(req): Json<VoiceTokenRequest>,
) -> Result<Json<VoiceTokenResponse>, ApiError> {
    let lk = state
        .livekit
        .as_ref()
        .ok_or(ApiError::Unavailable("servidor de voz"))?;
    let channel = find_channel(&state.db, req.channel_id).await?;
    if !is_member(&state.db, channel.server_id, user_id).await? {
        return Err(ApiError::NotFound("canal"));
    }
    if channel.kind != ChannelKind::Voice {
        return Err(ApiError::Validation("o canal não é de voz".into()));
    }
    let room = channel.id.to_string();
    let user = find_user(&state.db, user_id)
        .await?
        .ok_or(ApiError::Unauthorized)?;
    let token = AccessToken::with_api_key(&lk.api_key, &lk.api_secret)
        .with_identity(&user.id.to_string())
        .with_name(user.display_name.as_deref().unwrap_or(&user.username))
        .with_ttl(VOICE_TOKEN_TTL)
        .with_grants(VideoGrants {
            room_join: true,
            room: room.clone(),
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
        room,
    }))
}
