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
        })
    }
}
