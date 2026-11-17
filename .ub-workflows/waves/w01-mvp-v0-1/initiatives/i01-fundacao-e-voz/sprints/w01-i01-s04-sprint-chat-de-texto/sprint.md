---
artifact_id          : w01-i01-s04-sprint-chat-de-texto
artifact_type        : sprint_plan
status               : closed
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 480
---

# Sprint: Chat de texto via WebSocket

## Objective

Passo 4 do plano: mensagens em canais de texto, com histórico via REST e entrega em tempo real pelo gateway, no app desktop.

## Source

- Accepted discovery or reviewed preview: plano de ação, passo 4; coordenador liberou seguir com os próximos passos.

## Routing Preflight

- Trigger categories considered: authority (quem lê/escreve), runtime-boundary (fan-out em tempo real), interoperability (protocolo do gateway).
- Triggered categories: authority, runtime-boundary.
- Fail-closed result: `preview_ok`; fan-out em memória num nó só, com interface pronta para Redis.

## Discovery Triage

- Outcome: `preview_ok`.
- Outside evidence trigger: `not triggered` — padrão REST + evento de gateway já previsto na arquitetura do plano.
- Trigger-presumed check: autorização igual à de servidores (membro) e coberta por teste.
- Promotion rule: Redis pub/sub vira candidato quando houver mais de um nó.

## Why Preview Instead Of Discovery

- Routing trigger: rota do plano.
- Discovery trigger check: `none`.
- Promotion rule: n/a.

## Product Increment Contribution

- Contribution type: `direct`.
- Visible increment unblocked: amigos conversam por texto no app.
- Why not direct: n/a.
- Enabling-chain check: n/a.

## Bet Framing

- Appetite: 1 sessão.
- Success evidence: mensagem enviada por um usuário aparece na hora no app de outro.
- Circuit breaker: n/a.
- Non-goals: edição/exclusão de mensagens, menções, anexos (v0.2), reconexão automática.
- Deferral path: `options.md` local.

## Scope

1. Tabela `messages`; `GET/POST /v1/channels/{id}/messages` com paginação `before`/`limit`.
2. Hub de eventos e `MessageCreate` no gateway, só para membros (inclusive quem entra depois de conectar).
3. `client-core::gateway` (identify, heartbeat, canal de eventos) e métodos de mensagens no `ApiClient`.
4. App: painel de chat, histórico, envio, recebimento em tempo real.

## Scope Completion Rule

When this sprint uses words like `smallest`, `narrow`, or `only as needed`,
they mean the smallest objective-complete vertical slice. Every named owned
surface, required evidence gate, and exit criterion remains in scope unless
the operator explicitly changes the objective.

## Operational Surface Preflight

- Triggered: yes.
- Surfaces: rotas de mensagens; evento `message_create` no gateway; `crates/server-api/src/hub.rs`.
- Owner and lifecycle: `server-api` (rotas, hub, gateway), `client-core::gateway`, `apps/desktop`.
- Visibility and safety: não-membro recebe 404 e não recebe eventos; mensagem de 1 a 4000 caracteres; sessão atrasada no hub é desconectada.
- Inventory impact: README, `SOURCE_ATLAS.md`.
- Validation path: `text_chat_history_and_realtime` + app real.
- Promotion rule: n/a.

## Execution Slices

1. API + hub + gateway (feito).
2. Cliente e app (feito).

## Validation Plan

1. `cargo test --workspace` com Postgres.
2. `cargo clippy --workspace --all-targets` sem avisos.
3. App real sob Xvfb: histórico, envio e recebimento de outro usuário.

## Live Validation Matrix

| Gate | Status | Command | Credentials / env | Evidence path | Pass criteria | Redaction |
| --- | --- | --- | --- | --- | --- | --- |
| `chat_e2e` | passed | `cargo test --workspace` | `OXID_TEST_DATABASE_URL` | `evidence/index.md` | teste passa | Sem segredos. |
| `chat_desktop_real` | passed | `oxid-desktop` sob Xvfb + xdotool + curl | API local | `docs/screenshots/desktop-chat.png` | 3 mensagens visíveis, incluindo a recebida em tempo real | n/a |

## User Or Operator Evidence

- Status: `not triggered`.
- Evidence: n/a.
- Decision impact: n/a.

## Reviewed-Mode Start Checkpoint

Iniciado em modo `flow`.

## Context Receipt

Plano, s03 closeout, status.

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
