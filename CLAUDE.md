@AGENTS.md

# Claude Code no Oxid

As regras de governança e validação estão em `AGENTS.md` (importado acima). Este arquivo
só diz como o Claude Code trabalha aqui.

## Ciclo de trabalho

ANALISAR → PLANEJAR → IMPLEMENTAR → VERIFICAR → REVISAR → RELATAR.

- Tarefa pequena e isolada: plano mental, sem cerimônia.
- Tarefa complexa (vários crates, `proto`, migração, auth, voz, release): apresente o
  plano antes de editar (objetivo, comportamento atual e esperado, arquivos, riscos e
  validação) e espere o ok se o risco for alto.
- Implementar não é verificar, e verificar não é fazer deploy. Deploy nunca acontece sem pedido explícito.

## Skills

- `safe-implementation`: ao implementar ou corrigir algo que toca código em produção.
- `ub-workflow`: trabalho que precisa de plano durável (wave, initiative, sprint, discovery).
- `staff` (skill do usuário): tarefa grande com várias frentes independentes.
- `publicar-gitflow` (skill do usuário): commit, push e PR, quando pedido.

## Relatório final

Inclua o que foi feito, os arquivos alterados, as decisões, os checks executados com o
resultado real, o que não foi verificado e os riscos pendentes.
