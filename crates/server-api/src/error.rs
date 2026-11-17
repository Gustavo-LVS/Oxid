use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use oxid_proto::auth::ApiError as ApiErrorBody;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Validation(String),
    #[error("credenciais inválidas")]
    InvalidCredentials,
    #[error("não autenticado")]
    Unauthorized,
    #[error("{0} já está em uso")]
    Conflict(&'static str),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        Self::Internal(e.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            Self::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "validation"),
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid_credentials"),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            Self::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            Self::Internal(e) => {
                tracing::error!(error = ?e, "erro interno");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal")
            }
        };
        let message = match &self {
            Self::Internal(_) => "erro interno".to_string(),
            other => other.to_string(),
        };
        (
            status,
            Json(ApiErrorBody {
                code: code.into(),
                message,
            }),
        )
            .into_response()
    }
}
