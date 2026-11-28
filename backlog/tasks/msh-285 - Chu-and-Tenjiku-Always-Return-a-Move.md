---
id: MSH-285
title: Chu and Tenjiku Always Return a Move
status: Done
assignee:
  - '@codex'
created_date: '2026-11-28T10:01:00Z'
updated_date: '2026-11-28T11:56:00Z'
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
- [x] #3 All 43 blunder-suite observations match the frozen predecessor and native replays verify that the fallback is legal. The corrected Chu comparison at its established fast control shows no strength loss against the frozen predecessor. Record every outcome, cost and GCS path.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Enable time and node limit fallbacks for Chu and both Tenjiku profiles, preferring the best scored root move from the partial first iteration.
2. Check cancellation, legality, completed-depth telemetry, structured engine admission, console and desktop behavior, and rules perft. Update the applicable documentation.
3. Compare all 43 archived blunder positions, verify native fallback replay, and assess the corrected Chu fast-control comparison for a strength loss. Record all stops, expense and GCS evidence. Integrate when the revised criteria are met.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John revised AC3 on 2026-11-28. Accept identical results on the 43 blunder positions, native replays that verify legal fallbacks, and a corrected Chu comparison showing no loss. An SPRT win is unnecessary because the fallback acts too rarely to measure an Elo gain. MSH-286 owns the shared wall and fixed Batch duration.

Local checks: workspace format, Clippy and 1,175 Rust tests (10 ignored), including independent Chu and Tenjiku perft and differential tests. Desktop type checking and all 578 frontend tests passed on Node 24.19.0. Three native calibration pairs applied and replayed the one-node fallback in both colors for Chu, Adams and Muller. A frozen-main fixture generation found unrelated drift in four completed Tenjiku snapshots, now tracked in MSH-289.

Cloud runs compared candidate b0acbf0 with frozen 2ccc003 on c3-standard-4. The two Linux builds cost at most $0.0307 in VM compute. The root for all artifacts is gs://mshogi-experiments-216675633801/runs/msh-285/.

Blunders: tools.strength.cli blunders, depth 3, Chu 300,000 nodes and both Tenjiku profiles 500,000 nodes. All 43 choices, scores and node counts matched. VM upper bound $0.0419. Artifacts: blunders/.

Initial Chu: tools.strength.cli match, 10+0.1, 1,000 plies, 600 s pair wall. All 116 pairs replayed. Five games reached the inherited ply cap and one predecessor stopped before depth one. The candidate applied two depth-zero moves. Replaying the stopped 281-move prefix at its 109 ms budget also returned a legal depth-zero choice. The cap-affected block informed the corrected settings. VM upper bound $0.4955. Artifacts: chu-1/.

Tenjiku: tools.strength.cli match, 60+1.5, 768 plies, 1,200 s pair wall. All 16 pairs replayed. Five games in four pairs hit the shared wall. One predecessor stopped before depth one. Adams had five scored pairs and Muller six. Muller seed 6 spent 1,196.348 s in requests through 737 plies and the swapped game got zero turns. The stock strength route fixes a 1,800 s manifest and starts cleanup 300 s early, even when gcp.py accepts a larger VM duration. Two full color games can charge 2,544 s before overhead. MSH-286 tracks the fix. VM upper bound $0.6014. Artifacts: tenjiku-1/.

Corrected Chu: tools.strength.cli match, 10+0.1, 2,000 plies, 750 s pair wall. All 124 launched pairs accounted for. The 123 native reports replayed and matched their engine hashes and settings. Four individual games reached the ply cap. Pair 161 produced no report because its runner exited on SIGKILL, so both outcomes are unscored. The cause was undetermined. There were no native incomplete searches in the 123 reports, and the candidate played two legal depth-zero moves. The 242 concluded games were 128W/0D/114L. Across 119 fully scored pairs, the candidate scored 126W/0D/112L, paired Elo +20.5 with approximate 95% interval -26.8 to +68.5. The estimate favors the candidate. The original 0/15 SPRT gave no decision. VM upper bound $0.4651. Artifacts: chu-1-2000plies/.

Total VM compute upper bound $1.6343. All six Batch jobs, VMs and disks were removed. One Cloud Build produced the worker image in 34.4 s. The operator archive under operator/ preserves the commands and build inputs.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Chu and both Tenjiku profiles return a legal move when time or nodes run out before depth one. Search prefers its best scored partial root move and uses the first ordered legal action if no move was scored. Explicit cancellation suppresses the fallback. Local and native checks passed, all 43 blunder observations matched, and the corrected Chu result favored the candidate. MSH-286 tracks match timing and independent color games. MSH-289 tracks older completed Tenjiku snapshots.
<!-- SECTION:FINAL_SUMMARY:END -->
