---
id: MSH-185
title: Chess Strength Anchor
status: Done
assignee:
  - '@codex'
created_date: '2026-11-16T20:50:00Z'
updated_date: '2026-11-17T00:18:00Z'
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
- [x] #1 The frozen mShogi revision, binary hash, opponent engines (exact versions, sources, build hashes, options), CCRL snapshot URL and date, openings, time control, cloud machine type, CPU model and region, and actual compute (wall time, VM-hours, estimated cost) are recorded.
- [x] #2 Games run in the cloud at 60 s + 0.6 s for both engines on one machine type, within the approved 6-hour wall-time and US$15 caps; every scored result is a native conclusion that passes native replay, failures or unscored games are reported, and cleanup of all cloud resources is verified.
- [x] #3 The rating model, prior and stopping rule are recorded before play; opponents are chosen by the sequential bracketing and main-phase rules, and the report shows each bracketing round and every opponent change.
- [x] #4 The report gives one combined estimate from a joint fit of all scored games with a 95% interval, per-family estimates with an agreement check, and a comparison with MSH-139 and MSH-161; it states that the index is not an official CCRL or human rating.
- [x] #5 docs/m5/strength-roadmap.md records the result as the closing Chess checkpoint, and an independent review accepts the method and the report.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
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

b02:4W4D0L againstStash21=2713; all16cumulative games replayed, joint2834.42 with wide2564.39–3140.01 interval. Native longest game413searched plies/359.2s ended checkmate. Controller returned cleanup failure only because job record deletion outlasted short polling window; VMs/disks already absent. Later exact-job cleanup proof verifies allabsent; original failure preserved, no rerun. Actual b02 .5898206584VMh/USD.1189125633, cumulativeUSD.2184713133. Reviewed controller-only180s bounded cleanup/reconciliation patch passed33tests. b03 now running Weiss0.10=2846 (nearest admitted anchor), fresh pairs8–11, session27657, same original deadline and pool image; target/msh185/runs/b03. See b02/closure.md.

Current handoff: b03 finished 3W/1D/4L; b04 finished 0W/1D/7L, both against Weiss0.10 (2846). All 32 games pass native replay; b03/b04 cleanup and lifecycle records pass, with no allocation failures. Cumulative estimated compute is USD0.4410465867. Joint estimate2741.95; interval2555.31–2919.15 remains wide. Both Stash and Weiss results are retained; provisional family disagreement is visible. Fifth bracket msh185-b05-v1 is running against nearest admitted Stash21.0 (2713), fresh pairs16–19. Controller session89859 owns cleanup; target/msh185/runs/b05, raw gs://mshogi-experiments-216675633801/msh185/b05. Same campaign deadline, same pool image and worker/native/statistics sources. Do not repeat prior rounds. b03 compact evidence and independent b02-b03 review are saved; b04 compact packaging remains.

Bracketing ended after seven rounds and about56minutes, before another round would exceed the one-hour phase bound. All56games are retained and native-replay valid; no game failures, flags, caps or missing scores. Joint2728.68 with2592.57–2860.37 interval; the +/-100 guide was not reached. All prior jobs/VMs/disks are absent. Actual cumulative computeUSD0.7382314755; zero allocation failures. Main selection is Stash21.0(2713), Weiss0.9(2649), MrBob0.8.0(2638), all within91points, from the deployed admitted pool. First main batch msh185-b08-v1 is active:24fresh pairs/48games, eight workers, pairs28–51, controller session3806, target/msh185/runs/b08 and gs://mshogi-experiments-216675633801/msh185/b08. Same frozen source, image, clocks and original campaign deadline. Decision and forecast in main-selection.json; no precision claim yet.

First main batch b08 stopped on eight MrBob0.8.0 SIGSEGV failures during bestmove; these are unscored. Ten valid games (7W1D2L) are retained;30scheduled games never started. All66cumulative scored games pass native replay, joint2736.92 [2658.99,2814.85]. All b08 cloud resources are verified absent; actual b08 .8363486983VMh/USD.1686145884, cumulativeUSD.9068460638. Independent failure review is in progress. Replace failed MrBob with prebuilt full-strength Supernova2.3=2644 from the frozen source pool, within93points, without patching rated source. New image adds this binary only; same candidate/runner/worker, original campaign deadline. No games rerun or valid results discarded.

Correction from independent native-score review: b08 valid tally is6W1D3L (not7W1D2L in the prior note); fitted results were already computed from correct native scores. Eight MrBob failures are explained by its FEN handler retaining a fixed1024entry move-history stack across full-history commands; source is unchanged and release is withdrawn. Supernova2.3 replacement clears history on each position command; independent source/options review accepted admission. b09 is active with24fresh pairs52–75, Stash21/Weiss0.9/Supernova2.3, new image72b4a8072b2d3b1db36b21183f2f51476108d408256f531b141dcfa8d37959ee; controller session66282, target/msh185/runs/b09. Original deadline/caps continue.

b09 stopped on a measurement-classification defect: Supernova2.3 reached eight native conclusions but its normal quit handler sleeps1s; frozen runner waits100ms then intentionally kills/reaps. No native failure, diagnostic failure or cleanup failure occurred. Independent method review accepts a narrow exact-binary normal-postconclusion-termination correction without changing candidate, runner, clocks or game rules. Original receipts remain immutable; derived reclassification must bind report/receipt hashes and satisfy all replay, terminal-quit and process-settlement checks. Eight results5W1D2L are pending corrected-audit review;10other validgames5W3D2L retained,30assignmentsnotstarted. All b09 resources absent;1.0191979103VMh/USD.2054784523, totalUSD1.1123245161. No new cloudjob active while correction/tests/review complete.

Shutdown classification correction independently accepted;96strengthtests pass. All8Supernova native results5W1D2L admitted uniformly, with original rejectedreceipts and originalfit preserved plus hashbound deriveddecisions. Cumulative84validgames/42pairs, joint2746.40 [2678.89,2813.90]; eightMrBobcrashes remain unscored,60assignments neverstarted. No game reruns. b10 is active,24freshpairs76–99, same3families; session79501,target/msh185/runs/b10. Corrected worker image60163e4fb2f8f88a28320614b7c8106a3de9c37c5ba3cff5f20d00f64eb3e0cb; candidate/native runner/opponents unchanged. Original campaigncaps continue.

b10completed48/48valid with correctedworker; no new process failures, flags, caps or missing results. All132cumulative games native-replay valid; joint2742.23 [2690.41,2794.05], familyspread10.44. Maincoverage12Stash/14Weiss/12Supernovapairs, so precision/coverage stop not met. Actualb10 2.4770080343VMh/USD.4993846358; cumulativeUSD1.6117091519, all b10resourcesabsent. b11active with24freshpairs100–123, unchanged3families/image/engine, session11365,target/msh185/runs/b11. Expectedanother20–25min; campaign stillwellwithin6h/USD15. Include b09 explicit shutdownreclassification file in every laterfit.

b11 completed48/48valid, no new failures. Cumulative180validgames/90pairs, joint2752.62 [2711.45,2793.79], familyspread26.48. Maincoverage20Stash/22Weiss/20Supernovapairs meetsminimum; halfwidth41.17 remains above fixed40, so nextnormal24pairbatch required. Actualb11 2.6367224205VMh/USD.5315843337; cumulativeUSD2.1432934857, allresourcesabsent. b12 active on freshpairs124–147, unchangedthreefamilies and image, session40931,target/msh185/runs/b12. Original campaigndeadline continues; no reruns or extra study.

Match work stopped after b12:228scoredgames/114completepairs,98W76D54L. Final joint2750.5107 [2714.8730,2786.1484], halfwidth35.6377<=40; maincoverage28Stash/30Weiss/28Supernova pairs. Familyspread35.8284<150. No further games. All236launched attempts retained:228valid,8MrBobcrashes;60additional assignments neverstarted; no reruns/singletons. All12batches independently audited, native replay and hashes pass. Final resource totals13.1838928959VMh, estimatedUSD2.657978279,9571.9593seconds through finalbatchcleanup, within originalcaps. Final live check confirms allstudyjobs/VMs/disks absent. See final-summary.json, b12-fit.json and final-cleanup.json. Report/roadmap prepared; independent finalreview and integration remain.

Independent final review accepted source/report checkpoint1a954afe. Reproduction matches b12-fit byte for byte; separate native conclusion/color checks confirm98W76D54L. Recomputed VM lifecycle totals and all stopping/coverage/failure/cleanup records pass. No material findings remain. Current worker/collector/test source hashes still match the96-test checkpoint. Actual controller environment macOS27.0.1 arm64/Python3.11.0; actual cloud Linux x86-64. No Windows/GUI/release compatibility claim. Final integration into main remains before Done.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Measured frozen main f433ea13 against three independent, full-strength engine families at 60 s + 0.6 s in Google Cloud. The final conditional index is 2750.51, with approximate 95% interval 2714.87–2786.15, from 228 native-replayed games in 114 pairs (98 wins, 76 draws, 54 losses). The fixed precision and family coverage stops were met; family estimates differ by 35.83 points. This is not an official CCRL or human rating, or a measured Elo gain over older studies.

All eight opponent crashes and 60 unstarted assignments are recorded. A narrow measurement correction admits normal post-conclusion Supernova termination; independent review accepted it, and original receipts remain unchanged. No engine or game-rule change was made. The corrected source passed 96 strength tests; 28 earlier GCP checks passed. Final independent review accepted checkpoint 1a954afe and reproduced the fit byte for byte. Actual verification covered macOS control and Linux cloud play, not Windows, GUI or release compatibility.

The campaign used 13.183893 VM-hours, estimated US$2.657978 compute and 2 h 39 min 32 s through final cleanup. All study jobs, VMs and disks are absent. Stored evidence and images are retained. Report: docs/m5/chess-strength-anchor.md; durable evidence: docs/m5/evidence/msh185/README.md. Roadmap updated. Reviewed branch tip 92d1f475 was integrated into main by merge 22c4903105dd71edb37f496241ae823cac3c5d05. No blocker remains. Next eligible project-priority task: MSH-186 (Shogi Strength Harness).
<!-- SECTION:FINAL_SUMMARY:END -->
