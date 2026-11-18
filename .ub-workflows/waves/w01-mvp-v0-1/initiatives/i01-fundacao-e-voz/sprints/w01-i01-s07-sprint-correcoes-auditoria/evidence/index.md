---
artifact_id          : w01-i01-s07-evidence-index
artifact_type        : evidence_index
status               : active
context_tier         : T4
updated_at           : 2026-11-18
summary_budget_lines : 100
---

# Evidence Index

## State

2026-11-18: código pronto e validado localmente (Windows, Postgres 17 no Docker). Faltam os
gates ao vivo do updater e da reconexão, que dependem do Gustavo.

## Validated Claims

| Claim | Evidence | Gate | Status | Consumed By |
| --- | --- | --- | --- | --- |
| Login não revela se a conta existe | `login_time_does_not_reveal_whether_user_exists`: antes 2,8 ms x 336 ms (falhou); depois passa | validation | passed | closeout |
| Testes da API não passam mais sem banco | sem `OXID_TEST_DATABASE_URL`: 0 passed, 4 failed, com a mensagem de como rodar | validation | passed | closeout |
| Hash de senha limitado a 4 simultâneos | revisão do diff (`run_argon2` segura a vaga até o fim do `spawn_blocking`) | review | passed (por leitura; sem teste de carga) | closeout |
| Updater só baixa; instala ao reiniciar | revisão do diff + API do plugin 2.14.1 (`download` L819, `install` L910-924) | review | passed (por leitura) | closeout |
| Reconexão com espera crescente; para só em sessão recusada | `backoff_dobra_ate_o_teto` (passou na 1ª rodada do agente) + revisão | validation | passed; reexecução local bloqueada (ver abaixo) | closeout |
| `server-sfu`, `validator` e Redis removidos | `cargo check`, `docker compose config --services` = postgres, livekit | validation | passed | closeout |
| CI local | `cargo fmt --all --check` ok; `cargo clippy --workspace --all-targets -D warnings` ok; `cargo test --workspace --exclude oxid-desktop`: 21 testes ok | validation | passed | closeout |

## Evidence Files

| File | Type | Status | Open When |
| --- | --- | --- | --- |
| n/a (saídas de comando resumidas acima) | validation | n/a | n/a |

## Live Gates And Deferred Evidence

- Bloqueado: `cargo test -p oxid-desktop` no Windows do Gustavo — "Uma política de Controle de
  Aplicativo bloqueou este arquivo (os error 4551)". Roda no CI (Linux).
- Operator-deferred, obrigatório para fechar: `updater_real` (release beta; o app não fecha
  durante chamada e instala ao reiniciar) e `gateway_reconnect_real` (app aberto, reiniciar a
  API, o chat volta sozinho).

## Redaction And Retention

Sem segredos; senhas usadas são as fixas dos testes.

## Promotion

`closeout.md`.

## Read Policy

Open this index for claim verification, validation audit, closeout review, or
regression investigation. Open focused evidence files only through a specific
claim or gate trigger.
