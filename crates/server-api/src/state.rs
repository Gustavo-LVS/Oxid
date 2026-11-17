use oxid_server_common::auth::TokenKeys;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub tokens: TokenKeys,
}
