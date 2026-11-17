---
artifact_id          : w01-i01-s03-evidence-index
artifact_type        : evidence_index
status               : closed
context_tier         : T4
updated_at           : 2026-11-17
summary_budget_lines : 100
---

# Evidence Index

## State

Fechado em 2026-11-17. API e app validados.

## Validated Claims

| Claim | Evidence | Gate | Status | Consumed By |
| --- | --- | --- | --- | --- |
| Criar servidor gera dono-membro e canais #geral/Geral; nome validado | `tests/api.rs::servers_channels_invites_and_voice` | `servers_api_e2e` | passed | closeout |
| Não-membro recebe 404 em servidor, convite e voz; não-dono recebe 403 ao criar canal | idem | `servers_api_e2e` | passed | closeout |
| Convite de 1 uso: prévia, membro existente não consome, esgotado responde 410 | idem | `servers_api_e2e` | passed | closeout |
| Token de voz só em canal de voz, sala = id do canal | idem | `servers_api_e2e` | passed | closeout |
| App Tauri real faz login e lista servidores e canais | `docs/screenshots/desktop-servidor.png` | `tauri_list_servers` | passed | closeout |
| Fluxos da UI: cadastro, criar servidor, convite, entrar por link, erro de convite e de senha | script Playwright (scratch, não versionado) | `ui_flows_playwright` | passed | closeout |

## Evidence Files

| File | Type | Status | Open When |
| --- | --- | --- | --- |
| `crates/server-api/tests/api.rs` | proof | passed | regressão de servidores/convites |

## Live Gates And Deferred Evidence

Todos os obrigatórios passaram.

## Redaction And Retention

No raw secrets, raw transcripts, raw prompts, raw model responses, raw file
content, or raw tool arguments unless an explicit debug-export policy allows it.

## Promotion

`status.md`, `roadmap.md`.

## Read Policy

Open this index for claim verification, validation audit, closeout review, or
regression investigation. Open focused evidence files only through a specific
claim or gate trigger.
