# Pesquisa de stack — Oxid (chamadas em grupo para gamers e trabalho)

Data: 2026-11-17. Status: rascunho de pesquisa, aguardando o plano de ação do Gustavo para confirmar escopo.

## 1. Arquitetura de mídia

| Modelo | Como funciona | Serve para o Oxid? |
|---|---|---|
| P2P (mesh) | Cada participante envia mídia para todos | Só até ~4 pessoas; banda de upload explode |
| SFU (Selective Forwarding Unit) | Cada um envia 1 stream ao servidor, que repassa sem decodificar | **Sim.** Padrão de Discord, Meet, Zoom web. Baixa latência, escala para dezenas por sala |
| MCU | Servidor mixa tudo num stream só | Não. Caro em CPU e adiciona latência |

Recomendação: **SFU**, com simulcast para vídeo e Opus com DTX/FEC para voz.

## 2. Opções de SFU open source

| SFU | Linguagem | Prós | Contras |
|---|---|---|---|
| **LiveKit** (v1.13.x) | Go (Pion) | SDKs prontos (JS, React, Swift, Android, Flutter, Rust, Unity), sinalização inclusa, TURN embutido, egress/ingress, self-host ou cloud, Apache 2.0 | Menos controle fino que mediasoup |
| mediasoup | C++ + Node.js | Controle total, muito performático | Você escreve sinalização, rooms e SDK cliente |
| Janus | C | Maduro, plugins | API mais antiga, mais trabalho de integração |
| Pion (lib) | Go | Montar SFU próprio | Projeto grande; só vale para algo muito específico |

Recomendação: **LiveKit self-hosted** para o MVP. Reduz meses de trabalho (sinalização, reconexão, simulcast, TURN) e permite migrar para LiveKit Cloud se quiser.

## 3. Stack proposta (sujeita ao plano)

- **SFU/mídia:** LiveKit Server (Docker), Redis para multi-nó.
- **Backend (API):** Node.js + TypeScript (Fastify ou NestJS) ou Go. Responsável por contas, servidores/canais, convites, emissão de tokens de acesso LiveKit (JWT), presença.
- **Banco:** PostgreSQL (+ Prisma/Drizzle). Redis para presença e pub/sub.
- **Tempo real não-mídia (chat, presença):** WebSocket próprio ou LiveKit Data Channels.
- **Cliente web:** React + TypeScript + Vite, `livekit-client` + `@livekit/components-react`.
- **Cliente desktop (gamers):** **Tauri 2** (leve, Rust) reaproveitando o front web. Atenção: WebRTC no Linux via WebKitGTK é limitado; Electron é alternativa se suporte Linux for prioridade. Push-to-talk global e overlay exigem código nativo.
- **Mobile (depois):** React Native ou Flutter com SDKs LiveKit.
- **Áudio para gamers:** Opus 48 kHz, supressão de ruído (Krisp no LiveKit Cloud, ou RNNoise/WebAudio no cliente), push-to-talk, ajuste de bitrate por canal.
- **Infra:** Docker Compose em dev; em produção, VM com IP público e portas UDP abertas (50000–60000) + TURN/TLS na 443 para redes corporativas.

## 4. Documentação oficial para consultar

- LiveKit — self-hosting: https://docs.livekit.io/transport/self-hosting.md
- LiveKit server (Go): https://pkg.go.dev/github.com/livekit/livekit-server@v1.13.3
- LiveKit README / repositório: https://github.com/livekit/livekit/blob/master/README.md
- Tauri 2.0 estável: https://v2.tauri.app/blog/tauri-20/
- WebRTC no WebKitGTK (Linux) com Tauri: https://github.com/tauri-apps/tauri/discussions/8426
- Comparação de SFUs (mediasoup, Janus, LiveKit, Jitsi, Pion): https://www.forasoft.com/learn/video-streaming/articles-streaming/sfu-comparison-mediasoup-janus-livekit-jitsi-pion
- P2P vs SFU vs MCU (guia 2026): https://www.forasoft.com/blog/article/webrtc-architecture-guide-for-business-2026
- Media servers open source (2026): https://webrtc.ventures/2026/06/open-source-webrtc-media-servers/
- OpenVidu — comparação (OpenVidu 3 é baseado em LiveKit): https://openvidu.io/3.8/docs/comparing-openvidu/

## 5. Perguntas em aberto (dependem do plano)

1. Plataformas do MVP: web, desktop, mobile?
2. Vídeo e compartilhamento de tela no MVP, ou só voz?
3. Modelo: servidores/canais estilo comunidade, ou salas de reunião com link?
4. Self-host vs LiveKit Cloud (custo vs operação).
5. Gravação, transcrição ou recursos de IA?
