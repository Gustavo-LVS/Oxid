# Workflow Status

## Current Product Posture

Plano de ação recebido em 2026-11-17. Stack: Rust (axum, sqlx, Tauri 2), voz via LiveKit no MVP. Passo 1 do plano concluído.

## Current Operating State

State: active. Interaction mode: `flow`.

## WIP State

Sprints ativos: s02 (aguardando teste de voz ao vivo) e s03 (API pronta; falta o app Tauri). WIP acima do padrão de 1 sprint por decisão do coordenador, porque o s02 depende de máquina externa. Nenhuma discovery ativa.

## Active Pointers

- Wave: `waves/w01-mvp-v0-1/`
- Initiative: `waves/w01-mvp-v0-1/initiatives/i01-fundacao-e-voz/`
- Sprints ativos: `w01-i01-s02-sprint-voice-poc-livekit`, `w01-i01-s03-sprint-servidores-canais-convites`
- Último fechado: `w01-i01-s01-sprint-workspace-api-login`

## Blockers

1. Sem repositório GitHub: o CI ainda não roda.
2. s02: o teste ao vivo de voz precisa de uma máquina com Docker (LiveKit) e clang 21+; o ambiente de build atual não tem.

## Wave Sequence

1. W01 MVP v0.1 (ativa).
2. v0.2 e v0.3+ em `options.md`.

## Conditional Candidate Tracks

Nenhum.

## Retained-Context Routes

Use `SOURCE_PACK_ATLAS.md` before opening source packs.

## Next Allowed Action

1. Rodar os gates ao vivo do s02 (README) e fechar o s02.
2. Fatia Tauri do s03 (login e lista de servidores e canais).
