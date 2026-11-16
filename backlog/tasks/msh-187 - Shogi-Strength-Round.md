---
id: MSH-187
title: Shogi Strength Round
status: Ready
assignee: []
created_date: '2026-11-16T20:51:00Z'
labels:
  - handle:shogi-strength-round
milestone: m-5
dependencies:
  - MSH-186
documentation:
  - docs/m5/shared-search-core.md
  - docs/m5/strength-roadmap.md
  - docs/m5/strength-harness.md
  - docs/m5/evidence/msh180/README.md
priority: high
ordinal: 194000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Strength Round

## Problem
Shogi uses the shared search core since MSH-180 (Shogi Search Core Adoption), but its engine is still basic. On ordinary roots the median depth is 7 at 20,000 nodes. Quiescence has no check stage, although quiet checking drops often decide Shogi games. The exchange test is a one-step estimate. The evaluation is material with Pawn advance, adjacent king cover, hand flexibility and tempo; it is not tuned. The core margins (futility, razoring, aspiration, quiescence) assume Chess piece values. Desktop play and default USI use a node limit (300,000) instead of a time budget. See docs/m5/shared-search-core.md, "Shogi on the core" and "What Tori showed".

## Objective
A substantially stronger conventional Shogi engine, built by the same classical method that worked for Chess: speed and depth first, then selectivity and time, then a tuned evaluation. This prepares MSH-50 (Conventional Shogi) acceptance criterion 5 (a substantially complete conventional engine before the major strength study).

## Approach
- Follow AGENTS.md "Change direction when evidence shows little progress" and docs/m5/strength-roadmap.md "Game tests in this direction". Before each coherent set, record baseline, intended benefit, comparison method, resource limit and decision checkpoint. Freeze main at task start as the predecessor.
- Measure first with the MSH-186 (Shogi Strength Harness) tools: profile the search path (position updates, move and drop generation, cache identity and size) and depth at time.
- Candidate coherent sets (established techniques; the worker chooses order and detail after measurement):
  - Search: checking moves and drops at the first quiescence ply; a full exchange evaluation with attacker lists; margins scaled for Shogi piece values; move ordering for drops and promotions; review of reductions and null move for drop games (zugzwang is rare, but hand pieces change tactics).
  - Time: time-based default for desktop play and USI, using the MSH-179 (Clock and Session for All Games) session policy, instead of the fixed node limit.
  - Evaluation: Texel-tuned handwritten terms with Shogi meaning, for example piece and hand values (including value of pieces in hand by count), promoted pieces, king safety against attackers and possible drops near the king, castle shapes, piece-square terms and mobility. Neural learned evaluation is out of scope.
- Transfer techniques, not Chess constants. Native rules stay the authority for real moves, history, declarations and results; private analysis may approximate (design A-03).
- Shared-core changes must keep Chess, Tori, Xiangqi, Chu and Tenjiku tests passing. If a core change alters Chess search, show identical fixed-depth Chess output or run a Chess predecessor game test.
- If the work becomes too large for one task, finish a coherent set, record the result, and create a follow-up task for the next set.

## Completion
Each retained set passes Shogi perft and differential tests against the native rules, the tactical suite, and a sequential clock-mode game test against the frozen predecessor. The task records depth, tactical-suite and game-test results before and after, independent review, and an updated docs/m5/shared-search-core.md. Verification level 1, with level 2 checks only where the time-default change affects desktop or USI behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Before each coherent set, the task records baseline, intended benefit, comparison method, resource limit and decision checkpoint; each set is retained or rejected on that evidence.
- [ ] #2 Retained changes pass Shogi perft and randomized differential tests against the native rules, and all other games' search tests still pass.
- [ ] #3 Retained changes improve the Shogi tactical suite or depth at time, and win a sequential clock-mode game test against the frozen task-start predecessor.
- [ ] #4 Desktop play and default USI use a time budget through the shared session policy, with the affected desktop and USI checks recorded.
- [ ] #5 Results before and after, decisions, limitations and independent review are recorded, and docs/m5/shared-search-core.md is current.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
