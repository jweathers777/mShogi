---
id: MSH-214
title: Tenjiku Strength Harness
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-20T22:51:00Z'
updated_date: '2026-11-27T14:00:00Z'
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
- [x] #1 At least 100 balanced openings, with their source and the balance rule fixed before screening
- [x] #2 A tactical suite of at least 200 positions with independently established answers (for example from a native mate prover) and a held-out part, with the baseline score
- [ ] #3 A game-test control at which the median late-game depth is at least 8, or a lower target with its reason and cost recorded; a game test against a frozen predecessor runs end to end in the cloud
- [x] #4 A self-play export for Texel data with a split, and a time-odds measurement (2x time) recorded
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

Pilot1 finished on c3-standard-4. All 24 games remained in evidence: at equal 30 s + 0.5 s, Adams 5/6 and Muller 4/6 games concluded, with 2/3 and 1/3 complete predecessor pairs. Median depth from post-opening ply 40 was 4 for each profile (960 and 1551 accepted turns, including retained prefixes). At 2x, Adams 4 wins and 2 unfinished, Muller 3 wins, 1 loss and 2 unfinished for the longer clock. Pilot1 is a calibration failure: incomplete first iterations and 768-ply step limits, no SPRT decision. VM upper bound $0.461 and verified zero remaining resources. The next run, strength-msh214-control2, uses the already frozen 60 s + 1.5 s control and twelve further odds pairs, plus export/uniqueness checks. Branch codex/msh-214-tenjiku-harness.

The shared suite builder retained explicit partitions correctly but described every output as a hash split. Corrected the descriptive metadata for future runs; the frozen Tenjiku files state their three exact held-out families. Board records, partitions, winning actions and engine results are unchanged from the cloud proofs. All 56 strength-tool tests passed.

Depth-cost cross-check frozen before its searches: for each of the three equal-clock pilot openings per profile, choose the first color game reaching 40 post-opening plies, truncate the native history after exactly 40, and request fixed depth 8 with a 250000-node cap on c3-standard-4. Use the shared signature command with the same frozen engine on both sides. A missed depth is an explicitly incomplete fixed-depth observation. The target decision also uses all late-game accepted turns and the clocked pair wall cost.

Consolidated cross-shard re-screen through the shared native tool accepted all 100 of 100 source rows in each profile. The current board de-duplication, four-ply records, scores, order and IDs compare equal to the committed catalogs. Evidence control2/tasks/{1/adams,5/muller}-all-verified.json in the task GCS prefix.

Control2: equal 60 s + 1.5 s on three prespecified stratified opening pairs per profile, 3 pairs in parallel per c3-standard-4. Adams 5/6 games and 2/3 pairs concluded. Muller 3/6 games and 1/3 pairs concluded. In concluded games median late depth was 5 by pooled turns (Adams 1104, Muller 289) and by game in both profiles. Only 37 and 13 late turns reached depth 8. Completed pairs took 856 s and 1161 s Adams, 616 s Muller, at <=$0.0651/VM for the longest pair even if it alone occupied the machine.
Fixed 2x time odds, 12 pairs/profile, 4 concurrent pairs per c3-standard-4, same frozen binary, seeds 4..15, 120 s + 3 s vs 60 s + 1.5 s. Adams: 10 wins, 0 draws, 2 losses, 12 unfinished (9 depth-zero, 3 wall limit). Three complete pairs, pentanomial [0,0,0,0,3]. Muller: 10 wins, 0 draws, 3 losses, 11 unfinished (10 depth-zero, 1 wall limit). Four complete pairs [0,0,1,0,3]. Worst/best scores over all 24 scheduled games: Adams 41.7% to 91.7%, Muller 41.7% to 87.5%. Time odds inconclusive: no Elo or SPRT claim, no imputed game results, no pooling. All original report files and native replay receipts are in control2/tasks/.
Self-play export: shared export, every fourth board move, six equal-clock pilot games/profile. Only concluded games supplied samples; the manifests retain all stops. Adams train/validation/heldout 74/78/121; Muller 117/42/65. All three partitions populated with disjoint opening groups and no cross-split duplicate diagrams. Control2 Batch VM upper bound $0.606, jobs/VMs/disks removed. Next: inspect the fixed depth-8 cost check, run shared final checks, integrate. Branch codex/msh-214-tenjiku-harness.
<!-- SECTION:NOTES:END -->
