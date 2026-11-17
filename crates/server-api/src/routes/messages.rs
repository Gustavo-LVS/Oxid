//! Mensagens de texto: histórico via REST e entrega em tempo real pelo gateway.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use oxid_proto::{
    gateway::ServerEvent,
    message::{CreateMessageRequest, MAX_MESSAGE_CHARS, Message},
    server::{Channel, ChannelKind},
    user::User,
};
use serde::Deserialize;
use uuid::Uuid;

use super::servers::{find_channel, is_member};
use crate::{error::ApiError, extract::AuthUser, hub::HubEvent, state::AppState};

#[derive(sqlx::FromRow)]
struct MessageRow {
    id: Uuid,
    channel_id: Uuid,
    content: String,
    created_at: DateTime<Utc>,
    author_id: Uuid,
    username: String,
    display_name: Option<String>,
    avatar_url: Option<String>,
    author_created_at: DateTime<Utc>,
}

impl MessageRow {
    fn into_message(self, server_id: Uuid) -> Message {
        Message {
            id: self.id,
            channel_id: self.channel_id,
            server_id,
            author: User {
                id: self.author_id,
                username: self.username,
                display_name: self.display_name,
                avatar_url: self.avatar_url,
                created_at: self.author_created_at,
            },
            content: self.content,
            created_at: self.created_at,
        }
    }
}

const SELECT: &str =
    "SELECT m.id, m.channel_id, m.content, m.created_at, u.id AS author_id, u.username,
    u.display_name, u.avatar_url, u.created_at AS author_created_at
    FROM messages m JOIN users u ON u.id = m.author_id";

/// Canal de texto visível para o usuário.
async fn text_channel(
    state: &AppState,
    channel_id: Uuid,
    user_id: Uuid,
) -> Result<Channel, ApiError> {
    let channel = find_channel(&state.db, channel_id).await?;
    if !is_member(&state.db, channel.server_id, user_id).await? {
        return Err(ApiError::NotFound("canal"));
    }
    if channel.kind != ChannelKind::Text {
        return Err(ApiError::Validation("o canal não é de texto".into()));
    }
    Ok(channel)
}

#[derive(Deserialize)]
pub struct ListQuery {
    /// Mensagens anteriores a este id (paginação para trás).
    before: Option<Uuid>,
    limit: Option<i64>,
}

/// Histórico em ordem cronológica (mais antiga primeiro).
pub async fn list(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(channel_id): Path<Uuid>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<Message>>, ApiError> {
    let channel = text_channel(&state, channel_id, user_id).await?;
    let limit = q.limit.unwrap_or(50).clamp(1, 100);
    let mut rows = sqlx::query_as::<_, MessageRow>(&format!(
        "{SELECT} WHERE m.channel_id = $1 AND ($2::uuid IS NULL OR m.id < $2) ORDER BY m.id DESC LIMIT $3"
    ))
    .bind(channel_id)
    .bind(q.before)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;
    rows.reverse();
    Ok(Json(
        rows.into_iter()
            .map(|r| r.into_message(channel.server_id))
            .collect(),
    ))
}

pub async fn create(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(channel_id): Path<Uuid>,
    Json(req): Json<CreateMessageRequest>,
) -> Result<(StatusCode, Json<Message>), ApiError> {
    let channel = text_channel(&state, channel_id, user_id).await?;
    let content = req.content.trim();
    if content.is_empty() || content.chars().count() > MAX_MESSAGE_CHARS {
        return Err(ApiError::Validation(format!(
            "a mensagem deve ter de 1 a {MAX_MESSAGE_CHARS} caracteres"
        )));
    }
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO messages (id, channel_id, author_id, content) VALUES ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(channel_id)
    .bind(user_id)
    .bind(content)
    .execute(&state.db)
    .await?;
    let message = sqlx::query_as::<_, MessageRow>(&format!("{SELECT} WHERE m.id = $1"))
        .bind(id)
        .fetch_one(&state.db)
        .await?
        .into_message(channel.server_id);
    state.hub.publish(HubEvent::ToServer {
        server_id: channel.server_id,
        event: ServerEvent::MessageCreate {
            message: message.clone(),
        },
    });
    Ok((StatusCode::CREATED, Json(message)))
}
