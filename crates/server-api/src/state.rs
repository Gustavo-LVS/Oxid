use oxid_server_common::{auth::TokenKeys, config::LiveKitConfig};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::hub::Hub;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub tokens: TokenKeys,
    pub livekit: Option<LiveKitConfig>,
    pub hub: Hub,
    /// Limita hashes argon2 simultâneos (~19 MiB cada) para não estourar a memória da VM de 1 GB.
    pub argon2_permits: Arc<Semaphore>,
}
