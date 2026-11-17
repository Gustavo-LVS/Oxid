use oxid_server_common::{auth::TokenKeys, config::LiveKitConfig};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub tokens: TokenKeys,
    pub livekit: Option<LiveKitConfig>,
}
