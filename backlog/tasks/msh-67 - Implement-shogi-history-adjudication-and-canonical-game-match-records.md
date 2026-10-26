---
id: MSH-67
title: Implement shogi history adjudication and canonical game-match records
status: To Do
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T11:49:00Z'
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

MSH-67 attempted 2026-10-26 by @codex in temporary codex/shogi-history worktree, based986a527. BLOCKED and returned To Do, all AC/DoD unchecked. Unmerged partial implementation preserved and pushed at f259bb23834d6156f21aeec53db9ee00ef7242e4 (codex/shogi-history); do not merge as complete. It contains GameRoot/live admission, exact native history/context/undo, selected untimed outcomes for established cases, strict GameRecord replay,15 new focused tests and independent review reports. Local macOS arm64 Rust1.89 cargo fmt/clippy PASS; workspace275PASS,0failed,2existingignored. Separate AI sessions established independent21history/import+9declaration+7command fixtures,158 board-prefix corroborations; fresh code review33 targeted Rust tests and30case independent geometry/arithmetic audit PASS. No additional material code findings; minor declaration-eligibility documentation finding resolved. Exact blocker: no legal mutual-continuous-check history with sourced outcome and no independently verified impossibility argument. Nanka v3 section5/Table1 needs exhaustive exclusion of mixed-radius/shared-endpoint discovery-bishop configurations including intervening direct checks/captures/drops. See docs/m4/mutual-check-proof-review.md (source URL/version/SHA256 and concrete candidate). Provisional UnresolvedMutualCheck rejection would violate the complete live-action contract; no arbitrary outcome/policy adopted. Fixture expectations and research are integrated as handoff only; native implementation is NOT integrated. Required next action: complete that exact proof/authoritative-rules obligation, then re-review/final verification/integration. MSH-68/69/70 and later work not started. M2/M3,63-66,61 persistence and40 roadmap correction unchanged. CI for WIP source: https://github.com/jweathers777/mShogi/actions/runs/40317821544; initially queued, not represented as passed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Unfinished: MSH-67 is blocked on the mutual-continuous-check proof/interpretation gate. Independently derived fixture research is retained; tested native history/record work remains unmerged on codex/shogi-history at f259bb23834d6156f21aeec53db9ee00ef7242e4. No outcome convention or acceptance criterion was weakened. See docs/m4/mutual-check-proof-review.md and implementation notes for exact remaining lemma, review, test evidence and resumption instructions.
<!-- SECTION:FINAL_SUMMARY:END -->
