---
artifact_id          : w01-i01-s06-sprint-release-updater-site
artifact_type        : sprint_plan
status               : active
context_tier         : T3
updated_at           : 2026-11-17
summary_budget_lines : 480
---

# Sprint: Release, atualização automática e site de download

## Objective

Passo 6 do plano: CI com build, assinatura, updater e site de download.

## Source

- Accepted discovery or reviewed preview: plano de ação, passo 6 e seções 5 e 6.

## Routing Preflight

- Trigger categories considered: authority (chaves de assinatura), policy (segredos no CI), runtime-boundary (instalação de binários nos usuários).
- Triggered categories: authority, policy.
- Fail-closed result: `preview_ok`; nenhuma chave real é criada ou guardada pelo agente; segredos ficam com o Gustavo.

## Discovery Triage

- Outcome: `preview_ok`.
- Outside evidence trigger: plano seção 5 (tauri-plugin-updater, canais stable/beta/canary) e seção 8 (assinatura Windows/macOS).
- Trigger-presumed check: assinatura do updater obrigatória com versão assinada (`requireSignedVersion`).
- Promotion rule: Velopack (updates delta) fica como opção se o tamanho do download incomodar.

## Why Preview Instead Of Discovery

- Routing trigger: rota do plano.
- Discovery trigger check: `none`.
- Promotion rule: n/a.

## Product Increment Contribution

- Contribution type: `direct`.
- Visible increment unblocked: instalar pelo site e receber atualizações sozinho.
- Why not direct: n/a.
- Enabling-chain check: n/a.

## Bet Framing

- Appetite: 1 sessão.
- Success evidence: app 0.1.0 se atualiza sozinho para 0.1.1 assinado; manifesto adulterado é recusado; site detecta o sistema.
- Circuit breaker: n/a.
- Non-goals: compra do certificado do Windows, conta Apple, domínio, hospedagem.
- Deferral path: `options.md` raiz (assinatura de código) e status (bloqueios do Gustavo).

## Scope

1. Updater no app: checa ao abrir e a cada 6 h, baixa, avisa e reinicia; canal por build.
2. `release.yml`: tag -> build Windows/macOS/Linux, assinatura, release, manifesto por canal no R2.
3. `apps/website` em Astro: detecção de sistema, novidades, status, termos e privacidade (rascunhos).
4. CI compila o site.

## Scope Completion Rule

When this sprint uses words like `smallest`, `narrow`, or `only as needed`,
they mean the smallest objective-complete vertical slice. Every named owned
surface, required evidence gate, and exit criterion remains in scope unless
the operator explicitly changes the objective.

## Operational Surface Preflight

- Triggered: yes.
- Surfaces: workflow de release, segredos/variáveis do GitHub, manifesto `{base}/{canal}/latest.json`, site.
- Owner and lifecycle: `.github/workflows/release.yml`, `apps/desktop/src-tauri/src/updater.rs`, `apps/website`.
- Visibility and safety: chave privada só em segredo do GitHub; app recusa update sem assinatura ou com versão diferente da assinada.
- Inventory impact: README, `SOURCE_ATLAS.md`.
- Validation path: build assinado local + teste de update + Playwright no site.
- Promotion rule: n/a.

## Execution Slices

1. Updater (feito e testado).
2. Workflow de release (escrito; roda no primeiro push de tag).
3. Site (feito e testado).

## Validation Plan

1. `tauri build` com chave de teste gera `.deb`, `.AppImage` e `.sig`.
2. AppImage 0.1.0 + manifesto local com 0.1.1 -> arquivo substituído e aviso na tela.
3. Manifesto anunciando 0.1.2 com assinatura da 0.1.1 -> recusado.
4. Site: build e Playwright (Windows, macOS, iPhone, páginas).
5. Primeiro release real no GitHub (pendente: repositório e segredos).

## Live Validation Matrix

| Gate | Status | Command | Credentials / env | Evidence path | Pass criteria | Redaction |
| --- | --- | --- | --- | --- | --- | --- |
| `signed_bundle` | passed | `npx @tauri-apps/cli@2 build --bundles deb,appimage` | chave de teste descartável | `evidence/index.md` | bundles + `.sig` | chave de teste não versionada |
| `auto_update_e2e` | passed | AppImage 0.1.0 + servidor local | chave de teste | `docs/screenshots/desktop-update.png` | md5 igual ao 0.1.1 e aviso | n/a |
| `tampered_manifest` | passed | manifesto 0.1.2 + assinatura 0.1.1 | chave de teste | `evidence/index.md` | update recusado | n/a |
| `website` | passed | `npm run build` + Playwright | nenhum | `docs/screenshots/site.png` | 5 páginas, botão certo por sistema, sem rolagem lateral no celular | n/a |
| `release_workflow` | blocked | push de tag `v0.1.0-beta.1` | repositório + segredos | `evidence/index.md` | release publicado com instaladores dos 3 sistemas | n/a |

## User Or Operator Evidence

- Status: `deferred`.
- Evidence: repositório, chave do updater e certificados com o Gustavo.
- Decision impact: sem eles o workflow não roda.

## Reviewed-Mode Start Checkpoint

Iniciado em modo `flow`.

## Context Receipt

Plano seções 5, 6 e 8.

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
