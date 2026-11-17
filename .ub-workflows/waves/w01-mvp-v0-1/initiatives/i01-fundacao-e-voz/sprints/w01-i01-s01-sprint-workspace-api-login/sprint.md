---
artifact_id          : w01-i01-s01-sprint-workspace-api-login
artifact_type        : sprint_plan
status               : closed
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 480
---

# Sprint: Workspace, proto e API com login

## Objective

Passo 1 do plano: Cargo workspace com o crate `proto` compartilhado e a API (axum) com cadastro, login e banco PostgreSQL.

## Source

- Accepted discovery or reviewed preview: plano de ação do Gustavo (2026-11-17), passo 1; o pedido de "passar por todo o plano" vale como aprovação de início.

## Routing Preflight

- Trigger categories considered: runtime-boundary, policy (segurança de senha/token), interoperability (protocolo cliente/servidor).
- Triggered categories: runtime-boundary, policy.
- Fail-closed result: `preview_ok`; stack e bibliotecas já decididas no plano (axum, sqlx, argon2, jsonwebtoken).

## Discovery Triage

- Outcome: `preview_ok`.
- Outside evidence trigger: source pack `2026-11-17-stack-chamadas-webrtc` e documentação das crates.
- Trigger-presumed check: sem mudança de autoridade ou de agentes; segurança coberta por argon2id e JWT com segredo mínimo de 32 caracteres.
- Promotion rule: nada promovido.

## Why Preview Instead Of Discovery

- Routing trigger: rota definida pelo plano.
- Discovery trigger check: `none`.
- Promotion rule: n/a.

## Product Increment Contribution

- Contribution type: `enabling`.
- Visible increment unblocked: login no cliente e emissão de token de voz no s02.
- Why not direct: sem contas não há como autorizar entrada em sala de voz.
- Enabling-chain check: primeiro sprint; s02 é incremento direto (voz).

## Bet Framing

- Appetite: 1 sessão.
- Success evidence: teste ponta a ponta verde contra PostgreSQL real.
- Circuit breaker: n/a.
- Non-goals: servidores/canais, Redis, interface.
- Deferral path: roadmap s03/s04.

## Scope

1. Workspace com `proto`, `server-common`, `server-api` e crates reservados (`server-sfu`, `client-core`, `client-audio`).
2. Migração `users`; cadastro, login por usuário ou e-mail, `/users/@me`.
3. Gateway WebSocket: Hello, Identify, Ready, heartbeat.
4. docker-compose de dev, CI, README.

## Scope Completion Rule

When this sprint uses words like `smallest`, `narrow`, or `only as needed`,
they mean the smallest objective-complete vertical slice. Every named owned
surface, required evidence gate, and exit criterion remains in scope unless
the operator explicitly changes the objective.

## Operational Surface Preflight

- Triggered: yes.
- Surfaces: API HTTP `/v1/*`, gateway `/v1/gateway`, tabela `users`, variáveis `DATABASE_URL`, `OXID_JWT_SECRET`, `OXID_BIND_ADDR`, `OXID_TOKEN_TTL_SECS`.
- Owner and lifecycle: `crates/server-api`, `crates/server-common`, `migrations/`.
- Visibility and safety: senha só como hash argon2id; erro interno não vaza detalhes; segredo JWT obrigatório com 32+ caracteres.
- Inventory impact: `SOURCE_ATLAS.md` e `README.md` atualizados.
- Validation path: `crates/server-api/tests/api.rs`.
- Promotion rule: n/a.

## Execution Slices

1. Workspace + `proto` com testes de formato de fio.
2. `server-common` (config, db, auth, telemetria) com testes de senha e token.
3. `server-api` (rotas, erros, extrator de auth, gateway) com teste ponta a ponta.

## Validation Plan

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets` sem avisos
3. `OXID_TEST_DATABASE_URL=... cargo test --workspace`

## Live Validation Matrix

| Gate | Status | Command | Credentials / env | Evidence path | Pass criteria | Redaction |
| --- | --- | --- | --- | --- | --- | --- |
| `postgres_e2e` | passed | `cargo test --workspace` | `OXID_TEST_DATABASE_URL` (Postgres local descartável) | `evidence/index.md` | 6 testes passam | Sem segredos. |

## User Or Operator Evidence

- Status: `used`.
- Evidence: plano de ação do Gustavo e escolha do LiveKit.
- Decision impact: estrutura Rust em vez de Node/TS.

## Reviewed-Mode Start Checkpoint

Iniciado em modo `flow` a pedido do Gustavo.

## Context Receipt

Plano de ação, source pack de stack, templates do ub-workflow.

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
