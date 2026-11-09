---
id: MSH-148
title: Deferred Frontier Work
status: Done
assignee:
  - '@codex'
created_date: '2026-11-09T20:36:00Z'
updated_date: '2026-11-09T20:50:00Z'
labels:
  - handle:deferred-frontier-work
milestone: m-5
dependencies:
  - MSH-147
documentation:
  - docs/m5/chess-effectiveness-diagnosis.md
priority: high
ordinal: 153000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Deferred Frontier Work. MSH147 identifies millions of eager quiescent quiet-candidate projections before captures that usually cut off immediately. Remove demonstrably unused work while preserving native authority, history, claims, existing frontier admission and move ordering. This is a general execution-efficiency increment, not new pruning or a strength claim. Do not adopt the broad endgame-LMR experiment that lost an independently winning rook move.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Defer unnecessary quiet frontier work using the existing search/native boundary, preserving admitted move order, completed results at controlled fixed work, native effects/history/claims and finite cancellation/error restoration.
- [x] #2 Deterministic cutoff and reached-quiet regressions plus existing varied sacrifice/check-evasion/threat/defensive-resource coverage pass; bounded representative measurements establish actual avoided work and practical cost without claiming playing strength from speed.
- [x] #3 Level1 local verification and independent review support normal integration; evidence retains baseline/corrected sources and limitations, with later coherent evaluation work and adaptive reassessment recorded.
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
Root integrates on codex/rewrite after approved MSH147 atf8ffd96. Level1 portable search correction; rules/protocol/platform boundaries unchanged. Defer only settled quiet candidate projection until its already-ordered move is reached; retain entry-time ordering scores, all unsettled evasions/defences, frontier budget, native admission and cancellation/error unwinding. Delegate this isolated function/test edit, independently review it. Compare frozen0a6e8cf diagnostic binary against corrected source at identical bounded work over12positions plus independent defensive/error tests; measure actual saved projections and sequential repeated cost, then short realistic-time choices. No opponent matches/fullmatrix. Integrate after local search regressions, consumer compilation and independent source/evidence review.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Source0a94194 passes98search tests and independent source review.36paired fixed-work runs preserve exact result/PV/visits/all legacy counters except intentionally reached quiet-projection/threat counters. Quiet work decreases; aggregate median time4.439s→4.423s (0.36percent), within noise, so no robust speed gain claimed. All16timed pairs retain choices, six supplementary WDL choices preserved. Net cost may be offset by sorting more quiet candidates; avoid a larger picker redesign without evidence. Native any-legal query still eagerly allocates all candidates; profile-supported next opportunity is a bounded short-circuit native visitor before evaluation work. Canonical evidence/msh148/verification.json.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Integrated deferred quiet frontier projection at0a94194 on codex/rewrite, preserving native admission and exact ordered search behavior.98local search tests, formatting/clippy and independent source/final evidence review passed.36fixed-work pairs retain identical decisions/PV/work while avoiding1,572,132projections;16timed pairs retain choices and6independent WDLcases pass. Aggregate timing difference0.36percent is within noise: no robust speed/strength gain claimed. Canonical docs/m5/evidence/msh148/verification.json and chess-frontier-work.md retain sources, corrected fixture and limitations. Level1 acceptance; no fullmatrix or opponentmatches. Next MSH150 addresses measured eager native candidate construction, then145/149evaluation work before reassessment.
<!-- SECTION:FINAL_SUMMARY:END -->
