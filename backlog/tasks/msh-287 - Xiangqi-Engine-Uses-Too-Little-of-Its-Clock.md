---
id: MSH-287
title: Xiangqi Engine Uses Too Little of Its Clock
status: Done
assignee:
  - '@codex'
created_date: '2026-11-28T10:14:00Z'
updated_date: '2026-11-28T17:37:00Z'
labels:
  - xiangqi
  - time-management
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies:
  - MSH-286
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/strength-harness.md
priority: medium
ordinal: 265000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In the MSH-198 blunder screen (MSH-197 ladder, 60 s + 1 s), mShogi used a median of 1.5 s per move, and a 10 s rerun avoided 42% of all errors and 13 of 26 clear blunders. The old 30% single-mechanism fix rule blocked a fix round. Time allocation is shared across games (MSH-245 One Clock Policy for All Games), so a better allocation should be one shared change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Measure how the engine spends a 60 s + 1 s clock in Xiangqi (time per move by phase, unused clock at game end)
- [x] #2 A changed shared allocation (for example a larger share, or extra time when the best move or score is unstable) wins an SPRT against the predecessor in Xiangqi at 60 s + 1 s, or the task records why no candidate won
- [ ] #3 A kept change does not lose in a short SPRT in the other games that use the shared policy; results, cost and GCS path recorded
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Measure phase use and game-end balance from the runner at 60+1. Compare the 12-move share with the 30-move predecessor on the same clocks and run the timed blunder suite. Check the other games only if Xiangqi accepts the change. Remove the experimental selector before integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-28: MSH-287 may change the shared 30-move clock allocation, including the experiment runner policy he decided on 2026-11-25. Update “Clock allocation of the experiment runner” in docs/m5/strength-roadmap.md if the policy changes.

Feasibility before Batch: at 60+1 and 500 plies, the two clocks can charge 620 s per game and the default wall allows 682 s. The default 5M nodes and depth 64 bound each turn. Eight c3-standard-4 bins of 32 pairs at four games per VM project 14,843 s per task and about $6.84 in VM time, below six hours and the $10 task cap. The candidate starts at 5.445 s search time and the predecessor at 2.448 s, after the transport reserve. The frozen 0/+15 Elo test uses alpha/beta 0.05, at least 30 scored pairs and at most 256.

Match: predecessor 3162530, comparison runner ecd7593, identical Linux engine on both sides. Each game uses 60+1. Only the candidate uses the 12-move share. On c3-standard-4, all 512 games concluded and replayed. 133 wins, 282 draws and 97 losses gave +24.5 Elo (approximate 95% interval +4.6 to +44.5). Seed-ordered LLR reached +2.492 at the limit and never crossed either ±2.944 boundary. The candidate did not qualify. The 112 catalog openings cycle across seeds. No search hit the 5M-node limit. Reproduce with python3 -m tools.strength.cli match --game xiangqi --candidate ./mshogi --baseline ./mshogi --runner ./mshogi-match --helper ./strength --openings xiangqi-openings-v1.json --base 60 --increment 1 --first-seed SEED --max-pairs 32 --min-pairs 33 --out DIR, with SEED=1,33,…,225, then analyze the combined seed order. Match VM upper bound $2.603, build $0.017. Report and frozen manifest path gs://mshogi-experiments-216675633801/runs/msh-287/xiangqi/. Analysis and reproduction scripts are under operator/ in the same task prefix. All jobs, VMs and disks were verified absent.

Clock receipts from the current fixed-time runner, separate from the MSH-197 protocol ladder. In positions with 10–12, 5–9 and 0–4 Chariots, Horses or Cannons combined, median 30-move times were 2.312, 1.686 and 1.161 s (44,855 moves total). The 12-move times were 3.543, 1.449 and 1.012 s (44,884 moves total). Each charge includes request overhead. The median unused clock at game end fell from 15.596 s to 5.388 s (10th–90th percentiles 11.005–27.100 and 4.216–9.864 s). The median game was 171.5 post-opening plies. Historical ladder per-move reports were unavailable, so the timing measurements use the current runner.

Blunder suite: ecd7593 release against the same engine at both shares, one local worker, all 187 MSH-198 positions, recorded balances and a fresh process. The 12-move share avoided 88 known moves vs 79 at 30 (clear blunders 10/26 vs 11/26, held-out all errors 24/55 vs 23/55). It matched the stored judge move 51 times vs 52. The rerun measured move changes without fresh judging. Zero VM cost. Script and rows are under blunder-suite/ in the task GCS prefix.

Checks passed for the temporary comparison: cargo fmt, Clippy, Rust workspace tests, 57 Python strength tests and a two-color calibration at 60+1. Only the task record is integrated. Other-game SPRTs are conditional on keeping a change and do not apply. No new follow-up task. MSH-288 (Shogi Blunders in Time Trouble) is next.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Measured Xiangqi clock use at 60+1 in 512 replayed games. The 30-move share left 15.6 s unused at the median. The 12-move share left 5.4 s and put more time into the opening, with less time in the middle and end.

The candidate scored 133 wins, 282 draws and 97 losses, about +24 Elo. The SPRT reached 256 pairs without a decision. The timed blunder suite showed mixed results. The shared 30-move policy remains in use. Match and build VM cost totaled at most $2.62. All Batch resources were removed. The next task is MSH-288 (Shogi Blunders in Time Trouble).
<!-- SECTION:FINAL_SUMMARY:END -->
