---
id: MSH-45
title: Deliver complete xiangqi rules and functional product integration
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-10-30T02:51:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
  - MSH-72
  - MSH-73
  - MSH-76
  - MSH-80
  - MSH-84
  - MSH-103
  - MSH-105
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/m4/implementation-map.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/coverage.md
  - docs/m4/xiangqi/delivery-acceptance.md
ordinal: 45000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-01, P-03, P-04 and A-01 through A-07 require a complete xiangqi implementation, not a piece-movement demonstration. This scope owns the game-specific rules, adapters and conformance evidence across the product. Shared mechanisms and cross-game integration are owned once by MSH-46; coordinate their delivery without duplicating ownership here. Functional mShogi engine support is required in M4; measured strength targets and tuning belong to MSH-54 in M5.

Open design questions (D-07/D-08): Resolve the authoritative rules authority/version and conventions for movement restrictions, repetition, perpetual check/chase and adjudication; distinguish position legality from history-dependent match outcomes. Select the necessary state/action, notation and adapter contracts against the actual consumers, preserving complete actions and history; no speculative universal representation or protocol is prescribed. This To Do scope is not authorization to begin implementation or a change to the current M3 batch.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A sourced, identified and versioned xiangqi rules profile covers the complete game: initial state, every piece/action category, captures and derived effects, promotions and any drops/special choices, turn progression, legality, automatic/claimable outcomes and history-dependent adjudication. Consequential interpretations are recorded; unresolved required rules block conformance rather than disappearing from scope.
- [x] #2 The reusable rules library owns authoritative position, complete actions and atomic rejection, exact apply/undo, rules-relevant history and match/adjudication state. Saved-state, repetition and search identities are specified separately; clients and training tools do not duplicate legality or outcomes.
- [x] #3 Independent sourced xiangqi fixtures cover legal and illegal choices, transitions, special cases, history and outcomes. Exact reversal, sequence and record round trips, applicable perft/divide and differential/reference comparisons have reproducible evidence and provenance; self-consistency or legacy agreement alone is insufficient.
- [x] #4 The independently usable console can select and inspect xiangqi, list and apply complete actions, undo, report history/outcomes and run appropriate diagnostics with honest coverage and separate human/machine output. Game-specific command/notation adapters preserve the authoritative semantics.
- [x] #5 The native mShogi engine can search and play xiangqi using the same authoritative rules/history, with legal complete results, correct terminal/claim handling, request identity, limits, cancellation and stale-result safety. A functional baseline and reproducible self-play/match smoke evidence are recorded without claiming competitive strength.
- [x] #6 Canonical saved xiangqi records preserve profile/version, initial state, necessary history, complete actions and applicable match policies/settings/events. Replay restores semantics exactly, and unsupported profiles or lossy notation/adapters fail explicitly. This ticket owns game-specific encoding/replay coverage; common schema migration belongs to MSH-46.
- [x] #7 The desktop supports actual xiangqi human/engine play and bounded inspection/analysis, complete action entry, readable accessible presentation, outcomes/recovery, save/reopen and history inspection through the authoritative native session. Game-specific end-to-end evidence exists; common infrastructure, cross-game regression and packaged three-platform acceptance are owned by MSH-46 and verified at MSH-47.
- [x] #8 The game coverage ledger and independent review show all required xiangqi scope satisfied, with no required behavior relabeled as optional or deferred to M5. Shared integration findings remain visible in MSH-46 and block M4 closure in MSH-47.
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
Acceptance reconciliation 2026-10-30, primary builder/integrator Codex in the main codex/rewrite checkout at c80a95a9e9c149556aba6f95055e39cab0d5ccf8: verify all eight direct dependencies and current source against the final MSH-103 matrix; reconcile the original AC1-8 and DoD without new product implementation; add current game-delivery acceptance and coverage/handoff pointers preserving historical assessments; obtain proportionate independent record review, run dependency/source/link/JSON/doctor/diff checks, then finalize only MSH-45 and normally integrate/push. Reuse unchanged execution/GUI/CI evidence; preserve MSH-111/112, MSH-54 strength and MSH-46/88/89/47 ownership. Stop before MSH-106 or any other task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH62 planning refinement2026-10-26: original identity,status,AC and DoD remain intact. This complete-delivery record now depends explicitly on its required bounded component/shared work; docs/m4/delivery-plan.md traces each original criterion and single ownership. No game implementation or delivery completion occurred. Earlier M3-context wording remains historical; current authorization is planning-only.

MSH-80 refinement2026-10-27 preserves all original8AC/DoD and previous dependencies. Added required independent complete-game review103; docs/m4/xiangqi/handoff.md maps91–103 to all native/console/own-engine/protocol/record/desktop obligations. Functional delivery is M4; measured strength remains54/M5. No Xiangqi implementation started or profile selected;80 remains the decision/source gate.41/75Done,42–47/M4open and all prior evidence preserved.

Owner selected XIANGQI-P1 mshogi.xiangqi.wxf2018.v1 on2026-10-27: officialbilingualWXF2018, no blanketlanguageprecedence, thirdwarning/fourthcontinuedloss/appropriateWXFdraw, untimedclaimpolicy and twolevel3.2A verification. Original8AC/DoD/allprior dependencies preserved;105 exactverification is directrequireddependency as well as through84/103. Functional M4 completeness cannot be replaced by fastsubset-only results or deferred to54/M5. AXF/CXAfullcorpus acquisition is scopedresearch, not a blanketunaffectedimplementation gate.

Startup 2026-10-30: clean main codex/rewrite at c80a95a, fetched origin aligned 0/0. No other In Progress/In Review task or running subagent owner. Existing unrelated worktrees are preserved. Global Backlog1.55.0 is incompatible; pinned /private/tmp/mshogi-bootstrap-npm-cache/_npx/749414256ca1c44f/node_modules/backlog.md-darwin-arm64/backlog1.55.1 is used for all mutations after reading overview/execution/finalization guides. No temporary checkout is needed for this documentation/acceptance-only work.

Acceptance evidence ready 2026-10-30 at main codex/rewrite c80a95a9e9c149556aba6f95055e39cab0d5ccf8. All direct dependencies39/72/73/76/80/84/103/105 are Done with all AC/DoD checked. No crates/desktop/workflow/manifest production diff from independently corrected1c6b97e. Each original AC1-8 is mapped to final103/component evidence in docs/m4/xiangqi/delivery-acceptance.md; original criteria/dependencies/earlier notes preserved. Existing c80 Rust40965828435 and Desktop40965828489 metadata newly queried: all6 Windows2022/macOS15/Ubuntu24.04 jobs success. Existing 250-check review, 114235frontend/4native checks, normal/QA macOS GUI and correction-source CI reused; no product tests/build/GUI campaigns rerun.86 local references resolve, original/final review evidence JSON parses, historical103 assessment/failedprobe byte-unchanged, doctor/diff checks pass. Fresh read-only reviewer /root/accept45_review inspected original AC/DoD/deps,103/114, new acceptance/handoffs, source diffs and raw CI; approves with no material findings. Its only informational clarification is recorded: MSH-111 is already Done; its prior failures remain historical and its actual successful GUI campaign is Chess/Shogi, not Xiangqi. MSH-112 remains To Do; later CI success is not a fix. Exact/Unknown, no-strength claim, MSH-61 admission and all shared46/88/89/47 gates remain explicit. Completion pending final checks/normal integration; no106 or other task starts.

Final checks: all87 current local documentation references resolve; original eight AC/four DoD texts and complete dependency set unchanged; review JSON parses; doctor and git diff --check pass. Only MSH-45 and explanatory coverage/handoff documents change; original103 assessment/probe, other task statuses and production sources remain unchanged. All acceptance evidence and independent review complete. Normal codex/rewrite commit/push carries this documentation-only closure; do not infer final all-game/M4 delivery. MSH-106 remains Ready/unassigned and is not started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
DONE — complete game-specific Xiangqi delivery accepted2026-10-30. All original AC1-8 and four DoD items are supported: selected complete WXF2018 profile and consequential G-CYCLE/G-NOWIN decisions; authoritative complete actions/history/atomic reversal; independent fixtures/admitted reference; standalone console; own history-aware bounded engine/protocol/recorded matches; lossless canonical/desktop persistence; actual macOS desktop play/analysis/inspection/outcomes/recovery/Take back/Save/Open; completed independent delivery review and coverage ledger. MSH-39/72/73/76/80/84/103/105 are Done. Final103 assessment integrated c80a95a9e9c149556aba6f95055e39cab0d5ccf8 and independently corrected production1c6b97e2bf7ecf560be1902e01b4e95038ee3482 are unchanged; original541fc303 blocked assessment/F01 failing probe retained. Current acceptance/revision/AC matrix: docs/m4/xiangqi/delivery-acceptance.md. Fresh read-only /root/accept45_review approves with no material findings; independent source/evidence/CI review, not new test/GUI execution. Existing250focused checks,114235frontend/4native tests and actual normal/QA macOS evidence reused. Newly checked deps/source/docs/JSON/doctor/diff and existing c80 Rust40965828435/Desktop40965828489 allsix native-platform jobs PASS; no new expensive campaign. Historical111 Linux failures remain preserved;111 task is Done with its own Chess/Shogi campaign, not new Xiangqi Windows/Linux GUI evidence.112 runner race remains To Do despite later CI success. Exact bounded Unknown/unscored semantics, untimed play, measured limits and complete-save admission preserved. Competitive strength stays54/M5; shared six-game integration46, final all-game packagedGUI88, independent89 and M4closure47 remain open. MSH-106 diagnostic review becomes dependency-ready, unassigned/unstarted; no refactor or next task begins. This documentation/task acceptance is normally integrated/pushed on codex/rewrite; stop after45.
<!-- SECTION:FINAL_SUMMARY:END -->
