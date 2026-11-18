---
paths:
  - "crates/server-common/src/auth.rs"
  - "crates/server-common/src/config.rs"
  - "crates/server-api/src/**"
  - "apps/desktop/src-tauri/**"
  - "**/.env*"
---

# Segurança

- Senha só com argon2id e JWT só com HS256 via `server-common::auth`; não crie um caminho paralelo.
- Toda rota protegida extrai e valida o usuário; toda ação em servidor/canal confere se ele é membro
  ou tem permissão. Isso vale também para eventos do gateway e tokens do LiveKit.
- Valide DTOs de entrada como em `routes/auth.rs` (`ApiError::Validation`); nunca monte SQL concatenando strings (use as queries do sqlx).
- Segredos (`OXID_JWT_SECRET`, `LIVEKIT_API_SECRET`, chaves de assinatura) só via env; nunca em log,
  erro devolvido ao cliente ou commit.
- App desktop: o token fica no Rust; comandos Tauri novos não devolvem o token para a UI. Não amplie
  `capabilities/` sem necessidade.
- Não desligue um controle de segurança (CORS, auth, validação) para "facilitar o teste".
