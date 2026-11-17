---
artifact_id          : w01-i01-s06-decision-log
artifact_type        : sprint_decision_log
status               : active
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 140
---

# Sprint Decision Log

## Purpose

Record sprint-scoped decisions, reversals, deferrals, and evidence pointers.

## Decisions

1. `tauri-plugin-updater` em vez de Velopack para começar: já integrado ao Tauri e ao tauri-action; Velopack fica como opção (updates delta).
2. Canal escolhido na compilação (`OXID_CHANNEL`), manifesto em `{base}/{canal}/latest.json`; a tag define o canal (`-beta`, `-canary`).
3. `requireSignedVersion: true`: impede forçar downgrade com manifesto adulterado.
4. Chave pública injetada no `tauri.conf.json` pelo CI; sem ela (dev), o updater fica desligado.
5. Aviso de update guardado em estado e consultado pela interface ao carregar (o evento saía antes da interface escutar; bug achado no teste).
6. Site em Astro estático; botão por sistema via GitHub Releases API, com fallback para a página de releases.

## Reversals And Deferrals

1. Assinatura Windows: o workflow aceita um `.pfx` em segredo; a compra do certificado fica com o Gustavo.

## Evidence Pointers

Use `evidence/index.md` first when evidence exists.

## Carry Forward

Primeiro release beta quando houver repositório.
