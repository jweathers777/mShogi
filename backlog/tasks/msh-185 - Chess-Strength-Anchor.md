---
id: MSH-185
title: Chess Strength Anchor
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-16T20:50:00Z'
updated_date: '2026-11-16T21:39:00Z'
labels:
  - handle:chess-strength-anchor
milestone: m-5
dependencies: []
references:
  - https://www.computerchess.org/
  - https://github.com/official-stockfish/books
documentation:
  - docs/experiments/parallel-runner.md
  - docs/experiments/gcp-novice-setup.md
priority: high
ordinal: 192000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Strength Anchor

## Problem
The last external Chess measurements came before the classical search round. MSH-139 (Chess Strength Assessment) estimated about 1900 on a conditional local CCRL-referenced index at 1 s per move (range 1700-2200). MSH-161 (Competitive Gameplay Diagnosis) scored 52.5% against Oxidation 0.6.0 (CCRL 40/15 1943, November 2024 snapshot) on prepared openings. After that, MSH-164 (Shared Search Core), MSH-166 (Search Selectivity Completion), MSH-167 (Engine Time Management) and MSH-168 (Classical Endgame and King Evaluation) gave large gains, but only against frozen predecessors. Self-play gains usually overstate the real gain. We do not know the present strength.

## Objective
One anchored estimate of present Chess strength against independent rated engines. The result tells John how much of the self-play gain is real and closes the Chess round before Shogi work. It is a measurement, not a qualification: no target is set and the result does not block Shogi work.

## Owner approval (John, 2026-11-16)
This rating study against external engines is approved within these bounds:
- Time control 60 s + 0.6 s Fischer increment for both engines. mShogi uses its own time manager (MSH-167); no fixed move time.
- About three independent opponent engine families in the main phase, about 300-400 scored games in total including bracketing games.
- Run in the cloud (John, 2026-11-16), not on the local Mac.
- Hard caps: 6 hours of wall time for all match work and US$15 of estimated cloud compute, including bracketing, failures, reruns and smoke tests. Stop at the first cap reached and report what is complete. These are limits, not targets. With the existing 8-worker setup (below), the expected run is about 2.5-3.5 hours of wall time and about US$4-6: about 350 games of about 3 minutes (about 17-18 game-hours) on 8 workers at about US$1.61 per hour for all 8. Bracketing (3-4 sequential rounds of 8 games) takes about 30-40 minutes; the main phase takes about 2-2.5 hours. MSH-161 often got only 6 of 8 workers because of Google capacity shortages, which adds about 30-45 minutes. If the run is on course to exceed about 4.5 hours or US$10, find the cause and record it. If the 6-hour cap would stop the main phase early, stop at about 250-300 scored games; that still gives a useful interval.
Any larger sample, longer time control or different opponent class needs a new approval.

## Approach
- Freeze current main with `python3 -m tools.strength.cli build-engine` and record the revision and binary hash.
- Opponent pool: full-strength engines from one dated CCRL list snapshot. Prefer the CCRL Blitz list because it is nearest to this time control; record its URL and date and do not mix lists or 4-CPU rows. Use official sources, native ARM64 builds, 1 thread, equal hash (for example 64 MB), ponder off, own book off. Do not weaken opponents. Prepare a short list of candidates from several authors/families at about 100-150 point steps from roughly 2000 to 3000 before play starts.
- Rating model: fit one Bradley-Terry (logistic Elo) model to all scored games, with opponent ratings fixed at their CCRL values, using pairs as the unit or a pentanomial/draw-aware variance (BayesElo or Ordo style is acceptable). Every scored game, including bracketing games, enters this fit. Record the prior and the fit method before play.
- Phase 1, bracketing (sequential noisy bisection): start with a wide prior (about 2000-3000; MSH-139 gives about 1900 for the older engine, and later self-play gains point higher). Play 4 pairs against a candidate near the middle of the prior (about 2500). Refit, then play the next 4 pairs against the candidate nearest the current estimate. Repeat until the estimate is within about +/-100 (normally 3-4 rounds) or about one hour of wall time is used. One game near 50% gives about three times the information of one at 90%, so stop bracketing early.
- Phase 2, main sample: choose about three engines from different families within about +/-150 of the estimate. Rotate between them in batches; refit after each batch. Replace an opponent if its running score leaves about 25-75%. Stop when the 95% interval of the combined estimate is about +/-30-40, or at the 6-hour cap. Beyond about 300 games, systematic error (CCRL anchor uncertainty, hardware and time-control difference from the list, family style effects) exceeds the statistical error, so more games help little.
- Openings: the CC0 Stockfish 8moves_v3 book, as in MSH-139, one fresh opening per pair with colors reversed. The pair is the statistical unit.
- Cloud execution: reuse the existing Google Cloud Batch path of MSH-161 (Competitive Gameplay Diagnosis) (`tools/experiments/gcp.py`, docs/experiments/parallel-runner.md, docs/experiments/gcp-novice-setup.md, docs/m5/evidence/msh161/cloud-primary/README.md); do not build new cloud infrastructure. Use one machine type for all games and both phases; record machine type, CPU model, region, OS and image. Use the existing worker shape: one c3-standard-4 VM per worker, one game per VM, one engine thread each, so that games do not compete for CPU. Build mShogi and all opponents for the VM architecture from recorded sources and verify their hashes on the VM. Bracketing is sequential: run each bracketing round as a small batch and refit before the next. Run all 8 games of a bracketing round at the same time. Request 8 workers for the main phase; the project C3 quota is 32 vCPUs, so 8 is the maximum. Do not request a quota increase or more workers in this task. Record the actual number of workers and any failed allocations. Record VM-hours and estimated cost per batch, confirm that all VMs and jobs are deleted at the end, and keep games, logs and results in the repository evidence or a recorded storage location.
- Reuse the existing native external UCI runner and replay path that MSH-139 used (`docs/m5/chess-strength-assessment.md`, `docs/m5/evidence/msh139/`) and the strength harness clock mode where it fits. A minimal extension for the sequential opponent choice is acceptable; do not build a new framework.
- Native rules decide every result. No evaluation adjudication. Process failures, time losses caused by the harness and technical caps stay unscored and are reported.
- Report the combined estimate and also a per-opponent estimate for each main-phase family. If per-family estimates disagree by more than 150 points, report the disagreement and do not hide it in the combined number.

## Completion
The report states the estimate with its uncertainty and conditions, the bracketing path (each round's opponent, result and refit), compares the result with MSH-139 and MSH-161, and updates docs/m5/strength-roadmap.md. An independent review checks the method and report. Verification level: measurement only, no production change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The frozen mShogi revision, binary hash, opponent engines (exact versions, sources, build hashes, options), CCRL snapshot URL and date, openings, time control, cloud machine type, CPU model and region, and actual compute (wall time, VM-hours, estimated cost) are recorded.
- [ ] #2 Games run in the cloud at 60 s + 0.6 s for both engines on one machine type, within the approved 6-hour wall-time and US$15 caps; every scored result is a native conclusion that passes native replay, failures or unscored games are reported, and cleanup of all cloud resources is verified.
- [ ] #3 The rating model, prior and stopping rule are recorded before play; opponents are chosen by the sequential bracketing and main-phase rules, and the report shows each bracketing round and every opponent change.
- [ ] #4 The report gives one combined estimate from a joint fit of all scored games with a 95% interval, per-family estimates with an agreement check, and a comparison with MSH-139 and MSH-161; it states that the index is not an official CCRL or human rating.
- [ ] #5 docs/m5/strength-roadmap.md records the result as the closing Chess checkpoint, and an independent review accepts the method and the report.
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
1. Inspect the native clock runner and existing GCP execution path; freeze main f433ea13 and a dated single-CPU CCRL candidate list. 2. Record the joint rating model, pair-aware uncertainty, prior, stopping rules, opening allocation and resource accounting before play; obtain independent method review. 3. Build pinned Linux engines and minimal measurement glue, verify offline and cloud smoke checks. 4. Run sequential eight-game bracketing rounds, then rotated main batches on at most eight c3-standard-4 workers, enforcing six hours and USD15 across all match work. 5. Validate native replay, joint and family estimates, failure/resource records and cleanup; review the report, update roadmap, integrate and push.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex chat 01a1e634-7fc9-72c2-b4b4-07022a7df3ec. Branch codex/chess-strength-anchor in the primary checkout. Startup: clean main f433ea13, no active Backlog owners, other relevant app chats idle or not loaded. Backlog 1.55.3 matches baseline. Measurement only; no production engine change. Linux cloud execution plus focused local harness checks and independent method/report review; broader platform checks are not applicable.

Pre-play checkpoint: method.md, opponent-candidates.md and frozen openings under docs/m5/evidence/msh185. Independent method, statistics, worker and batch reviews completed; identified native command-size and uniform-score precision defects were corrected before play. Actual Linux config/replay probe passed with nonexistent engine paths (zero games). First three opponents and candidate passed non-search UCI/Hash64 handshakes. Initial anchor Stash20.0.1=2511 selected before play for normal increment support; MrBob0.7=2505 ignores increment in its original time policy. GCP storage prefix empty, no VMs, C3 quota32 unused; existing Pub/Sub verified. Local Docker used only to build Linux binaries/images and run offline/non-search checks. Cloud match clock has not started.

First cloud bracket submitted as msh185-b01-v1 (UID msh185-b01-v1-9b664eb9-6f5a-4d1e-923d0) at 2026-11-16T21:29:58Z. Campaign clock and caps in docs/m5/evidence/msh185/campaign.json; deadline is fixed at start+21600s. Source checkpoint20d3471e; image digest6f050b230db16669df5df3aba93e94d5cad60ad8e35dfb49c7e1ffc7fddf73f1. Eight single-game tasks, Stash20.0.1, first four frozen opening pairs. Active local controller session13382 owns completion, retrieval and cleanup; output target/msh185/runs/b01, log target/msh185/batches/b01-controller.log. Durable raw store gs://mshogi-experiments-216675633801/msh185/b01. Do not duplicate submission. Startup status SCHEDULED, eight pending tasks, observed boot image batch-cos-stable-official-20261110-00-p01; no result yet.

b01 completed/reviewed:5W3D0L, all8native replays and73artifact hashes pass;5checkmates,2threefold draws,1dead-position draw; no flags/failures. All8C3 workers on IntelSapphireRapids/us-central1-a;0.4938234094VM-hours, estimatedUSD0.09955875, zero allocation failures. Job/VM/disk absence verified. Provisional joint2742.66, broad2384.43–3191.01 interval. Next nearest admitted built anchor Stash21=2713; b02 four fresh pairs submitted under same campaign clock and pool image8290f64d7051548c5e86f62929c8f00e1504ceb77836c474832756224016b627. Active controller session77601;target/msh185/runs/b02. Full opponent build evidence saved; Raven0.80 excluded before use for demonstrated upstream out-of-bounds write, no gameplay patch.
<!-- SECTION:NOTES:END -->
