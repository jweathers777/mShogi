---
id: MSH-288
title: Shogi Blunders in Time Trouble
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:14:00Z'
updated_date: '2026-11-28T19:00:00Z'
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
  - docs/m5/shogi-blunder-screen.md
  - docs/m5/strength-roadmap.md
priority: medium
ordinal: 266000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In the MSH-193 Shogi blunder screen (304 MSH-191 games at 60 s + 1 s against external engines), 28% of the 226 clear blunders were in time trouble, and a 10 s rerun avoided 28% of all errors. The old 30% single-mechanism fix rule blocked a fix round. Find why the engine reaches time trouble (allocation early in the game, increment handling, overhead) and fix it in the shared clock policy where the cause is not Shogi-specific. Coordinate with the Xiangqi clock task so that only one shared allocation change is made.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 From the MSH-191 game records, measure clock use per move and identify where the time-trouble positions come from
- [ ] #2 A fix for the measured cause wins an SPRT against the predecessor in Shogi at 60 s + 1 s, or the task records why no candidate won
- [ ] #3 Results, cost and GCS path recorded; shared clock documentation updated if the policy changes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Audit the 304 MSH-191 USI game ledgers and join each MSH-193 blunder to its clock charge. Compare a smaller hard cutoff with the frozen predecessor through the native clocked USI route at 60+1. Run the timed blunder suite and use the 0/+15 Elo SPRT to decide whether to keep the shared protocol change. Keep the fixed-time experiment runner on its 30-move share.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Historical MSH-191 protocol ledgers: 304 games, 15,805 mShogi moves, all 226 MSH-193 labels matched. Median charge was 2.622/2.070/1.361/1.000 s for own moves 1–15/16–30/31–45/46 onward. Median final balance 4.155 s. 187 games first reached <2 s at median own move 41; 2,782 moves met that clock threshold. These contained 64 clear blunders (2.30 per 100 vs 1.24 at higher balances, with game phase confounding that comparison). The median time-trouble blunder had 1.098 s left and used 1.000 s. 67.7% of moves exceeded the nominal budget, with 8,232 s gross excess in the first 30 own moves across 304 games. Median position-to-go transport was 0.021 ms; the old logs have no internal timing breakdown. MSH-192 tested a larger move count only with new King weights at 15+0.25 and was inconclusive. MSH-287 tested a fixed-move-time runner, which cannot use the USI hard headroom invoked in MSH-191. Use the same nominal share and increment as the predecessor, but cap the shared protocol hard budget at 1.5 times nominal, which preserves the largest adaptive target and limits overshoot. No VM cost for this analysis.

Handoff: historical ledger analysis complete on codex/msh-288-shogi-clock. Next, run the matched USI comparison and timed blunder suite.

Feasibility before Batch: both sides have 60 s + 1 s, depth 64 and 5M nodes per request, with the native 320-ply ceiling. Both clocks can charge 440 s before grace; even a full 0.5 s grace on each of 320 plies adds at most 160 s, leaving about 82 s of the 682 s game watchdog for protocol setup and cleanup. Eight c3-standard-4 bins of 32 pairs at four games per VM take at most 16 game waves per task, about 11,032 s with staging, plus provisioning and cleanup. Upper bound about $5.3 including the two builds and the comparison image, within $10. Use the same 0/+15 Elo SPRT as MSH-287, alpha/beta 0.05, at least 30 pairs and at most 256, in catalog order. Build the frozen predecessor 097b11b and the one-line candidate separately; use the current native shogi-match and verify replay.
<!-- SECTION:NOTES:END -->
