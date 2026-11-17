use std::{env, net::SocketAddr};

use anyhow::{Context, Result};

/// Configuração lida de variáveis de ambiente.
#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub jwt_secret: String,
    /// Validade do token de sessão, em segundos.
    pub token_ttl_secs: i64,
    /// Credenciais do SFU. Sem elas, o endpoint de voz responde 503.
    pub livekit: Option<LiveKitConfig>,
}

#[derive(Debug, Clone)]
pub struct LiveKitConfig {
    /// URL que o cliente usa para conectar, ex.: `ws://localhost:7880`.
    pub url: String,
    pub api_key: String,
    pub api_secret: String,
}

impl LiveKitConfig {
    fn from_env() -> Option<Self> {
        Some(Self {
            url: env::var("LIVEKIT_URL").ok()?,
            api_key: env::var("LIVEKIT_API_KEY").ok()?,
            api_secret: env::var("LIVEKIT_API_SECRET").ok()?,
        })
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let jwt_secret = env::var("OXID_JWT_SECRET").context("OXID_JWT_SECRET não definido")?;
        anyhow::ensure!(
            jwt_secret.len() >= 32,
            "OXID_JWT_SECRET precisa de pelo menos 32 caracteres"
        );
        Ok(Self {
            bind_addr: env::var("OXID_BIND_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:8080".into())
                .parse()
                .context("OXID_BIND_ADDR inválido")?,
            database_url: env::var("DATABASE_URL").context("DATABASE_URL não definido")?,
            jwt_secret,
            token_ttl_secs: env::var("OXID_TOKEN_TTL_SECS")
                .ok()
                .map(|v| v.parse())
                .transpose()
                .context("OXID_TOKEN_TTL_SECS inválido")?
                .unwrap_or(60 * 60 * 24 * 30),
            livekit: LiveKitConfig::from_env(),
        })
    }
}
