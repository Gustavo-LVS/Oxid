//! Cliente HTTP da API do Oxid.

use oxid_proto::{
    auth::{ApiError, AuthResponse, LoginRequest, RegisterRequest},
    voice::{VoiceTokenRequest, VoiceTokenResponse},
};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("falha de rede: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API recusou ({status}): {body:?}")]
    Api { status: u16, body: Option<ApiError> },
}

#[derive(Clone)]
pub struct ApiClient {
    http: reqwest::Client,
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            token: None,
        }
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub async fn register(&mut self, req: &RegisterRequest) -> Result<AuthResponse, ClientError> {
        let res: AuthResponse = self.post("/v1/auth/register", req).await?;
        self.token = Some(res.token.clone());
        Ok(res)
    }

    pub async fn login(
        &mut self,
        login: &str,
        password: &str,
    ) -> Result<AuthResponse, ClientError> {
        let req = LoginRequest {
            login: login.into(),
            password: password.into(),
        };
        let res: AuthResponse = self.post("/v1/auth/login", &req).await?;
        self.token = Some(res.token.clone());
        Ok(res)
    }

    pub async fn voice_token(&self, room: &str) -> Result<VoiceTokenResponse, ClientError> {
        self.post("/v1/voice/token", &VoiceTokenRequest { room: room.into() })
            .await
    }

    async fn post<B: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<R, ClientError> {
        let mut req = self
            .http
            .post(format!("{}{path}", self.base_url))
            .json(body);
        if let Some(t) = &self.token {
            req = req.bearer_auth(t);
        }
        let res = req.send().await?;
        if !res.status().is_success() {
            let status = res.status().as_u16();
            return Err(ClientError::Api {
                status,
                body: res.json().await.ok(),
            });
        }
        Ok(res.json().await?)
    }
}
