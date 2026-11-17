use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use oxid_proto::user::User;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::ApiError, extract::AuthUser, state::AppState};

#[derive(sqlx::FromRow)]
pub(crate) struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        User {
            id: r.id,
            username: r.username,
            display_name: r.display_name,
            avatar_url: r.avatar_url,
            created_at: r.created_at,
        }
    }
}

pub(crate) async fn find_user(db: &PgPool, id: Uuid) -> Result<Option<User>, ApiError> {
    let row = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, display_name, avatar_url, created_at FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;
    Ok(row.map(Into::into))
}

pub async fn me(
    State(state): State<AppState>,
    AuthUser(id): AuthUser,
) -> Result<Json<User>, ApiError> {
    find_user(&state.db, id)
        .await?
        .map(Json)
        .ok_or(ApiError::Unauthorized)
}
