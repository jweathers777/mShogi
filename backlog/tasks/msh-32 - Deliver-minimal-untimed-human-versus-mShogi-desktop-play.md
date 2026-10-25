---
id: MSH-32
title: Deliver minimal untimed human-versus-mShogi desktop play
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T14:24:00Z'
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
- [ ] #1 A packaged running application supports new untimed standard chess games as either color, human moves and real bounded mShogi replies, automatic outcomes and explicit current/intended draw claims.
- [ ] #2 Actual final engine observations are displayed; cancellation, new game, retry and failures cannot apply obsolete work or fabricate a chess conclusion; unsupported modes fail clearly.
- [ ] #3 Focused integrated tests, actual desktop play/screenshots, independent review, normal rewrite integration/push and applicable native CI are recorded, along with honest launch instructions and remaining limitations.
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
Review and exercise the integrated playable application, then add public-controller/real-process tests for both colors, actual telemetry, cancellation/new-game/retry and unscored failures; verify native snapshot fixture freshness; retain package and shutdown evidence, independently review, integrate and collect applicable native CI while reusing unchanged app and M2 evidence.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @codex with isolated native integration-test builder /root/desktop_engine; branch codex/m3-desktop, reusable checkout .claude/worktrees/next-task. Alldependencies29/30/31Done; final board source d2883df passed both3platformCIworkflows. Runtime application already implements minimalplay; remainingworkisfocusedcomplete-sliceverification and evidence, nofutureM3featureimplementation.

Focused final-slice verification on local arm64 macOS: cargo test --locked -p mshogi-console --test desktop_play passed all four public-controller actual-process tests (3.17 s); focused Clippy with -D warnings and workspace formatting passed. Both human colors play multiple native replies; checks replay legal history/PV, real bounded observations/perspective/searched version, one-time acceptance, cancel then successful retry, new-game supersession, stale command rejection, and actual MissingResponse/UnexpectedExit staying paused and unscored. Native frontend fixture regeneration produced no diff; Rust CI now enforces it. Production application/frontend/engine sources remain unchanged from d2883df, whose three-platform native and package evidence is retained in docs/desktop/first-batch-evidence.md and the platform JSON reports. Final integrated macOS screenshot added; standard-opening GUI journeys do not claim rare promotion/claim GUI reachability, which have composed native/frontend tests. Independent review requested from /root/m3_contract_review; integration/native CI follow before completion.

Independent review /root/m3_contract_review: no actionable findings. Reviewer independently ran cargo test -p mshogi-console --test desktop_play --locked --offline (4 passed, 3.09 s), regenerated all four native frontend payloads into a temporary directory and byte-compared exact matches. Review assessed meaningful actual-process/controller assertions, development-only dependency, fixture-drift CI, platform reports, final screenshot and honest separation of packaged GUI journeys, composed rare-state tests and later M3 work. Final integration and applicable three-platform Rust CI remain the completion gate.
<!-- SECTION:NOTES:END -->
