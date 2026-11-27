---
id: MSH-53.3
title: Tenjiku Reference Ladder and Blunder Screen
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-27T21:36:00Z'
updated_date: '2026-11-27T22:20:00Z'
labels:
  - tenjiku
  - strength
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-53.2
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
  - docs/m5/strength-harness.md
parent_task_id: MSH-53
ordinal: 260000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Measure how the accepted classical Tenjiku engine scales with search resources and screen its moves with a substantially deeper mShogi search. Cover Adams and Muller independently with the shared strength tools. John authorized both measurements; no external opponents or judges, architecture work, or baseline stop decision in this step.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Freeze the accepted revision, resource rungs, paired sampling and scoring method, and time and dollar limits before the campaign. Report the same-family scaling curve separately for Adams and Muller, with uncertainty, incomplete results and limitations.
- [ ] #2 Freeze the sampling, deeper judge protocol, blunder rule and classes before inspecting results. Report blunders per 100 moves and mean win-probability loss for each profile with coverage and limits; preserve reproducible blunder positions and apply the roadmap follow-up rule.
- [ ] #3 Record exact commands, revisions, inputs, run evidence, costs, and cleanup. Relevant checks pass and the parent task and roadmap reflect the evidence. Leave the baseline stop decision to John.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Frozen before play. Accepted engine 0bbfd9c68abb18b424bb82f2b4aa118c5a22ce5a. Reuse its qualified Linux executable from 6855d62, SHA256 84fb903bb5c1320166fdae80407f43ff68a76be9f53571407fa11a4496eede83. Between these revisions the crate diffs are tests/fixtures only. The hash-pinned shared helper, native runner, both 100-opening catalogs and Cloud Build worker image come from the MSH-53.2 games-1 manifest and intent. No new image or product changes.
1. Separate fixed-node round robin for Adams and Muller. Compare 16000 vs 4000, 64000 vs 16000, and 64000 vs 4000 nodes/turn, depth ceiling 64, both colors of catalog indices 1..16 for every edge. Fixed nodes avoid known first-iteration clock variability. Four pairs in parallel per c3-standard-4, 8 VMs per block, one block per edge, no SPRT early exit, retries or substitutes. Max 768 events/game, 1200 s/pair, 1260 s/bin, default fixed-work watchdog. Keep all unscored/partial pairs. Report higher-rung W/D/L, unscored reasons, complete-pair pentanomial scores and shared approximate 95% interval, late depths at ply 40 and worst/best score bounds for all scheduled games. Direct edge estimates only, no assumption of additive Elo.
2. Judge both colors and both players in ALL available 16000 vs 4000 reports, including incomplete prefixes. Shared evenly spaced midpoints up to eight applied board moves/game (256 moves/profile maximum). A 512000-node fresh accepted engine, depth 64, on each full native history before and after; native terminal score overrides after. Keep failed or missing samples visible. Report combined and by player budget for each profile; record actual depths, nodes and paired depth deltas. Qualify as materially deeper for the full sample only if median depth gain >=2 and >=75% deeper before; show the deeper-only sensitivity. Same-family fixed-work results only. Cluster bootstrap the opening pairs, 2000 replicates with seed 533, for rate and mean. Report observable coverage and limits for rare/zero events.
3. Frozen scoring: shared logistic p=1/(1+exp(-cp/600)); mover loss=max(0,p_before-p_after); clear if p_before>=.30 and loss>=.30. First-match symptoms: same chosen move (unstable estimate), opponent terminal/forced win within 9 plies, missed own forced win within 9 plies, missed promotion, missed stationary/multi-landing alternative, opponent stationary/multi-landing reply, opponent promotion reply, other. A move missed is one where only the judge alternative has that property. No score-driven sampling. Preserve all clear histories, played/reference actions and opening-group SHA256 split. A follow-up needs a single diagnosed general mechanism for >=30% of clear errors in a profile with corroborating lines from independent openings. Broad classes or shared-search disagreement alone do not establish a mechanism. Record eligible follow-up, leave fixes and baseline decision separate.
4. At most seven sequential 30-minute Batch allocations: three ladder blocks plus four judge blocks (four consecutive openings/profile/block). Aggregate campaign allocation wall <=3 h 30 min including controller cleanup reserves, no simultaneous campaigns, <=8 VMs, no retries. $0.201608/c3-standard-4-hour from MSH-53.2 gives <=$5.65 VM cap; reserve $1 for incidental services, total <=$6.65 and always <=John’s $10. No replacement samples. Carry prior resource bounds, stop on unreconciled resources or exhausted caps. Archive manifests, reports, summaries, judge positions, work identities and receipts in the expiring msh-53.3 prefix; verify jobs/VMs/disks removed after each block. Update parent, roadmap and final checks.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Claimed on codex/msh-53.3-tenjiku-assessment. Next freeze the paired ladder and deeper judge protocol from the accepted evaluation and shared harness, then run both authorized measurements.

Pre-dispatch controller setup failure in strength-msh533-16k-4k: a missing local google-cloud-pubsub module stopped completion monitoring and the controller deleted the job before any worker started. tasks.json empty, zero worker claims and zero games. Retain this original prefix and charge $0.009295 upper bound. Install tools/experiments/requirements-controller.txt and verify both imports. Before any play: submit the unchanged first edge to 16k-4k-v2, then the two other edges. Judge groups 9..12 and 13..16 will share one allocation (two frozen commands per VM); groups 1 and 5 as planned. Seven total allocations maximum, identical sampling and caps, no played seed repeated. Inputs are hash-verified under gs://mshogi-experiments-216675633801/runs/msh-53.3/inputs/. Branch codex/msh-53.3-tenjiku-assessment.

First 16000/4000 block: 32/32 raw pairs replayed and verified. Adams 0/32 native scores, Muller 1 higher-work win/32; no complete pairs. Node-limit stops 32/31, mostly on the 4000-node side. Keep this failed measurement and all prefixes. Spending bound through two cleaned allocations is recorded in operator receipts. Continue both remaining frozen node edges without change.
Supplement, frozen independently before clock or judge results: a separate Fischer clock curve on fresh catalog indices 41..46, both colors and same engine. Compare 60+1.5 vs 30+0.75 and 120+3 vs 60+1.5. Two bins of three concurrent pairs per profile per edge, c3-standard-4 only. Same 768 events/game, 1200 s/pair, 1260 s/bin, depth-64/5M safety limits, no substitution, no early decision, same independent scoring, late depth and censor bounds. The clocks inherit the accepted 60+1.5 control, and avoid a small hard node limit in difficult positions. Four cells, six openings each, limited precision. This is an independent supplement, not replacement games.
To make room INSIDE the original seven allocations, three-and-a-half hours and $6.65 total bound: judge in two blocks of eight successive openings/profile (1..8 and 9..16), two independent report lanes per VM. Preserve the same 512000-node/depth-64 native helper, all eight frozen midpoint samples and unchanged criteria. The pre-dispatch scheduler groups only reports; no scored sample has been inspected. A thin Cloud Build layer on the existing worker permits two concurrent shared judge calls and records individual missing searches (original 60 s native watchdog), including if the cleanup reserve precludes a call. Preserve worker errors, unjudged histories, and a cluster sensitivity by coverage. Reserve the seventh 30-minute allocation for both supplemental edges. Carry all previous costs and retain $1 for incidental services. Next complete remaining node blocks, both judge blocks, clock supplement, then audit.

The accepted workflow passes its 58 strength tests. The two-lane operator Cloud Build 73e25cba-d4bf-4aa3-9f2f-800c824652dc succeeded in 31.34 seconds, pinned worker sha256:1ac7359e852b8c9d7a2ffef7d07ffc19bc1bba40d0cf8189d089d46047b8261e on top of the existing worker sha256:37596ae339ac4dd92b9ab29c3597d1598783e41451e9e8cbfedb632011853f7e. Native engine, search limits, source sampling and scoring are identical on both report lanes. The original failed setup and first completed node-edge cleanup receipts give cumulative VM bound $0.10592. Next: audit 64000/16000, then 64000/4000; judge first-edge prefixes and fresh-opening clocks under previously recorded budgets.

64000/16000: all 32 pairs passed exact config, binary, native replay, hash and independent score checks. No complete pairs. Adams 1W/0D/5L with 26 unscored; Muller 3W/0D/2L with 27 unscored. All 53 stopped at the 16000-node player. Bounds over all games 3.1..84.4% Adams and 9.4..93.8% Muller. Both late medians 4 vs 3. The worker returned nonzero for incomplete games as specified. No residual job/VM/disk. Next: finish 64000/4000, judge the frozen prefixes and run the separate clock supplement. The new worker image and exact task-owned layers and source are retained in operator/images; its staging source generation is verified absent.

64000/4000: every one of 64 games stopped at the lower side’s node budget. All native reports and cleanup verified. Late depths Adams 5/2 and Muller 4.5/2. Across three edges, no complete color pair (12/192 individual games concluded), so no fixed-node Elo estimate. Four allocations checked empty; VM cost ceiling $0.529025. The first edge has 924 Adams and 860 Muller played moves, 256 frozen judge positions per profile, with only eight per profile at or beyond post-opening ply 40. The independent 512000-node first judge block is submitted at judge-1/. Next: its second block judge-9/, then clocks/, preserving split membership, limits and receipts.

John asked whether the node stops were artificial. Verified every final observation and every applied action in 96 native two-game reports (192 games). All 180 unscored endings had consumed the full artificial node cap and returned incomplete, completed_depth=0, no choice, no score, an empty principal variation and applied=false; 117 at 4000 nodes and 63 at 16000, none at 64000. Another 6373 turns hit a node limit after at least one completed depth, returned the last completed choice and were applied; 28 additional turns completed normally. There was no rejection of a valid last completed move. These outcomes measure the chosen caps and cannot establish normal-clock playing strength. I selected node limits without first checking that representative full games from the accepted engine could finish on each profile. A short representative full-game completion pilot would have caught this before committing the fixed ladder. No evidence or score was removed. The independent clock supplement addresses completion at ordinary playing controls. Evidence: node_stops.py and node-stops.json in the final operator archive.

John asked to reexamine avoidable stops and wasted cost. New cloud submissions held before the second judge or any clock game. First judge was productive, 14/16 reports available at the checkpoint: 112/112 judged per profile, Adams 110 deeper (median depth 3 to 6, paired gain 3), Muller 111 deeper (2 to 6, gain 4), no timeout. Only 0 and 3 late samples at that checkpoint. Completing the active first job preserves evidence; the second weak-node prefix block and original broad clocks are held.
Independent feasibility pilot, frozen before any clock results. Drop the 30+0.75 lower control. Use the accepted 60+1.5 normal control and its 2x opponent 120+3, fresh catalog openings 41–42 and 81–82 per profile (two different source batches), both colors. Four c3-standard-4 VMs total, two concurrent pairs/VM as in MSH-53.2, identical settings required for later production. Base clocks allocate normal 30-move share, subtract first/subsequent transport reserve up to 300/50 ms, then charge elapsed Tenjiku setup and reserve min(50 ms, remaining request hard time/10) for result validation in RequestTime::limits. The exact nominal first requests after integer division are 2823 and 5948 ms; the value actually recorded also reflects any elapsed transport/clock charge. Tenjiku returns a previous completed iteration when available, no move at depth zero. Symmetric depth64/5M nodes, 60s request watchdog, 768 events/game, 1280s wall shared by both serial color games (80s more than the original), 1330s bin and 45s shutdown reserve. 30-minute absolute Batch allocation including 5-minute controller cleanup reserve. Cap for pilot < $0.404 VM; 6th of original 7 allocations. Launch once after first judge cleanup. No backfill.
Per-profile gates for any separate production cohort: all four paired openings launched, all native reports verified; at least 3/4 pairs and 7/8 games concluded; zero first-iteration depth-zero, clock forfeit, pair wall, watchdog, artifact or transport failure; >=120s headroom on EVERY completed pair. Inspect accepted depths and time slices and second-game completion. Report the entire pilot including failures, separately from any production cohort. Only the pilot is released now. Stop if gates fail, offer final-allocation tradeoff before further spending. No repeat of fixed-node ladder.
<!-- SECTION:NOTES:END -->
