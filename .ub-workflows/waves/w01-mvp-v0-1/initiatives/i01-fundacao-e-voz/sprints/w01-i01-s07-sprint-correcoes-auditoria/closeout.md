---
artifact_id          : w01-i01-s07-closeout
artifact_type        : sprint_closeout
status               : draft
context_tier         : T3
updated_at           : 2026-11-18
summary_budget_lines : 220
---

# Sprint Closeout

## Outcome Summary

Os 7 achados da auditoria foram corrigidos e validados localmente (2026-11-18). O sprint fica
aberto até os gates ao vivo `updater_real` e `gateway_reconnect_real`.

## Outcome And Learning Review

- Achieved: o updater não fecha mais o app no meio da chamada; o chat volta sozinho; a API
  resiste a rajada de logins e não revela contas pelo tempo; os testes da API falham alto sem
  banco; menos código morto.
- Project improvement: o `cargo test` local agora prova de verdade o que diz provar.
- Could be better: no Windows do Gustavo, o Controle de Aplicativo bloqueia o binário de teste
  do `oxid-desktop`; esses testes só rodam no CI.
- Follow-up disposition: rate limit por IP e reconexão da voz como opções, se o beta pedir.

## Forecast Delta

- Planned vs actual: conforme planejado; 1 volta de revisão (reconexão).
- Hidden prerequisite discovered: `none`.
- Remaining forecast impact: o beta passou para s08.
- Owner updates: `roadmap.md`, `status.md`.

## Evidence

`evidence/index.md`.

## Validation

`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` e
`cargo test --workspace --exclude oxid-desktop` (21 testes) verdes; os testes do desktop
estão bloqueados localmente pelo Windows.

## Broader Writebacks

`README.md`, `SOURCE_ATLAS.md`, `deploy/README.md`.

## Mini-Retro

- Process friction: a política de Controle de Aplicativo do Windows bloqueia binários de teste.
- Evidence cost: normal.
- Context cost: baixo (a staff trabalhou em 3 frentes paralelas, cada arquivo com um único dono).
- Decision latency: normal.
- Workflow adjustment: `none`.
- Retro evidence check: sinal objetivo e controlável (exclusão no Controle de Aplicativo ou rodar no CI).
