---
id: MSH-206
title: Tori Evaluation Round
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-21T10:53:00Z'
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
<!-- SECTION:NOTES:END -->
