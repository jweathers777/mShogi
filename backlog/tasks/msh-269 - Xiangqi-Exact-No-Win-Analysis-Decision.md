---
id: MSH-269
title: Remove Xiangqi Exact No-Win Analysis
status: Ready
assignee: []
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-25T12:41:00Z'
labels:
  - rules
  - xiangqi
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 3700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John (2026-11-25): remove the optional Xiangqi exact Article 3.2A analysis and keep only the fast sound no-win recognition that game adjudication uses. Remove the solver and request (mshogi-core xiangqi/exact.rs and exact_request.rs, tests/xiangqi_exact.rs), the desktop evidence and its UI (shared_xiangqi.rs exact observations, shared_xiangqi_verification.rs, the NativeGame Evidence type if nothing else needs it, XiangqiAssessment.tsx parts) and the xiangqi_exact_probe example. The selected Xiangqi rules profile names this bounded exact path, so update the profile and its related documents.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The exact solver, its request and evidence types, the desktop evidence and UI and the probe example are removed; no code or user interface offers the exact analysis; the fast assessment is unchanged.
- [ ] #2 The Xiangqi rules profile (docs/m4/xiangqi/profile-proposal.md) and related documents state that only the fast recognition applies; no other rule changes.
- [ ] #3 Xiangqi rules tests of the remaining behavior keep their expected results; perft and workspace, frontend and affected checks pass; production line counts before and after are recorded.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John (MSH-209, 2026-11-25): apply decision 1 (one Chess judging mode) to Xiangqi as well, and keep only the fast sound no-win recognition? Recommendation: yes, for the same reasons as for Chess: expensive exact proofs are not delivery gates, and the usual practice is to detect only material cases. It removes about 1,150 lines in mshogi-core (solver, request and tests) plus the desktop evidence and UI. If John says no, archive this task. Set it to Ready when John approves.

John, 2026-11-25: Yes, remove it. This answers the question above and approves the rules profile change.
<!-- SECTION:NOTES:END -->
