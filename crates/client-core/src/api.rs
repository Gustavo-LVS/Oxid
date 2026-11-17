//! Cliente HTTP da API do Oxid.

use oxid_proto::{
    auth::{ApiError, AuthResponse, LoginRequest, RegisterRequest},
    server::{CreateInviteRequest, CreateServerRequest, Invite, Server, ServerDetail},
    voice::{VoiceTokenRequest, VoiceTokenResponse},
};
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

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

    pub async fn create_server(&self, name: &str) -> Result<ServerDetail, ClientError> {
        self.post("/v1/servers", &CreateServerRequest { name: name.into() })
            .await
    }

    pub async fn servers(&self) -> Result<Vec<Server>, ClientError> {
        self.get("/v1/servers").await
    }

    pub async fn server(&self, id: Uuid) -> Result<ServerDetail, ClientError> {
        self.get(&format!("/v1/servers/{id}")).await
    }

    pub async fn create_invite(
        &self,
        server_id: Uuid,
        req: &CreateInviteRequest,
    ) -> Result<Invite, ClientError> {
        self.post(&format!("/v1/servers/{server_id}/invites"), req)
            .await
    }

    pub async fn accept_invite(&self, code: &str) -> Result<Server, ClientError> {
        self.post(&format!("/v1/invites/{code}"), &()).await
    }

    pub async fn voice_token(&self, channel_id: Uuid) -> Result<VoiceTokenResponse, ClientError> {
        self.post("/v1/voice/token", &VoiceTokenRequest { channel_id })
            .await
    }

    async fn get<R: DeserializeOwned>(&self, path: &str) -> Result<R, ClientError> {
        self.send(self.http.get(format!("{}{path}", self.base_url)))
            .await
    }

    async fn post<B: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<R, ClientError> {
        self.send(
            self.http
                .post(format!("{}{path}", self.base_url))
                .json(body),
        )
        .await
    }

    async fn send<R: DeserializeOwned>(
        &self,
        mut req: reqwest::RequestBuilder,
    ) -> Result<R, ClientError> {
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
