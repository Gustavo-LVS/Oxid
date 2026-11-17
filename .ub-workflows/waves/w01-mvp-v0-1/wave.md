# Wave W01: MVP v0.1

## Outcome

Um grupo de amigos instala o Oxid, cria conta, entra num servidor por convite e conversa por voz com push-to-talk ou ativação por voz, supressão de ruído e indicador de quem fala, com o app atualizando sozinho.

## Why Now

É a fase 1 do plano de ação do Gustavo (2026-11-17). Sem voz funcionando não existe produto.

## Scope Boundaries

1. Cadastro e login, perfil com avatar.
2. Servidores/comunidades com canais de texto e de voz; convites por link.
3. Voz: push-to-talk ou ativação por voz, mutar/ensurdecer, volume por usuário, supressão de ruído, indicador de fala.
4. Instalador e atualização automática; site de download.

## Bet Framing

- Appetite: 3 a 6 meses para uma pessoa (estimativa do plano).
- Success evidence: beta fechado com amigos usando o app de verdade, com latência e banda medidas.
- Circuit breaker: se a prova de conceito de voz (passo 2) não ficar estável, parar e reavaliar o SFU antes de construir interface.
- Deferral path: itens da v0.2 e v0.3 ficam em `.ub-workflows/options.md`.

## Outcome Signals

- Product/user signal: amigos conseguem entrar numa chamada sem ajuda.
- Delivery/flow signal: um passo do plano por sprint, sem dois sprints seguidos só de preparação.
- Quality/stability signal: testes ponta a ponta verdes no CI; chamadas sem queda.
- Context/evidence cost signal: status e roadmap bastam para retomar o trabalho.

## Forecast And Appetite

- Appetite: 3 a 6 meses.
- Forecast range/count: 7 passos da ordem de execução do plano.
- Confidence: média; a voz é o maior risco.
- Throughput basis: 1 sprint concluído (s01).
- Known unknowns: SDK Rust do LiveKit no Tauri; assinatura de código no Windows a partir do Brasil.
- Scope hammers: adiar ativação por voz ou volume por usuário; site de download só com link do GitHub Releases.
- Expansion trigger: qualquer item da v0.2 entrando no MVP exige decisão do Gustavo.

## Non-Goals

1. Mensagens diretas, amigos, cargos, moderação, uploads, compartilhamento de tela (v0.2).
2. Vídeo, overlay, bots, E2EE, mobile, multirregião (v0.3+).
3. Design final da interface (fica para depois, conforme o Gustavo).

## Status

State: active.

## Initiative Map

1. `i01-fundacao-e-voz` (ativa): passos 1 a 7 do plano.

## Retained Inputs

- `../../source-packs/2026-11-17-stack-chamadas-webrtc/` — pesquisa de stack.
- Plano de ação do Gustavo (mensagem de 2026-11-17 no chat do projeto).

## Transition And Reroute Rules

1. Se a voz via LiveKit falhar no passo 2, abrir discovery para str0m ou outro SFU antes do passo 3.
2. Fechar a wave depois do beta (passo 7) com medição de latência e banda.
