---
id: MSH-269
title: Xiangqi Exact No-Win Analysis Decision
status: To Do
assignee: []
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-25T12:23:00Z'
labels:
  - rules
  - xiangqi
  - needs-john
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7890
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Xiangqi has an exact Article 3.2A graph search: mshogi-core xiangqi/exact.rs (381 lines) and exact_request.rs (160), tests/xiangqi_exact.rs (611). Only an explicit desktop analysis request starts it, and the desktop keeps its reports as session evidence (desktop-session shared_xiangqi.rs exact_observations, shared_xiangqi_verification.rs, the NativeGame Evidence type, desktop/src/product/XiangqiAssessment.tsx). Game adjudication does not use it: xiangqi/game.rs uses the fast sound recognition of xiangqi/assessment.rs. It is the Xiangqi parallel of the Chess Full reachability that John removed on 2026-11-25 (MSH-246, One Chess Judging Mode). Removing a desktop analysis feature changes product behavior, so it needs John.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 John's choice is recorded in the notes.
- [ ] #2 If John approves removal: the exact solver, its request and evidence types, the desktop evidence and its UI are removed; the fast assessment stays; Xiangqi rules tests whose expected results come from the rules pass; one review by a fresh agent session.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John (MSH-209, 2026-11-25): apply decision 1 (one Chess judging mode) to Xiangqi as well, and keep only the fast sound no-win recognition? Recommendation: yes, for the same reasons as for Chess: expensive exact proofs are not delivery gates, and the usual practice is to detect only material cases. It removes about 1,150 lines in mshogi-core (solver, request and tests) plus the desktop evidence and UI. If John says no, archive this task. Set it to Ready when John approves.
<!-- SECTION:NOTES:END -->
