---
artifact_id          : w01-i01-s07-decision-log
artifact_type        : sprint_decision_log
status               : active
context_tier         : T3
updated_at           : 2026-11-18
summary_budget_lines : 140
---

# Sprint Decision Log

## Purpose

Record sprint-scoped decisions, reversals, deferrals, and evidence pointers.

## Decisions

1. 4 vagas de argon2 (`ARGON2_PERMITS`): 4 x ~19 MiB cabe na VM de 1 GB e ainda atende o grupo de amigos.
2. Hash fixo em `LazyLock`: a primeira tentativa com usuário inexistente após subir a API paga um hash a mais; não vaza nada.
3. Reconexão: para só em `GatewayError::InvalidSession` (variante nova no `client-core`). A 1ª versão parava em qualquer `Protocol`, o que desistiria quando a API cai no meio do handshake num deploy; devolvida ao agente e corrigida.
4. Depois de reconectar, a UI recarrega as últimas 50 mensagens do canal aberto (dedup por `data-id`). Mais de 50 mensagens durante a queda deixam um buraco até trocar de canal; aceito para o tamanho atual.
5. `restart_to_update` reinicia mesmo se o `install` falhar (o erro vai para o stderr).

## Reversals And Deferrals

- Rate limit por IP no login: fora do escopo; o semáforo já limita a velocidade de tentativa.
- Reconexão da voz: fora do escopo.

## Evidence Pointers

`evidence/index.md`.

## Carry Forward

Gates ao vivo `updater_real` e `gateway_reconnect_real`.
