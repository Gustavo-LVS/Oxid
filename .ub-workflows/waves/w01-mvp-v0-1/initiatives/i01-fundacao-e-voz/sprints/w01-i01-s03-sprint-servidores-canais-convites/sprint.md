---
artifact_id          : w01-i01-s03-sprint-servidores-canais-convites
artifact_type        : sprint_plan
status               : closed
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 480
---

# Sprint: Servidores, canais e convites (API) + cliente Tauri

## Objective

Passo 3 do plano: cliente Tauri com a lista de servidores e canais. Primeira fatia (feita): a API de servidores, canais e convites, com o token de voz ligado ao canal. Segunda fatia (feita): o app Tauri consumindo essa API.

## Source

- Accepted discovery or reviewed preview: plano de ação, passo 3; o coordenador liberou seguir com a API enquanto o teste de voz do s02 não roda.

## Routing Preflight

- Trigger categories considered: authority (quem vê/cria o quê), policy, runtime-boundary.
- Triggered categories: authority.
- Fail-closed result: `preview_ok`; regra simples e documentada (membros veem; só o dono cria canais) até cargos na v0.2.

## Discovery Triage

- Outcome: `preview_ok`.
- Outside evidence trigger: `not triggered` — modelo servidor/canal/convite é o do plano (seção 7, MVP).
- Trigger-presumed check: autorização coberta por testes (404 para não-membro, 403 para não-dono).
- Promotion rule: cargos e permissões ficam em `options.md` (v0.2).

## Why Preview Instead Of Discovery

- Routing trigger: escopo definido pelo plano.
- Discovery trigger check: `none`.
- Promotion rule: n/a.

## Product Increment Contribution

- Contribution type: `direct`.
- Visible increment unblocked: amigo entra por link e cai no mesmo canal de voz.
- Why not direct: n/a.
- Enabling-chain check: s02 foi direto.

## Bet Framing

- Appetite: 2 sessões (API + Tauri).
- Success evidence: testes ponta a ponta da API; app Tauri listando servidores e canais.
- Circuit breaker: se a voz do s02 falhar ao vivo, pausar o Tauri e abrir discovery de SFU.
- Non-goals: design final da interface; cargos; chat de texto (s04).
- Deferral path: roadmap e `options.md`.

## Scope

1. Migração `servers`, `server_members`, `channels`, `invites` (feito).
2. Rotas de servidores, canais, convites; voz por `channel_id` (feito).
3. `ApiClient` com os novos métodos; `voice_poc` cria servidor e convite (feito, sem compilar a parte de voz).
4. `apps/desktop` Tauri 2 com login e lista de servidores/canais (feito).

## Scope Completion Rule

When this sprint uses words like `smallest`, `narrow`, or `only as needed`,
they mean the smallest objective-complete vertical slice. Every named owned
surface, required evidence gate, and exit criterion remains in scope unless
the operator explicitly changes the objective.

## Operational Surface Preflight

- Triggered: yes.
- Surfaces: rotas `/v1/servers*`, `/v1/invites/*`; mudança de contrato em `/v1/voice/token` (`room` -> `channel_id`).
- Owner and lifecycle: `crates/server-api/src/routes/{servers,invites,voice}.rs`, `migrations/20261117000002_servers.sql`.
- Visibility and safety: não-membro recebe 404 (não revela existência); aceite de convite com incremento condicional; quem já é membro não gasta uso; código de 10 caracteres alfanuméricos.
- Inventory impact: README, `SOURCE_ATLAS.md`.
- Validation path: `servers_channels_invites_and_voice` em `tests/api.rs`.
- Promotion rule: n/a.

## Execution Slices

1. API (feito).
2. Tauri (feito).

## Validation Plan

1. `cargo test --workspace` com Postgres.
2. `cargo clippy --workspace --all-targets` sem avisos.
3. App Tauri abrindo, logando e listando (feito sob Xvfb).

## Live Validation Matrix

| Gate | Status | Command | Credentials / env | Evidence path | Pass criteria | Redaction |
| --- | --- | --- | --- | --- | --- | --- |
| `servers_api_e2e` | passed | `cargo test --workspace` | `OXID_TEST_DATABASE_URL` | `evidence/index.md` | teste passa | Sem segredos. |
| `tauri_list_servers` | passed | `cargo run -p oxid-desktop` sob Xvfb + xdotool | API local | `docs/screenshots/desktop-servidor.png` | login e lista de servidores/canais | n/a |
| `ui_flows_playwright` | passed | script Playwright com `invoke` ligado à API real | API local | `evidence/index.md` | cadastro, criar servidor, convite, entrar por link, erros | n/a |

## User Or Operator Evidence

- Status: `not triggered`.
- Evidence: n/a.
- Decision impact: regras de autorização seguem o MVP do plano.

## Reviewed-Mode Start Checkpoint

Iniciado em modo `flow`.

## Context Receipt

Plano, s02, status.

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
