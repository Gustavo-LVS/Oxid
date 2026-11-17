---
artifact_id          : w01-i01-s03-closeout
artifact_type        : sprint_closeout
status               : closed
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 220
---

# Sprint Closeout

## Outcome Summary

Passo 3 entregue: API de servidores, canais e convites e app Tauri 2 que faz login, lista servidores e canais, cria servidor, gera convite e entra por código ou link.

## Outcome And Learning Review

- Achieved: primeiro app desktop rodando de ponta a ponta contra a API.
- Project improvement: um amigo já consegue entrar no servidor pelo convite; falta só a voz no app.
- Could be better: a UI não tem teste automatizado versionado (o script Playwright ficou fora do repositório).
- Follow-up disposition: integrar voz no app depois do teste ao vivo do s02; versionar testes de UI quando o design definitivo chegar.

## Forecast Delta

- Planned vs actual: conforme planejado.
- Hidden prerequisite discovered: dependências de sistema do Tauri (webkit2gtk) no Linux e no CI.
- Remaining forecast impact: nenhum.
- Owner updates: `roadmap.md`, `status.md`.

## Evidence

`evidence/index.md`.

## Validation

`cargo test --workspace` (7 testes), `cargo clippy` limpo, app real sob Xvfb, fluxos de UI via Playwright.

## Broader Writebacks

README, `SOURCE_ATLAS.md`, CI (dependências do Tauri).

## Mini-Retro

- Process friction: bug de CSS (`[hidden]` sobrescrito por `display: grid`) só apareceu no WebKit real; vale sempre abrir o app real.
- Evidence cost: normal.
- Context cost: normal.
- Decision latency: normal.
- Workflow adjustment: `none`.
- Retro evidence check: sinal objetivo (screenshots e testes), controlável, não repetido.

## Handoff

s04: chat de texto via WebSocket.
