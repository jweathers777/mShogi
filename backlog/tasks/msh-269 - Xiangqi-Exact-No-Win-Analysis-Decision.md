---
id: MSH-269
title: Remove Xiangqi Exact No-Win Analysis
status: Done
assignee:
  - '@claude'
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-25T19:18:00Z'
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
- [x] #1 The exact solver, its request and evidence types, the desktop evidence and UI and the probe example are removed; no code or user interface offers the exact analysis; the fast assessment is unchanged.
- [x] #2 The Xiangqi rules profile (docs/m4/xiangqi/profile-proposal.md) and related documents state that only the fast recognition applies; no other rule changes.
- [x] #3 Xiangqi rules tests of the remaining behavior keep their expected results; perft and workspace, frontend and affected checks pass; production line counts before and after are recorded.
- [x] #4 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Core: delete exact.rs, exact_request.rs, the probe example and the exact tests (keep their independent palace legality oracle and fast-miss facts); remove UnknownExactIncomplete, ExactReport::assessment and NATURAL_WIN_GOAL; rename the fast miss to NoWinAssessment::Unknown.
2. Search/engine/console: remove search_requiring_root_exact, Decision::ExactNoWin, AssessmentEvidence, the Unresolved enum and SearchError::Exact; XiangqiResult::searched infallible; console keeps only nowin fast.
3. Desktop: remove XiangqiExactEvidence, exact_observations, the generic NativeGame Evidence type and its session plumbing, the xiangqi-exact-evidence verification fixture and GUI script phase, and the exact report UI; regenerate fixtures.
4. Docs: profile and related documents state that only the fast recognition applies.
5. Checks, line counts, fresh review, integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John (MSH-209, 2026-11-25): apply decision 1 (one Chess judging mode) to Xiangqi as well, and keep only the fast sound no-win recognition? Recommendation: yes, for the same reasons as for Chess: expensive exact proofs are not delivery gates, and the usual practice is to detect only material cases. It removes about 1,150 lines in mshogi-core (solver, request and tests) plus the desktop evidence and UI. If John says no, archive this task. Set it to Ready when John approves.

John, 2026-11-25: Yes, remove it. This answers the question above and approves the rules profile change.

Checkpoint (claude, branch claude/msh-269-remove-xiangqi-exact): code removal committed (core solver/request/test/probe, search exact path and AssessmentEvidence/Unresolved enums, engine, console nowin exact, desktop evidence and Evidence type, frontend UI, fixtures regenerated, GUI script phase). fmt and clippy pass. Next: docs, full tests, perft, frontend checks, review, integrate.

Checkpoint 2 (claude): code and docs done on branch claude/msh-269-remove-xiangqi-exact. Decision: the fast miss variant UnknownExactUnavailable is renamed NoWinAssessment::Unknown (wire 'unknown'), because the old name promised an exact request that no longer exists; results are unchanged. search::Unresolved and AssessmentEvidence collapsed to Option<PendingAdjudication> and NoWinAssessment; XiangqiResult::searched is now infallible. Mixed tests kept their fast parts (search: fast miss is Unknown and not a draw, possible win is not a forced-win score; engine handoff: fast proof/Unknown across reopen; console: nowin fast is read-only). Checks so far: fmt, clippy (workspace and desktop-session all features), cargo test --workspace, desktop-session --all-features (197 passed), Xiangqi perft incl. ignored (release), npm run check, vitest 588 passed, python test_gui_smoke+test_macos_verification 141 passed. Production lines (crates/*/src without test/conformance files): 69742 before, 68548 after; Xiangqi-named files 10182 -> 9084; desktop/src + src-tauri without tests/fixtures 3960 -> 3930. Next: fresh review, merge main, integrate.

Review (fresh subagent, opus, read-only, diff origin/main...HEAD): no bugs; fast assessment results unchanged; rename to Unknown consistent; no other rule changed. Fixed: (1) the deleted exact test also held an independent palace legality oracle (General+General and +Advisor universes, 108/510/510 states, legal sets and transitions) - restored without the exact assertions in tests/xiangqi_board.rs; (2) restored the two-ply fast-miss facts (5k3/R8 b: fast Unknown, only f9e9, f9e9 a8f8 stalemate) in xiangqi_game_boundary.rs; (3) removed NATURAL_WIN_GOAL (no users; the goal stays documented); merged search run into search_with_state; removed a stale doc phrase; removed two record checks that named the removed bounded_exact mode (generic unknown-field rejection stays tested); trimmed dated history notes in docs (kept in the profile and D-15). Not changed: rustfmt-only reformat in xiangqi_uci.rs (caused by a shorter string).

Final checks after review fixes: fmt, clippy (workspace; desktop-session all features), cargo test --workspace (1163 passed, 0 failed), desktop-session --all-features (197 passed), Xiangqi perft incl. ignored in release (core board 6, core lib 25, search position 19 passed), npm run check, vitest 588 passed (Node 24.22.3), python test_gui_smoke+test_macos_verification 141 passed. CI ubuntu-24.04: push routine checks (45682082744) and workflow_dispatch fixtures (45682649914), process (45682654170), persistence (45682658347) all passed. Final production lines (crates/*/src without test/conformance files): 69742 before, 68538 after (-1204); Xiangqi-named files 10182 -> 9074; desktop/src + src-tauri without tests/fixtures 3960 -> 3930. No strength match, cloud job or spending.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed the optional Xiangqi exact Article 3.2A no-win analysis, as John decided on 2026-11-25; only the sound fast recognition remains, with unchanged results. Removed: the core solver and request (xiangqi/exact.rs, exact_request.rs), the xiangqi_exact_probe example and exact tests, the search exact root path (search_requiring_root_exact, Decision::ExactNoWin, AssessmentEvidence, Unresolved/SearchError exact variants), the console nowin exact/poll/wait/cancel worker, the desktop exact evidence with the generic NativeGame Evidence type and session plumbing, the exact report UI and types, and the xiangqi-exact-evidence verification fixture and GUI-script phase. Simplified: the fast miss is NoWinAssessment::Unknown (wire 'unknown'); search unresolved is Option<PendingAdjudication>; XiangqiResult::searched is infallible; unused NATURAL_WIN_GOAL removed. Kept-behavior checks from the deleted tests were preserved (palace legality oracle, two-ply fast miss). The rules profile, D-15 and related Xiangqi, desktop and search docs now state that only the fast recognition applies; no other rule changed. All local checks, perft, frontend, script tests and Ubuntu CI (routine, fixtures, process, persistence) passed. Production lines 69742 -> 68538. Review: no bugs; findings fixed. Follow-up: none.
<!-- SECTION:FINAL_SUMMARY:END -->
