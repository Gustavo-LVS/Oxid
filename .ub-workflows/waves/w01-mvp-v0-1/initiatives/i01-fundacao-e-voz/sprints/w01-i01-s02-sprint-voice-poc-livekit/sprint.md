---
artifact_id          : w01-i01-s02-sprint-voice-poc-livekit
artifact_type        : sprint_plan
status               : active
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 480
---

# Sprint: Prova de conceito de voz com LiveKit

## Objective

Passo 2 do plano: dois clientes falando entre si via SFU (LiveKit), com a API emitindo o token de sala.

## Source

- Accepted discovery or reviewed preview: plano de ação, passo 2; decisão do Gustavo de usar LiveKit agora e str0m depois (2026-11-17).

## Routing Preflight

- Trigger categories considered: runtime-boundary (SFU externo), interoperability (SDK Rust do LiveKit), policy (autorização de sala).
- Triggered categories: runtime-boundary, interoperability.
- Fail-closed result: `preview_ok`; o SFU foi decidido pelo operador.

## Discovery Triage

- Outcome: `preview_ok`.
- Outside evidence trigger: docs do LiveKit (source pack de stack); crates `livekit` 0.11 e `livekit-api` 0.10.
- Trigger-presumed check: sala livre para qualquer usuário autenticado só nesta prova de conceito; no s03 passa a ser canal com permissão.
- Promotion rule: nada promovido.

## Why Preview Instead Of Discovery

- Routing trigger: rota decidida pelo operador.
- Discovery trigger check: compilação do libwebrtc exige clang 21+ (descoberto durante a execução; não muda a rota).
- Promotion rule: se a voz ficar instável, abrir discovery de SFU (regra da wave).

## Product Increment Contribution

- Contribution type: `direct`.
- Visible increment unblocked: duas pessoas se ouvindo pelo Oxid.
- Why not direct: n/a.
- Enabling-chain check: s01 foi enabling; este é direto.

## Bet Framing

- Appetite: 1 a 2 sessões.
- Success evidence: dois `voice_poc` na mesma sala imprimindo volume RMS do outro.
- Circuit breaker: perda de áudio ou desconexões frequentes em rede local.
- Non-goals: microfone real (s05), interface (s03), canais (s03).
- Deferral path: roadmap.

## Scope

1. `POST /v1/voice/token` na API com `livekit-api` (feito, testado).
2. `client-core`: `ApiClient` e `VoiceSession` (feature `voice`).
3. Exemplo `voice_poc` com tom senoidal e medidor de volume.
4. LiveKit no docker-compose e variáveis em `.env.example`.

## Scope Completion Rule

When this sprint uses words like `smallest`, `narrow`, or `only as needed`,
they mean the smallest objective-complete vertical slice. Every named owned
surface, required evidence gate, and exit criterion remains in scope unless
the operator explicitly changes the objective.

## Operational Surface Preflight

- Triggered: yes.
- Surfaces: rota `/v1/voice/token`; variáveis `LIVEKIT_URL`, `LIVEKIT_API_KEY`, `LIVEKIT_API_SECRET`; serviço `livekit` no compose.
- Owner and lifecycle: `crates/server-api/src/routes/voice.rs`, `crates/client-core`, `deploy/`.
- Visibility and safety: token de sala vale 10 min; sem as variáveis o endpoint responde 503; chaves `--dev` só em desenvolvimento.
- Inventory impact: README e `.env.example`.
- Validation path: teste ponta a ponta (token) + gate ao vivo `voice_poc_two_clients`.
- Promotion rule: n/a.

## Execution Slices

1. Endpoint de token (feito).
2. Cliente e exemplo (escrito; compila só com clang 21+).
3. Gate ao vivo com LiveKit real (pendente).

## Validation Plan

1. `cargo test --workspace` com Postgres.
2. `cargo build -p oxid-client-core --features voice --example voice_poc` com clang 21+.
3. Dois `voice_poc` contra o docker-compose.

## Live Validation Matrix

| Gate | Status | Command | Credentials / env | Evidence path | Pass criteria | Redaction |
| --- | --- | --- | --- | --- | --- | --- |
| `voice_token_e2e` | passed | `cargo test --workspace` | `OXID_TEST_DATABASE_URL` | `evidence/index.md` | token com room/roomJoin/iss/sub corretos | Sem segredos. |
| `voice_poc_build` | blocked | `cargo build -p oxid-client-core --features voice --example voice_poc` | clang 21+ | `evidence/index.md` | compila sem erros | n/a |
| `voice_poc_two_clients` | blocked | dois `voice_poc` (README) | docker-compose com LiveKit `--dev` | `evidence/index.md` | cada cliente ouve o outro (RMS ~0.2) | Sem tokens no log. |

## User Or Operator Evidence

- Status: `used`.
- Evidence: escolha do LiveKit pelo Gustavo.
- Decision impact: SDK LiveKit no cliente; `server-sfu` fica reservado.

## Reviewed-Mode Start Checkpoint

Iniciado em modo `flow`.

## Context Receipt

Plano, source pack de stack, s01 closeout.

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
