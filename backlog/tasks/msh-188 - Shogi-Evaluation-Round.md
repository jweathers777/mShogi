---
id: MSH-188
title: Shogi Evaluation Round
status: Ready
assignee: []
created_date: '2026-11-17T08:58:00Z'
labels:
  - handle:shogi-evaluation-round
milestone: m-5
dependencies:
  - MSH-187
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
Choose one bounded evaluation set after inspecting the completed MSH-187 evidence. Establish a useful native-validated corpus and separate training from held-out games and openings before Texel tuning. Native rules remain authoritative. Record baseline, intended benefit, comparison method, resource cap and checkpoint before implementation or data generation. Use the existing harness and cloud path for long work. Preserve the rejected quiet-check and full-exchange experiments; do not restore them without new evidence. Neural evaluation, external rating studies and substantial-cost campaigns remain outside standing authorization.

## Completion
Retained changes pass native perft and differential checks, the development tactical suite and a winning sequential fast game test against the frozen task-start predecessor. Record held-out decision evidence, depth and speed tradeoffs, actual compute, independent review and limitations. Finish one coherent set without an open-ended tuning campaign.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Before work, record a frozen baseline, intended benefit, corpus provenance and held-out split, comparison method, resource cap and decision checkpoint.
- [ ] #2 Retained changes pass Shogi native perft and differential checks and the development tactical suite; shared consumers remain correct.
- [ ] #3 Record held-out decision results and depth and speed tradeoffs; retain changes only after a winning sequential clock-mode game test against the frozen predecessor.
- [ ] #4 Record actual compute, failed attempts, independent review, limitations and current conventional-engine documentation.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
