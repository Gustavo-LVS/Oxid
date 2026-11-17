//! App desktop: a interface (HTML/JS em `../ui`) chama estes comandos, que
//! falam com a API usando o `client-core`. O token fica só no lado Rust.

mod updater;

use oxid_client_core::{
    api::{ApiClient, ClientError},
    gateway,
};
use oxid_proto::{
    auth::{AuthResponse, RegisterRequest},
    message::Message,
    server::{CreateInviteRequest, Invite, Server, ServerDetail},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tokio::{sync::Mutex, task::JoinHandle};
use uuid::Uuid;

#[derive(Default)]
struct Session {
    api: Mutex<Option<ApiClient>>,
    /// Tarefa que repassa eventos do gateway para a interface.
    gateway: Mutex<Option<JoinHandle<()>>>,
}

/// Evento emitido para a interface quando o gateway cai.
const GATEWAY_EVENT: &str = "gateway";
const GATEWAY_CLOSED: &str = "gateway-closed";

/// Abre o gateway e repassa cada evento do servidor para a interface.
async fn start_session(app: &AppHandle, session: &Session, api: ApiClient) -> CmdResult<()> {
    let token = api.token().unwrap_or_default().to_string();
    let mut gw = gateway::connect(api.base_url(), &token)
        .await
        .map_err(|e| e.to_string())?;
    let app = app.clone();
    let task = tokio::spawn(async move {
        while let Some(ev) = gw.events.recv().await {
            let _ = app.emit(GATEWAY_EVENT, ev);
        }
        let _ = app.emit(GATEWAY_CLOSED, ());
    });
    if let Some(old) = session.gateway.lock().await.replace(task) {
        old.abort();
    }
    *session.api.lock().await = Some(api);
    Ok(())
}

type CmdResult<T> = Result<T, String>;

fn msg(e: ClientError) -> String {
    match e {
        ClientError::Api { body: Some(b), .. } => b.message,
        other => other.to_string(),
    }
}

async fn client(session: &Session) -> CmdResult<ApiClient> {
    session
        .api
        .lock()
        .await
        .clone()
        .ok_or_else(|| "faça login primeiro".to_string())
}

#[tauri::command]
async fn login(
    app: AppHandle,
    session: tauri::State<'_, Session>,
    api_url: String,
    login: String,
    password: String,
) -> CmdResult<AuthResponse> {
    let mut api = ApiClient::new(api_url.trim_end_matches('/'));
    let res = api.login(&login, &password).await.map_err(msg)?;
    start_session(&app, &session, api).await?;
    Ok(res)
}

#[tauri::command]
async fn register(
    app: AppHandle,
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
    start_session(&app, &session, api).await?;
    Ok(res)
}

#[tauri::command]
async fn logout(session: tauri::State<'_, Session>) -> CmdResult<()> {
    *session.api.lock().await = None;
    if let Some(task) = session.gateway.lock().await.take() {
        task.abort();
    }
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

#[tauri::command]
async fn list_messages(
    session: tauri::State<'_, Session>,
    channel_id: Uuid,
    before: Option<Uuid>,
) -> CmdResult<Vec<Message>> {
    client(&session)
        .await?
        .messages(channel_id, before)
        .await
        .map_err(msg)
}

#[tauri::command]
async fn send_message(
    session: tauri::State<'_, Session>,
    channel_id: Uuid,
    content: String,
) -> CmdResult<Message> {
    client(&session)
        .await?
        .send_message(channel_id, &content)
        .await
        .map_err(msg)
}

/// Estado do push-to-talk. O `AudioProcessor` (client-audio) lê este mesmo
/// `AtomicBool` quando a voz estiver no app.
#[derive(Default, Clone)]
pub struct PushToTalk(pub Arc<AtomicBool>);

/// Atalho global padrão do push-to-talk: Alt+V (funciona com o jogo em foco).
fn ptt_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::ALT), Code::KeyV)
}

#[tauri::command]
fn ptt_shortcut_label() -> String {
    "Alt+V".into()
}

/// Reinicia o app para aplicar a atualização já baixada.
#[tauri::command]
fn restart_to_update(app: AppHandle) {
    app.restart();
}

#[tauri::command]
fn pending_update(state: tauri::State<'_, updater::PendingUpdate>) -> Option<updater::UpdateReady> {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
fn update_channel() -> &'static str {
    updater::CHANNEL
}

pub fn run() {
    let ptt = PushToTalk::default();
    let ptt_state = ptt.0.clone();
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if shortcut != &ptt_shortcut() {
                        return;
                    }
                    let pressed = event.state() == ShortcutState::Pressed;
                    // Ignora repetição de tecla segurada.
                    if ptt_state.swap(pressed, Ordering::Relaxed) != pressed {
                        let _ = app.emit("ptt", pressed);
                    }
                })
                .build(),
        )
        .plugin({
            let mut b = tauri_plugin_updater::Builder::new();
            if let Some(key) = updater::pubkey() {
                b = b.pubkey(key);
            }
            b.build()
        })
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            tauri::async_runtime::spawn(updater::run(app.handle().clone()));
            if let Err(e) = app.global_shortcut().register(ptt_shortcut()) {
                // Outro programa pode já usar o atalho; o app segue sem push-to-talk.
                eprintln!("não foi possível registrar o atalho de push-to-talk: {e}");
            }
            Ok(())
        })
        .manage(ptt)
        .manage(updater::PendingUpdate::default())
        .manage(Session::default())
        .invoke_handler(tauri::generate_handler![
            login,
            register,
            logout,
            list_servers,
            server_detail,
            create_server,
            create_invite,
            accept_invite,
            list_messages,
            send_message,
            ptt_shortcut_label,
            restart_to_update,
            update_channel,
            pending_update
        ])
        .run(tauri::generate_context!())
        .expect("falha ao iniciar o Oxid");
}
