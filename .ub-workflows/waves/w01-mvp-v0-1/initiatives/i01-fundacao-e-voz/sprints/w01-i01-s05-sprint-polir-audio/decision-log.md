---
artifact_id          : w01-i01-s05-decision-log
artifact_type        : sprint_decision_log
status               : active
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 140
---

# Sprint Decision Log

## Purpose

Record sprint-scoped decisions, reversals, deferrals, and evidence pointers.

## Decisions

1. Cancelamento de eco e ganho automático pelo APM do WebRTC que já vem no LiveKit, em vez do crate `webrtc-audio-processing` (evita uma segunda cópia do libwebrtc).
2. Reamostragem linear própria em vez de `rubato`: suficiente para voz e sem dependência; trocar se aparecer aliasing audível.
3. Ativação por voz: limiar 0,6 de probabilidade do RNNoise, hangover de 300 ms e 200 ms de aquecimento (sem aquecimento, o início do ruído abria o microfone).
4. Push-to-talk padrão Alt+V; configurável depois.
5. RNNoise quase não reduz ruído branco puro (~50%), mas elimina ruído colorido de ambiente; testes usam ruído colorido, que é o caso real.

## Reversals And Deferrals

1. Escolha de dispositivo e configuração do atalho adiadas.

## Evidence Pointers

Use `evidence/index.md` first when evidence exists.

## Carry Forward

Ligar `AudioProcessor` + PTT à voz dentro do app.
