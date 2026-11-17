# Product Vision

## Product Promise

Oxid é um app de chamadas em grupo leve, de baixa latência e com áudio limpo, para jogar com amigos e para trabalhar em equipe. Servidores (comunidades) com canais de texto e de voz, cliente desktop que atualiza sozinho.

## Audiences

1. Gamers brasileiros que precisam de voz estável, push-to-talk e baixo consumo durante o jogo.
2. Equipes de trabalho que querem canais de voz e texto simples, com convite por link.

## Durable Principles

1. Latência e qualidade de áudio vêm antes de qualquer recurso visual. Servidores de voz perto do usuário (São Paulo primeiro).
2. Cliente leve (Tauri + Rust), sem roubar CPU/RAM do jogo.
3. Protocolo compartilhado (`crates/proto`): cliente e servidor quebram juntos na compilação.
4. Usar WebRTC; não reinventar protocolo UDP no começo.
5. Privacidade e LGPD desde o início.

## Capability Pillars

1. Contas, perfis, servidores e canais (API Rust/axum + PostgreSQL + Redis).
2. Voz em grupo via SFU (LiveKit no MVP; SFU próprio em str0m depois).
3. Áudio de qualidade: supressão de ruído, cancelamento de eco, push-to-talk.
4. Distribuição: instalador assinado, atualização automática com canais stable/beta/canary, site de download.

## Evidence Questions

1. Qual latência ponta a ponta e consumo de banda/CPU com 10 pessoas numa sala?
2. O beta com amigos prefere o Oxid à ferramenta que usa hoje? Por quê?

## Change Rule

Update this vision only through reviewed discovery, closeout, transition, or an explicit product decision.
