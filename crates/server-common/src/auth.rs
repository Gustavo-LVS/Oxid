//! Hash de senha (Argon2id) e tokens de sessão (JWT HS256).

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("falha ao gerar hash de senha")]
    Hash,
    #[error("token inválido ou expirado")]
    InvalidToken,
}

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| AuthError::Hash)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .map(|parsed| {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok()
        })
        .unwrap_or(false)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone)]
pub struct TokenKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
    ttl_secs: i64,
}

impl TokenKeys {
    pub fn new(secret: &str, ttl_secs: i64) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
            ttl_secs,
        }
    }

    pub fn issue(&self, user_id: Uuid) -> String {
        let now = Utc::now().timestamp();
        let claims = Claims {
            sub: user_id,
            iat: now,
            exp: now + self.ttl_secs,
        };
        encode(&Header::default(), &claims, &self.encoding).expect("HS256 encode não falha")
    }

    pub fn verify(&self, token: &str) -> Result<Uuid, AuthError> {
        decode::<Claims>(token, &self.decoding, &Validation::default())
            .map(|d| d.claims.sub)
            .map_err(|_| AuthError::InvalidToken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_roundtrip() {
        let h = hash_password("segredo-forte").unwrap();
        assert!(verify_password("segredo-forte", &h));
        assert!(!verify_password("errada", &h));
    }

    #[test]
    fn token_roundtrip_and_tamper() {
        let keys = TokenKeys::new("0123456789abcdef0123456789abcdef", 60);
        let id = Uuid::now_v7();
        let t = keys.issue(id);
        assert_eq!(keys.verify(&t).unwrap(), id);
        let other = TokenKeys::new("outro-segredo-outro-segredo-outro!", 60);
        assert!(other.verify(&t).is_err());
    }

    #[test]
    fn expired_token_is_rejected() {
        let keys = TokenKeys::new("0123456789abcdef0123456789abcdef", -120);
        assert!(keys.verify(&keys.issue(Uuid::now_v7())).is_err());
    }
}
