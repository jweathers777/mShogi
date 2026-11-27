---
id: MSH-53.1
title: Tenjiku Selectivity Round
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-27T15:26:00Z'
updated_date: '2026-11-27T15:38:00Z'
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
<!-- SECTION:NOTES:END -->
