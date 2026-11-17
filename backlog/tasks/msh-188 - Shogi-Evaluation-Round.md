---
id: MSH-188
title: Shogi Evaluation Round
status: Ready
assignee: []
created_date: '2026-11-17T08:58:00Z'
updated_date: '2026-11-17T16:42:00Z'
labels:
  - handle:shogi-evaluation-round
milestone: m-5
dependencies:
  - MSH-187
  - MSH-189
  - MSH-190
documentation:
  - docs/m5/shogi-strength-round.md
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 195000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Evaluation Round

## Problem
MSH-187 (Shogi Strength Round) improves search speed, exact history matching and timed defaults. The handwritten Shogi evaluation remains basic and untuned. The small data generation demonstration is not an independent training or validation corpus.

## Objective
Improve conventional Shogi decisions with classical evaluation and measured benefit against a frozen predecessor. Support MSH-50 (Conventional Shogi) without claiming an external rating.

## Approach
Choose one bounded evaluation set after inspecting the completed MSH-187 evidence. Establish a useful native-validated corpus and separate training from held-out games and openings before Texel tuning. Native rules remain authoritative. Record baseline, intended benefit, comparison method, resource cap and checkpoint before implementation or data generation. Use the existing harness and cloud path for long work. Neural evaluation, external rating studies and substantial-cost campaigns remain outside standing authorization.

## Completion
Retained changes pass native perft and differential checks, the development tactical suite and a winning sequential fast game test against the frozen task-start predecessor. Record held-out decision evidence, depth and speed tradeoffs, actual compute, independent review and limitations. Finish one coherent set without an open-ended tuning campaign.

## Additions (John, 2026-11-17)
- Order: start after MSH-189 (Shogi Request Overhead) and MSH-190 (Shogi Selectivity Round). MSH-190 owns the quiescence check stage, full SEE, margins and drop ordering; this task does not reopen them.
- Corpus: the MSH-186 export (497 positions) is a demonstration only. Generate at least about 500,000 labeled quiet positions from self-play games of the current engine (cloud, under the existing path and caps), from many openings, with game outcome labels. Split by game, not by position: about 90% training, 10% validation, plus held-out games from openings not used in training. Record generation conditions, size, duplicates and label balance.
- Terms: Shogi-meaningful handwritten terms (piece and promoted-piece values, hand-piece values by count, king safety against attackers and possible drops near the king, castle shapes, piece-square terms, mobility). Tune weights jointly by Texel regression; report validation loss before and after.
- Tests: reuse the MSH-190 opening set (at least 100 openings), tactical suite and game-test control (median late-game depth at least 8). Use H1 about 10 Elo for evaluation changes (docs/m5/strength-roadmap.md).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Before work, record a frozen baseline, intended benefit, corpus provenance and held-out split, comparison method, resource cap and decision checkpoint.
- [ ] #2 A labeled corpus of at least about 500,000 quiet positions from self-play exists with a game-level training/validation split and held-out games from unused openings; its generation conditions and statistics are recorded.
- [ ] #3 Retained changes pass Shogi native perft and differential checks and the development tactical suite; shared consumers remain correct.
- [ ] #4 Record validation loss, held-out decision results and depth and speed tradeoffs; retain changes only after a winning sequential game test against the frozen predecessor using the MSH-190 opening set and control.
- [ ] #5 Record actual compute, failed attempts, independent review, limitations and current conventional-engine documentation.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
