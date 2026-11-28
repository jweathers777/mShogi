---
id: MSH-53.4
title: Tenjiku Time-Odds Scaling
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:02:00Z'
updated_date: '2026-11-28T13:48:00Z'
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
- [ ] #2 120+3 vs 60+1.5 (optionally also 60+1.5 vs 30+0.75), 24 to 32 opening pairs per profile, fixed maximum; report W/D/L, unscored games with reasons, pentanomial Elo estimate with 95% interval per profile
- [ ] #3 Cost (inside the US$10 cap), commands, revisions and GCS path recorded; roadmap Tenjiku row updated; baseline decision left to John
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Play the same current accepted Tenjiku engine on both sides at 120+3 vs 60+1.5. Use 32 consecutive openings (1–32) from each checked-in MSH-214 catalog and both colors. Run the shared match and Batch tools on c3-standard-4. Use fixed samples, score every verified native conclusion, and compute each profile’s pentanomial estimate and approximate 95% interval from complete pairs. Verify native replays and cloud cleanup, then update the roadmap for John’s decision.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Feasibility before play. In 80 MSH-53.2 equal-clock games per profile, 77 Adams and 74 Muller concluded. Concluded plies (median/max) were 106/535 and 95/367. MSH-285 saw a Muller game still in play at 737 plies, so allow 1,024. MSH-53.2 used up to 730,368 nodes per turn; the symmetric clock-mode limit of 5,000,000 leaves headroom at 2x time, and MSH-285 returns a legal move even at depth zero. The clock allows at most 2,484 s charged at 1,024 plies. MSH-286 gives each game 2,732.4 s wall, with a 60 s move watchdog. Four games per c3-standard-4 and eight pairs per bin give four waves, 11,891 s per bin, 11,981 s Batch task runtime and 12,401 s campaign deadline including provisioning and cleanup. Eight VMs at $0.201608/h reserve $5.56. Allow $0.21 for the Linux build and $1 for Cloud Build, storage and transfer, or under $6.77 total. Prior reports supply the feasibility sample.

Checkpoint on codex/msh-53.4-tenjiku-time-odds. Claimed on main and reviewed MSH-53.2, MSH-53.3, MSH-285 and MSH-286. Next build the accepted source with the two dependency fixes and launch the fixed sample.

Campaign checkpoint on codex/msh-53.4-tenjiku-time-odds. The Linux build of 747e7db completed on c3-standard-4 and cleanup verified no build job, VM or disk ($0.0166 compute bound). The fixed sample is running as strength-msh534-time-odds, with the pinned worker and inputs in gs://mshogi-experiments-216675633801/runs/msh-53.4/. All 57 strength driver tests passed. Next verify all 128 game outcomes, paired estimates and resource cleanup, then integrate the roadmap update.
<!-- SECTION:NOTES:END -->
