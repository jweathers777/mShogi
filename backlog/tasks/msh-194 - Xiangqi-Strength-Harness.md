---
id: MSH-194
title: Xiangqi Strength Harness
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-18T19:25:00Z'
updated_date: '2026-11-18T20:31:00Z'
labels:
  - handle:xiangqi-strength-harness
  - xiangqi
  - strength
milestone: m-5
dependencies: []
references:
  - docs/m5/strength-roadmap.md
ordinal: 201000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Strength Harness

**Problem.** Xiangqi runs on the shared search core (MSH-181) with a basic evaluation (material, placement, tempo). It has no measurement harness of the Shogi standard: the native Xiangqi runner is untimed, starts a fresh engine process per turn and accepts at most four openings. Without a harness, later rounds cannot be measured.

**Objective.** Give Xiangqi the harness step of the per-game classical strength program (docs/m5/strength-roadmap.md).

**Approach.** Reuse the Shogi harness pattern (MSH-186, MSH-190, the clocked shogi-match route of MSH-191) and the shared tools. Adjust for Xiangqi rules (WXF repetition, chase and count claims stay native).

**Completion.** The criteria below are met with recorded evidence; the opponent and judge proposal is ready for John.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 At least 100 balanced Xiangqi openings, with their source and a balance check, are in tools/strength/data.
- [ ] #2 A tactical suite of at least 200 positions with independently established answers (for example native-verified forced mates) and a held-out part exists, with the baseline score recorded.
- [ ] #3 A clocked, persistent-process Xiangqi match route plays mShogi against its frozen predecessor and an external UCI engine, admits every reply natively, and replays recorded games natively; tests cover it.
- [ ] #4 A predecessor game-test control is chosen at which the median late-game depth is at least 8, with the measurement recorded.
- [ ] #5 Self-play export of quiet positions with game results for Texel tuning works, with a split by game or opening.
- [ ] #6 A proposed external opponent pool and a stronger judge engine (builds, licences, limits, any rating reference) are recorded for John's approval; no external study runs in this task.
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
1. Clocked persistent Xiangqi match route in mshogi-runner (xiangqi-match / xiangqi-match-replay), modelled on shogi_match.rs, UCI Pikafish 0.9 dialect, controller-owned Fischer clocks, native admission and replay; tests.
2. Opening catalog: original transcriptions of conventional Xiangqi opening trunks with development extensions, native validation and a fixed engine balance screen; at least 100.
3. Tactical suite: forced mates mined from self-play reports and proved by a native reference prover (checking-move AND/OR search), answer set = all shortest-mate first moves, split by source game; baseline score.
4. Self-play/export example for depth measurement and Texel rows; choose the predecessor control with median late-game depth >= 8.
5. Opponent and judge proposal (Pikafish, Fairy-Stockfish classical, ElephantEye) with builds, licences, limits and rating-reference limits for John.
6. Docs, evidence, review, integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Progress 2026-11-18 (branch codex/msh-194-xiangqi-harness, commits a9bcb915, c97fde2b, b3a0442b). Verification level 1 (local macOS arm64) plus protocol/process change in runner and console (targeted). Report: docs/m5/xiangqi-strength-harness.md; evidence docs/m5/evidence/msh194.
- Found and fixed: Xiangqi UCI had no clocks; per-request overhead grew ~2.5 ms/ply (native submit copies history, 3-4 replays per request). Added native Game::fork, session event-prefix reuse, incremental position, timed fallback move. Overhead now flat ~3 ms.
- Clocked route xiangqi-match/xiangqi-match-replay with shared runner::clocked host (Shogi route refactored onto it; its tests pass).
- Openings: 112 admitted (depth 11, +-150), 12 families.
- Tactical suite: 373 positions (91 held-out), complete native answer sets; baseline 373/373 at 200 ms: saturated, regression guard only.
- Control 5 s + 0.1 s: late-game fresh depth median 10 at 100 ms; 1,120 games all native conclusions.
- Export + texel split for xiangqi verified.
- Finding: 47/1,120 games lost by perpetual chase (search scores chase cycles as draw).
- Opponent/judge proposal in report; no external games run.
<!-- SECTION:NOTES:END -->
