---
id: MSH-34
title: Complete desktop outcomes and recovery interactions
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T15:39:00Z'
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
- [ ] #1 Claims, concluded chess outcomes, unresolved unscored match outcomes, resignation and engine failures have distinct user-facing states and commands respecting the core profile.
- [ ] #2 Recovery and cancellation interactions preserve authoritative history and permit safe explicit retry/new-game behavior; no evaluation-based result is invented.
- [ ] #3 Focused native and desktop interaction tests cover intended claims, unresolved outcomes, failure/retry and cancellation races; any timed-game addition first records and verifies suspension/resumption and event-order policy.
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
Implement a serialized native resignation command with M2 proof/uncertainty semantics and atomic history-preserving recovery guards; expose clear outcome/recovery UI with confirmation and technical details. Add focused native/frontend tests and isolated verification-only state construction for difficult packaged journeys. Exercise normal and labeled verification packages, independently review, integrate/push and collect applicable native/package CI before Done.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner clarification 2026-10-25: next authorized batch order is MSH-34 then MSH-33 then MSH-35 after dependencies. Complete distinct user-facing claims, concluded chess outcomes, resignation, unresolved unscored outcomes, cancelled/incomplete work and engine failures. Preserve authoritative match/game history through cancellation/recovery/retry. Continue untimed only; no evaluation-based resignation/draw adjudication. Existing acceptance criteria and eleven-area mapping are preserved. Actual running packaged recovery/retry journeys required; difficult states may use a separately labeled verification build, excluded from normal packages. See docs/desktop/m3-design.md second-batch clarifications.

Owner @codex; reusable branch codex/m3-desktop at 47209e3, checkout .claude/worktrees/next-task. MSH-32 dependency is Done. Delegate isolated native and frontend implementation under one primary integrator; no future-ticket implementation before its turn. M2 and MSH-27–32 remain closed/Done; M3 remains open.

Implementation and local evidence ready for review. Native Resign uses core M2 no-witness feasibility (unknown unresolved/unscored; sound impossibility draw), works during either turn, retires old work. Cancel/Retry reject inappropriate states atomically and preserve exact canonical game/match history. Focused verification: 16 native all-feature tests, four actual-process controller tests, native/host Clippy -D warnings and format, host tests/verification-feature check, strict TypeScript and 22 frontend tests all passed. Seven generated native fixtures byte-match. Actual normal macOS packaged e2e4/a7a5 resignation preserved history and displayed unresolved/unscored. Labeled verification app exercised intended claim without a move, real zero-node incomplete search then successful Retry, actual MissingResponse after e2e4 then restored-engine Retry a7a5, and slow child cancellation preserving three plies with child absence confirmed. Normal host SHA f5b149037f6f82d92e9fbf0dc9caeb8175dac2637a0aa22ba1de57bb6e1d07db excludes fixture initializer; actual engine SHA1461e608869d948b945e811075df7e5ebc07c49b3d8e8a91c72aa6117083a60e unchanged. Screenshots/report and exact scope in docs/desktop/second-batch-evidence.md. Independent /root/m3_contract_review assessment requested; integrate/native package CI before Done.

Independent review /root/m3_contract_review found no actionable issues. Reviewer independently passed 16 all-feature native tests, four real-process controller tests (3.22 s), 22 frontend tests and strict TypeScript; independently confirmed the normal package has no fixture marker and matches retained host SHA. Review checked M2 resignation semantics, either-turn/stale confirmation protection, atomic Cancel/Retry history preservation, feature exclusion and honest screenshot/evidence scope. Normal integration and exact-source native/package CI remain before completion.
<!-- SECTION:NOTES:END -->
