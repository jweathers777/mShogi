---
id: MSH-53.4
title: Tenjiku Time-Odds Scaling
status: Done
assignee:
  - '@codex'
created_date: '2026-11-28T10:02:00Z'
updated_date: '2026-11-28T15:20:00Z'
labels:
  - tenjiku
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies:
  - MSH-285
  - MSH-286
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/strength-harness.md
parent_task_id: MSH-53
priority: high
ordinal: 264000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replaces the open scaling criterion of MSH-53.3, whose fixed-node ladder and shared-wall clock pilot gave no estimate. Measure Tenjiku scaling the same way as the accepted Tori and Chu baselines: the same accepted engine at 2x against 1x clock, both colors, for Adams and Muller. Estimate game length from the existing MSH-53.2 reports first; no extra pilot is needed unless that data is missing.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Before play, a short feasibility check from earlier data shows that each game can finish within its clock, ply, wall and job limits, and gives the projected cost
- [x] #2 120+3 vs 60+1.5 (optionally also 60+1.5 vs 30+0.75), 24 to 32 opening pairs per profile, fixed maximum; report W/D/L, unscored games with reasons, pentanomial Elo estimate with 95% interval per profile
- [x] #3 Cost (inside the US$10 cap), commands, revisions and GCS path recorded; roadmap Tenjiku row updated; baseline decision left to John
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Play the same current accepted Tenjiku engine on both sides at 120+3 vs 60+1.5. Use 32 consecutive openings (1–32) from each checked-in MSH-214 catalog and both colors. Run the shared match and Batch tools on c3-standard-4. Use fixed samples, score every verified native conclusion, and compute each profile’s pentanomial estimate and approximate 95% interval from complete pairs. Verify native replays and cloud cleanup, then update the roadmap for John’s decision.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Feasibility before play. In 80 MSH-53.2 equal-clock games per profile, 77 Adams and 74 Muller concluded. Concluded plies (median/max) were 106/535 and 95/367. MSH-285 saw a Muller game still in play at 737 plies, so allow 1,024. MSH-53.2 used up to 730,368 nodes per turn. The symmetric 5,000,000-node safety limit leaves headroom at 2x time, and MSH-285 returns a legal move at depth zero. At 1,024 plies, the two clocks provide up to 2,484 s. The independent game wall is 2,732.4 s with a 60 s move watchdog. Four games per c3-standard-4 and eight pairs per bin need 11,891 s per bin, 11,981 s Batch task runtime and a 12,401 s campaign deadline. Eight VMs at $0.201608/h reserve $5.56. Allow $0.21 for the Linux build and $1 for Cloud Build, storage and transfer, for a projected total below $6.77. The prior reports supply the feasibility sample.

Linux build 747e7db uses the accepted 0bbfd9c evaluation plus MSH-285 and MSH-286. Built `mshogi`, `mshogi-match` and `strength` on one c3-standard-4 with `python3 -m tools.strength.cloud_build pack` and `submit`. VM cost bound $0.017. The matching worker image was built with `gcloud builds submit --tag us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker:msh534-747e7db` and run by digest.

Match strength-msh534-time-odds used the same 747e7db binary on both sides. All 32 opening pairs per profile ran at 120+3 vs 60+1.5. Adams 46W/0D/16L and two unscored at 1,024 plies (opening 16 first and 21 second). Its 30 complete pairs gave [1,0,13,0,16], +191 Elo (approximate 95% interval +106 to +305). Muller 47W/0D/17L and zero unscored. Its 32 complete pairs gave [0,0,17,0,15], +177 Elo (+104 to +267). The estimates use complete pairs. All 128 native reports replayed and matched the inputs, clocks, colors, hashes and scores. Neither profile hit the game wall, and the largest single search used 1,567,232 nodes. The match VM bound is $2.407. The combined VM bound is $2.424. With a $1 allowance for Cloud Build, storage and transfer, the total estimate is below $3.43. Both jobs, all VMs and disks, the task image, and the build staging object were removed.

Reproduce with `python3 -m tools.strength.cli match --game tenjiku --profile PROFILE --candidate ./mshogi --baseline ./mshogi --runner ./mshogi-match --helper ./strength --openings CATALOG --base 60 --increment 1.5 --candidate-time-factor 2 --first-seed FIRST --max-pairs 8 --max-steps 1024 --out DIR` for each profile and FIRST=1,9,17,25, with the default four concurrent games per c3-standard-4. Submit with `python3 -m tools.strength.cli cloud-submit --manifest M --config C --image IMAGE@sha256:DIGEST --job strength-msh534-time-odds --out DIR`. Exact manifests, binaries, all reports and the analysis are in gs://mshogi-experiments-216675633801/runs/msh-53.4/. The operator archive is `operator/operator-final.tar.gz`.

Handoff from codex/msh-53.4-tenjiku-time-odds. Results and limits are in the roadmap and this task. Next is John’s Tenjiku baseline decision in MSH-53.

Checks passed. All 57 strength-driver tests, native replay and evidence verification, shared changed-path checks, Markdown links and Backlog doctor passed. The task and roadmap changes selected no product suites.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Measured 32 same-engine opening pairs per Tenjiku profile at 120+3 vs 60+1.5. Doubling the clock gained +191 Elo in Adams (95% interval +106 to +305) and +177 in Muller (+104 to +267). Two Adams games reached the ply limit. All 128 reports replayed, cloud cleanup passed, and the estimated cost stayed below $3.43. The next task is John’s baseline decision in MSH-53.
<!-- SECTION:FINAL_SUMMARY:END -->
