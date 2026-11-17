---
artifact_id          : w01-i01-s04-decision-log
artifact_type        : sprint_decision_log
status               : closed
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 140
---

# Sprint Decision Log

## Purpose

Record sprint-scoped decisions, reversals, deferrals, and evidence pointers.

## Decisions

1. Envio de mensagem por REST (resposta confirma e devolve a mensagem); gateway só entrega. Evita duas rotas de escrita.
2. Hub em memória (`tokio::broadcast`) atrás de uma interface; Redis pub/sub quando houver mais de um nó.
3. Sessão assina o hub antes de carregar os servidores do usuário, para não perder eventos; `MemberJoined` atualiza a sessão quando o usuário entra num servidor.
4. Paginação por id (UUID v7 é ordenado por tempo).

## Reversals And Deferrals

1. Reconexão automática do gateway no app adiada; hoje o app avisa e pede para entrar de novo.

## Evidence Pointers

Use `evidence/index.md` first when evidence exists.

## Carry Forward

Reconexão automática; presença online.
