---
artifact_id          : w01-i01-s05-evidence-index
artifact_type        : evidence_index
status               : active
context_tier         : T4
updated_at           : 2026-11-17
summary_budget_lines : 100
---

# Evidence Index

## State

Ativo em 2026-11-17. Processamento e PTT validados; microfone ao vivo pendente.

## Validated Claims

| Claim | Evidence | Gate | Status | Consumed By |
| --- | --- | --- | --- | --- |
| Conversão para mono 48 kHz em quadros de 10 ms (inclui 44,1 kHz e estéreo) | `crates/client-audio/src/frames.rs` testes | `audio_pipeline` | passed | closeout |
| Com fala real + ruído: ativação por voz abre em 468/472 quadros de fala, 0 antes e 0 depois | `crates/client-audio/tests/pipeline.rs` | `audio_pipeline` | passed | closeout |
| RNNoise remove ruído de fundo (1,32 -> 0,00) e preserva a fala (34,9 -> 31,6) | idem | `audio_pipeline` | passed | closeout |
| PTT e mudo controlam a transmissão; hangover correto | `processor.rs` testes | `audio_pipeline` | passed | closeout |
| Atalho global Alt+V acende e apaga o indicador no app real | `docs/screenshots/desktop-ptt.png` | `ptt_desktop_real` | passed | closeout |
| Voz do microfone chega limpa ao outro cliente | — | `mic_live` | blocked | closeout |

## Evidence Files

| File | Type | Status | Open When |
| --- | --- | --- | --- |
| `crates/client-audio/tests/pipeline.rs` | proof | passed | regressão de VAD/supressão |
| `crates/client-audio/tests/fixtures/fala-ptbr.wav` | fixture | n/a | trocar a amostra de fala |

## Live Gates And Deferred Evidence

Obrigatório pendente: `mic_live` (máquina com microfone, Docker e clang 21+).

## Redaction And Retention

No raw secrets, raw transcripts, raw prompts, raw model responses, raw file
content, or raw tool arguments unless an explicit debug-export policy allows it.

## Promotion

`status.md`, `roadmap.md`.

## Read Policy

Open this index for claim verification, validation audit, closeout review, or
regression investigation. Open focused evidence files only through a specific
claim or gate trigger.
