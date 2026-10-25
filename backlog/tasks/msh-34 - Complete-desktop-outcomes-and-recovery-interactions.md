---
id: MSH-34
title: Complete desktop outcomes and recovery interactions
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T15:50:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-32
documentation:
  - docs/desktop/m3-design.md
ordinal: 34000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Minimal play must grow into clear complete outcome and recovery handling without conflating chess results and engine failures.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Claims, concluded chess outcomes, unresolved unscored match outcomes, resignation and engine failures have distinct user-facing states and commands respecting the core profile.
- [x] #2 Recovery and cancellation interactions preserve authoritative history and permit safe explicit retry/new-game behavior; no evaluation-based result is invented.
- [x] #3 Focused native and desktop interaction tests cover intended claims, unresolved outcomes, failure/retry and cancellation races; any timed-game addition first records and verifies suspension/resumption and event-order policy.
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
Implement a serialized native resignation command with M2 proof/uncertainty semantics and atomic history-preserving recovery guards; expose clear outcome/recovery UI with confirmation and technical details. Add focused native/frontend tests and isolated verification-only state construction for difficult packaged journeys. Exercise normal and labeled verification packages, independently review, integrate/push and collect applicable native/package CI before Done.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner clarification 2026-10-25: next authorized batch order is MSH-34 then MSH-33 then MSH-35 after dependencies. Complete distinct user-facing claims, concluded chess outcomes, resignation, unresolved unscored outcomes, cancelled/incomplete work and engine failures. Preserve authoritative match/game history through cancellation/recovery/retry. Continue untimed only; no evaluation-based resignation/draw adjudication. Existing acceptance criteria and eleven-area mapping are preserved. Actual running packaged recovery/retry journeys required; difficult states may use a separately labeled verification build, excluded from normal packages. See docs/desktop/m3-design.md second-batch clarifications.

Owner @codex; reusable branch codex/m3-desktop at 47209e3, checkout .claude/worktrees/next-task. MSH-32 dependency is Done. Delegate isolated native and frontend implementation under one primary integrator; no future-ticket implementation before its turn. M2 and MSH-27–32 remain closed/Done; M3 remains open.

Implementation and local evidence ready for review. Native Resign uses core M2 no-witness feasibility (unknown unresolved/unscored; sound impossibility draw), works during either turn, retires old work. Cancel/Retry reject inappropriate states atomically and preserve exact canonical game/match history. Focused verification: 16 native all-feature tests, four actual-process controller tests, native/host Clippy -D warnings and format, host tests/verification-feature check, strict TypeScript and 22 frontend tests all passed. Seven generated native fixtures byte-match. Actual normal macOS packaged e2e4/a7a5 resignation preserved history and displayed unresolved/unscored. Labeled verification app exercised intended claim without a move, real zero-node incomplete search then successful Retry, actual MissingResponse after e2e4 then restored-engine Retry a7a5, and slow child cancellation preserving three plies with child absence confirmed. Normal host SHA f5b149037f6f82d92e9fbf0dc9caeb8175dac2637a0aa22ba1de57bb6e1d07db excludes fixture initializer; actual engine SHA1461e608869d948b945e811075df7e5ebc07c49b3d8e8a91c72aa6117083a60e unchanged. Screenshots/report and exact scope in docs/desktop/second-batch-evidence.md. Independent /root/m3_contract_review assessment requested; integrate/native package CI before Done.

Independent review /root/m3_contract_review found no actionable issues. Reviewer independently passed 16 all-feature native tests, four real-process controller tests (3.22 s), 22 frontend tests and strict TypeScript; independently confirmed the normal package has no fixture marker and matches retained host SHA. Review checked M2 resignation semantics, either-turn/stale confirmation protection, atomic Cancel/Retry history preservation, feature exclusion and honest screenshot/evidence scope. Normal integration and exact-source native/package CI remain before completion.

Integrated and normally pushed to codex/rewrite at 0776f7ea02fa489a0c1b61491b5095288a2f662b. Exact-source Rust CI https://github.com/jweathers777/mShogi/actions/runs/40208003600 passed macOS15 job121213005021, Ubuntu24.04 job121213005152, Windows2022 job121213005189; includes all-feature verification-constructor checks and native fixture drift. Normal package CI https://github.com/jweathers777/mShogi/actions/runs/40208003599 passed Ubuntu24.04 job121213005119, macOS15 job121213005356, Windows2022 job121213005400; strictTS,22frontendtests, nativehost checks including verification-feature compilation, normalpackage build, fixture-marker exclusion and actual packaged UCI/structured searches. AC1: native resign/M2 uncertainty and distinct frontend claims/conclusions/unresolved/failure states with tests and actual normal resignation. AC2: exact-record recovery guards plus actual missing-response→real retry and cancellation journeys. AC3: focused16native/4actualprocess/22frontend tests, intended-claim/uncertainty/failure/race coverage, actual GUI evidence; timedplay is explicitly excluded. Independent review clean; normal integration/push complete. M2/27–32 remain closed; M3 remains open.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed untimed desktop outcomes and recovery. Native resignation retains M2 proof/uncertainty semantics; current/intended claims, concluded results, unresolved unscored outcomes, cancellation/incomplete work and engine failures remain distinct. Atomic recovery guards preserve exact match/game history, stale confirmations/results cannot alter newer state, and explicit Retry resumes real engine play. Focused native/frontend/process checks, actual normal and separately labeled verification-package GUI journeys and independent review passed. Integrated/pushed as0776f7e on codex/rewrite; Rust40208003600 and package40208003599 passed all three native systems. Evidence/launch: docs/desktop/second-batch-evidence.md and desktop/README.md. No clocks or evaluation-based results added. MSH-33 thenMSH-35 are next in authorized batch; M3 stays open.
<!-- SECTION:FINAL_SUMMARY:END -->
