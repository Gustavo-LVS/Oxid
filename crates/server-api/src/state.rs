use oxid_server_common::{auth::TokenKeys, config::LiveKitConfig};
use sqlx::PgPool;

use crate::hub::Hub;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub tokens: TokenKeys,
    pub livekit: Option<LiveKitConfig>,
    pub hub: Hub,
}
