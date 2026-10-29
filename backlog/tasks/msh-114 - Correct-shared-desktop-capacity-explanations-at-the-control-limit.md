---
id: MSH-114
title: Correct shared desktop capacity explanations at the control limit
status: Ready
assignee: []
created_date: '2026-10-29T20:22:00Z'
labels:
  - m4
  - desktop
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-99
  - MSH-102
  - MSH-113
references:
  - desktop/src/product/NativeSessionPanels.tsx
  - crates/mshogi-desktop-session/src/shared_xiangqi_archive.rs
  - crates/mshogi-desktop-session/src/shared_archive.rs
documentation:
  - docs/m4/xiangqi/delivery-review.md
priority: medium
type: bug
ordinal: 114000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-103 fresh independent review found that NativeRecovery prioritizes any diagnostic containing 256 as an observation-count limit. Xiangqi check_capacity and Shogi counts emit combined diagnostics naming both 256 observations and 8192 controls. A control-only refusal with zero observations therefore tells the player that 256 observations were recorded. Native admission and complete saveability remain correct. Correct this shared user-facing resource diagnosis without changing accepted rules, archive compatibility, limits, retained history or MSH-61 guarantees. This is the bounded corrective prerequisite to final MSH-103 acceptance; no broader UI redesign or architecture review is authorized.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 At a real control-only refusal, including an active document with 8191 controls and one reserved stop/close slot but zero observations, both Xiangqi and Shogi show a truthful control/application-limit explanation and never claim that 256 observations were recorded.
- [ ] #2 The genuine 256-to-257 observation boundary remains accurately explained, including Xiangqi ordinary plus exact observations. Byte, native-event and replay-work resource failures are not mislabeled as observation/control exhaustion, illegal actions or game outcomes.
- [ ] #3 Current complete history and accepted observations remain saveable without discard or raised constants. Save and Save As remain available where native admission permits them; Save/Open does not reset structural limits and the user is not told to retry an unchanged impossible operation.
- [ ] #4 Focused tests exercise actual native diagnostic payloads through shared presentation for control-only, observation-only and ambiguous combined text in both consumers, with current native capacity/stop-reserve regressions reused or rerun as applicable. Verify the user interaction in a running packaged app using separately labeled instrumentation if necessary, excluded from normal packages.
- [ ] #5 Record focused checks and exact source/package evidence, independent review and resolved findings, applicable CI and normal integration/push. Reconcile the MSH-103 matrix and acceptance blocker after correction without closing MSH-45 or starting MSH-106.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
