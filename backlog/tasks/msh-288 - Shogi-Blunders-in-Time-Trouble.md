---
id: MSH-288
title: Shogi Blunders in Time Trouble
status: Done
assignee:
  - '@codex'
created_date: '2026-11-28T10:14:00Z'
updated_date: '2026-11-28T20:30:00Z'
labels:
  - shogi
  - time-management
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies:
  - MSH-287
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/shogi-anchor-method.md
priority: medium
ordinal: 266000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In the MSH-193 Shogi blunder screen (304 MSH-191 games at 60 s + 1 s against external engines), 28% of the 226 clear blunders were in time trouble, and a 10 s rerun avoided 28% of all errors. The old 30% single-mechanism fix rule blocked a fix round. Find why the engine reaches time trouble (allocation early in the game, increment handling, overhead) and fix it in the shared clock policy where the cause is not Shogi-specific. Coordinate with the Xiangqi clock task so that only one shared allocation change is made.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 From the MSH-191 game records, measure clock use per move and identify where the time-trouble positions come from
- [x] #2 A fix for the measured cause wins an SPRT against the predecessor in Shogi at 60 s + 1 s, or the task records why no candidate won
- [x] #3 Results, cost and GCS path recorded; shared clock documentation updated if the policy changes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Measure clock use in the MSH-191 USI ledgers. Test a lower shared protocol cutoff against the predecessor at 60+1 and rerun the MSH-193 blunder suite. Keep the candidate only if it wins the SPRT.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-191 historical ledgers at 60+1 through USI: 304 games and 15,805 mShogi moves. All 226 MSH-193 positions matched. Median charges for own moves 1–15, 16–30, 31–45 and 46 onward were 2.622, 2.070, 1.361 and 1.000 s. The median balance after own move 30 was 11.7 s, and the final balance was 4.155 s. A total of 187 games reached less than 2 s, at median own move 41. The 2,782 low-clock moves contained 64 clear blunders (2.30 per 100, against 1.24 at higher balances). Game phase differs between the groups. For the median low-clock blunder, mShogi had 1.098 s and spent 1.000 s. 67.7% of moves exceeded the nominal share, with 8,232 s gross excess in the first 30 own moves. Position-to-go transport median was 0.021 ms. The old logs do not measure engine setup separately. Historical analysis and charges are at gs://mshogi-experiments-216675633801/runs/msh-288/historical/. No VM cost.

MSH-192 tested a larger move count with new King weights at 15+0.25. MSH-287 measured the fixed-time runner, which gives the engine the nominal share without the USI hard headroom. The new candidate kept the nominal share and cut the shared protocol hard cutoff from 3× to 1.5× nominal. The feasibility check used depth 64, 5M nodes per request and the native 320-ply ceiling. Both clocks can charge 440 s plus at most 160 s of grace, within 682 s including setup. Eight c3-standard-4 tasks of 32 pairs with four concurrent games each plus their suite searches allowed an 11,182 s task limit. Projected total compute was under $5.30.

Build and calibration: predecessor 097b11b and candidate 10ed038 built with Rust 1.99.1 on c3-standard-4. The first Cloud Build attempt named an unavailable prior worker image. The retry used the pinned builder-v2 base. Four 60+1 calibration games replayed, longest 419 s. Build and calibration VM time cost at most $0.063. Calibration reports: gs://mshogi-experiments-216675633801/runs/msh-288/calibration/.

Match at 60+1: 256 openings, two independent colors, eight c3-standard-4 VMs. Use the archived USI operator with `submit --manifest M --config C --image I --job strength-msh288-shogi --out DIR`. All 512 games scored and replayed. No search hit the node cap. Longest game 421 s. The 0/+15 Elo SPRT first reached H0 at pair 226 (LLR −3.021, limits ±2.944). The full sample was 218 wins, 22 draws, 272 losses, about −36.8 Elo (approximate 95% interval −66.2 to −7.9). The candidate had 291 low-clock turns in 20 games, compared with 7,634 in 405 games for the predecessor. Median final balances were 10.330 and 4.410 s. The 90th-percentile charge on own moves 1–15 fell from 4.879 to 3.622 s. The cutoff shortened the longest early searches and saved time for later moves. The strength result rejected that trade-off. Median charge beyond reported search was 1 ms for both engines.

The same campaign reran the 226 MSH-193 positions in fresh processes with their original histories and clocks. Candidate and predecessor avoided the recorded move in 76 and 65 positions, 24 and 22 of 62 held-out positions, and 18 and 19 of 64 time-trouble positions. No fresh judging. Match VM upper bound $1.818. Total estimated compute under $1.94 including both builds and calibration. All Batch jobs, VMs and disks were verified absent. Reports: gs://mshogi-experiments-216675633801/runs/msh-288/shogi/. Reproduction code, summaries and cleanup records are in the operator directory of the same prefix.

The experimental cutoff and its UCI assertions were reverted on codex/msh-288-shogi-clock. Candidate format, Clippy and workspace checks passed (1,176 tests, 10 ignored). The final Backlog-only diff passed whitespace, link and task checks. No follow-up task.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
In 304 historical games, mShogi exceeded its nominal USI share on 68% of turns and reached less than 2 s in 187 games. A tighter cutoff reduced low-clock turns in current 60+1 self-play from 7,634 to 291.

The candidate lost about 37 Elo over 512 replayed games and crossed the SPRT rejection boundary at pair 226. The timed suite was mixed. The existing clock policy stays in use. Estimated compute was under $1.94. All Batch resources were removed. No follow-up task.
<!-- SECTION:FINAL_SUMMARY:END -->
