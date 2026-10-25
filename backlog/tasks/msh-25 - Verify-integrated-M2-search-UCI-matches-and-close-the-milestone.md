---
id: MSH-25
title: Verify integrated M2 search, UCI matches and close the milestone
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-24T20:48:00Z'
updated_date: '2026-10-25T00:13:00Z'
labels: []
milestone: m-2
dependencies:
  - MSH-15
  - MSH-16
  - MSH-17
  - MSH-18
  - MSH-19
  - MSH-20
  - MSH-21
  - MSH-22
  - MSH-23
  - MSH-24
documentation:
  - docs/design.md
ordinal: 25000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M2 component checks must be connected by evidence from the actual game/search/cancellation/protocol/runner path before declaring the baseline integrated. Reuse M1 evidence for unchanged boundaries; this is a focused M2 closure, not another M1 audit.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Actual rules-owned game, history-sensitive search/adjudication, cancellation/stale-result rejection, UCI process and runner integration are exercised together with a recorded independent-engine match.
- [x] #2 Canonical games plus match commands, claim policy and clock/failure events replay deterministically; documented work-limited reruns and timed-run observations meet their separate promises.
- [x] #3 Independent review resolves material integration findings and maps required M2 deliverables to evidence and honest limitations, including the named M4 chess-completeness obligation.
- [x] #4 The integrated revision has appropriate native Linux/macOS/Windows build/test/process evidence; platform omissions remain explicit and block unsupported closure claims.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Map completed required M2 deliverables to actual integration evidence, preserving M1 deep/oracle results. Record fresh independent review of game/history/search/cancellation/UCI/runner and21-game replay corpus, including clock/failure semantics and conditional reruns. Publish a focused closure review and current coverage/entrypoint status, resolve any material gaps, integrate normally and verify native platform CI before markingDone.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
All M2 dependencies completed, including MSH-23 source47243cd nativeCI40145526937 passed Linux/macOS/Windows. Fresh independent m2_closure_review inspected required15..24 records and actual implementations/evidence; no material integration findings. Independently passed93focused checks (search28includingdoctest,runner18,UCI11,corehistory/adjudication/match/completeness36), without repeatingM1deep/perft/oracle work. ActualStockfishexecutionmacOSonly; nativeLinux/Windowsexerciseportableprocesses/offlinereplay. Parent prepares durableclosurematrix and records final sourceCI; no outstandingproductdecisions.

Focused closure matrix published in docs/review/m2-closure.md, with README entrypoint and ledger ownership. Fresh independent m2_closure_review found no material integration gaps, passed93focused checks, then approved documentaccuracy/evidenceboundaries without repeatingtests. One wordingprecision correction applied: claim precedesengine move requestgo, notsetuphandshake. RequiredM2tasks15..24Done, MSH26exactopt-incompleteness alsoDone; defaultsubset/resourceUnknown/no-strength/physical-procedure and platform limits retained. Exactsource47243cd nativeCI40145526937 passedUbuntu24.04job121025908283,macOS15job121025907952,Windows2022job121025908161; parentverifiedheadSha andalljobs. ActualSFexecutionmacOSonly, othernativejobsrealportableprocesses+offline21-game replay. ExistingM1deep/oracle evidence reused. Closurechanges aredocumentation/ledgeronly; local links anddiffcheck passed. Finalnormalintegration/sourceCI followsbeforeDOD4andDone.
<!-- SECTION:NOTES:END -->
