---
id: MSH-111
title: Diagnose Linux packaged GUI driver proxy resets without replaying actions
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-28T13:58:00Z'
updated_date: '2026-10-28T14:52:00Z'
labels:
  - ci
  - verification
milestone: m-4
dependencies: []
references:
  - https://github.com/jweathers777/mShogi/actions/runs/40692691108
  - https://github.com/jweathers777/mShogi/actions/runs/40698788452
documentation:
  - docs/m4/xiangqi/session-contract.md
  - docs/desktop/linux-gui-proxy.md
ordinal: 111000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Linux tauri-driver2.0.6 intermittently loses the downstream connection with Hyper SendRequest/ECONNRESET. Baseline74a639 and integrated2b50ac2/6f6ea69 show this on screenshots, DOM reads and New game clicks, followed by successful same-session captures. Aborted journeys cause consequential missing-save errors. Native, frontend/host and package checks pass. Pooling is a hypothesis, not an established cause. Diagnose separately without gating unaffected Xiangqi delivery or changing game semantics; MSH-98 owns native session contracts rather than this infrastructure campaign.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Concretely diagnose or reproduce the failures with exact driver, WebKit, package, source and environment provenance; distinguish proxy failures from application/native IPC failures and retain original failed evidence.
- [x] #2 Any correction is external to normal distributables, bounded and independently reviewed. Never automatically replay ambiguously delivered mutations, input or lifecycle actions; semantic errors and exhausted observations remain fatal and all journey assertions remain intact.
- [x] #3 Record a complete successful actual Linux installed/packaged Chess/Shogi GUI campaign across all current phases, including play, analysis/cancellation, claims, recovery, Save/Open, history and native close/quit. Distinguish driver tests, package checks and actual GUI evidence.
- [ ] #4 Run focused boundary tests and appropriate compatibility checks for shared verification changes; record exact tested/integrated revisions and CI including failures, without inferring other-platform success.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Preserve failed Linux reports/logs and exact package, source, tauri-driver/WebKit/environment provenance; inspect proxy and verifier boundaries. 2. Reproduce the downstream reset with a controlled external-driver test; evaluate a bounded Linux-only correction outside all product packages, with no automatic mutation replay. 3. Add focused failure-after-delivery, semantic/exhaustion and compatibility checks; obtain independent review of the bounded change. 4. Run all current actual installed Linux Chess/Shogi GUI phases in CI; retain failures and package identities, integrate only supported results into codex/rewrite, and record exact revisions and remaining limits.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex root; isolated branch codex/msh111-gui-proxy, app-managed worktree /Users/johnweathers/.codex/worktrees/msh111-gui-proxy/mShogi at base 06f629f. MSH-99 and its files are outside this task. Pinned Backlog.md 1.55.1 used for mutations; PATH 1.55.0 used only for initial reads.

Implementation b8beae446b1ab303f91275c8b89603164275b7a0 pushed for CI: Desktop 40706231031, Rust 40706230906. Linux external driver uses checksum-pinned tauri-driver2.0.6 source, changes only downstream pool_max_idle_per_host(0), retains retry_canceled_requests(false), and builds baseline/corrected binaries with Rust1.90 and locked dependencies. Shared wait now propagates semantic and exhausted observation failures. Local macOS Python3.11 GUI boundary discovery:36 passed. Independent reviewer msh111_review found no actionable implementation issues; actual Linux evidence pending. Original baseline/integrated/retry failed reports, failure captures, driver logs and package manifests copied byte-for-byte with archive/file SHA-256 index under docs/desktop/evidence/msh111/prior.

Independent review found a real issue in the synthetic pooling harness: urllib forces Connection: close so its seed did not deterministically exercise reuse. Resolved in 2611d0921dd3c637e76b7bb08bc0a23b49d84f33 with explicit keep-alive only for the reuse stress case, recorded/asserted headers and connection IDs; delivered/semantic cases retain actual Webview transport. Reviewer re-reviewed and found no further code issues. Controlled reuse reproduction is not claimed as proof of the original intermittent WebKit close-timing cause. Initial b8beae4 CI runs cancellation requested as superseded, not passing evidence. Corrected Desktop40706737407 / Rust40706737330 pending.

Corrected CI2611d09: Rust40706737330 passed on Ubuntu24.04, macOS15 and Windows2022. Linux Desktop40706737407 passed frontend/native checks, normal Debian build/smoke/install identity checks,36 focused boundary tests and all six real-proxy experiments; separate verification package and actual14-phase GUI campaign still pending. Independent reviewer verified retained original workflow log and resolved provenance finding. A final mechanical workflow change copies provenance to workspace gui-driver-provenance.json before tests and uploads that relative path, preserving existing artifact roots; independently reviewed with no findings or need to repeat unchanged behavior tests.

Actual Linux Desktop40706737407/job122880769223 at2611d09 PASSED all14installed/packaged phases; independent reviewer audited all223GUI file checksums,13normal-package manifest files, verification Deb hash, executed driver/provenance, all14native host/engine absence events and six real-proxy cases, with no findings. No ordinary observation/lookup retry events; five final guard transport closures remain qualified by independent native absence. Windows job122880769625 PASSED all14actual GUI phases; root verified196report file hashes and14shutdown events. Raw reports, exact job logs/package identities/proxy provenance and selected native captures preserved under docs/desktop/evidence/msh111/success with artifact IDs/digests and SHA256index. Final macOS package build still running; no macOS GUI claim. Ready for independent final evidence assessment and integration.

Final independent assessment recommends acceptance/integration: Windows14phases/196hashes, all142selected success-index hashes, Linux14phases/223hashes/sixproxy experiments, allplatform Desktop40706737407 and Rust40706737330 metadata verified at2611d09. No code/evidence blockers; all findings resolved. macOS evidence is native/frontend/package smoke only. Primary MSH-99 remains In Progress with its own owner; no other dependency-ready Ready task currently listed. Integration criterion4/checklist completion awaits recorded codex/rewrite commit and preservation check.
<!-- SECTION:NOTES:END -->
