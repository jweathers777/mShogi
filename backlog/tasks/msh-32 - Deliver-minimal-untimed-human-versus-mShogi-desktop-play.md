---
id: MSH-32
title: Deliver minimal untimed human-versus-mShogi desktop play
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T14:27:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-29
  - MSH-30
  - MSH-31
documentation:
  - docs/desktop/m3-design.md
ordinal: 32000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The first desktop slice must play real chess through the bundled engine while remaining responsive and protecting the live state.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A packaged running application supports new untimed standard chess games as either color, human moves and real bounded mShogi replies, automatic outcomes and explicit current/intended draw claims.
- [x] #2 Actual final engine observations are displayed; cancellation, new game, retry and failures cannot apply obsolete work or fabricate a chess conclusion; unsupported modes fail clearly.
- [x] #3 Focused integrated tests, actual desktop play/screenshots, independent review, normal rewrite integration/push and applicable native CI are recorded, along with honest launch instructions and remaining limitations.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Review and exercise the integrated playable application, then add public-controller/real-process tests for both colors, actual telemetry, cancellation/new-game/retry and unscored failures; verify native snapshot fixture freshness; retain package and shutdown evidence, independently review, integrate and collect applicable native CI while reusing unchanged app and M2 evidence.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @codex with isolated native integration-test builder /root/desktop_engine; branch codex/m3-desktop, reusable checkout .claude/worktrees/next-task. Alldependencies29/30/31Done; final board source d2883df passed both3platformCIworkflows. Runtime application already implements minimalplay; remainingworkisfocusedcomplete-sliceverification and evidence, nofutureM3featureimplementation.

Focused final-slice verification on local arm64 macOS: cargo test --locked -p mshogi-console --test desktop_play passed all four public-controller actual-process tests (3.17 s); focused Clippy with -D warnings and workspace formatting passed. Both human colors play multiple native replies; checks replay legal history/PV, real bounded observations/perspective/searched version, one-time acceptance, cancel then successful retry, new-game supersession, stale command rejection, and actual MissingResponse/UnexpectedExit staying paused and unscored. Native frontend fixture regeneration produced no diff; Rust CI now enforces it. Production application/frontend/engine sources remain unchanged from d2883df, whose three-platform native and package evidence is retained in docs/desktop/first-batch-evidence.md and the platform JSON reports. Final integrated macOS screenshot added; standard-opening GUI journeys do not claim rare promotion/claim GUI reachability, which have composed native/frontend tests. Independent review requested from /root/m3_contract_review; integration/native CI follow before completion.

Independent review /root/m3_contract_review: no actionable findings. Reviewer independently ran cargo test -p mshogi-console --test desktop_play --locked --offline (4 passed, 3.09 s), regenerated all four native frontend payloads into a temporary directory and byte-compared exact matches. Review assessed meaningful actual-process/controller assertions, development-only dependency, fixture-drift CI, platform reports, final screenshot and honest separation of packaged GUI journeys, composed rare-state tests and later M3 work. Final integration and applicable three-platform Rust CI remain the completion gate.

Integrated and normally pushed to codex/rewrite as 111ef41a0aa6aa11f473b687d32143e2af38268c. Native CI https://github.com/jweathers777/mShogi/actions/runs/40202107810 passed on the exact source: macOS 15 job 121195260977, Ubuntu 24.04 job 121195261183, Windows 2022 job 121195261269. Each ran locked workspace format, warnings-denied Clippy, tests including the four actual-process desktop_play tests, native frontend fixture drift verification, build and console smoke. Production app/frontend/bundled-engine sources are unchanged from d2883dfea338e0048d31e138b4c5ae49cc6cd4a8; successful native package CI https://github.com/jweathers777/mShogi/actions/runs/40200520366 and retained platform reports remain applicable. Redundant package workflow 40202107806 was intentionally cancelled, not reported as passed.

Acceptance coverage: AC1 uses actual normal macOS packaged play as both colors plus native/frontend coverage for automatic outcomes, all promotions and current/intended claims; exact GUI journeys/screenshots and rare-state scope are in docs/desktop/first-batch-evidence.md. AC2 has real final observations, actual-process cancellation/new-game/retry/failure checks, prior exact-context/stale-generation/native atomicity coverage and explicit Full refusal. AC3 has focused checks, clean independent review, durable screenshots and platform reports, desktop/README.md launch instructions, normal integration/push and exact-source native CI above. M2 evidence is unchanged and reused. Full installed-app interactions on all systems remain MSH-38.

Batch boundary: MSH-27 through MSH-32 are complete. M2 stays closed and M3 stays open. No implementation of MSH-33 through MSH-39 is included. Next eligible work is MSH-33 bounded analysis, MSH-34 fuller outcomes/recovery, or MSH-36 the labeled synthetic board fixture; saving/history, integrated accessibility, all-platform installed UI and M3 closure follow their dependencies.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered the first desktop batch: a normal packaged Tauri application supports untimed human-versus-mShogi chess as either color, native legal choices/promotions/explicit claims, click/keyboard interaction and orientation, bounded real engine replies with final telemetry, cancellation/retry and unscored failures. Rust owns serialized authoritative state and exact-context result acceptance. Four real-process controller integration tests, native-generated frontend fixture drift checks, independent review and exact-source native CI passed; unchanged three-platform package evidence is reused. Integrated/pushed on codex/rewrite at 111ef41a0aa6aa11f473b687d32143e2af38268c. Launch/evidence: desktop/README.md and docs/desktop/first-batch-evidence.md. Games remain unsaved; analysis, historical viewing, broader recovery, synthetic fixture, full accessibility/installed-platform verification and M3 closure remain explicit dependent MSH-33–39 work. M2 stays closed; M3 remains open.
<!-- SECTION:FINAL_SUMMARY:END -->
