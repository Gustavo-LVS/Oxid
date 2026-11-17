# Initiative: Fundação e voz

## Summary

Executa a ordem do plano de ação: workspace e API, prova de conceito de voz, cliente Tauri, chat de texto, polimento de áudio, CI/release/site e beta.

## Outcome Bet

- Appetite: a wave inteira (3 a 6 meses).
- Success evidence: beta com amigos funcionando e medido.
- Circuit breaker: voz instável no passo 2.
- Deferral path: `options.md` desta iniciativa e `options.md` raiz.

## Outcome Signals

- Product/user signal: duas pessoas conversando pelo app.
- Delivery/flow signal: um passo do plano por sprint.
- Quality/stability signal: `cargo clippy -D warnings` e testes verdes.
- Context/evidence cost signal: cada sprint fecha com `evidence/index.md`.

## Forecast And Appetite

- Appetite: 3 a 6 meses.
- Forecast range/count: 7 sprints, um por passo.
- Confidence: média.
- Throughput basis: s01 concluído em 1 sessão.
- Known unknowns: tamanho do build do SDK LiveKit (libwebrtc) dentro do Tauri.
- Scope hammers: ver wave.
- Expansion trigger: decisão do Gustavo.

## Goals

1. Voz em grupo estável via LiveKit.
2. Cliente Tauri com servidores, canais e chat de texto.
3. Release assinado com atualização automática.

## Non-Goals

1. Design final da interface.
2. SFU próprio em str0m (reservado em `crates/server-sfu`).

## Constraints

1. Evolution mode: `forward-only`.
2. Rust edition 2024, workspace único; `proto` compartilhado por cliente e servidor.

## Current Status

- Interaction mode: `flow` (o Gustavo pediu para passar pelo plano inteiro).
- Active discovery: `none`
- Active sprint: `none` (s01 fechado)
- Current gate: `sprint_closed`
- Next allowed action: preparar s02 (prova de conceito de voz com LiveKit).

## Durable Decisions

1. 2026-11-17: SFU do MVP é LiveKit self-hosted; str0m depois (escolha do Gustavo no card de decisão).
2. 2026-11-17: estrutura do código segue o Cargo workspace do plano, não a sugestão inicial em Node/TS.
