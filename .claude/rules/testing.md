---
paths:
  - "crates/**"
  - "apps/**"
---

# Testes

- Antes de mudar, leia os testes existentes do crate (`crates/server-api/tests/api.rs`,
  `crates/client-audio/tests/pipeline.rs`, `#[cfg(test)]` nos módulos).
- Comportamento novo ou alterado ganha teste; bug corrigido ganha teste que falhava antes.
- Os testes da API precisam do Postgres de dev e de `OXID_TEST_DATABASE_URL`; se não estiver
  disponível, diga que não rodou em vez de pular em silêncio.
- O clippy roda com `-D warnings`: warning novo quebra o CI.
- Relate o resultado real de cada check executado e liste os que não rodaram.
