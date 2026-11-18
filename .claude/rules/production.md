---
paths:
  - "deploy/**"
  - "Dockerfile"
  - ".dockerignore"
  - ".github/workflows/**"
  - "migrations/**"
  - "apps/desktop/src-tauri/tauri.conf.json"
  - "apps/desktop/src-tauri/src/updater.rs"
---

# Proteção de produção

- `deploy/prod/` descreve a VM real. Editar o arquivo pode; executar contra a VM, só com aprovação.
- `release.yml` roda a cada tag `v*` e publica o manifesto do updater: todos os apps instalados
  no canal recebem a versão. Não crie nem envie tags sem pedido explícito.
- Mudança no updater (`updater.rs`, chave pública, endpoints no `tauri.conf.json`) pode deixar os
  apps sem conseguir se atualizar: trate como risco alto.
- `migrations/` vai para produção no próximo deploy. Só migração nova e aditiva; destrutiva exige
  aprovação e plano de backup (`pg_dump`, ver `deploy/prod/README.md`).
- Não suponha que os valores de dev (`devkey`/`secret`, `oxid:oxid`) valem em produção.
