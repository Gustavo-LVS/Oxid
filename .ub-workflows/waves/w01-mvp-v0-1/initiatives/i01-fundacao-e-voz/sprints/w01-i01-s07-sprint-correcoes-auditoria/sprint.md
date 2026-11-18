---
artifact_id          : w01-i01-s07-sprint-correcoes-auditoria
artifact_type        : sprint_plan
status               : active
context_tier         : T3
updated_at           : 2026-11-18
summary_budget_lines : 480
---

# Sprint: Correções da auditoria (2026-11-18)

## Objective

Corrigir os 7 achados da auditoria do repositório (ponytail-audit, 2026-11-18): updater
que fecha o app no Windows, argon2 sem limite de concorrência, gateway sem reconexão,
testes da API que passam sem rodar, crate `server-sfu` vazio, `validator`/Redis sem uso e
login que revela contas pelo tempo de resposta.

## Source

- Accepted discovery or reviewed preview: relatório da auditoria na sessão de 2026-11-18;
  o Gustavo pediu para atacar todos os pontos com a staff.

## Routing Preflight

- Trigger categories considered: authority (auth/login), runtime-boundary (updater,
  gateway), interoperability (eventos do gateway para a UI), agent-behavior (staff).
- Triggered categories: authority, runtime-boundary.
- Fail-closed result: `preview_ok`. As correções estão definidas no relatório; nenhuma
  muda contrato de API, `proto` ou esquema do banco.

## Discovery Triage

- Outcome: `preview_ok`.
- Outside evidence trigger: documentação do tauri-plugin-updater (a instalação no Windows
  encerra o processo), conferida na fonte do plugin pelo agente desktop.
- Trigger-presumed check: auth muda só por dentro (mesmas respostas HTTP); sem mudança de
  permissão.
- Promotion rule: se o plugin não permitir separar download e instalação, parar e decidir
  com o Gustavo.

## Why Preview Instead Of Discovery

- Routing trigger: defeitos concretos com correção mínima conhecida.
- Discovery trigger check: `none`.
- Promotion rule: n/a.

## Product Increment Contribution

- Contribution type: `direct`.
- Visible increment unblocked: chamada de voz não cai com atualização; chat volta sozinho
  depois de deploy ou queda de rede; servidor resiste a rajada de logins.
- Why not direct: n/a.
- Enabling-chain check: n/a.

## Bet Framing

- Appetite: 1 sessão.
- Success evidence: testes novos falhando antes e passando depois; CI local verde.
- Circuit breaker: se a reconexão exigir mudança no protocolo do gateway, ela sai do
  sprint e vira opção.
- Non-goals: rate limit por IP, presença online, eventos de canal criado, reconexão da voz.
- Deferral path: `options.md` local.

## Scope

1. Updater: só baixa em segundo plano; instala ao clicar em reiniciar (achado 1).
2. Semáforo de hash de senha na API (achado 2).
3. Reconexão automática do gateway no app com espera crescente (achado 3).
4. Testes da API falham sem `OXID_TEST_DATABASE_URL` (achado 4).
5. Remover `crates/server-sfu`, `validator` do workspace e Redis do compose de dev (5 e 6).
6. Login de usuário inexistente gasta o mesmo argon2 (achado 7).

## Scope Completion Rule

When this sprint uses words like `smallest`, `narrow`, or `only as needed`,
they mean the smallest objective-complete vertical slice. Every named owned
surface, required evidence gate, and exit criterion remains in scope unless
the operator explicitly changes the objective.

## Operational Surface Preflight

- Triggered: yes.
- Surfaces: eventos Tauri novos para a UI (`gateway-reconnecting`, `gateway-reconnected`);
  compose de dev sem Redis.
- Owner and lifecycle: `apps/desktop` (eventos), `deploy/docker-compose.yml`.
- Visibility and safety: nenhuma rota HTTP ou evento do gateway muda; o token continua só
  no Rust.
- Inventory impact: `SOURCE_ATLAS.md`, `README.md`, `deploy/README.md`.
- Validation path: testes abaixo.
- Promotion rule: n/a.

## Execution Slices

Divisão da staff (cada arquivo com um único dono):

1. Dudu (desktop): itens 1 e 3 — `updater.rs`, `lib.rs` do desktop, `ui/main.js`.
2. Dudu (servidor): itens 2, 4 e 6 — `server-api` (`auth.rs`, `state.rs`, `lib.rs`, `tests/api.rs`).
3. Edu: item 5 — `crates/server-sfu`, `Cargo.toml`, `Cargo.lock`, compose e docs.
4. Opus: plano, revisão de cada entrega, validação final, artefatos do workflow.

## Validation Plan

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `OXID_TEST_DATABASE_URL=postgres://oxid:oxid@localhost:5432/oxid_test cargo test --workspace`
4. Teste de tempo do login: falha antes e passa depois.

## Live Validation Matrix

| Gate | Status | Command | Credentials / env | Evidence path | Pass criteria | Redaction |
| --- | --- | --- | --- | --- | --- | --- |
| `ci_local` | pending | fmt + clippy + test (acima) | `OXID_TEST_DATABASE_URL` | `evidence/index.md` | tudo verde | Sem segredos. |
| `updater_real` | operator-deferred | release beta real | segredos do release | n/a | app não fecha durante chamada; instala ao reiniciar | n/a |
| `gateway_reconnect_real` | operator-deferred | app real + reinício da API | API local | n/a | chat volta sozinho | n/a |

## User Or Operator Evidence

- Status: `used`.
- Evidence: pedido do Gustavo para corrigir todos os achados.
- Decision impact: os 7 itens entram no escopo.

## Reviewed-Mode Start Checkpoint

Iniciado em modo `flow`.

## Context Receipt

`status.md`, `SOURCE_ATLAS.md`, sprint s04 (formato), relatório da auditoria.

## Exit Criteria

1. Every accepted objective claim has fresh named passing proof.
2. Required validation and live gates pass after the final relevant change.
3. Evidence is routed through `evidence/index.md`, with required objective
   gates separated from optional, not-triggered, blocked, out-of-scope, or
   operator-deferred checks.
4. Required objective proof is not deferred while closing the original
   objective as passed.
5. Decision log and closeout are current, including the closeout outcome and
   learning review.
