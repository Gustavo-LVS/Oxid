---
artifact_id          : w01-i01-s01-decision-log
artifact_type        : sprint_decision_log
status               : closed
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 140
---

# Sprint Decision Log

## Purpose

Record sprint-scoped decisions, reversals, deferrals, and evidence pointers.

## Decisions

1. JWT HS256 com validade padrão de 30 dias; sessões opacas podem substituir depois.
2. Unicidade de usuário e e-mail sem diferenciar maiúsculas (índices `lower()`).
3. Consultas sqlx em runtime (sem macros com banco no build), para compilar sem Postgres.
4. IDs UUID v7 (ordenáveis por tempo).

## Reversals And Deferrals

1. Estrutura Node/TS inicial removida em favor do Cargo workspace do plano.
2. Presença via Redis adiada para s03/s04.

## Evidence Pointers

Use `evidence/index.md` first when evidence exists.

## Carry Forward

Endpoint de token LiveKit no s02.
