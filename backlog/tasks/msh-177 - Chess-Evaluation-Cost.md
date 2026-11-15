---
id: MSH-177
title: Chess Evaluation Cost
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-15T22:04:00Z'
updated_date: '2026-11-15T22:10:00Z'
labels:
  - handle:chess-evaluation-cost
milestone: m-5
dependencies:
  - MSH-168
ordinal: 184000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Evaluation Cost.

**Problem:** MSH-168 (Classical Endgame and King Evaluation) reduced measured throughput from about 1.27 to 1.16 million engine nodes/s. Its larger search tree is out of scope.

**Objective:** Reduce per-node Chess evaluation cost without changing any term or score.

**Approach:** Freeze the present evaluator; compare every term and final score on many random legal-game positions for both colors. Require exactly 466994 benchmark nodes. Use repeated baseline/candidate throughput runs with no other heavy work. Stop if the gain is below a few percent. No tuning, search-tree changes or game test.

**Completion:** Integrate an equivalent measured optimization, or record a small/no-gain result and stop.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Every evaluation term and final score match the frozen accepted evaluator on many random legal-game positions for both colors, with independent review.
- [ ] #2 Every measured fixed-depth benchmark run totals exactly 466994 nodes; weights and search behavior are unchanged.
- [ ] #3 Repeated idle-machine baseline/candidate throughput, environment and source identities are recorded; stop if gain is below 3 percent.
- [ ] #4 Relevant local correctness and consumer checks pass; retain only an equivalent optimization or restore the baseline and record the small/no-gain result.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze accepted source28b12c56 and binaryc3363d3d/SHA5e8d0346; baseline remains466994 nodes. Preserve the pre-change weights preparation as test-only reference, with unchanged raw term evaluation. 2. Maintain raw placement totals per color and piece kind incrementally alongside existing totals, preserving per-kind integer rounding. Replace nearest-pawn scans with precomputed Chebyshev-distance masks. No weights, phase, terms or search changes. 3. Compare every white/black term, scale, specialized result and final score with the frozen reference over at least20000 positions from seeded native legal games, plus color-reflected and targeted promotion/castling/en-passant/undo cases. Run native differential/perft and consumer checks. 4. After builds/tests/review stop, measure alternating repeated baseline/candidate benchmarks on the same machine with process-load snapshots and no concurrent heavy work. Use20 runs per binary after warmups, assert466994 nodes every time, report wall and engine NPS and variation. A gain below3percent ends this bounded round without additional optimization attempts. 5. Independent review, source-bound evidence, normal main integration and cleanup. Level1 local verification; no game test by explicit user instruction when equivalence and node counts pass; Linux/Windows/GUI deferred to nextLevel3.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner Codex /root, branch codex/chess-evaluation-cost in existing chat checkout /Users/johnweathers/.codex/worktrees/msh-168-evaluation/mShogi. Main28b12c56 clean at start; other historical worktree left untouched. Backlog1.55.3 matches baseline. This is one measured optimization round;3percent operationalizes the user stop threshold of a few percent.

Implemented one coherent cost reduction: incrementally retain raw placement totals by kind, preserving the old aggregate totals and per-kind rounding; replace nearest-pawn scans with exact precomputed distance rings. Frozen reference retains original piece/pawn scans and rebuilds from native positions. Full library81passed/1ignored; final three equivalence tests passed including20000native positions, undos, reflections and explicit rejected pinned moves.29consumer tests passed. Clippy initially rejected duplicate dead_code allowances; redundant outer allowance removed and final Clippy passed. Independent production review found no correctness defect; added96byte position/cache-update cost remains for benchmark decision. Raw evaluator, all weights and weighted arithmetic byte-identical to baseline; only preparation/cache paths changed.
<!-- SECTION:NOTES:END -->
