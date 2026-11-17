---
artifact_id          : w01-i01-s01-evidence-index
artifact_type        : evidence_index
status               : closed
context_tier         : T4
updated_at           : 2026-11-17
summary_budget_lines : 100
---

# Evidence Index

## State

Fechado em 2026-11-17. Todos os gates obrigatórios passaram.

## Validated Claims

| Claim | Evidence | Gate | Status | Consumed By |
| --- | --- | --- | --- | --- |
| Formato de fio do gateway é estável | `crates/proto/src/gateway.rs` testes | `cargo test` | passed | closeout |
| Senha argon2id e JWT (adulteração e expiração rejeitadas) | `crates/server-common/src/auth.rs` testes | `cargo test` | passed | closeout |
| Cadastro, duplicado (409), validação (422), login, `/users/@me`, gateway Hello/Identify/Ready/Heartbeat, token inválido | `crates/server-api/tests/api.rs` | `cargo test` com Postgres | passed | closeout |
| Código limpo | `cargo fmt --check`, `cargo clippy --all-targets` | lint | passed | closeout |

## Evidence Files

| File | Type | Status | Open When |
| --- | --- | --- | --- |
| `crates/server-api/tests/api.rs` | proof | passed | regressão na API |

## Live Gates And Deferred Evidence

Obrigatório: `postgres_e2e` passou. Fora de escopo: CI no GitHub (sem repositório ainda).

## Redaction And Retention

No raw secrets, raw transcripts, raw prompts, raw model responses, raw file
content, or raw tool arguments unless an explicit debug-export policy allows it.

## Promotion

`closeout.md`, `roadmap.md`, `status.md`.

## Read Policy

Open this index for claim verification, validation audit, closeout review, or
regression investigation. Open focused evidence files only through a specific
claim or gate trigger.
