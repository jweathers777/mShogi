---
id: MSH-67
title: Implement shogi history adjudication and canonical game-match records
status: To Do
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T11:48:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-66
  - MSH-63
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/shogi-profile.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 67000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Profile clause-to-fixture ledger including declaration thresholds/evidence, continuous-check intervals, imported history and exact terminal/length precedence; strict canonical replay and rejection tests.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Rules-owned history separates saved-state,repetition and search identity; repetition includes promoted ownership,both hands and turn, with complete checking intervals and explicit known-history origin.
- [ ] #2 All selected-profile conclusions and precedence are implemented: fourfold/perpetual check,no-legal-action loss,resignation,formal entering-king declaration and declared length policy; advisory probes,illegal API rejection and tournament penalties remain distinct.
- [ ] #3 Canonical native game/match records preserve initial state,complete actions,commands/events,profile/edition,modes,limits,policies,counters,conclusions and score/replay semantics; strict replay validates independently expected history and rejects unsupported/lossy input.
- [ ] #4 Imported-SFEN counter/history semantics and overlapping boundary fixtures are resolved under the ratified profile; completing a history-dependent losing cycle is not erased by generic invalid-request rejection.
- [ ] #5 History/record roundtrips,undo,prefix reconstruction and same-position/different-history cases are independently verified. Timing or external-event policies are either explicitly supported or clearly unsupported,never inferred from chess.
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
Establish and independently review HIST-INTERVAL, DECL-BOUNDARY and IMPORT-LENGTH fixtures from the approved primary clauses, including mutual-check resolution. Define explicit initial/imported live roots and provenance, rejecting nonmoving-king attack without narrowing Position analysis. Implement native untimed event-owned Game with complete check/occurrence history, versioned command admission, selected conclusions/precedence, exact undo and event-prefix context. Add strict bounded versioned game/match record encoding that authoritatively replays every prefix, preserves policies/counters/actors/outcomes/scoring and rejects duplicate/unsupported/forged data. Run focused fixture/rejection/reversal and existing native regressions, obtain fresh independent review, integrate/push and normal CI, complete task evidence, then archive the temporary worktree. Stop before68/69/70; preserve prior evidence and M2/M3/61/40.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-10-26 SHOGI-P1 selected option A mshogi.shogi.csa36.v1 against integration6db8500. Profile/coverage docs now name HIST-INTERVAL (SH-F028–030), DECL-BOUNDARY (034–037), IMPORT-LENGTH (041) as concrete independently reviewed fixture/research gates before affected MSH-67 implementation. No implementation started by this note; existing dependency/acceptance scope preserved.

2026-10-26: Owner authorizes MSH67 only under approved mshogi.shogi.csa36.v1; no profile reapproval needed. Primary integrator Codex, temporary branch codex/shogi-history, app-managed /Users/johnweathers/.codex/worktrees/shogi-history/mShogi based on986a527. Main checkout remains codex/rewrite. User clarification: initial live layer rejects nonmoving-king attack; broad Position analysis preserved; Position::HistoryOrigin::Initial alone is never accepted as an action log. Worktree will be archived after integration/evidence.

MSH-67 implementation in codex/shogi-history now includes explicit standard/imported GameRoot admission, full-context native commands, complete occurrence/check history, selected untimed conclusions for independently established cases, exact game undo/prefixes, and strict versioned GameRecord replay. Independent fixture/research agent established21 history/import,9 declaration and7 command cases with158 board-prefix corroborations; expectations were derived before native adjudication. Focused history and record suites pass, as does native workspace test run and current Clippy -D warnings. Fresh implementation review is in progress. The mutual-continuous-check gate remains under independent inspection of Nanka2024 proof, including drop treatment and omitted bishop geometry; the provisional error branch is explicitly NOT accepted as full legal-action/profile completion. Do not integrate or close67 until that proof/interpretation issue is resolved. No68/69/70 work started.

BLOCKED after bounded independent research/review: no concrete legal mutual continuous-check history with sourced adjudication, and no independently verified impossibility proof. Nanka v3 (2024-03-09, SHA256 cd5f987133d89a32bd6de8391015354b16d066a97cea7f719b67a5089275e065) section5 Table1 leaves overlapping mixed-radius/shared-endpoint discovery-bishop configurations without a certified exhaustive exclusion when intervening direct checks/captures/drops are allowed. See docs/m4/mutual-check-proof-review.md. Do not choose an arbitrary outcome or integrate provisional UnresolvedMutualCheck rejection as completed profile behavior. Status returned To Do for this explicit blocker; all AC/DoD remain unchecked. Unaffected implementation retained on codex/shogi-history. Local cargo fmt/clippy/workspace tests PASS (275 passed,0 failed,2 existing ignored). Three separate AI sessions established fixtures, reviewed game implementation, and freshly reviewed complete implementation; fresh targeted tests33PASS and independent geometry/arithmetic audit30fixturesPASS. No additional material code finding; advisory eligibility doc finding resolved. MSH-68/69/70 and later work not started.
<!-- SECTION:NOTES:END -->
