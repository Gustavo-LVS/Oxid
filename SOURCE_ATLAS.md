# Source Atlas

This file routes source-code work before agents read implementation files. It is seeded once by `ub-workflow` bootstrap from visible project roots and then maintained when source boundaries change.

## First-Read Order

For non-trivial source work:

1. Root `AGENTS.md`.
2. `.ub-workflows/status.md`.
3. `SOURCE_ATLAS.md`.
4. Nearest relevant folder `AGENTS.md` when one exists.
5. Selected source files and tests.
6. `.ub-workflows/vision.md` only when product direction matters.

Do not read unrelated folder guidance unless the task crosses that boundary.

## Map Use Rule

Module maps are routing aids, not source truth. Use them to choose files to open first, then inspect source and tests with fast local search before changing behavior. If source ownership changes, update this atlas in the same change.

## Detected Source Routes

Cargo workspace (Rust, edition 2024):

- `crates/proto/` — DTOs e eventos do gateway compartilhados por cliente e servidor. Mudanças aqui quebram os dois lados de propósito.
- `crates/server-common/` — `config` (env), `db` (pool + migrações; `build.rs` recompila quando `migrations/` muda), `auth` (argon2id, JWT HS256), `telemetry`.
- `crates/server-api/` — binário axum. Rotas em `src/routes/` (`auth`, `users`, `gateway`, `voice`, `servers`, `invites`, `messages`); fan-out de eventos em `src/hub.rs`; testes ponta a ponta em `tests/api.rs` (precisam de `OXID_TEST_DATABASE_URL`).
- `crates/server-sfu/` — reservado para SFU próprio em str0m (MVP usa LiveKit).
- `crates/client-core/` — `ApiClient` (HTTP), `gateway` (WebSocket com heartbeat) e `voice::VoiceSession` (feature `voice`, LiveKit; exige clang 21+). Exemplo `examples/voice_poc.rs`.
- `crates/client-audio/` — `frames` (mono 48 kHz, 10 ms), `processor` (RNNoise, ativação por voz, PTT, mudo), `capture` (cpal, feature `capture`). Teste com fala real em `tests/pipeline.rs`.
- `apps/desktop/src-tauri/` — app Tauri 2 (`oxid-desktop`); comandos em `src/lib.rs` usam `client-core`; token fica só no Rust.
- `apps/desktop/ui/` — interface provisória em HTML/CSS/JS sem build (design definitivo depois).
- `apps/website/` — site de download (passo 6), ainda vazio.
- `migrations/` — SQL do sqlx, aplicado ao subir a API.
- `deploy/` — docker-compose de dev (Postgres, Redis, LiveKit).

Validação local: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`.

## Workflow Route

- Workflow state, waves, discoveries, initiatives, sprints, and source packs: start at `.ub-workflows/WORKFLOW_ATLAS.md`, then `.ub-workflows/AGENTS.md`.

## Local Guidance Policy

Local `AGENTS.md` files should answer only what the folder owns, when work starts there, local boundaries, what to inspect before editing, likely tests, and what to avoid. Add local guidance only when a folder has a meaningful responsibility boundary, rule set, or repeated navigation cost.

## Source Route Note

Before editing, identify the owning folder, local guidance read, expected source files, expected tests, and boundaries intentionally avoided.

## Maintenance

Update this file and the nearest relevant folder `AGENTS.md` when a change adds, removes, renames, or moves a source boundary, plugin lane, public API area, runtime/protocol/host responsibility, major package, or test topology.
