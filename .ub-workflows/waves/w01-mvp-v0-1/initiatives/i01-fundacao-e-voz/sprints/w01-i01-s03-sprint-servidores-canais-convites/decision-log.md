---
artifact_id          : w01-i01-s03-decision-log
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

1. Sala do LiveKit = id do canal de voz; `/v1/voice/token` passa a receber `channel_id` (o campo `room` do s02 saiu).
2. Permissões do MVP: membro vê servidor e cria convites; só o dono cria canais. Cargos na v0.2.
3. Não-membro recebe 404, não 403, para não revelar servidores.
4. `build.rs` em `server-common` para recompilar quando surgir migração nova (bug encontrado: migração nova não era aplicada).

## Reversals And Deferrals

1. Contrato de voz do s02 alterado (sem clientes em produção, `forward-only`).

## Evidence Pointers

Use `evidence/index.md` first when evidence exists.

## Carry Forward

Fatia Tauri.
