---
id: MSH-214
title: Tenjiku Strength Harness
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-20T22:51:00Z'
updated_date: '2026-11-27T13:27:00Z'
labels:
  - tenjiku
  - search
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-235
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: the harness step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Tenjiku, modelled on MSH-204 (Tori Strength Harness). It gives the later speed, selectivity and evaluation tasks a way to show that a change makes play better. Use the existing clocked native experiment route (mshogi-match experiment-run, structured internal protocol); add no text engine protocol. No external opponents (John, 2026-11-19). Fire Demon burning, range-jumping generals and the Lion moves must be covered by the tactical suite. The board is large and search is slow, so the depth target may need a recorded adjustment.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 At least 100 balanced openings, with their source and the balance rule fixed before screening
- [ ] #2 A tactical suite of at least 200 positions with independently established answers (for example from a native mate prover) and a held-out part, with the baseline score
- [ ] #3 A game-test control at which the median late-game depth is at least 8, or a lower target with its reason and cost recorded; a game test against a frozen predecessor runs end to end in the cloud
- [ ] #4 A self-play export for Texel data with a split, and a time-odds measurement (2x time) recorded
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Freeze 2e4b9ad as baseline and unchanged candidate. For each profile separately, standard start plus four seeded uniformly indexed legal actions from the shared openings generator. Four shards start at seeds 214000, 215000, 216000 and 217000. In each take the first 25 distinct live boards with a completed depth-2 search, at most 500000 nodes and absolute score at most 100 pawn hundredths. Up to 1000 attempts per shard. Deduplicate across shards and replace duplicates with the next eligible seed under the same rule. Record all attempts. No opening scores have been seen.
Compose at least 200 diagrams per profile in families for active Fire Demon burns, ranked jumps that bare the opponent, and complete Lion moves. Keep entire geometry/motif groups within one partition; freeze the rows and split before either baseline probe. Use suite --proof one-ply to exhaustively try every legal move through Game and accept only terminal wins. Probe both partitions at depth 2 and 500000 nodes. Record the population and limitations.
Use separate cloud c3-standard-4 measurements for both profiles. Calibrate clock budgets with the native runner. Equal clocks on frozen engine against itself certify the route; 2x both base and increment measures time odds. Pair colors, no cutoff scores. Use post-opening ply 40 onward for late-game depth. Export concluded equal-clock self-play into all three opening-separated partitions. Bound jobs, record incomplete games, cost and cleanup, compare achieved depth with 8, and check shared sources and documentation before integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-25 (John): Tenjiku comes last, after the codebase cleanup (MSH-237 to MSH-235). Set to Ready when John releases Tenjiku.

John, 2026-11-25: use the game-independent strength tool of MSH-243 (Game-Independent Strength Tool); do not write new Tenjiku-only copies. MSH-240 removed the earlier per-game tools; MSH-243 records where to read them.

Backlog consolidation, 2026-11-26: MSH-235 is the final cleanup prerequisite and transitively includes MSH-243 (Game-Independent Strength Tool). The existing instruction that John must release Tenjiku remains in force; needs-john makes that gate visible. No harness work has started.

2026-11-27 (John): Released MSH-214 for implementation. The architectural tasks MSH-277 through MSH-283 remain deferred and do not block this work.

Claimed on codex/msh-214-tenjiku-harness from 2e4b9ad. Next: freeze the opening design and tactical proof protocol before screening. No compute spend yet.

Frozen engine/runner/helper from 2e4b9ad built in 169 s on one c3-standard-4. Build VM/disks/job deleted, conservative VM cost $0.0166. The corrected measurement worker is pinned to sha256:37596ae339ac4dd92b9ab29c3597d1598783e41451e9e8cbfedb632011853f7e (Cloud Build 470d5397); initial worker-layer build failed at the redundant system pip install. Tactical sources and composition recipe frozen under gs://mshogi-experiments-216675633801/runs/msh-214/inputs/. Nine whole motif groups, 24 authored positions each per profile; three entire families reserved for held-out. Shared screen/proof run strength-msh214-screen1 dispatched under a $0.81 VM ceiling. Next: inspect proofs, baseline and opening counts, then run color-swapped clock trials. Branch codex/msh-214-tenjiku-harness.

Clock pilot sampling frozen before game results: for each profile use the first admitted opening in each Texel hash stratum (train/validation/heldout) for equal-clock predecessor pairs. Use the first three catalog openings in order for 2x-time pilot pairs. Start at 30 s + 0.5 s per move with both base and increment doubled for the odds side. Target a further fixed 12 distinct odds pairs per profile at a viable control, subject to the task cap. A failed trial remains unscored in the record; adjust the control only for measured feasibility. Record small-sample uncertainty and all failures.

Screen/proof job strength-msh214-screen1 complete and cleanup verified (no remaining job, disks or VMs; VM cost <= $0.131). Adams admitted 100/505 (scores -97 to +92), Muller 100/448 (-100 to +98). The native one-ply proof and both baseline splits completed for every row. Each profile contains 216 independent rule-proved diagrams, 144 development and 72 held-out. Each scored 144/144 and 72/72 at depth 2 and 500000 nodes. These composed elementary tactics are saturated at that budget. Grouped families and all alternative winning actions are retained. Source, proof, probe and screen receipts are under runs/msh-214/screen1 in the experiment bucket. Next: verify consolidated cross-shard uniqueness and inspect equal-clock and 2x pilot results; no task completion yet.

The 30 s + 0.5 s pilot exposed incomplete first iterations at 608 to 1231 ms, including both colors of a Muller odds pair. Both profiles also completed full equal-clock pairs; typical late depth was 3 to 4. Calibration consequence fixed before the next run: increase to 60 s + 1.5 s, double both terms for time odds, play the same three Texel-stratified openings at equal clocks and all 12 additional catalog openings (indices 4 to 15, 1-based) at 2x. Keep unfinished pairs and do not use them to decide an SPRT. Use all six equal-clock pilot games per profile for export, with zero samples from unscored games. Re-screen each consolidated catalog to verify cross-shard board uniqueness. Per-pair wall budget 1200 s; no more than four pairs per c3-standard-4.
<!-- SECTION:NOTES:END -->
