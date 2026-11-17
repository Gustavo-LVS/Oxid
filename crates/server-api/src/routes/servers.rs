//! Servidores (comunidades) e canais.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use oxid_proto::server::{
    Channel, ChannelKind, CreateChannelRequest, CreateServerRequest, Server, ServerDetail,
};
use sqlx::{PgExecutor, PgPool};
use uuid::Uuid;

use crate::{error::ApiError, extract::AuthUser, state::AppState};

#[derive(sqlx::FromRow)]
pub(crate) struct ServerRow {
    id: Uuid,
    name: String,
    owner_id: Uuid,
    created_at: DateTime<Utc>,
}

impl From<ServerRow> for Server {
    fn from(r: ServerRow) -> Self {
        Server {
            id: r.id,
            name: r.name,
            owner_id: r.owner_id,
            created_at: r.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
pub(crate) struct ChannelRow {
    id: Uuid,
    server_id: Uuid,
    name: String,
    kind: String,
    position: i32,
}

impl TryFrom<ChannelRow> for Channel {
    type Error = ApiError;
    fn try_from(r: ChannelRow) -> Result<Self, ApiError> {
        let kind = ChannelKind::parse(&r.kind).ok_or_else(|| {
            ApiError::Internal(anyhow::anyhow!("tipo de canal desconhecido: {}", r.kind))
        })?;
        Ok(Channel {
            id: r.id,
            server_id: r.server_id,
            name: r.name,
            kind,
            position: r.position,
        })
    }
}

const SERVER_COLS: &str = "s.id, s.name, s.owner_id, s.created_at";

pub(crate) fn validate_name(name: &str, what: &str) -> Result<String, ApiError> {
    let n = name.trim();
    if (1..=100).contains(&n.chars().count()) {
        Ok(n.to_string())
    } else {
        Err(ApiError::Validation(format!(
            "{what} deve ter de 1 a 100 caracteres"
        )))
    }
}

pub(crate) async fn find_server<'e>(db: impl PgExecutor<'e>, id: Uuid) -> Result<Server, ApiError> {
    sqlx::query_as::<_, ServerRow>(&format!(
        "SELECT {SERVER_COLS} FROM servers s WHERE s.id = $1"
    ))
    .bind(id)
    .fetch_optional(db)
    .await?
    .map(Into::into)
    .ok_or(ApiError::NotFound("servidor"))
}

pub(crate) async fn is_member<'e>(
    db: impl PgExecutor<'e>,
    server_id: Uuid,
    user_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM server_members WHERE server_id = $1 AND user_id = $2)",
    )
    .bind(server_id)
    .bind(user_id)
    .fetch_one(db)
    .await?)
}

/// Garante que o usuário é membro. Para não revelar que o servidor existe,
/// quem não é membro recebe 404.
pub(crate) async fn require_member(
    db: &PgPool,
    server_id: Uuid,
    user_id: Uuid,
) -> Result<Server, ApiError> {
    let server = find_server(db, server_id).await?;
    if is_member(db, server_id, user_id).await? {
        Ok(server)
    } else {
        Err(ApiError::NotFound("servidor"))
    }
}

async fn list_channels(db: &PgPool, server_id: Uuid) -> Result<Vec<Channel>, ApiError> {
    sqlx::query_as::<_, ChannelRow>(
        "SELECT id, server_id, name, kind, position FROM channels WHERE server_id = $1 ORDER BY position, created_at",
    )
    .bind(server_id)
    .fetch_all(db)
    .await?
    .into_iter()
    .map(TryInto::try_into)
    .collect()
}

pub(crate) async fn find_channel(db: &PgPool, id: Uuid) -> Result<Channel, ApiError> {
    sqlx::query_as::<_, ChannelRow>(
        "SELECT id, server_id, name, kind, position FROM channels WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or(ApiError::NotFound("canal"))?
    .try_into()
}

async fn insert_channel<'e>(
    db: impl PgExecutor<'e>,
    server_id: Uuid,
    name: &str,
    kind: ChannelKind,
) -> Result<Channel, ApiError> {
    sqlx::query_as::<_, ChannelRow>(
        "INSERT INTO channels (id, server_id, name, kind, position)
         VALUES ($1, $2, $3, $4, (SELECT COALESCE(MAX(position) + 1, 0) FROM channels WHERE server_id = $2))
         RETURNING id, server_id, name, kind, position",
    )
    .bind(Uuid::now_v7())
    .bind(server_id)
    .bind(name)
    .bind(kind.as_str())
    .fetch_one(db)
    .await?
    .try_into()
}

/// Cria o servidor com o dono como membro e dois canais padrão.
pub async fn create(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(req): Json<CreateServerRequest>,
) -> Result<(StatusCode, Json<ServerDetail>), ApiError> {
    let name = validate_name(&req.name, "o nome do servidor")?;
    let mut tx = state.db.begin().await?;
    let server: Server = sqlx::query_as::<_, ServerRow>(
        "INSERT INTO servers (id, name, owner_id) VALUES ($1, $2, $3) RETURNING id, name, owner_id, created_at",
    )
    .bind(Uuid::now_v7())
    .bind(&name)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?
    .into();
    sqlx::query("INSERT INTO server_members (server_id, user_id) VALUES ($1, $2)")
        .bind(server.id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;
    let text = insert_channel(&mut *tx, server.id, "geral", ChannelKind::Text).await?;
    let voice = insert_channel(&mut *tx, server.id, "Geral", ChannelKind::Voice).await?;
    tx.commit().await?;
    Ok((
        StatusCode::CREATED,
        Json(ServerDetail {
            server,
            channels: vec![text, voice],
        }),
    ))
}

/// Servidores dos quais o usuário é membro.
pub async fn list(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<Server>>, ApiError> {
    let rows = sqlx::query_as::<_, ServerRow>(&format!(
        "SELECT {SERVER_COLS} FROM servers s JOIN server_members m ON m.server_id = s.id
         WHERE m.user_id = $1 ORDER BY m.joined_at"
    ))
    .bind(user_id)
    .fetch_all(&state.db)
    .await?;
    Ok(Json(rows.into_iter().map(Into::into).collect()))
}

pub async fn detail(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(server_id): Path<Uuid>,
) -> Result<Json<ServerDetail>, ApiError> {
    let server = require_member(&state.db, server_id, user_id).await?;
    let channels = list_channels(&state.db, server_id).await?;
    Ok(Json(ServerDetail { server, channels }))
}

/// Só o dono cria canais (cargos e permissões chegam na v0.2).
pub async fn create_channel(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(server_id): Path<Uuid>,
    Json(req): Json<CreateChannelRequest>,
) -> Result<(StatusCode, Json<Channel>), ApiError> {
    let server = require_member(&state.db, server_id, user_id).await?;
    if server.owner_id != user_id {
        return Err(ApiError::Forbidden);
    }
    let name = validate_name(&req.name, "o nome do canal")?;
    let channel = insert_channel(&state.db, server_id, &name, req.kind).await?;
    Ok((StatusCode::CREATED, Json(channel)))
}
