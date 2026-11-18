---
name: safe-implementation
description: Fluxo seguro para implementar, corrigir ou refatorar código do Oxid que já está em produção (API axum, migrações sqlx, proto/gateway, app Tauri, voz, updater, release). Use ao implementar feature, corrigir bug, mudar rota/evento/DTO, mexer em migração ou em qualquer coisa que afete o servidor ou os apps já instalados.
---

# Safe Implementation (Oxid)

Objetivo: mudar o sistema em produção com risco mínimo e evidência verificável.
As regras gerais estão em `AGENTS.md`; aqui fica o procedimento.

## 1. Entender antes de mudar

1. `git status` — anote mudanças que já existiam e não as toque.
2. Leia `SOURCE_ATLAS.md` para achar o crate dono; leia o código e os testes dele.
3. Ache os consumidores do que vai mudar (grep pelo tipo, rota ou evento). Para
   `crates/proto`, os consumidores são `server-api`, `client-core` e o app desktop.
4. Se possível, rode os testes do crate antes (linha de base) para separar falhas
   antigas das novas.

## 2. Classificar o risco

| Risco | Exemplos no Oxid | Conduta |
| --- | --- | --- |
| Baixo | UI em `apps/desktop/ui`, site, docs, bug isolado sem contrato nem dado | Implementar e validar |
| Médio | Rota nova ou alterada, regra de negócio, query, migração aditiva, processamento de áudio, várias crates | Plano escrito e teste de regressão focado |
| Alto | Migração destrutiva, mudança incompatível em `proto`/gateway, auth/JWT, capabilities do Tauri, updater, `release.yml`, `deploy/prod`, tag `v*` | Parar e pedir aprovação antes de executar |

## 3. Implementar em passos pequenos

- Reuse o que existe (`ApiClient`, `hub.rs`, `ApiError`, erros com `thiserror`).
- Mantenha os contratos: em `proto`, adicione campos opcionais com `#[serde(default)]`;
  não renomeie nem remova sem plano de migração dos clientes.
- Migração: arquivo novo em `migrations/`, aditivo e compatível com a versão anterior
  da API (ela roda sozinha no próximo deploy). Nunca edite uma migração já commitada.
- Nada de `unwrap()` em caminho de requisição nem de `catch-all` que esconda erro.

## 4. Verificar

Rode o que o CI roda (comandos em `AGENTS.md`) e mais o que for específico:

- Rota/DB: `cargo test -p oxid-server-api` com `OXID_TEST_DATABASE_URL` e o Postgres de dev.
- Áudio: `cargo test -p oxid-client-audio` (usa `tests/fixtures/fala-ptbr.wav`).
- Voz ao vivo e microfone não são testáveis aqui: diga isso no relatório.
- Site: `npm run build` em `apps/website`.

Compare com a linha de base. Nunca diga que passou sem ter executado.

## 5. Revisar

`git diff` completo: só arquivos pretendidos, sem segredo, sem mudança não relacionada,
`SOURCE_ATLAS.md` atualizado se uma fronteira de módulo mudou.

## 6. Relatar

Objetivo, o que mudou, arquivos, decisões, checks executados e resultado, o que não foi
verificado e riscos/pendências (ex.: "precisa de deploy", "testar voz com 2 máquinas").
