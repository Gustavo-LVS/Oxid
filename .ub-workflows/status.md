# Workflow Status

## Current Product Posture

Plano de ação recebido em 2026-11-17. Stack: Rust (axum, sqlx, Tauri 2), voz via LiveKit no MVP. Passo 1 do plano concluído.

## Current Operating State

State: active. Interaction mode: `flow`.

## WIP State

Sprint ativo: `w01-i01-s02-sprint-voice-poc-livekit`. Nenhuma discovery ativa.

## Active Pointers

- Wave: `waves/w01-mvp-v0-1/`
- Initiative: `waves/w01-mvp-v0-1/initiatives/i01-fundacao-e-voz/`
- Sprint ativo: `w01-i01-s02-sprint-voice-poc-livekit`
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

Rodar os gates ao vivo do s02 (README, seção prova de conceito) e fechar o sprint; depois preparar s03 (cliente Tauri com servidores e canais).
