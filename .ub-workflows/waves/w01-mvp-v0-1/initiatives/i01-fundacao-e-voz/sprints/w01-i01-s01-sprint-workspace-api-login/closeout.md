---
artifact_id          : w01-i01-s01-closeout
artifact_type        : sprint_closeout
status               : closed
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 220
---

# Sprint Closeout

## Outcome Summary

Passo 1 entregue: workspace Rust, `proto`, `server-common` e `server-api` com cadastro, login, `/users/@me` e gateway WebSocket, testados contra PostgreSQL real.

## Outcome And Learning Review

- Achieved: base do servidor e protocolo compartilhado.
- Project improvement: o s02 já pode emitir token de sala de voz para usuários autenticados.
- Could be better: CI ainda não roda no GitHub (falta repositório).
- Follow-up disposition: criar repositório; s02.

## Forecast Delta

- Planned vs actual: conforme planejado.
- Hidden prerequisite discovered: `none`.
- Remaining forecast impact: nenhum.
- Owner updates: `roadmap.md`, `status.md`.

## Evidence

`evidence/index.md`.

## Validation

`cargo fmt --check`, `cargo clippy --workspace --all-targets` (0 avisos), `cargo test --workspace` (6 testes, todos passaram).

## Broader Writebacks

`SOURCE_ATLAS.md`, `README.md`, `vision.md`, `status.md`.

## Mini-Retro

- Process friction: o plano chegou depois da primeira estrutura (Node/TS), que precisou ser refeita.
- Evidence cost: normal.
- Context cost: normal.
- Decision latency: normal (decisão do SFU respondida em minutos).
- Workflow adjustment: `none`.
- Retro evidence check: sinal objetivo (testes), controlável, não repetido.

## Handoff

Preparar s02: prova de conceito de voz com LiveKit.
