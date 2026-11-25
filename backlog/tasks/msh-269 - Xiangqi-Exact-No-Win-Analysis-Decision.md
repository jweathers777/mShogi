---
id: MSH-269
title: Remove Xiangqi Exact No-Win Analysis
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-25T18:43:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Core: delete xiangqi/exact.rs, exact_request.rs, tests/xiangqi_exact.rs, examples/xiangqi_exact_probe.rs; remove UnknownExactIncomplete and ExactReport::assessment; rename the fast miss UnknownExactUnavailable to Unknown (the old name promises an exact request that no longer exists).
2. Search: remove search_requiring_root_exact, Decision::ExactNoWin, Unresolved::Exact, SearchError::Exact and AssessmentEvidence (result keeps the fast NoWinAssessment); update engine, console UCI and the xiangqi_baseline example.
3. Console: remove nowin exact/poll/wait/cancel and the worker; keep nowin fast.
4. Desktop session: remove XiangqiExactEvidence, exact_observations, the generic SessionGame Evidence type and its session plumbing, and the xiangqi-exact-evidence verification fixture; keep the other verification fixtures.
5. Frontend: remove the exact report UI from XiangqiAssessment.tsx and native.ts types; regenerate fixtures with write_frontend_fixtures; remove the xiangqi-exact-evidence phase from the GUI scripts only.
6. Docs: profile-proposal.md and related Xiangqi, desktop and search documents state that only the fast recognition applies.
7. Checks, line counts, fresh review, integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John (MSH-209, 2026-11-25): apply decision 1 (one Chess judging mode) to Xiangqi as well, and keep only the fast sound no-win recognition? Recommendation: yes, for the same reasons as for Chess: expensive exact proofs are not delivery gates, and the usual practice is to detect only material cases. It removes about 1,150 lines in mshogi-core (solver, request and tests) plus the desktop evidence and UI. If John says no, archive this task. Set it to Ready when John approves.

John, 2026-11-25: Yes, remove it. This answers the question above and approves the rules profile change.
<!-- SECTION:NOTES:END -->
