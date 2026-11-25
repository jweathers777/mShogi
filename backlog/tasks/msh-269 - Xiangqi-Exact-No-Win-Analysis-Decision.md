---
id: MSH-269
title: Remove Xiangqi Exact No-Win Analysis
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-25T19:01:00Z'
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

Checkpoint (claude, branch claude/msh-269-remove-xiangqi-exact): code removal committed (core solver/request/test/probe, search exact path and AssessmentEvidence/Unresolved enums, engine, console nowin exact, desktop evidence and Evidence type, frontend UI, fixtures regenerated, GUI script phase). fmt and clippy pass. Next: docs, full tests, perft, frontend checks, review, integrate.

Checkpoint 2 (claude): code and docs done on branch claude/msh-269-remove-xiangqi-exact. Decision: the fast miss variant UnknownExactUnavailable is renamed NoWinAssessment::Unknown (wire 'unknown'), because the old name promised an exact request that no longer exists; results are unchanged. search::Unresolved and AssessmentEvidence collapsed to Option<PendingAdjudication> and NoWinAssessment; XiangqiResult::searched is now infallible. Mixed tests kept their fast parts (search: fast miss is Unknown and not a draw, possible win is not a forced-win score; engine handoff: fast proof/Unknown across reopen; console: nowin fast is read-only). Checks so far: fmt, clippy (workspace and desktop-session all features), cargo test --workspace, desktop-session --all-features (197 passed), Xiangqi perft incl. ignored (release), npm run check, vitest 588 passed, python test_gui_smoke+test_macos_verification 141 passed. Production lines (crates/*/src without test/conformance files): 69742 before, 68548 after; Xiangqi-named files 10182 -> 9084; desktop/src + src-tauri without tests/fixtures 3960 -> 3930. Next: fresh review, merge main, integrate.
<!-- SECTION:NOTES:END -->
