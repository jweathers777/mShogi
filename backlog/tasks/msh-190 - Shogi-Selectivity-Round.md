---
id: MSH-190
title: Shogi Selectivity Round
status: Ready
assignee: []
created_date: '2026-11-17T16:42:00Z'
updated_date: '2026-11-17T16:52:00Z'
labels:
  - handle:shogi-selectivity-round
milestone: m-5
dependencies:
  - MSH-189
documentation:
  - docs/m5/shogi-strength-round.md
  - docs/m5/evidence/msh187/position-ablations/README.md
  - docs/m5/shared-search-core.md
  - docs/m5/strength-roadmap.md
  - docs/m5/strength-harness.md
priority: high
ordinal: 197000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Selectivity Round

## Problem
MSH-187 (Shogi Strength Round) planned Shogi search selectivity but retained only speed, history and time changes. Its first candidate combined a full exchange evaluation (SEE) with quiet checking moves at the first quiescence ply. On one bishop-exchange root that candidate did not finish depth 1, even without transport or history overhead; without the check stage it finished depth 1, and with the old exchange estimate depth 4. Fixed-depth nodes rose 17.4% (docs/m5/evidence/msh187/position-ablations/README.md). This shows a faulty, unbounded implementation, not that checks in quiescence are wrong for Shogi: quiet checking drops are central to Shogi tactics. Shogi-scaled pruning margins and drop/promotion ordering were never tried. The core margins (futility 150/300/450, aspiration 30, quiescence 200, razoring) still assume Chess piece values.

The test conditions were also weak for changes smaller than MSH-187's: six openings played five times each, a 5 s + 0.5 s control with late-game depth 3-6, and a tactical suite of 12 positions (6 geometries) that both versions solve completely.

## Objective
Stronger Shogi search through standard selectivity, measured under conditions that can detect gains of about 15-30 Elo.

## Approach
1. Measurement additions (first; reused by MSH-188 (Shogi Evaluation Round)):
   - Opening set of at least 100 distinct balanced openings from recorded licensed sources (professional game prefixes or established opening lines), with no repeats inside one game test.
   - A tactical suite of at least 200 positions with independently established answers (for example tsume problems with known solutions and checked game positions), of which the task-start engine fails a meaningful share at 1 s. Keep a held-out part.
   - Choose a game-test control at which the median late-game completed depth is at least 8 (measure it; about 20 s + 0.2 s or a per-move budget near 0.5-1 s is a starting point). Run long tests in the cloud.
2. One coherent selectivity set, then test it as a set:
   - Quiescence check stage done in the standard bounded way: only at the first quiescence ply, checking moves and drops filtered by SEE (no losing checks), a hard cap per node, and no recursion of the check stage.
   - Full SEE with board and hand values, used for capture ordering and for pruning losing captures in quiescence.
   - Margins (futility, razoring, aspiration, quiescence delta) scaled to Shogi piece values.
   - Move ordering for drops and promotions (for example drops near the enemy king, checking drops, promotion first).
3. Before tests, record baseline, intended benefit, comparison method, resource cap and checkpoint (AGENTS.md). Freeze the predecessor at task start.
3a. Dedicated mate search (John, 2026-11-17). Strong Shogi engines use a separate search that only looks for forced mates by continuous checks (tsume), usually proof-number search (df-pn), because long mating attacks with drops decide many games and alpha-beta finds them late. Build it in the shared core as a game-independent mate search over checking moves and evasions that the game supplies, so Tori, Chu and Tenjiku can use it later. Use it at least at the root with a bounded node or time share, and decide from measurement whether short mate checks inside the main search help. It must respect the Pawn-drop-mate rule and perpetual-check repetition (a repeated checking cycle is not a mate), and handle repeated positions correctly. A found mate is private analysis; the native rules still admit the move and decide the result. Test on tsume problems with known solutions (the tactical suite above) and measure solve rate and time; then include it in the set game test or test it as its own set.
4. If the set does not reach H1, ablate parts and retest, with a game-test result for each removed part. A failure to finish an iteration is a defect to fix (bound the stage), not evidence against the technique.

## Completion
Every item in steps 2 and 3a has a recorded outcome: retained with evidence, or rejected with a game-test or measured tactical result. A timing failure alone does not count as a rejection. Verification Level 1. Independent review.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 An opening set of at least 100 distinct licensed openings and a tactical suite of at least 200 positions (with a held-out part, and a meaningful failure share for the task-start engine at 1 s) exist with recorded sources, and the game-test control reaches median late-game depth of at least 8.
- [ ] #2 Each of the four selectivity items (bounded quiescence check stage, full SEE, Shogi-scaled margins, drop and promotion ordering) has a recorded outcome: retained with evidence, or rejected with a game-test or tactical-suite result; no item is rejected only for a timing failure.
- [ ] #3 A shared-core mate search (for example df-pn) over game-supplied checks and evasions exists, respects the Pawn-drop-mate rule and perpetual-check repetition, and its tsume solve rate and time are measured against problems with known solutions; its use in play has a recorded outcome with game-test evidence.
- [ ] #4 Retained changes pass Shogi perft and differential checks against the native rules, every timed request still returns a legal move, and other games' search tests pass.
- [ ] #5 Retained changes improve the tactical suite and win a sequential game test against the frozen task-start predecessor under the new opening set and control.
- [ ] #6 Results, compute, failed attempts and independent review are recorded, and docs/m5/shared-search-core.md is current.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Shared-core rule (John, 2026-11-17): build each search technique in the shared search core (crates/mshogi-search/src/alphabeta), with Shogi supplying only game data through the position trait. The core margins are now fixed constants for Chess values (for example FUTILITY [0,150,300,450] in alphabeta/mod.rs); make them scale from a value the game supplies (for example its smallest piece value), so every game gets correct margins. The quiescence check stage and SEE-based pruning are core features that a game turns on and feeds with its own move and exchange data. Do not create a Shogi-only search path. Other games keep their present behavior unless a change is tested for them.
<!-- SECTION:NOTES:END -->
