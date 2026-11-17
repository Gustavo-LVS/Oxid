//! Testes ponta a ponta contra um PostgreSQL real.
//! Defina OXID_TEST_DATABASE_URL (banco descartável) para rodar; sem ela, os testes são pulados.

use futures_util::{SinkExt, StreamExt};
use oxid_proto::{
    auth::AuthResponse,
    gateway::{ClientEvent, ServerEvent},
    user::User,
};
use oxid_proto::{
    server::{Channel, ChannelKind, Invite, InvitePreview, Server, ServerDetail},
    voice::VoiceTokenResponse,
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
}

async fn register(http: &reqwest::Client, base: &str, prefix: &str) -> AuthResponse {
    let username = unique(prefix);
    let res = http
        .post(format!("{base}/v1/auth/register"))
        .json(&json!({"username": username, "email": format!("{username}@exemplo.com"), "password": "senha-forte-123"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    res.json().await.unwrap()
}

#[tokio::test]
async fn servers_channels_invites_and_voice() {
    let Some(addr) = spawn().await else { return };
    let http = reqwest::Client::new();
    let base = format!("http://{addr}");
    let ana = register(&http, &base, "ana_").await;
    let bia = register(&http, &base, "bia_").await;

    // Ana cria um servidor: vem com #geral (texto) e Geral (voz)
    let res = http
        .post(format!("{base}/v1/servers"))
        .bearer_auth(&ana.token)
        .json(&json!({"name": "  Esquadrão  "}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let detail: ServerDetail = res.json().await.unwrap();
    assert_eq!(detail.server.name, "Esquadrão");
    assert_eq!(detail.server.owner_id, ana.user.id);
    let kinds: Vec<_> = detail
        .channels
        .iter()
        .map(|c| (c.name.as_str(), c.kind, c.position))
        .collect();
    assert_eq!(
        kinds,
        [
            ("geral", ChannelKind::Text, 0),
            ("Geral", ChannelKind::Voice, 1)
        ]
    );
    let sid = detail.server.id;
    let text_ch = detail.channels[0].id;
    let voice_ch = detail.channels[1].id;

    // Nome inválido
    let res = http
        .post(format!("{base}/v1/servers"))
        .bearer_auth(&ana.token)
        .json(&json!({"name": "   "}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 422);

    // Bia não é membro: não vê o servidor nem pega token de voz
    let res = http
        .get(format!("{base}/v1/servers/{sid}"))
        .bearer_auth(&bia.token)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);
    let res = http
        .post(format!("{base}/v1/voice/token"))
        .bearer_auth(&bia.token)
        .json(&json!({"channel_id": voice_ch}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);
    let res = http
        .post(format!("{base}/v1/servers/{sid}/invites"))
        .bearer_auth(&bia.token)
        .json(&json!({}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 404);

    // Convite de 1 uso
    let res = http
        .post(format!("{base}/v1/servers/{sid}/invites"))
        .bearer_auth(&ana.token)
        .json(&json!({"max_uses": 1, "expires_in_secs": 3600}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let invite: Invite = res.json().await.unwrap();
    assert_eq!(invite.code.len(), 10);
    let res = http
        .post(format!("{base}/v1/servers/{sid}/invites"))
        .bearer_auth(&ana.token)
        .json(&json!({"max_uses": 0}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 422);

    let preview: InvitePreview = http
        .get(format!("{base}/v1/invites/{}", invite.code))
        .bearer_auth(&bia.token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!((preview.server.id, preview.member_count), (sid, 1));
    assert_eq!(
        http.get(format!("{base}/v1/invites/naoexiste"))
            .bearer_auth(&bia.token)
            .send()
            .await
            .unwrap()
            .status(),
        404
    );

    // Ana já é membro: aceitar não consome o uso
    let s: Server = http
        .post(format!("{base}/v1/invites/{}", invite.code))
        .bearer_auth(&ana.token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(s.id, sid);
    // Bia entra com o convite
    let s: Server = http
        .post(format!("{base}/v1/invites/{}", invite.code))
        .bearer_auth(&bia.token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(s.id, sid);
    // Convite esgotado para um terceiro
    let caio = register(&http, &base, "caio_").await;
    assert_eq!(
        http.post(format!("{base}/v1/invites/{}", invite.code))
            .bearer_auth(&caio.token)
            .send()
            .await
            .unwrap()
            .status(),
        410
    );

    // Bia agora lista e vê o servidor
    let mine: Vec<Server> = http
        .get(format!("{base}/v1/servers"))
        .bearer_auth(&bia.token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(mine.iter().map(|s| s.id).collect::<Vec<_>>(), [sid]);
    let d: ServerDetail = http
        .get(format!("{base}/v1/servers/{sid}"))
        .bearer_auth(&bia.token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(d.channels.len(), 2);

    // Só a dona cria canais
    let res = http
        .post(format!("{base}/v1/servers/{sid}/channels"))
        .bearer_auth(&bia.token)
        .json(&json!({"name": "ranked", "kind": "voice"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 403);
    let ch: Channel = http
        .post(format!("{base}/v1/servers/{sid}/channels"))
        .bearer_auth(&ana.token)
        .json(&json!({"name": "ranked", "kind": "voice"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!((ch.kind, ch.position), (ChannelKind::Voice, 2));

    // Token de voz: só em canal de voz, sala = id do canal
    let res = http
        .post(format!("{base}/v1/voice/token"))
        .bearer_auth(&bia.token)
        .json(&json!({"channel_id": text_ch}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 422);
    assert_eq!(
        http.post(format!("{base}/v1/voice/token"))
            .json(&json!({"channel_id": voice_ch}))
            .send()
            .await
            .unwrap()
            .status(),
        401
    );
    let voice: VoiceTokenResponse = http
        .post(format!("{base}/v1/voice/token"))
        .bearer_auth(&bia.token)
        .json(&json!({"channel_id": voice_ch}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(voice.url, "ws://localhost:7880");
    assert_eq!(voice.room, voice_ch.to_string());
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
    assert_eq!(claims["sub"], bia.user.id.to_string());
    assert_eq!(claims["video"]["room"], voice_ch.to_string());
    assert_eq!(claims["video"]["roomJoin"], true);
}
