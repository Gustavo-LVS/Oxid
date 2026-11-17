---
artifact_id          : w01-i01-s02-evidence-index
artifact_type        : evidence_index
status               : active
context_tier         : T4
updated_at           : 2026-11-17
summary_budget_lines : 100
---

# Evidence Index

## State

Ativo em 2026-11-17. Gate de token passou; gates ao vivo bloqueados por ambiente.

## Validated Claims

| Claim | Evidence | Gate | Status | Consumed By |
| --- | --- | --- | --- | --- |
| API emite token LiveKit válido só para usuário autenticado, com sala validada | `crates/server-api/tests/api.rs` | `voice_token_e2e` | passed | closeout |
| `voice_poc` compila | `crates/client-core/examples/voice_poc.rs` | `voice_poc_build` | blocked (clang 18 no ambiente de build; libwebrtc exige 21+) | closeout |
| Dois clientes se ouvem via SFU | README, seção prova de conceito | `voice_poc_two_clients` | blocked (sem LiveKit no ambiente de build) | closeout |

## Evidence Files

| File | Type | Status | Open When |
| --- | --- | --- | --- |
| `crates/server-api/tests/api.rs` | proof | passed | regressão de token |

## Live Gates And Deferred Evidence

Obrigatórios ainda pendentes: `voice_poc_build` e `voice_poc_two_clients`. O sprint não fecha sem eles.

## Redaction And Retention

No raw secrets, raw transcripts, raw prompts, raw model responses, raw file
content, or raw tool arguments unless an explicit debug-export policy allows it.

## Promotion

`status.md` (bloqueio), `roadmap.md`.

## Read Policy

Open this index for claim verification, validation audit, closeout review, or
regression investigation. Open focused evidence files only through a specific
claim or gate trigger.
