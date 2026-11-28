---
id: MSH-285
title: Chu and Tenjiku Always Return a Move
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:01:00Z'
updated_date: '2026-11-28T11:49:00Z'
labels:
  - chu
  - tenjiku
  - search
  - model:medium
  - effort:medium
milestone: m-5
dependencies: []
documentation:
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 262000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Chu and Tenjiku search use restricted_board! in crates/mshogi-search (chu.rs, tenjiku.rs), so TIMED_FALLBACK is false. When the clock or node limit stops the search before the first iteration completes, the engine returns no move. Shogi and Xiangqi use timed_moves! and always return a legal move. This caused 180 of 192 unscored games in MSH-53.3, the native incomplete:Stopped games in MSH-53.2 and the first-iteration losses in MSH-52.3. In the desktop application the same case can leave the engine without a move under time pressure. Prefer the best root move of the partial first iteration; fall back to the first ordered legal move.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A timed or node-limited Chu or Tenjiku search that stops before depth 1 completes returns a legal move (best partial-iteration root move when one is scored, else the first ordered legal move); explicit cancellation still returns no move
- [x] #2 Focused tests cover a stop before the first completed iteration for both games; rules perft and existing search tests pass
- [ ] #3 All 43 blunder-suite observations match the frozen predecessor and native replays verify that the fallback is legal. The corrected Chu comparison at its established fast control shows no strength loss against the frozen predecessor. Record every outcome, cost and GCS path.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Enable time and node limit fallbacks for Chu and both Tenjiku profiles, preferring the best scored root move from the partial first iteration.
2. Check cancellation, legality, completed-depth telemetry, structured engine admission, console and desktop behavior, and rules perft. Update the applicable documentation.
3. Compare all 43 archived blunder positions, verify native fallback replay, and assess the corrected Chu fast-control comparison for a strength loss. Record all stops, expense and GCS evidence. Integrate when the revised criteria are met.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Feasibility: Freeze main at 2ccc003 for both comparisons. Use c3-standard-4 with the prior normal clocks and default 5M-node and depth-64 safety caps. Chu at 10+0.1 can charge at most 240 s per 1,000-ply color pair before overhead, within the prior 600 s limit. Tenjiku at 60+1.5 scored 72/80 pairs in MSH-53.2 with 768 plies per game, a 1,200 s pair limit, two concurrent pairs per VM and a 1,260 s command within the 1,800 s job. Three of 160 games hit the wall, so retain every cutoff as unscored. At $0.201608 per VM-hour, a full eight-VM block reserves $0.81, earlier Tenjiku blocks cost $0.32–$0.61, and two Linux builds reserve $0.41. The builds, blunder screen and the first block for each game/profile fit under $4. Continue within the $10 task cap.

Handoff: Candidate b0acbf0 on codex/msh-285-always-return-move. Search and structured engine fallback implemented with specific tests. Next: finish workspace checks, run Linux blunder and predecessor checks, and integrate after acceptance.

Search and engine suites passed before the final consumer fixture update, including both Tenjiku profiles, Chu, perft and differential tests. Updated console integration tests and the three desktop snapshots that cover first-iteration node exhaustion. Full workspace and frontend checks are running. The Linux builds compare b0acbf0 with 2ccc003. Subsequent edits so far are test or verification-fixture only.

Local checks passed: workspace format, Clippy and 1,175 Rust tests (10 ignored), including existing search, independent Chu/Tenjiku perft, differential checks and real-console admission. Desktop type checking and all 578 frontend tests passed with Node 24.19.0. The native runner applied and replayed both one-node fallbacks for Chu, Adams and Muller in three local calibration pairs. A frozen-main regeneration proved the four completed Tenjiku snapshots already differed from their generator. Filed MSH-289 and kept these four snapshots outside the fix.

Handoff: Product candidate b0acbf0 with consumer test checkpoints on codex/msh-285-always-return-move. Linux candidate and frozen 2ccc003 built on c3-standard-4. Both build cleanups verified. Next: compare blunders and clocked SPRT, then integrate only after AC3 resolves. Cloud evidence: gs://mshogi-experiments-216675633801/runs/msh-285/.

b0acbf0 vs 2ccc003, c3-standard-4: tools.strength.cli blunders at depth 3, Chu 300K nodes and Adams/Muller 500K nodes, using MSH-52.3 and MSH-53.3 suites. All 43 positions match in choices, scores and nodes. Both Linux builds and the screen cost at most $0.0725 combined VM compute, with no jobs, VMs or disks left. GCS gs://mshogi-experiments-216675633801/runs/msh-285/blunders/.

Apply the roadmap search SPRT (0/15 Elo, alpha=beta=.05, minimum 30 scored pairs independently for Chu, Adams and Muller) before the first timed match. Use the established fast clocks. Next: Chu, then the two Tenjiku profiles on branch codex/msh-285-always-return-move.

Chu b0acbf0 vs 2ccc003, c3-standard-4, tools.strength.cli match at 10+0.1: the inherited 1,000-ply setting capped five games in 116 replayed pairs. One predecessor stopped before depth one, and the candidate applied two depth-zero choices. Preserve the cap-affected block as a setting failure. VM upper bound $0.4955; no Batch resources remain. gs://mshogi-experiments-216675633801/runs/msh-285/chu-1/.

Handoff: Adams and Muller are running at 60+1.5 in strength-msh285-tenjiku-1. Next: after verified cleanup, repeat Chu with 2,000 plies per game and a 750 s pair wall. The first five capped pairs completed their recorded work in 202–250 s. For 2,000 plies the pair clock bound is 440 s, plus 193 s extrapolated at the highest observed non-request overhead (49 ms per ply). The 750 s wall fits the existing 1,500 s command and 1,800 s job. Keep the engines, control, openings, concurrency and SPRT hypotheses. Task branch codex/msh-285-always-return-move.

Tenjiku b0acbf0 vs 2ccc003, c3-standard-4, tools.strength.cli match at 60+1.5: all 16 pairs replayed. Four pairs hit the shared wall (five games, one swapped game with zero turns). One other game stopped before depth one on the predecessor. Adams had five scored pairs and Muller six. No SPRT decision. VM upper bound $0.6014 and cleanup verified; gs://mshogi-experiments-216675633801/runs/msh-285/tenjiku-1/.

Tenjiku blocker: gcp.py accepts longer VM duration, but tools/strength/batch.py fixes the match manifest to 1,800 s and anchor_cloud.py begins cleanup 300 s earlier. Increasing max_run_seconds does not extend the 1,500 s controller window. Muller seed 6 consumed 1,196.348 s of request time through 737 plies in its first game and the swapped game did not start. Two full 768-ply color games can charge 2,544 s before overhead. MSH-286 tracks independent color games.

Handoff: Corrected Chu is running in strength-msh285-chu-1-2000 at 10+0.1, 2,000 plies, and 750 s per pair. Next: verify terminal result and cleanup, then record the remaining strength check on local branch codex/msh-285-always-return-move.

John revised AC3 on 2026-11-28. Accept the 43 identical blunder observations, native legal fallback replays, and corrected Chu at its normal fast control if the scored comparison shows no loss. An SPRT win is unnecessary because the fallback acts too rarely to demonstrate an Elo gain. The remaining shared-wall and 1,800 s Batch duration work belongs to MSH-286. Use this decision in place of the old strength gate.
<!-- SECTION:NOTES:END -->
