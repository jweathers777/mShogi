---
id: MSH-53.2
title: Tenjiku Evaluation Round
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-27T16:53:00Z'
updated_date: '2026-11-27T17:08:00Z'
labels:
  - tenjiku
  - evaluation
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-53.1
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
parent_task_id: MSH-53
ordinal: 259000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Run the classical evaluation round for both accepted Tenjiku profiles using the shared Texel fitter, MSH-214 harness and accepted MSH-202 engine. Preserve the game rules. Later strength measurement and baseline acceptance are separate steps.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Use about 500,000 or more quiet self-play positions with rules-adjudicated outcomes, duplicate control and a split by game or opening. Record held-out loss before and after fitting with coverage of both profiles and report any shortfall.
- [ ] #2 Evaluation features agree with search scoring through moves, captures, promotion and undo, including applicable compound effects. Required correctness and tactical checks pass.
- [ ] #3 Keep weights only with qualifying clocked evidence against the frozen predecessor, using the predeclared method and H1 around 10 Elo. Record incomplete games and unsupported candidates and revert unqualified weights.
- [ ] #4 Required checks and applicable review pass. Update relevant durable documentation and record revisions, measurement settings, costs, run paths and cleanup in Backlog.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Freeze d6274cb as the predecessor (identical accepted search at b0ab5ac). Fit the existing static terms only: material per movement face (Pawn and royals fixed), light-piece advance, Lion/Lion Hawk/Fire Demon capped advance and central file, other-piece central file, and tempo. Bounds material 0.5x..2x, placement -32..32, tempo 0..64; shared calibration K .01.. .71, steps 64..1 and validation selection, fixed 900 s. Quiet requires no pending bare reply, no immediate royal peril, and no positive or uncertain tactical exchange under the existing full compound SEE. Compare extracted and incremental scores after fixture moves and undo in both profiles.
Use shared native selfplay/export, every board ply, no samples from unfinished games. Start with the first catalog opening per SHA256 split for each profile (indices Adams 1,4,16; Muller 1,2,3). Pilot fixed 2000 nodes and equal 60 s + 1.5 s, 768 steps, one pair per opening/control, fixed-work 180 s / clock 1200 s pair cutoff. Include same-engine MSH-214 equal-clock control2 data as a separately identified historical diagnostic supplement if needed, outside the production count. Split by catalog opening, remove all cross-split duplicates and retain one within a partition. Report quiet counts, outcomes, coverage, cost and projected yield. Scale up only if 500000 unique quiet boards with both profiles appear affordable, reserving funds for a match. If not, fit the available stratified data once as a diagnostic only and record each profile’s heldout loss; do not install diagnostic weights.
For a qualifying full corpus: jointly fit once, require lower heldout MSE separately for both profiles, no tactical decline on either MSH-214 suite (depth 2, 500000 nodes); compare against frozen d6274cb with color swaps at 60 s + 1.5 s on c3-standard-4, 768 steps/1200 s pair. Independent pentanomial SPRT 0/10, alpha=beta=.05, min 30, max 600 pairs/profile, consecutive batches of eight seeds/profile, two pairs/VM. Stop on budget, >half incomplete in either batch, H0 or H1. Retain only with H1 for both profiles; incomplete games unscored, no pooling or selective retries. Total task ceiling US$10. Verify both-profile correctness, suites, resource cleanup, docs and shared checks.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Claimed on codex/msh-53.2-tenjiku-evaluation from b0ab5ac. The frozen predecessor is the accepted MSH-202 product at d6274cb. Next: freeze the sampling, fitting and qualification gates before looking at pilot outcomes.

Pilot protocol f7870d4 is published. Clock pilot strength-msh532-clock (six separate opening pairs, two tactical probes) and fixed-2000-node pilot strength-msh532-fixed (six pairs) use frozen MSH-202 engine binaries and digest-pinned MSH-214 worker. Directories under gs://mshogi-experiments-216675633801/runs/msh-53.2/. Added an offline model for the shared fitter, compound SEE quiet filtering, a score check in the existing differential and undo suite, and deduplication independent of physical piece IDs. Product search is unchanged. Next: inspect exact game stops, export and check quiet yield before scaling, run checks and fit. Branch codex/msh-53.2-tenjiku-evaluation.

Fixed 2000-node pilot finished: all 12 games stopped at an incomplete iteration, 11..56 applied plies, no labels. All six native reports replayed, cleanup verified, VM bound $0.062943. Add one prespecified production-feasibility check to address the low node ceiling before declaring a shortfall: same six catalog roots, one pair each, both sides depth 1 with a 500000-node safety cap, 768 plies, 600 seconds per pair. This gives expensive first iterations room to finish without spending time on deeper iterations. Use frozen native engine/helper/runner with only depth fields changed after config generation. New control goes into the same family split if games conclude. Its yield and minimum per-game execution cost (four simultaneous pairs per four-core machine for projection) also inform the scaling decision. No change to match protocol; no results from this new control seen.

Unchanged accepted engine passes 21 release Tenjiku tests (2 manual timing tests ignored), including compound perft, both-profile rules/search differential and score restoration. Three offline model tests pass, including invalid diagrams, royal threats and positive exchanges. Both fixed-node exports independently replayed six games per profile and produced zero labels as expected. Next: measure the remaining controls and assess feasibility. Branch codex/msh-53.2-tenjiku-evaluation.

Depth-one native control: all six pairs concluded and replayed. Adams produced 302 unique quiet boards in 48.182 pair-seconds; Muller 189 in 27.738. Within each fixed-work pair the two colors have the same trajectory. Measured generation-only projection for 500000 on four concurrent lanes per c3-standard-4 is $1.083 at $0.201608/VM-hour. Actual production also needs random continuation, replay, export, storage and split. This is feasible enough to test. Pilot data is excluded from production fitting.
Production frozen before generation: both players use d6274cb, depth 1, 500000-node safety cap, 768 plies, 600 s per pair. Cycle the full MSH-214 100-opening catalogs separately by scope seed; add four reproducible random legal plies using the shared helper and keep the original catalog family. Per profile use seeds starting at 101, assigned round robin to 16 lanes. Start with up to 4096 pairs per profile, stop starting pairs six minutes before a 60-minute campaign deadline; retain all already launched results and saves every completed game with its exact native conclusion. Four workers per c3-standard-4, 8 VMs (4/profile); one additional bounded batch of new unused seeds only if yield justifies it. Preserve unscored cutoffs. Each board ply of every concluded game; use existing shared exporter, split/dedup/filter across all production inputs. Whole-catalog opening hash split frozen earlier. For at least 500000 unique quiet positions with both profiles fit once; report individual heldout losses, then enforce the frozen tactical and 0/10 clock qualification. No external opponent. Reserve at least $3 for qualification; hard $10 task ceiling. Branch codex/msh-53.2-tenjiku-evaluation. Next: launch bounded production and measure realized yield.
<!-- SECTION:NOTES:END -->
