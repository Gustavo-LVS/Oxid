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
- Completed count: 3
- Registered remaining: 4
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
4. ~~s04: chat de texto via WebSocket~~ (concluído 2026-11-17; hub em memória, Redis quando houver vários nós).
5. s05: polir áudio (feito 2026-11-17; falta teste com microfone ao vivo).
6. s06: CI com build, assinatura, updater e site de download (feito 2026-11-17; falta o primeiro release real).
7. s07: correções da auditoria de 2026-11-18 (código pronto e validado; faltam os testes ao vivo do updater e da reconexão).
8. s08: beta fechado com amigos, medir latência e banda.
9. Pre-audit continuation window (`adaptive insertion`).
10. Final audit (`terminal audit`).

## Sequence Changes

2026-11-18: s07 inserido (correções da auditoria); o beta fechado passou para s08.

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
