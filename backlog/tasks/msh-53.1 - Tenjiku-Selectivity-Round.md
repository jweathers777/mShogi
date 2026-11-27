---
id: MSH-53.1
title: Tenjiku Selectivity Round
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-27T15:26:00Z'
updated_date: '2026-11-27T16:35:00Z'
labels:
  - tenjiku
  - search
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-202
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
parent_task_id: MSH-53
ordinal: 258000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Run the classical selectivity round for both accepted Tenjiku profiles using the MSH-214 harness and the MSH-202 search baseline. Preserve the rules and use the shared strength tool. Evaluation, later strength measurement and the baseline stop decision are separate steps.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Quiescence checks, full SEE, game-scaled margins, move ordering and shared df-pn mate search each have a measured result or a concrete reason they do not apply, for both profiles as applicable.
- [ ] #2 Freeze the predecessor and record the test method, reachable depth, caps and decision threshold before trials. Kept changes pass applicable perft, differential and tactical checks and qualify in a clocked predecessor test when search behavior changes. Record rejected or inconclusive trials and unscored games.
- [ ] #3 Required checks pass, affected durable documentation is current, and task notes record revisions, settings, measurements, costs, run paths and cleanup.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Freeze af4a48c, using the accepted Linux engine d6274cb (cd4233cc0177224b0f1c2b9c6b8256b4cc08433e0ea4c71662b9ec87849a2117; only a test differs in crates/). MSH-214 measured median late depth 5 for both profiles at 60 s + 1.5 s. MSH-202 established that the first iteration already misses this clock on some openings. Test one set that acts even at depth 1: up to eight non-losing quiet royal threats at the first quiescence ply, ordered with the existing quiet placement delta times 16. Use complete move effects, root restrictions and the existing exchange filter. Existing SEE and the 100-point Pawn margin are already game-specific. The shared mate solver requires check with no legal replies and excludes rule-score wins, whereas Tenjiku permits royal exposure and wins by final royal capture or bare-royal rules.
For each profile on c3-standard-4, use the six frozen MSH-202 roots (first three catalog openings, three ply-40 histories). Alternate baseline and candidate at depth 3, 500000 nodes, three cold repeats. Run MSH-214 depth-2/500000-node tactics (144 development, 72 held out) and opening calibration on catalog indices 1..8 at 60 s + 1.5 s. Pilot rejection if any tactical regression, a depth-3 failure on a root completed by the baseline, or fixed-depth sum of per-root median times greater than 2x baseline. The 2x guard limits the added cost on the existing clock, where the baseline already sometimes misses the first iteration. Record actual clocked admission separately. A pilot rejection is a cost or tactical result only. If the combined set fails, test quiet ordering alone at the same roots and tactics; advance it only with perfect tactics, all roots complete, and lower summed fixed-depth time on both profiles, since its intended benefit is efficiency. Keep one common candidate only with evidence for both profiles.
For a surviving set use color-swapped native predecessor games at the same clock, 768 plies, 1200 s per pair, one c3-standard-4 per two concurrent pairs, catalog seed order, separate profiles, no external engines. Pentanomial H0=0 H1=15, alpha=beta=.05, minimum 30 scored pairs per profile, maximum 600 per profile subject to the US$10 and 90-minute campaign cap. Stop at a complete batch, keep a behavior change only for H1 in both profiles. Report budget-censored or unresolved trials as no decision and revert. Report every launched, unscored and scored game, never score calibration or an incomplete pair. Run applicable perft/differential tests and final shared checks, document accepted status, cost and cleanup.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Claimed on codex/msh-53.1-tenjiku-selectivity. Freeze af4a48c as the predecessor. Next: inspect existing profiles, prior round results and shared search gates, then freeze trial settings before running either profile.

Combined candidate 6592f51 is frozen and pushed. The generator checks complete quiet moves after projection, deduplicates equivalent threats and honors root restrictions. The bounded Google Batch build is build-msh531-combined. Focused single-core perft, differential, exchange and root threat tests are running. Next: execute the frozen comparison and calibration. Branch codex/msh-53.1-tenjiku-selectivity, evidence prefix gs://mshogi-experiments-216675633801/runs/msh-53.1/.

Combined pilot 6592f51 vs d6274cb completed all twelve depth-3 roots in every repeat. Sums of per-root median times (baseline/candidate) were 3.690/4.353 s Adams and 3.697/4.479 s Muller, nodes 56080/65604 and 200241/195246. Tactics stayed 144/144 development and 72/72 held out for both profiles. Focused single-core release tests: 22 passed, including both-profile differential and perft, discovered threat and root restrictions; two manual timing tests ignored. Cloud build and pilot jobs, disks and VMs verified absent; compute upper bounds $0.01637 and $0.03902. The placement-only ablation is conditional on combined-set pilot failure, so it is not triggered.
Game batching fixed before any game result: eight distinct seeds per profile per batch, four c3-standard-4 VMs per profile, two pairs per VM. Start with seeds 1..8 for both. Use the original full-opening population and clock. The paired driver stops for diagnosis on any unfinished pair. Stop further batches as inconclusive if over half of either profile's pairs in a batch lack a native result in both colors: a majority-censored, clock-dependent sample cannot qualify this population. Otherwise continue consecutive batches until a decision or the original caps. Preserve every completed side and unscored side separately. No selective retries, clock substitutions, or pooling across profiles. First-move calibration is running independently; no game result seen. Next: run clocked pairs, verify replay hashes and resources, and retain or revert on the frozen thresholds.

Calibration finished on eight paired full-opening roots per profile, 60 s + 1.5 s, two pairs per c3-standard-4. Both engines had all 8/8 first moves admitted in both profiles. Adams candidate depth 4/5 on 5/3 roots, predecessor 6/2; Muller both depth 3/4/5 on 1/6/1. All 32 single-turn games stopped at the planned step limit; they supply no score. All 16 native replay report hashes match. Calibration resources verified absent, VM upper bound $0.06564. Strength job strength-msh531-games-1 was submitted for seeds 1..8 per profile. Full workspace checks run with one build/test worker. Current branch codex/msh-53.1-tenjiku-selectivity. Evidence and operator helper are in the task GCS prefix. Next: finish all launched game pairs and inspect the predeclared censoring guard.

Full-game batch 1, seeds 1..8 for each profile on c3-standard-4 at 60 s + 1.5 s: Adams 4W/0D/9L and 3 unscored games (one incomplete depth-zero candidate search, two wall stops), 5/8 scored pairs [2,0,3,0,0]. Muller 5W/0D/7L and 4 unscored games (three incomplete depth-zero candidate searches, one wall stop), 4/8 scored pairs [1,0,3,0,0]. All 16 pair report hashes and replays verified. Median late depth of admitted turns candidate/baseline: 6/6 Adams, 5/6 Muller. At most half the pairs in either profile are incomplete, so the next batch is seeds 9..16, strength-msh531-games-2. No SPRT decision. Batch 1 cost upper bound $0.60313; cumulative $0.725. All four completed jobs have verified cleanup. Focused deterministic snapshot test required updating only the two node counts, not actions or scores (candidate snapshot cb1baba). Full workspace Rust 1176 passed/10 ignored, Clippy and format pass. Desktop checks selected by the snapshot are running with standalone Node 24.21.0 after correcting a broken default runtime.

Full-game batch 2, seeds 9..16. Adams 3W/0D/8L, 5 unscored (2 incomplete searches, 3 wall stops), 4/8 complete pairs [2,0,2,0,0]. Muller 3W/0D/8L, 5 unscored (4 incomplete searches, 1 wall stop), 3/8 complete pairs [1,0,2,0,0]. The Muller sample crossed the predeclared majority-incomplete stop. No further games. For all 16 pairs in each profile: Adams 7W/0D/17L, 8 unscored (5 wall, 2 candidate and 1 predecessor incomplete searches), 9 complete pairs [4,0,5,0,0]. Muller 8W/0D/15L, 9 unscored (2 wall, 6 candidate and 1 predecessor incomplete searches), 7 complete pairs [2,0,5,0,0]. Counts of individual wins and losses include native conclusions from incomplete pairs; the paired test uses only complete pairs. Both fell short of the 30-pair minimum. Neither H1 nor H0 was established. The observed score ratios do not establish an Elo estimate or a strength loss. All 32 full-game pair reports have matching hashes and successful native replays.
Technique decisions for Adams and Muller: (1) Eight quiet royal threats at the quiescence frontier were tested with (2) quiet placement ordering. Both were removed after the set failed to qualify; no individual strength effect claimed. The ordering-only pilot was conditional on combined pilot rejection and was not triggered. Existing table, tactical gain, killers, history and continuation ordering remain. (3) The existing bounded exchange search already uses both players' complete recapture moves and allows either to stop, including off-target burns, compound captures, promotion and suicide. Work exhaustion retains the move. Tenjiku differential and exchange tests passed. A single-square swap list cannot represent these effects. (4) MARGIN_UNIT 100 equals the board Pawn and captures have no hand value. No rescaling applies. (5) The shared df-pn proof uses check with no legal replies and excludes rule-score terminals. Both profiles allow a royal to remain exposed and end on royal capture or bare-royal outcomes. That solver does not apply.
Restored the accepted engine, tests and snapshot in 731e510. Final product and tooling diff against af4a48c is empty. The candidate passed fmt, Clippy, 1176 Rust tests (10 ignored), TypeScript and 578 frontend tests. The standalone compatible Node 24.21.0 rerun resolved the default runtime problem without any repository changes. Final documentation is updated in the strength roadmap and shared search reference. Final shared checks select no product suites. No rules, format or large refactor survives to require separate review. Next step is Tenjiku evaluation under MSH-53.
Reproduction: shared bench, probe and match CLI arguments, input and binary hashes and pinned images are in the project storage manifests. Campaigns used c3-standard-4 only, Rust 1.99.1 release Linux x86-64. Cumulative VM upper bound $1.322431 (build $0.016364, pilot $0.039014, calibration $0.065633, games $0.603121 + $0.598302, figures rounded). Incidental storage excluded. All five Batch jobs, VMs and disks verified absent. All run outputs, operator scripts and checks under gs://mshogi-experiments-216675633801/runs/msh-53.1/. No new container build. Branch codex/msh-53.1-tenjiku-selectivity. Next: finish integration and remove its worktree and local run and build files.
<!-- SECTION:NOTES:END -->
