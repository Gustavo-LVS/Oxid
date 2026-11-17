//! Testes ponta a ponta contra um PostgreSQL real.
//! Defina OXID_TEST_DATABASE_URL (banco descartável) para rodar; sem ela, os testes são pulados.

use futures_util::{SinkExt, StreamExt};
use oxid_proto::voice::VoiceTokenResponse;
use oxid_proto::{
    auth::AuthResponse,
    gateway::{ClientEvent, ServerEvent},
    user::User,
};
use oxid_server_common::{
    config::{Config, LiveKitConfig},
    db,
};

const LK_SECRET: &str = "segredo-livekit-de-teste-com-32-chars!!";
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;

async fn spawn() -> Option<String> {
    let Ok(url) = std::env::var("OXID_TEST_DATABASE_URL") else {
        eprintln!("OXID_TEST_DATABASE_URL não definido; pulando");
        return None;
    };
    let pool = db::connect_and_migrate(&url).await.unwrap();
    let config = Config {
        bind_addr: "127.0.0.1:0".parse().unwrap(),
        database_url: url,
        jwt_secret: "segredo-de-teste-com-mais-de-32-caracteres".into(),
        token_ttl_secs: 3600,
        livekit: Some(LiveKitConfig {
            url: "ws://localhost:7880".into(),
            api_key: "devkey".into(),
            api_secret: LK_SECRET.into(),
        }),
    };
    let listener = tokio::net::TcpListener::bind(config.bind_addr)
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, oxid_server_api::app(pool, &config))
            .await
            .unwrap()
    });
    Some(format!("127.0.0.1:{}", addr.port()))
}

fn unique(prefix: &str) -> String {
    format!(
        "{prefix}{}",
        &uuid::Uuid::now_v7().simple().to_string()[20..]
    )
}

#[tokio::test]
async fn register_login_me_and_gateway() {
    let Some(addr) = spawn().await else { return };
    let http = reqwest::Client::new();
    let base = format!("http://{addr}");
    let username = unique("ana_");
    let email = format!("{username}@exemplo.com");

    // Cadastro
    let res = http
        .post(format!("{base}/v1/auth/register"))
        .json(&json!({"username": username, "email": email, "password": "senha-forte-123"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let reg: AuthResponse = res.json().await.unwrap();
    assert_eq!(reg.user.username, username);

    // Duplicado (case-insensitive)
    let res = http.post(format!("{base}/v1/auth/register"))
        .json(&json!({"username": username.to_uppercase(), "email": "outro@exemplo.com", "password": "senha-forte-123"}))
        .send().await.unwrap();
    assert_eq!(res.status(), 409);

    // Validação
    let res = http
        .post(format!("{base}/v1/auth/register"))
        .json(&json!({"username": "x", "email": "ruim", "password": "123"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 422);

    // Login por e-mail, senha errada e certa
    let res = http
        .post(format!("{base}/v1/auth/login"))
        .json(&json!({"login": email, "password": "errada-errada"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    let login: AuthResponse = http
        .post(format!("{base}/v1/auth/login"))
        .json(&json!({"login": email.to_uppercase(), "password": "senha-forte-123"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(login.user.id, reg.user.id);

    // /users/@me
    assert_eq!(
        http.get(format!("{base}/v1/users/@me"))
            .send()
            .await
            .unwrap()
            .status(),
        401
    );
    let me: User = http
        .get(format!("{base}/v1/users/@me"))
        .bearer_auth(&login.token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(me.id, reg.user.id);

    // Gateway: Hello -> Identify -> Ready -> Heartbeat -> Ack
    let (mut ws, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/v1/gateway"))
        .await
        .unwrap();
    let next = |m: Message| serde_json::from_str::<ServerEvent>(m.to_text().unwrap()).unwrap();
    assert!(matches!(
        next(ws.next().await.unwrap().unwrap()),
        ServerEvent::Hello { version: 1, .. }
    ));
    let send = |e: ClientEvent| Message::Text(serde_json::to_string(&e).unwrap().into());
    ws.send(send(ClientEvent::Identify {
        token: login.token.clone(),
    }))
    .await
    .unwrap();
    match next(ws.next().await.unwrap().unwrap()) {
        ServerEvent::Ready { user, .. } => assert_eq!(user.id, reg.user.id),
        other => panic!("esperava Ready, veio {other:?}"),
    }
    ws.send(send(ClientEvent::Heartbeat)).await.unwrap();
    assert_eq!(
        next(ws.next().await.unwrap().unwrap()),
        ServerEvent::HeartbeatAck
    );

    // Gateway com token inválido
    let (mut ws, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/v1/gateway"))
        .await
        .unwrap();
    ws.next().await.unwrap().unwrap();
    ws.send(send(ClientEvent::Identify {
        token: "lixo".into(),
    }))
    .await
    .unwrap();
    assert!(matches!(
        next(ws.next().await.unwrap().unwrap()),
        ServerEvent::InvalidSession { .. }
    ));

    // Token de voz (LiveKit)
    let res = http
        .post(format!("{base}/v1/voice/token"))
        .json(&json!({"room": "sala-1"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 401);
    let res = http
        .post(format!("{base}/v1/voice/token"))
        .bearer_auth(&login.token)
        .json(&json!({"room": "sala com espaço"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 422);
    let voice: VoiceTokenResponse = http
        .post(format!("{base}/v1/voice/token"))
        .bearer_auth(&login.token)
        .json(&json!({"room": "sala-1"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(voice.url, "ws://localhost:7880");
    let mut v = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    v.validate_aud = false;
    let claims = jsonwebtoken::decode::<serde_json::Value>(
        &voice.token,
        &jsonwebtoken::DecodingKey::from_secret(LK_SECRET.as_bytes()),
        &v,
    )
    .unwrap()
    .claims;
    assert_eq!(claims["iss"], "devkey");
    assert_eq!(claims["sub"], reg.user.id.to_string());
    assert_eq!(claims["video"]["room"], "sala-1");
    assert_eq!(claims["video"]["roomJoin"], true);
}
