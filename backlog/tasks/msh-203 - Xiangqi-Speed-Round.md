---
id: MSH-203
title: Xiangqi Speed Round
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T17:17:00Z'
updated_date: '2026-11-19T18:38:00Z'
labels:
  - handle:xiangqi-speed-round
  - xiangqi
  - strength
milestone: m-5
dependencies: []
references:
  - docs/m5/strength-roadmap.md
ordinal: 210000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Speed Round

**Problem.** After the Xiangqi program (MSH-194..198), the profile still spends about half of search time in Chariot/Cannon ray walks (targets) and attack tests; every move gets a full legality and check test after make; the search table is fixed at 16 MB while opponents in MSH-197 used 64 MB. A 10 s rerun avoided 42% of mShogi's errors (MSH-198), so depth still pays.

**Objective.** Measure what speed and table size are worth, then make Xiangqi search faster with classical techniques, without changing native authority.

**Approach.** John, 2026-11-19 ("please proceed" on the proposal): first a time-odds test (2x time vs 1x) and a table-size test; then one coherent speed set: rank/file occupancy attack tables for Chariot and Cannon, and a per-node danger set so that only risky moves get the full legality test. Validate with perft, native differential tests and the sequential predecessor test.

**Completion.** Measurements recorded; the speed set kept on evidence or reported as null.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Time-odds and table-size measurements at the program control are recorded with compute.
- [ ] #2 Rank/file attack tables and the danger-set legality filter give identical perft and native differential results, with tests.
- [ ] #3 Speed (nodes per second, fixed-depth time) and late-game depth are measured against the predecessor.
- [ ] #4 The set passes the sequential predecessor test or the null result is reported and reverted; independent review is recorded.
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
Predecessor: main 4d8497fd (MSH-196 engine). Baseline speed: ~0.9-1.0 M nodes/s single thread (middlegame, macOS arm64); late-game fresh depth at 100 ms median 11 / p10 9; table 16 MB.
1. Infrastructure: UCI Hash option for Xiangqi (MB); per-engine clocks (time odds) in the clocked route, replay-checked.
2. Value measurements (local, standing authorization): (a) time odds: mShogi at 2x time (10 s + 0.2 s) vs 1x (5 s + 0.1 s), sequential test H0=0, H1=30, max 300 pairs; (b) table size: 64 MB vs 16 MB at 5 s + 0.1 s, H1=10, max 600 pairs, then 256 MB if (b) is positive. Decision checkpoint: if a doubling is worth little (< about 30 Elo), stop the speed set and report.
3. Speed set: rank/file occupancy tables for Chariot and Cannon targets and attacks; per-node danger set (own General's lines for screens and pins, Horse legs, facing file, General moves, in-check) so only risky moves get the full legality test.
4. Checks: perft and native differential tests unchanged; fixed-depth search identical (pure speed change); speed and depth measurements; sequential test only if search output changes (identical search needs no game test per roadmap) - plus the table default change, which does change play, tested in 2(b).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-19 measurements (local, macOS arm64, 8 games in parallel):
- Time odds 10 s + 0.2 s vs 5 s + 0.1 s (same engine): H1 at 40 pairs, about +209 Elo, pentanomial [0,0,5,27,8].
- Table 64 MB vs 16 MB at 5 s + 0.1 s: no decision at 600 pairs, +11.6 Elo (-1.2 to +24.4); 64 MB adopted on recorded grounds (consistent positive estimate, larger effect expected at longer controls, opponents in MSH-197 used 64 MB).
- Speed set: rank/file occupancy tables for Chariot/Cannon (1.2 M table checks vs ray walk), direct reverse attack checks for jumping pieces and in SEE, danger-set legality and check filter in make, redundant pseudo-legality check removed from make. Throughput about 1.06 -> 1.28 M nodes/s (+20%). Fixed-depth best moves identical; node counts differ slightly in 2 of 4 positions (SEE attacker choice among equal Cannons/Chariots).
- Game test of the set (candidate e491bcdb vs predecessor main) running: 5 s + 0.1 s, H1 = 15.
<!-- SECTION:NOTES:END -->
