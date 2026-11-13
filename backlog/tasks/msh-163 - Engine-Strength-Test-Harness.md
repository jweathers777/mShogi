---
id: MSH-163
title: Engine Strength Test Harness
status: Ready
assignee: []
created_date: '2026-11-13T21:28:00Z'
labels:
  - handle:engine-strength-test-harness
milestone: m-5
dependencies: []
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: high
ordinal: 167000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Engine Strength Test Harness.

**Problem:** Chess engine changes were judged on small position panels and by explanation. No quick test showed if a change made the engine stronger, so work moved to isolated cases.

**Objective:** Give each later engine change a quick, repeatable acceptance test.

**Approach:** Three parts. (1) A fixed speed benchmark: nodes per second and time to depth on a fixed position set. (2) A development blunder suite from the confirmed level-to-lost positions of MSH-161 (Competitive Gameplay Diagnosis), each with its full history and the reference alternative; these positions are exposed and are for development only. (3) A game test of a candidate against a frozen predecessor at a fast time control, with paired openings and a sequential stopping rule, on the existing experiment tools and cloud runner (see MSH-162 (Parallel Experiment Runner)). Do not build a second framework.

**Completion:** A recorded baseline for the current engine on all three parts, and one command per part that a later task can run.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The speed benchmark reports nodes per second and depth for the current Chess engine on a fixed, recorded position set
- [ ] #2 The blunder suite holds the MSH-161 confirmed level-to-lost positions with full histories and reference alternatives, and reports how many the engine avoids at a stated time
- [ ] #3 The game test plays candidate against predecessor with paired openings and a sequential stopping rule, and records games, result and actual compute
- [ ] #4 A baseline result for the current engine is recorded for all three parts
- [ ] #5 A self-test of the game test (current engine against itself) gives a result consistent with equal strength
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
