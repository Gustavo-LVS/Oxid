# Oxid

Sistema de chamadas em grupo para gamers e trabalho. Cliente desktop em Tauri 2, servidor em Rust (axum) e voz via SFU (LiveKit no MVP, str0m depois).

## Estrutura (Cargo workspace)

```
crates/
  proto/          tipos compartilhados (DTOs, eventos do gateway)
  server-api/     REST + WebSocket gateway (axum)
  server-sfu/     reservado para o SFU próprio (str0m)
  server-common/  config, banco, auth (argon2 + JWT), telemetria
  client-core/    lógica do cliente
  client-audio/   captura, Opus, supressão de ruído, eco
apps/desktop/     Tauri 2
apps/website/     site de download
migrations/       SQL (sqlx)
deploy/           docker-compose, coturn, Caddy
```

## Rodando local

```bash
docker compose -f deploy/docker-compose.yml up -d
cp .env.example .env   # ajuste OXID_JWT_SECRET
set -a; . ./.env; set +a
cargo run -p oxid-server-api
```

API atual:

| Método | Rota | O que faz |
|---|---|---|
| GET | `/health` | Verificação de vida |
| POST | `/v1/auth/register` | Cadastro (`username`, `email`, `password`) |
| POST | `/v1/auth/login` | Login por usuário ou e-mail; devolve token |
| GET | `/v1/users/@me` | Usuário do token (`Authorization: Bearer`) |
| GET | `/v1/gateway` | WebSocket: Hello → Identify → Ready, heartbeat |
| POST | `/v1/voice/token` | Token do LiveKit para um canal de voz (`channel_id`); só membros |
| GET/POST | `/v1/servers` | Lista meus servidores / cria (`name`), já com #geral e Geral (voz) |
| GET | `/v1/servers/{id}` | Servidor com canais (só membros; outros recebem 404) |
| POST | `/v1/servers/{id}/channels` | Cria canal (`name`, `kind`: `text`/`voice`); só o dono |
| POST | `/v1/servers/{id}/invites` | Cria convite (`max_uses`, `expires_in_secs` opcionais) |
| GET/POST | `/v1/invites/{code}` | Prévia do convite / aceita e entra no servidor |
| GET/POST | `/v1/channels/{id}/messages` | Histórico (`before`, `limit`) / envia mensagem; chega aos membros pelo gateway (`message_create`) |

## App desktop (Tauri 2)

Interface provisória (HTML/JS simples em `apps/desktop/ui`; o design vem depois): login e cadastro, lista de servidores e canais, criar servidor, gerar convite, entrar por código ou link, chat de texto em tempo real e push-to-talk global (Alt+V) com indicador de fala.

```bash
# Linux: sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev librsvg2-dev libayatana-appindicator3-dev libxdo-dev
cargo run -p oxid-desktop
```

## Prova de conceito de voz (passo 2)

Dois clientes entram na mesma sala via LiveKit; cada um publica um tom (no lugar do microfone) e imprime o volume de quem está ouvindo.

Requisito: **clang 21 ou mais novo** (o libwebrtc pré-compilado do LiveKit exige).

```bash
# com a API e o docker-compose rodando, crie dois usuários (ana e bia) via /v1/auth/register.
# Terminal 1: cria um servidor e imprime OXID_INVITE=<código>
OXID_LOGIN=ana OXID_PASSWORD=senha-forte-123 OXID_TONE_HZ=440 cargo run -p oxid-client-core --features voice --example voice_poc
# Terminal 2: entra pelo convite no mesmo canal de voz
OXID_LOGIN=bia OXID_PASSWORD=senha-forte-123 OXID_TONE_HZ=660 OXID_INVITE=<código> cargo run -p oxid-client-core --features voice --example voice_poc
```

Com `OXID_MIC=1`, o cliente usa o microfone com supressão de ruído e ativação por voz no lugar do tom (Linux: `libasound2-dev`).

Sucesso: cada terminal mostra `recebendo áudio de ...` e um volume RMS por volta de 0.2.

## Testes

```bash
OXID_TEST_DATABASE_URL=postgres://oxid:oxid@localhost:5432/oxid_test cargo test --workspace
```

## Planejamento

O projeto usa a skill `ub-workflow`. Comece por `.ub-workflows/status.md`; a visão está em `.ub-workflows/vision.md` e a pesquisa de stack em `.ub-workflows/source-packs/`.
