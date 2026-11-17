# Workflow Status

## Current Product Posture

Plano de ação recebido em 2026-11-17. Stack: Rust (axum, sqlx, Tauri 2), voz via LiveKit no MVP. Passo 1 do plano concluído.

## Current Operating State

State: active. Interaction mode: `flow`.

## WIP State

Sprints s02, s05 e s06 aguardando o Gustavo (teste de voz/microfone e repositório com segredos para o release); s03 e s04 fechados. Nenhuma discovery ativa.

## Active Pointers

- Wave: `waves/w01-mvp-v0-1/`
- Initiative: `waves/w01-mvp-v0-1/initiatives/i01-fundacao-e-voz/`
- Sprints ativos (bloqueados no Gustavo): `w01-i01-s02-sprint-voice-poc-livekit`, `w01-i01-s05-sprint-polir-audio`, `w01-i01-s06-sprint-release-updater-site`
- Último fechado: `w01-i01-s04-sprint-chat-de-texto`
- Último fechado: `w01-i01-s01-sprint-workspace-api-login`

## Blockers

1. Sem repositório GitHub: CI e release ainda não rodam.
2. s02/s05: testes ao vivo de voz precisam de máquina com Docker (LiveKit), clang 21+ e microfone.

## Wave Sequence

1. W01 MVP v0.1 (ativa).
2. v0.2 e v0.3+ em `options.md`.

## Conditional Candidate Tracks

Nenhum.

## Retained-Context Routes

Use `SOURCE_PACK_ATLAS.md` before opening source packs.

## Next Allowed Action

1. Gustavo roda `voice_poc` (tom e `OXID_MIC=1`) para fechar s02 e s05.
2. Criar o repositório, gerar a chave do updater (`npx @tauri-apps/cli signer generate`), cadastrar segredos e fazer a tag `v0.1.0-beta.1` para fechar s06.
3. Ligar a voz dentro do app (opção local) e s07: beta com amigos.
