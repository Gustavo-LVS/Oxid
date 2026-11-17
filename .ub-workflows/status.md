# Workflow Status

## Current Product Posture

Plano de ação recebido em 2026-11-17. Stack: Rust (axum, sqlx, Tauri 2), voz via LiveKit no MVP. Passo 1 do plano concluído.

## Current Operating State

State: active. Interaction mode: `flow`.

## WIP State

Sprint s02 aguardando teste de voz ao vivo na máquina do Gustavo; s03 e s04 fechados. Nenhuma discovery ativa.

## Active Pointers

- Wave: `waves/w01-mvp-v0-1/`
- Initiative: `waves/w01-mvp-v0-1/initiatives/i01-fundacao-e-voz/`
- Sprint ativo: `w01-i01-s02-sprint-voice-poc-livekit` (bloqueado em teste ao vivo)
- Último fechado: `w01-i01-s04-sprint-chat-de-texto`
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
2. s05: polir áudio (supressão de ruído, eco, push-to-talk) em `client-audio`.
