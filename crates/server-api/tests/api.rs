//! Testes ponta a ponta contra um PostgreSQL real.
//! Defina OXID_TEST_DATABASE_URL (banco descartável) para rodar; sem ela, os testes falham.

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

async fn spawn() -> String {
    let url = std::env::var("OXID_TEST_DATABASE_URL").expect(
        "OXID_TEST_DATABASE_URL não definido: suba o banco com `docker compose -f deploy/docker-compose.yml up -d postgres` e defina a variável (ex.: postgres://oxid:oxid@localhost:5432/oxid_test)",
    );
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
    format!("127.0.0.1:{}", addr.port())
}

fn unique(prefix: &str) -> String {
    format!(
        "{prefix}{}",
        &uuid::Uuid::now_v7().simple().to_string()[20..]
    )
}

#[tokio::test]
async fn register_login_me_and_gateway() {
    let addr = spawn().await;
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
    let addr = spawn().await;
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

type Ws =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

async fn gateway(addr: &str, token: &str) -> Ws {
    let (mut ws, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/v1/gateway"))
        .await
        .unwrap();
    ws.next().await.unwrap().unwrap(); // Hello
    let identify = ClientEvent::Identify {
        token: token.into(),
    };
    ws.send(Message::Text(
        serde_json::to_string(&identify).unwrap().into(),
    ))
    .await
    .unwrap();
    assert!(matches!(
        next_event(&mut ws).await,
        ServerEvent::Ready { .. }
    ));
    ws
}

async fn next_event(ws: &mut Ws) -> ServerEvent {
    let msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws.next())
        .await
        .expect("nenhum evento em 5 s")
        .unwrap()
        .unwrap();
    serde_json::from_str(msg.to_text().unwrap()).unwrap()
}

#[tokio::test]
async fn text_chat_history_and_realtime() {
    let addr = spawn().await;
    let http = reqwest::Client::new();
    let base = format!("http://{addr}");
    let ana = register(&http, &base, "ana_").await;
    let bia = register(&http, &base, "bia_").await;
    let caio = register(&http, &base, "caio_").await;

    let detail: ServerDetail = http
        .post(format!("{base}/v1/servers"))
        .bearer_auth(&ana.token)
        .json(&json!({"name": "Chat"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let text = detail.channels[0].id;
    let voice = detail.channels[1].id;
    let invite: Invite = http
        .post(format!("{base}/v1/servers/{}/invites", detail.server.id))
        .bearer_auth(&ana.token)
        .json(&json!({}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // Bia conecta no gateway ANTES de entrar no servidor; Caio nunca entra.
    let mut bia_ws = gateway(&addr, &bia.token).await;
    let mut caio_ws = gateway(&addr, &caio.token).await;
    let res = http
        .post(format!("{base}/v1/invites/{}", invite.code))
        .bearer_auth(&bia.token)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    // Não-membro não lê nem escreve
    let url = format!("{base}/v1/channels/{text}/messages");
    assert_eq!(
        http.get(&url)
            .bearer_auth(&caio.token)
            .send()
            .await
            .unwrap()
            .status(),
        404
    );
    assert_eq!(
        http.post(&url)
            .bearer_auth(&caio.token)
            .json(&json!({"content": "oi"}))
            .send()
            .await
            .unwrap()
            .status(),
        404
    );
    // Validação e canal de voz
    assert_eq!(
        http.post(&url)
            .bearer_auth(&ana.token)
            .json(&json!({"content": "   "}))
            .send()
            .await
            .unwrap()
            .status(),
        422
    );
    let long = "a".repeat(4001);
    assert_eq!(
        http.post(&url)
            .bearer_auth(&ana.token)
            .json(&json!({"content": long}))
            .send()
            .await
            .unwrap()
            .status(),
        422
    );
    assert_eq!(
        http.post(format!("{base}/v1/channels/{voice}/messages"))
            .bearer_auth(&ana.token)
            .json(&json!({"content": "oi"}))
            .send()
            .await
            .unwrap()
            .status(),
        422
    );

    // Ana manda; Bia recebe em tempo real
    let res = http
        .post(&url)
        .bearer_auth(&ana.token)
        .json(&json!({"content": "  bora jogar?  "}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);
    let sent: oxid_proto::message::Message = res.json().await.unwrap();
    assert_eq!(sent.content, "bora jogar?");
    match next_event(&mut bia_ws).await {
        ServerEvent::MessageCreate { message } => {
            assert_eq!(message, sent);
            assert_eq!(message.author.username, ana.user.username);
        }
        other => panic!("esperava MessageCreate, veio {other:?}"),
    }
    // Caio não recebe: o próximo evento dele é a resposta ao heartbeat
    caio_ws
        .send(Message::Text(
            serde_json::to_string(&ClientEvent::Heartbeat)
                .unwrap()
                .into(),
        ))
        .await
        .unwrap();
    assert_eq!(next_event(&mut caio_ws).await, ServerEvent::HeartbeatAck);

    // Histórico em ordem e paginação
    for i in 1..=3 {
        http.post(&url)
            .bearer_auth(&bia.token)
            .json(&json!({"content": format!("msg {i}")}))
            .send()
            .await
            .unwrap();
    }
    let all: Vec<oxid_proto::message::Message> = http
        .get(&url)
        .bearer_auth(&bia.token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let contents: Vec<_> = all.iter().map(|m| m.content.as_str()).collect();
    assert_eq!(contents, ["bora jogar?", "msg 1", "msg 2", "msg 3"]);
    let page: Vec<oxid_proto::message::Message> = http
        .get(format!("{url}?limit=2&before={}", all[3].id))
        .bearer_auth(&bia.token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        page.iter().map(|m| m.content.as_str()).collect::<Vec<_>>(),
        ["msg 1", "msg 2"]
    );
}

#[tokio::test]
async fn login_time_does_not_reveal_whether_user_exists() {
    let addr = spawn().await;
    let http = reqwest::Client::new();
    let base = format!("http://{addr}");
    let username = unique("tim_");
    let res = http
        .post(format!("{base}/v1/auth/register"))
        .json(&json!({"username": username, "email": format!("{username}@exemplo.com"), "password": "senha-forte-123"}))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 201);

    let time_login = |login: String| {
        let (http, url) = (http.clone(), format!("{base}/v1/auth/login"));
        async move {
            let t = std::time::Instant::now();
            let res = http
                .post(url)
                .json(&json!({"login": login, "password": "errada-errada"}))
                .send()
                .await
                .unwrap();
            assert_eq!(res.status(), 401);
            t.elapsed()
        }
    };
    let existing = time_login(username).await;
    let missing = time_login(unique("nao_existe_")).await;
    eprintln!("existente={existing:?} inexistente={missing:?}");
    // Sem a equalização, o inexistente responde sem rodar argon2 (muito mais rápido).
    assert!(
        missing * 4 >= existing,
        "login de usuário inexistente ({missing:?}) bem mais rápido que o de existente ({existing:?})"
    );
}
