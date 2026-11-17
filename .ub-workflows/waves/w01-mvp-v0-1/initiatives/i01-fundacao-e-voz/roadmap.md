# Initiative Roadmap

## Objective

Entregar o MVP v0.1 seguindo a ordem de execução do plano de ação.

## Current Position

- Current state: `in progress`
- Active or next sprint: `w01-i01-s02` (prova de conceito de voz), ativo; falta o teste ao vivo
- Active discovery: `none`
- WIP state: one discovery allowed, no active delivery sprint.

## Forecast Control

- Appetite state: `within_appetite`
- Completed count: 2
- Registered remaining: 5
- Forecast delta: none.
- Confidence: média.
- Next scope tradeoff: none yet.
- Operator buy-more state: not needed.

## Active Or Next Sprint Route

s01 fechado. s02 ativo: prova de conceito de voz (2 clientes via LiveKit, endpoint de token na API).

## Adaptive Plan

1. ~~s01: workspace, crate `proto`, API com login e banco~~ (concluído 2026-11-17).
2. s02: prova de conceito de voz, 2 clientes falando via LiveKit; API emite token de sala.
3. ~~s03: cliente Tauri com lista de servidores e canais~~ (concluído 2026-11-17).
4. s04: chat de texto via WebSocket (gateway + Redis pub/sub).
5. s05: polir áudio: supressão de ruído, cancelamento de eco, push-to-talk.
6. s06: CI com build, assinatura, updater e site de download.
7. s07: beta fechado com amigos, medir latência e banda.
8. Pre-audit continuation window (`adaptive insertion`).
9. Final audit (`terminal audit`).

## Sequence Changes

No insertions, splits, reroutes, or deferrals yet.

## Revalidation Rules

- Revalidate stale candidates on touch.
- Promote discovery-driven sequence changes into this roadmap before accepting
  the discovery.
- Promote closeout-driven next-route changes into this roadmap before closing
  the sprint.
- Keep product-level future-wave options in root `options.md`; keep
  initiative-local possible insertions in local `options.md` until promoted,
  moved, rejected, or removed.

## Update Rules

Update this roadmap only when objective, current position, active route,
candidate order, insertion points, WIP state, revalidation rules, or final
audit posture changes. Keep token history in `index.md`, not here.
