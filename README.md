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
docker compose -f deploy/docker-compose.yml up -d postgres redis
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

## Testes

```bash
OXID_TEST_DATABASE_URL=postgres://oxid:oxid@localhost:5432/oxid_test cargo test --workspace
```

## Planejamento

O projeto usa a skill `ub-workflow`. Comece por `.ub-workflows/status.md`; a visão está em `.ub-workflows/vision.md` e a pesquisa de stack em `.ub-workflows/source-packs/`.
