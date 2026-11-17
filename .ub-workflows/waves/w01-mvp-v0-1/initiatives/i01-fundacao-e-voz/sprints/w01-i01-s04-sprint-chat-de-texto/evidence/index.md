---
artifact_id          : w01-i01-s04-evidence-index
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
| Membro envia e lista mensagens em ordem, com paginação | `tests/api.rs::text_chat_history_and_realtime` | `chat_e2e` | passed | closeout |
| Não-membro: 404 em ler/escrever e não recebe eventos; validação de tamanho; canal de voz recusa texto | idem | `chat_e2e` | passed | closeout |
| Quem entra no servidor com o gateway já aberto recebe as mensagens em tempo real | idem | `chat_e2e` | passed | closeout |
| App real mostra histórico, envia e recebe mensagem de outro usuário na hora | `docs/screenshots/desktop-chat.png` | `chat_desktop_real` | passed | closeout |

## Evidence Files

| File | Type | Status | Open When |
| --- | --- | --- | --- |
| `crates/server-api/tests/api.rs` | proof | passed | regressão de chat |
| `docs/screenshots/desktop-chat.png` | live | passed | revisão visual |

## Live Gates And Deferred Evidence

Nenhum pendente.

## Redaction And Retention

No raw secrets, raw transcripts, raw prompts, raw model responses, raw file
content, or raw tool arguments unless an explicit debug-export policy allows it.

## Promotion

`closeout.md`, `roadmap.md`, `status.md`.

## Read Policy

Open this index for claim verification, validation audit, closeout review, or
regression investigation. Open focused evidence files only through a specific
claim or gate trigger.
