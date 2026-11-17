//! Convites por link.

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Duration, Utc};
use oxid_proto::server::{CreateInviteRequest, Invite, InvitePreview, Server};
use rand::{Rng, distr::Alphanumeric};
use uuid::Uuid;

use super::servers::{find_server, is_member, require_member};
use crate::{error::ApiError, extract::AuthUser, state::AppState};

const CODE_LEN: usize = 10;
const MAX_EXPIRY_SECS: i64 = 60 * 60 * 24 * 30;

fn new_code() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(CODE_LEN)
        .map(char::from)
        .collect()
}

#[derive(sqlx::FromRow)]
struct InviteRow {
    code: String,
    server_id: Uuid,
    max_uses: Option<i32>,
    uses: i32,
    expires_at: Option<DateTime<Utc>>,
}

impl InviteRow {
    fn usable(&self) -> bool {
        self.max_uses.is_none_or(|m| self.uses < m)
            && self.expires_at.is_none_or(|e| e > Utc::now())
    }
}

impl From<InviteRow> for Invite {
    fn from(r: InviteRow) -> Self {
        Invite {
            code: r.code,
            server_id: r.server_id,
            max_uses: r.max_uses,
            uses: r.uses,
            expires_at: r.expires_at,
        }
    }
}

const INVITE_COLS: &str = "code, server_id, max_uses, uses, expires_at";

pub async fn create(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(server_id): Path<Uuid>,
    Json(req): Json<CreateInviteRequest>,
) -> Result<(StatusCode, Json<Invite>), ApiError> {
    require_member(&state.db, server_id, user_id).await?;
    if req.max_uses.is_some_and(|m| !(1..=1000).contains(&m)) {
        return Err(ApiError::Validation("max_uses deve ser de 1 a 1000".into()));
    }
    if req
        .expires_in_secs
        .is_some_and(|s| !(60..=MAX_EXPIRY_SECS).contains(&s))
    {
        return Err(ApiError::Validation(
            "expires_in_secs deve ser de 60 s a 30 dias".into(),
        ));
    }
    let expires_at = req
        .expires_in_secs
        .map(|s| Utc::now() + Duration::seconds(s));
    let row = sqlx::query_as::<_, InviteRow>(&format!(
        "INSERT INTO invites (code, server_id, created_by, max_uses, expires_at)
         VALUES ($1, $2, $3, $4, $5) RETURNING {INVITE_COLS}"
    ))
    .bind(new_code())
    .bind(server_id)
    .bind(user_id)
    .bind(req.max_uses)
    .bind(expires_at)
    .fetch_one(&state.db)
    .await?;
    Ok((StatusCode::CREATED, Json(row.into())))
}

async fn find_usable(state: &AppState, code: &str) -> Result<InviteRow, ApiError> {
    let row = sqlx::query_as::<_, InviteRow>(&format!(
        "SELECT {INVITE_COLS} FROM invites WHERE code = $1"
    ))
    .bind(code)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound("convite"))?;
    if row.usable() {
        Ok(row)
    } else {
        Err(ApiError::Gone("convite expirado ou esgotado".into()))
    }
}

pub async fn preview(
    State(state): State<AppState>,
    AuthUser(_): AuthUser,
    Path(code): Path<String>,
) -> Result<Json<InvitePreview>, ApiError> {
    let invite = find_usable(&state, &code).await?;
    let server = find_server(&state.db, invite.server_id).await?;
    let member_count =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM server_members WHERE server_id = $1")
            .bind(server.id)
            .fetch_one(&state.db)
            .await?;
    Ok(Json(InvitePreview {
        code: invite.code,
        server,
        member_count,
    }))
}

/// Aceita o convite. Quem já é membro não consome um uso.
pub async fn accept(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(code): Path<String>,
) -> Result<Json<Server>, ApiError> {
    let invite = find_usable(&state, &code).await?;
    let mut tx = state.db.begin().await?;
    if !is_member(&mut *tx, invite.server_id, user_id).await? {
        // Incremento condicional: dois aceites simultâneos não passam do limite.
        let claimed = sqlx::query_scalar::<_, Uuid>(
            "UPDATE invites SET uses = uses + 1
             WHERE code = $1 AND (max_uses IS NULL OR uses < max_uses) AND (expires_at IS NULL OR expires_at > now())
             RETURNING server_id",
        )
        .bind(&code)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| ApiError::Gone("convite expirado ou esgotado".into()))?;
        sqlx::query("INSERT INTO server_members (server_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(claimed)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
    }
    let server = find_server(&mut *tx, invite.server_id).await?;
    tx.commit().await?;
    Ok(Json(server))
}
