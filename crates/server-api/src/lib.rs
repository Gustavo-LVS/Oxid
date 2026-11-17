//! Gateway REST + WebSocket.

mod error;
mod extract;
mod hub;
mod routes;
mod state;

use axum::{
    Router,
    routing::{get, post},
};
use oxid_server_common::{auth::TokenKeys, config::Config};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub use state::AppState;

pub fn app(pool: PgPool, config: &Config) -> Router {
    let state = AppState {
        db: pool,
        tokens: TokenKeys::new(&config.jwt_secret, config.token_ttl_secs),
        livekit: config.livekit.clone(),
        hub: Default::default(),
    };
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/v1/auth/register", post(routes::auth::register))
        .route("/v1/auth/login", post(routes::auth::login))
        .route("/v1/users/@me", get(routes::users::me))
        .route("/v1/gateway", get(routes::gateway::upgrade))
        .route("/v1/voice/token", post(routes::voice::token))
        .route(
            "/v1/servers",
            get(routes::servers::list).post(routes::servers::create),
        )
        .route("/v1/servers/{server_id}", get(routes::servers::detail))
        .route(
            "/v1/servers/{server_id}/channels",
            post(routes::servers::create_channel),
        )
        .route(
            "/v1/servers/{server_id}/invites",
            post(routes::invites::create),
        )
        .route(
            "/v1/channels/{channel_id}/messages",
            get(routes::messages::list).post(routes::messages::create),
        )
        .route(
            "/v1/invites/{code}",
            get(routes::invites::preview).post(routes::invites::accept),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
