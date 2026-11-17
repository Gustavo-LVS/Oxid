---
artifact_id          : w01-i01-s05-sprint-polir-audio
artifact_type        : sprint_plan
status               : active
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 480
---

# Sprint: Polir o áudio

## Objective

Passo 5 do plano: supressão de ruído, cancelamento de eco e push-to-talk (mais ativação por voz, item do MVP).

## Source

- Accepted discovery or reviewed preview: plano de ação, passo 5; coordenador liberou seguir.

## Routing Preflight

- Trigger categories considered: runtime-boundary (dispositivos de áudio, atalho global do SO), interoperability (LiveKit APM).
- Triggered categories: runtime-boundary.
- Fail-closed result: `preview_ok`; bibliotecas indicadas no plano.

## Discovery Triage

- Outcome: `preview_ok`.
- Outside evidence trigger: plano (seção 4: cpal, nnnoiseless, webrtc-audio-processing, rubato, tauri-plugin-global-shortcut).
- Trigger-presumed check: atalho global pode conflitar com outro programa; o app segue sem PTT e avisa no log.
- Promotion rule: n/a.

## Why Preview Instead Of Discovery

- Routing trigger: rota do plano.
- Discovery trigger check: `none`.
- Promotion rule: n/a.

## Product Increment Contribution

- Contribution type: `direct`.
- Visible increment unblocked: microfone limpo e push-to-talk.
- Why not direct: n/a.
- Enabling-chain check: n/a.

## Bet Framing

- Appetite: 1 sessão.
- Success evidence: teste com fala real + ruído: ativação por voz abre na fala e fecha no ruído; ruído suprimido; PTT global funcionando no app.
- Circuit breaker: n/a.
- Non-goals: escolha de dispositivo, volume por usuário, voz dentro do app (depende do teste do s02).
- Deferral path: `options.md` local.

## Scope

1. `client-audio`: `FrameAssembler` (mono, 48 kHz, 10 ms), `AudioProcessor` (RNNoise, ativação por voz com hangover e aquecimento, PTT, mudo), `capture` (cpal).
2. Eco e ganho automático: via APM do WebRTC no LiveKit (`AudioSourceOptions`, já ligado em `VoiceSession`).
3. `voice_poc` com `OXID_MIC=1`.
4. App: atalho global Alt+V e indicador de fala.

## Scope Completion Rule

When this sprint uses words like `smallest`, `narrow`, or `only as needed`,
they mean the smallest objective-complete vertical slice. Every named owned
surface, required evidence gate, and exit criterion remains in scope unless
the operator explicitly changes the objective.

## Operational Surface Preflight

- Triggered: yes.
- Surfaces: atalho global Alt+V; acesso ao microfone.
- Owner and lifecycle: `crates/client-audio`, `apps/desktop/src-tauri/src/lib.rs`.
- Visibility and safety: microfone só abre no `voice_poc` com `OXID_MIC=1` (no app, quando a voz entrar); falha ao registrar atalho não derruba o app.
- Inventory impact: README, `SOURCE_ATLAS.md`.
- Validation path: testes de `client-audio` + app real sob Xvfb.
- Promotion rule: n/a.

## Execution Slices

1. Processamento e captura (feito).
2. PTT no app (feito).
3. Microfone real ponta a ponta com LiveKit (pendente, mesma máquina do s02).

## Validation Plan

1. `cargo test -p oxid-client-audio` (inclui fala real sintetizada com espeak-ng).
2. App sob Xvfb: segurar Alt+V acende o indicador.
3. `voice_poc` com `OXID_MIC=1` (máquina com microfone, Docker e clang 21+).

## Live Validation Matrix

| Gate | Status | Command | Credentials / env | Evidence path | Pass criteria | Redaction |
| --- | --- | --- | --- | --- | --- | --- |
| `audio_pipeline` | passed | `cargo test -p oxid-client-audio` | nenhum | `evidence/index.md` | 8 testes | n/a |
| `ptt_desktop_real` | passed | `oxid-desktop` sob Xvfb + `xdotool keydown alt+v` | nenhum | `docs/screenshots/desktop-ptt.png` | indicador acende e apaga | n/a |
| `mic_live` | blocked | `OXID_MIC=1 ... voice_poc` | microfone, Docker, clang 21+ | `evidence/index.md` | o outro cliente ouve a voz, sem ruído de fundo | n/a |

## User Or Operator Evidence

- Status: `deferred`.
- Evidence: teste com microfone na máquina do Gustavo.
- Decision impact: pode ajustar limiar (0.6) e hangover (300 ms).

## Reviewed-Mode Start Checkpoint

Iniciado em modo `flow`.

## Context Receipt

Plano seção 4, s02.

## Exit Criteria

1. Every accepted objective claim has fresh named passing proof.
2. Required validation and live gates pass after the final relevant change.
3. Evidence is routed through `evidence/index.md`, with required objective
   gates separated from optional, not-triggered, blocked, out-of-scope, or
   operator-deferred checks.
4. Required objective proof is not deferred while closing the original
   objective as passed.
5. Decision log and closeout are current, including the closeout outcome and
   learning review.
