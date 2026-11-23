---
id: MSH-213
title: Chu Strength Harness
status: In Progress
assignee:
  - codex
created_date: '2026-11-20T22:51:00Z'
updated_date: '2026-11-23T21:19:00Z'
labels:
  - chu
  - search
milestone: m-5
dependencies: []
ordinal: 220000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: the harness step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Chu, modelled on MSH-204 (Tori Strength Harness). It gives the later speed, selectivity and evaluation tasks a way to show that a change makes play better. Use the existing clocked native experiment route (mshogi-match experiment-run, structured internal protocol); add no text engine protocol. No external opponents (John, 2026-11-19). Lion double moves, igui and the Lion-capture rules must be covered by the tactical suite.
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
1. Reuse native clocked route, add Chu catalog support and nominal clock allocation. 2. Generate deterministic openings and screen under the rule recorded before results. 3. Establish tactical answers with native exhaustive proof independent of alpha-beta; retain complete answers, Lion cases and grouped held-out data. 4. Freeze predecessor binaries; cloud pilot, control/depth, self-play export and 2x-time measurement within US$10. 5. Required checks, review if applicable, evidence upload, integration and cleanup.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main 30e08e4. Freeze this predecessor for Chu harness measurements; reuse native clocked experiments and shared harness infrastructure. No game-rule, search-strength or evaluation change is planned. Budget cap US$10; heavy runs use Google Cloud Batch, one machine type per comparison. Branch codex/msh-213-chu-strength-harness.

Pre-screen protocol: source 30e08e4 engine, no search changes. Generate standard-root quiet rear-zone prefixes with xorshift seeds 1..600 (8 plies, no captures/promotions, royals/Lions fixed); deduplicate full repetition positions. Screen at fixed depth 3, at most 500000 nodes; require completed depth and absolute side-to-move score <=100 Pawn hundredths, no native conclusion. If fewer than 100 pass, report the shortage and fix a revised rule before rescreening. Existing eight mirrored reduced-material diagram families are separate calibration/control candidates, not claimed to be reachable standard openings. Chu adopts nominal clock allocation for new measurements only. Any unscored Chu pair stops the game campaign; no length-limit exclusions. H0=0/H1=15, alpha=beta=.05 for predecessor smoke and time odds; bounded samples may give no decision. Late-game depth means applied board plies >=100, with per-game medians reported and no short-game substitution.

Checkpoint: implemented (not yet integrated) native Chu catalog loading, Python clock/catalog enablement with nominal allocation, shared unchanged calibration roots, deterministic opening screen helper, finite full-width native proof and a 315-position constructed one-turn tactical corpus. It includes double capture, igui, qualifying defended-Lion capture and two-royal capture; independent answers enumerate all native winning actions, with held-out placements fixed before probing. Limitation: this is an elementary tactical screen, not a deep tactics sample; shared construction templates cross development/held-out. Added native-report Texel export and bounded Batch worker. Local compile and existing Python tests pass; more checks remain. Frozen predecessor 30e08e4 source uploaded; single-VM job build-msh213-baseline-1 submitted, controller handles cleanup, no result yet. Next: finish focused tests, build current tooling in cloud, generate/screen openings, probe tactics and pilot clocked games; choose control from measured depth. Branch codex/msh-213-chu-strength-harness.

Review found an igui coverage gap: ordinary captures also win the one-turn positions. Fixed by a separately scored exact capture-return contract in native and search positions, including restored Lion origin and removed victim; ordinary captures fail this contract. Complete valid winning answer sets are unchanged. Focused native tests pass (2); Python clock/catalog/time-odds and outcome-only split tests pass (7); workspace Clippy passes. Full debug workspace running. First baseline Batch attempt failed before VM creation due incorrect subnet; cleanup verified no jobs/VMs/disks. Correct subnet is mshogi-batch-us-central1. Tooling source 4da7b49 built successfully on c3-standard-4 (Rust1.99.1, 152 s); controller cleaning resources. Worker image digest sha256:17facfe4756343f40d6b624d53cc03796e683b79f995072d6f232f0d00729b04. Fixed 600-candidate depth3 opening screen submitted as msh213-screen-1. Measurements remain pending; no acceptance criteria checked.

Opening screen complete on c3-standard-4: all 600 candidates completed depth3 and passed |score|<=100 (observed -25..+10). Freeze first 128 admitted seeds in ascending order as chu-openings-v1; full 600-row screen retained under runs/msh-213/screen-1 in GCS. Native catalog test confirms 128 distinct live 92-piece standard prefixes with eight events. Config-writer tests 4/4 passed; full debug workspace passed. Independent review correction includes actual generated search-move membership for igui. Before game results: pilot uses the standard catalog, nominal Fischer 10 s +0.1 s, 1000-event cap, 300 s pair watchdog, one color pair; report any unfinished game without scoring it. Targeted process CI run45267832263 is active. Final tactical helper cloud build is active; its predecessor/search remains unchanged.
<!-- SECTION:NOTES:END -->
