---
id: MSH-206
title: Tori Evaluation Round
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-21T18:12:00Z'
labels:
  - handle:tori-evaluation-round
  - tori
  - strength
milestone: m-5
dependencies:
  - MSH-205
references:
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 213000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Evaluation Round

**Problem.** The Tori evaluation is material plus placement with fixed values.

**Objective.** The evaluation step of the program for Tori.

**Approach.** As MSH-188 and MSH-196: linear terms with incremental static parts, fitted with the shared Texel tool on at least about 500,000 quiet self-play positions with a held-out split. MSH-196's lesson: test per-node terms separately from static terms.

**Completion.** A fitted evaluation that passes the sequential predecessor test, or a reported null result.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A corpus of at least about 500,000 quiet positions with a split; held-out loss before and after recorded.
- [ ] #2 The kept evaluation passes the sequential predecessor test (H1 about 10) or the null result is reported and reverted; independent review recorded.
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
1. Tunable Tori evaluation (tori/eval.rs): 148 weights; static terms (board and hand values, rank and column tables per face) incremental; per-node terms (Phoenix safety, mobility) pinned at INITIAL in the first fit; INITIAL = earlier evaluation (tested).
2. Shared compact-diagram codec in mshogi-core; Tori Texel model and tori_texel tool; shared loader accepts object position fields.
3. Corpus: msh206-corpus-1, 3,200 self-play pairs of main da3969d at 2 s + 0.05 s (c3-standard-4, 8 VMs); export every ply; split by opening.
4. fit1 (static terms), held-out check, SPRT H1 10 against da3969d at 5 s + 0.1 s; then fit2 with per-node terms against the fit1 result.
5. Checks, review, docs, merge.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint: branch claude/msh-206-tori-evaluation. Predecessor build da3969d0 (store gs://mshogi-experiments-216675633801/build-da3969d0, <=0.06 VM-h). Corpus image worker@sha256:5993056f (tag msh206-corpus-da3969d0). Campaign msh206-corpus-1 submitted (store gs://.../msh206). Eval code in progress (not committed).

2026-11-21 checkpoint (@claude):
- Corpus batch 1 msh206-corpus-1 (predecessor da3969d0, 3,200 pairs, 2 s + 0.05 s, 8 x c3-standard-4, 4,863 s, about 10.8 VM-h, about US$2.18, VMs verified deleted): 6,400 games (11 step-limit), 1,100,705 rows; split by opening: train 770,866 / validation 71,787 / held-out 180,027 rows; quiet: 295,498 / 28,570 / 70,887 = 394,955. Below 500k (38% quiet), so batch 2 msh206-corpus-2 (seeds 3201-6400, store msh206-corpus2) is running.
- Tool fixes (45e3f49): export loads reports in parallel (was 0.9 s per report on one core); corpus batches by first seed; Tori cloud cap set to the task cap US$10.
- Tori K range 0.01-0.71 as Shogi and Xiangqi (default 0.25-2.0 put K at its bound). Pilot static fit on batch 1: K 0.17, validation MSE 0.22178 -> 0.20876, 33 table entries at +-150. Pilot weights compiled in d86c874f for a pilot game test.
- Pilot build build-d86c874f-1 stayed queued 55 min (regional quota 8 VMs / 32 C3 CPUs used by the corpus job); no VM, no cost. Rerun after batch 2. Run one cloud job at a time.
- Next: when batch 2 ends, rebuild pilot; image with Cloud Build (not local Docker); pilot SPRT; export and split both batches; final static fit; held-out check; SPRT; then per-node fit.

2026-11-21 checkpoint 2 (@claude):
- Corpus batch 2 msh206-corpus-2 (seeds 3201-6400, 4,791 s on 8 x c3-standard-4, about 10.6 VM-h, about US$2.15, VMs verified deleted).
- Both batches (corpus-all): 12,800 games (26 step-limit), 2,195,634 rows; split by opening: train 1,519,748 / validation 143,991 / held-out 357,269 rows; quiet: 581,623 / 57,277 / 138,820 = 777,720 (AC1 size met).
- fit1 (static terms, per-node pinned at INITIAL; K 0.17; 900 s cap, stopped at validation patience after 12.8 s): validation MSE 0.21974 -> 0.20528; held-out (read once) 0.21259 -> 0.20474. 26 table entries at +-150. Compiled in ec1a9085.
- Pilot (batch-1 fit) game test not run: config time limit differed (refused before submission, no cost). Dropped in favor of fit1.
- Cloud Build API enabled in project mshogi (John approved 2026-11-21); images are built there. Images: worker@sha256:67b2571f (pilot, unused), worker@sha256:dd9b1e00 (fit1, tag msh206-fit1-ec1a9085).
- Running: msh206-fit1-match-1, fit1 vs da3969d0, 5 s + 0.1 s, 224 pairs, collect with --h1 10. Spend so far about US$4.7.

2026-11-21 results (@claude):
- Game test fit1 (ec1a9085) vs predecessor da3969d0: msh206-fit1-match-1, 5 s + 0.1 s, 224 pairs, 8 x c3-standard-4, 619 s, <=US$0.33. Pentanomial [5,0,74,0,145], +255 Elo (218-298). Sequential SPRT H0 0 / H1 10: H1 at pair 41. Late-game depth median 9, p10 8.
- fit2 (per-node terms fitted, PER_NODE_FITTED = true): validation 0.21974 -> 0.20520 (fit1 0.20528); held-out (read once) 0.21259 -> 0.20436 (fit1 0.20474). Phoenix adjacent attacked -64, near enemy -24, drop threat 0; mobility Eagle 30, Left Quail 16, Right Quail 30; tempo 8.
- Game test fit2 (4ca61fd9) vs fit1 (ec1a9085): msh206-fit2-match-1 (seeds 1-224, 741 s, <=US$0.38): [36,0,106,0,80], +70 (37-104), sequential undecided (LLR 2.44 < 2.94; Tori has no draws, so the pair variance is high). Continuation msh206-fit2-match-2 (seeds 225-448, 715 s, <=US$0.38): [22,0,100,0,102] +130. Sequential test over both in seed order: H1 at pair 238 (LLR 2.96). Combined [58,0,206,0,182], +99 Elo (76-123), 446 scored pairs (2 length-limited). Kept fit2. Unlike Xiangqi (MSH-196), the Tori per-node terms win.
- Collect fix: the sequential test skipped no unscored pair and failed on a length-limited pair; it now skips them (tori_cloud.sequential, unit test).
- Test changes: the search and protocol choice tests take a Crane (not a Swallow) so that the capture is clearly best, and compare the score with the evaluation after it (exact). The legacy-observation test keeps the Swallow position. tori_selfplay depth-1 budget 256 -> 1,024 nodes (quiescence needs more). Frozen fixed search: Tori 4g-3f Phoenix, 358 nodes, score 8.
- Shared core fix (alphabeta/mod.rs): when a root mate is proven and the alpha-beta mate is kept (not longer), the result is Complete. With fit2, the MSH-205 mate test found the mate in 9 by alpha-beta and returned NodeLimit.
- Compute: Batch conservative bounds US$6.05 in all (corpus 2.24 + 2.20, four builds 4 x 0.13, games 0.33 + 0.38 + 0.38), plus Cloud Build (4 builds, about 15 s each). VMs verified deleted after every job.
- Data: gs://mshogi-experiments-216675633801/runs/msh-206/ (fits, held-out checks, match summaries and manifests, split manifest, train/validation/held-out corpus gzipped).
- Follow-up: MSH-220 (Tori Exchange Values from the Fitted Evaluation).
<!-- SECTION:NOTES:END -->
