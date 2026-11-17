//! App desktop: a interface (HTML/JS em `../ui`) chama estes comandos, que
//! falam com a API usando o `client-core`. O token fica só no lado Rust.

use oxid_client_core::api::{ApiClient, ClientError};
use oxid_proto::{
    auth::{AuthResponse, RegisterRequest},
    server::{CreateInviteRequest, Invite, Server, ServerDetail},
};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Default)]
struct Session(Mutex<Option<ApiClient>>);

type CmdResult<T> = Result<T, String>;

fn msg(e: ClientError) -> String {
    match e {
        ClientError::Api { body: Some(b), .. } => b.message,
        other => other.to_string(),
    }
}

async fn client(session: &Session) -> CmdResult<ApiClient> {
    session
        .0
        .lock()
        .await
        .clone()
        .ok_or_else(|| "faça login primeiro".to_string())
}

#[tauri::command]
async fn login(
    session: tauri::State<'_, Session>,
    api_url: String,
    login: String,
    password: String,
) -> CmdResult<AuthResponse> {
    let mut api = ApiClient::new(api_url.trim_end_matches('/'));
    let res = api.login(&login, &password).await.map_err(msg)?;
    *session.0.lock().await = Some(api);
    Ok(res)
}

#[tauri::command]
async fn register(
    session: tauri::State<'_, Session>,
    api_url: String,
    username: String,
    email: String,
    password: String,
) -> CmdResult<AuthResponse> {
    let mut api = ApiClient::new(api_url.trim_end_matches('/'));
    let res = api
        .register(&RegisterRequest {
            username,
            email,
            password,
        })
        .await
        .map_err(msg)?;
    *session.0.lock().await = Some(api);
    Ok(res)
}

#[tauri::command]
async fn logout(session: tauri::State<'_, Session>) -> CmdResult<()> {
    *session.0.lock().await = None;
    Ok(())
}

#[tauri::command]
async fn list_servers(session: tauri::State<'_, Session>) -> CmdResult<Vec<Server>> {
    client(&session).await?.servers().await.map_err(msg)
}

#[tauri::command]
async fn server_detail(
    session: tauri::State<'_, Session>,
    server_id: Uuid,
) -> CmdResult<ServerDetail> {
    client(&session).await?.server(server_id).await.map_err(msg)
}

#[tauri::command]
async fn create_server(
    session: tauri::State<'_, Session>,
    name: String,
) -> CmdResult<ServerDetail> {
    client(&session)
        .await?
        .create_server(&name)
        .await
        .map_err(msg)
}

#[tauri::command]
async fn create_invite(session: tauri::State<'_, Session>, server_id: Uuid) -> CmdResult<Invite> {
    client(&session)
        .await?
        .create_invite(server_id, &CreateInviteRequest::default())
        .await
        .map_err(msg)
}

#[tauri::command]
async fn accept_invite(session: tauri::State<'_, Session>, code: String) -> CmdResult<Server> {
    // Aceita tanto o código quanto um link terminado no código.
    let code = code
        .trim()
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .to_string();
    client(&session)
        .await?
        .accept_invite(&code)
        .await
        .map_err(msg)
}

pub fn run() {
    tauri::Builder::default()
        .manage(Session::default())
        .invoke_handler(tauri::generate_handler![
            login,
            register,
            logout,
            list_servers,
            server_detail,
            create_server,
            create_invite,
            accept_invite
        ])
        .run(tauri::generate_context!())
        .expect("falha ao iniciar o Oxid");
}
