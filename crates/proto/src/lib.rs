//! Protocolo compartilhado do Oxid.
//!
//! Qualquer mudança aqui quebra a compilação do cliente e do servidor ao mesmo
//! tempo, o que mantém os dois lados em sincronia.

pub mod auth;
pub mod gateway;
pub mod user;

/// Versão do protocolo do gateway. Incrementar em mudanças incompatíveis.
pub const GATEWAY_VERSION: u32 = 1;
pub mod message;
pub mod server;
pub mod voice;
