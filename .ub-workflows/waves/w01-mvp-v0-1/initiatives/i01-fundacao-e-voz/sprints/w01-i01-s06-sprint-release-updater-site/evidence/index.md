---
artifact_id          : w01-i01-s06-evidence-index
artifact_type        : evidence_index
status               : active
context_tier         : T4
updated_at           : 2026-11-17
summary_budget_lines : 100
---

# Evidence Index

## State

Ativo em 2026-11-17. Updater e site validados localmente; workflow de release aguarda repositório e segredos.

## Validated Claims

| Claim | Evidence | Gate | Status | Consumed By |
| --- | --- | --- | --- | --- |
| Build gera instaladores e assinaturas do updater | `.deb` 5,5 MiB, `.AppImage` 80 MiB, `.sig` (build local) | `signed_bundle` | passed | closeout |
| App 0.1.0 baixa e instala a 0.1.1 sozinho e mostra "Reiniciar agora" | md5 do AppImage passou a ser o da 0.1.1; `docs/screenshots/desktop-update.png` | `auto_update_e2e` | passed | closeout |
| Manifesto adulterado (versão diferente da assinada) é recusado | log: "The update was signed for version 0.1.1 but the update endpoint announced version 0.1.2" | `tampered_manifest` | passed | closeout |
| Site: botão por sistema (Windows, macOS), celular sem sistema errado, 5 páginas 200 | Playwright; `docs/screenshots/site.png` | `website` | passed | closeout |
| Release real nos 3 sistemas | — | `release_workflow` | blocked | closeout |

## Evidence Files

| File | Type | Status | Open When |
| --- | --- | --- | --- |
| `docs/screenshots/desktop-update.png` | live | passed | revisar aviso de update |
| `docs/screenshots/site.png` | live | passed | revisar site |

## Live Gates And Deferred Evidence

Obrigatório pendente: `release_workflow`.

## Redaction And Retention

No raw secrets, raw transcripts, raw prompts, raw model responses, raw file
content, or raw tool arguments unless an explicit debug-export policy allows it.

## Promotion

`status.md`, `roadmap.md`.

## Read Policy

Open this index for claim verification, validation audit, closeout review, or
regression investigation. Open focused evidence files only through a specific
claim or gate trigger.
