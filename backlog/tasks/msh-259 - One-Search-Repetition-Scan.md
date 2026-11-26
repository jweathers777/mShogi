---
id: MSH-259
title: One Search Repetition Scan
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T20:37:00Z'
labels:
  - search
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7450
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each search position scans its key history in rule_score with its own loop: Shogi looks back over the whole game history at each node (shogi/position.rs), Tori only back to the last capture or drop (tori/position.rs; a recorded approximation that can miss repetitions), Xiangqi and Chu back to the last irreversible move, and Tenjiku checks every earlier key, both sides to move included, with contains (tenjiku/position.rs). The scoring is a game rule; the scan is not. No measurement is recorded for the different bounds of the two drop games.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 One shared repetition-history helper in the search core gives each game its earlier occurrences; per-game code keeps only its scoring.
- [ ] #2 Shogi and Tori use one scan bound rule, chosen by measurement; a change in search behavior passes the blunder suite and an SPRT match against the frozen predecessor for each affected game, and each unchanged game shows identical fixed-depth results.
- [x] #3 Perft and differential tests pass.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Measure both drop-game bounds on native repetition fixtures and timed key histories. Preserve null-move boundaries. 2. Share occurrence traversal and retain the scoring and priority of each game. 3. Compare fixed-depth decisions, PV, scores and nodes against 7c15e68. For the game whose bound changes, compare the published blunder suite and test non-inferiority with paired openings at 5 s + 0.1 s, H0 -15 Elo, H1 0, alpha and beta 0.05, at most 1200 pairs. Keep the simpler search only if the strength and blunder results support it. 4. Run perft, differential and repository checks, obtain review, integrate and clean up. All cloud work within US$10.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-259-repetition. Frozen predecessor 7c15e68. Compare whole-game and irreversible-move bounds before selecting the shared drop-game scan. Next: measure bounds, consolidate, and verify all six games.

Branch codex/msh-259-repetition. At Tori fixture TORI-H-CAPTURE-DROP plies 6 and 11, full history found both hand-derived prior positions; the capture/drop cutoff missed both. M1 Pro release microbenchmark, 1M lookups per case (15/16 absent): full 8/32/128/320-ply histories 9/23/70/171 ns per lookup; four-ply bound 3–5 ns. Selected full known history since last null for Shogi and Tori. Shared scan in all six positions; native rules untouched. Bounded Batch frozen build in progress. Next: review, paired blunder and SPRT evidence, fixed-depth results, perft/differentials/checks.

Independent review found the old Tori bound and score in docs/m5/shared-search-core.md. Corrected the guide. Review found no defect in scan bounds, null undo, root thresholds or score priority. Focused capture/drop regression passed on the M1 Pro.

Matched both bounds on every prefix from 315 Shogi and 218 Tori published openings. M1 Pro single core, optimized scan (bounded/full): Shogi 4,498 queries, 14,591/16,095 keys, 0/0 hits, 3.09/3.25 ns. Tori 1,744 queries, 934/3,488 keys, 2/2 hits, 0.92/1.83 ns. Frozen 7c15e68 vs candidate 75a10d2, c3-standard-4: 134 identical fixed-depth signatures (move, score, PV, nodes): Chess 36, Shogi 18, Xiangqi 28, Chu 28, Tenjiku 24 including both profiles. Full 280-position Tori blunder suite at depth 5: identical 280 choices, 163 avoid the played error, 65 match the judge. Held out 43/64 avoided and 17/64 matched. First match submission used the wrong safety caps (512 nodes, baseline depth 1); stopped at the first incomplete:NodeLimit and discarded all scores. MSH259-match2 uses the harness clock caps symmetrically (5M nodes, depth 64), same planned clock and hypotheses; no invalid data pooled. The first cloud checks archive carried macOS extended metadata; clean archive resubmitted as msh259-checks2. Next: monitor checks and match, collect cleanup/cost, integrate if gates pass. Evidence gs://mshogi-experiments-216675633801/runs/msh-259/.

The exact shared pre-integration command passed on 268ae68 in msh259-checks3, c3-standard-4: workspace rustfmt, Clippy with warnings denied, 1,166 tests passed, zero failed, 10 ignored by default. Regular perft and randomized differential tests cover all six search positions. Explicit release deep Chess perft passed and native depth-4 totals matched 197281, 4085603, 422333. The preceding checks2 worker reached the end of workspace tests but could not replace the first log because it has create-only bucket access; checks3 wrote a fresh prefix and all three commands passed. Match2 is active: only its ordered SPRT and cleanup remain. Baseline/candidate Tori blunder-suite node totals 4,271,347/4,275,371; 11 positions changed node count, all 280 choices identical.

Full linear scan, 75a10d2 vs 7c15e68: msh259-match2 closed at the preset 1200-pair cap, 1197 scored, 3 step-limited (native reports replayed). W/L/D 1154/1240/0, pentanomial [269,0,702,0,226], Elo -12.49 [-25.16,+0.15], LLR -1.799 (bounds +/-2.944), no decision. This candidate did not pass AC2. A root membership mask in a timing diagnostic on the two actual games from seed 1 found all 120,000 occurrences in 3.56M queries and reduced lookup from 29.03 to 4.93 ns. Revise the implementation to skip pre-root positions only when no full key can match. Independently test the new candidate on predeclared disjoint seeds 1201..3600, 5s +0.1s, H0 -15/H1 0, alpha=beta .05, minimum 30 scored, maximum 2400 attempts; no pooling with the first comparison. Repeat full blunders, unchanged-game fixed depth, perft, shared checks and review. All compute stays under the task cap.

Revised candidate 93de32f uses one 1024-bit pre-root mask shared by the two drop games. A clear bit can exclude the immutable prefix only; a matching bit always checks full 64-bit keys. Focused collision, null-undo and capture/drop tests passed. Independent review of the revised candidate found no defects. Original SPRT was closed at its cap and job deleted. Current jobs msh259-filter-build and msh259-filter-check; next run msh259-filter-probe and msh259-filter-match with the predeclared comparison. No action on main yet.
<!-- SECTION:NOTES:END -->
