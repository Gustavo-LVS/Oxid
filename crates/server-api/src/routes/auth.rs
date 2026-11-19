use std::sync::LazyLock;

use axum::{Json, extract::State, http::StatusCode};
use oxid_proto::auth::{AuthResponse, LoginRequest, RegisterRequest};
use oxid_server_common::auth::{hash_password, verify_password};
use uuid::Uuid;

use super::users::UserRow;
use crate::{error::ApiError, state::AppState};

// Hash fixo para gastar o mesmo tempo de argon2 quando o usuário não existe.
static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| {
    hash_password("senha-inexistente").expect("hash de uma constante não deve falhar")
});

/// Roda argon2 fora do runtime, com no máximo `ARGON2_PERMITS` ao mesmo tempo.
async fn run_argon2<T: Send + 'static>(
    state: &AppState,
    f: impl FnOnce() -> T + Send + 'static,
) -> Result<T, ApiError> {
    let _permit = state
        .argon2_permits
        .acquire()
        .await
        .map_err(anyhow::Error::from)?;
    Ok(tokio::task::spawn_blocking(f)
        .await
        .map_err(anyhow::Error::from)?)
}

fn validate(req: &RegisterRequest) -> Result<(), ApiError> {
    let u = req.username.trim();
    if !(2..=32).contains(&u.chars().count())
        || !u
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
    {
        return Err(ApiError::Validation(
            "nome de usuário deve ter 2 a 32 caracteres: letras, números, _ ou .".into(),
        ));
    }
    let e = req.email.trim();
    if e.len() > 254 || !e.contains('@') || e.starts_with('@') || e.ends_with('@') {
        return Err(ApiError::Validation("e-mail inválido".into()));
    }
    if !(8..=128).contains(&req.password.chars().count()) {
        return Err(ApiError::Validation(
            "a senha deve ter de 8 a 128 caracteres".into(),
        ));
    }
    Ok(())
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ApiError> {
    validate(&req)?;
    let password = req.password.clone();
    let hash = run_argon2(&state, move || hash_password(&password))
        .await?
        .map_err(anyhow::Error::from)?;

    let row = sqlx::query_as::<_, UserRow>(
        "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)
         RETURNING id, username, display_name, avatar_url, created_at",
    )
    .bind(Uuid::now_v7())
    .bind(req.username.trim())
    .bind(req.email.trim())
    .bind(hash)
    .fetch_one(&state.db)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db) if db.constraint() == Some("users_username_lower_idx") => {
            ApiError::Conflict("nome de usuário")
        }
        sqlx::Error::Database(db) if db.constraint() == Some("users_email_lower_idx") => {
            ApiError::Conflict("e-mail")
        }
        _ => e.into(),
    })?;

    let token = state.tokens.issue(row.id);
    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: row.into(),
        }),
    ))
}

#[derive(sqlx::FromRow)]
struct LoginRow {
    password_hash: String,
    #[sqlx(flatten)]
    user: UserRow,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let row = sqlx::query_as::<_, LoginRow>(
        "SELECT id, username, display_name, avatar_url, created_at, password_hash FROM users
         WHERE lower(username) = lower($1) OR lower(email) = lower($1)",
    )
    .bind(req.login.trim())
    .fetch_optional(&state.db)
    .await?;

    let Some(row) = row else {
        // Mesmo custo do caso "senha errada", para o tempo não revelar quem existe.
        run_argon2(&state, move || verify_password(&req.password, &DUMMY_HASH)).await?;
        return Err(ApiError::InvalidCredentials);
    };
    let (password, hash) = (req.password, row.password_hash.clone());
    let ok = run_argon2(&state, move || verify_password(&password, &hash)).await?;
    if !ok {
        return Err(ApiError::InvalidCredentials);
    }
    let token = state.tokens.issue(row.user.id);
    Ok(Json(AuthResponse {
        token,
        user: row.user.into(),
    }))
}
