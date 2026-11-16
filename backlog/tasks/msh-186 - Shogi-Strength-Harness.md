---
id: MSH-186
title: Shogi Strength Harness
status: Ready
assignee: []
created_date: '2026-11-16T20:51:00Z'
labels:
  - handle:shogi-strength-harness
milestone: m-5
dependencies:
  - MSH-185
documentation:
  - docs/m5/strength-harness.md
  - docs/m5/strength-roadmap.md
  - docs/m5/shared-search-core.md
  - docs/m5/experiments.md
  - docs/m5/evidence/msh180/README.md
priority: high
ordinal: 193000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Strength Harness

## Problem
Chess progress came from a measurement harness: a speed benchmark, a development blunder suite, curated openings and a sequential game test against the frozen predecessor (MSH-163 (Engine Strength Test Harness), docs/m5/strength-harness.md). Shogi has only part of this. The Shogi game test (`tools/strength/shogi_match.py` through `native_match.py`, used by MSH-180 (Shogi Search Core Adoption)) uses fixed node limits and seeded random short openings. Shogi has no benchmark positions, no tactical or blunder suite, no curated opening set, no clock-mode game test and no labeled positions for Texel tuning. It also has no approved independent opponent pool, which MSH-50 (Conventional Shogi) needs before any strength verdict.

## Objective
Give Shogi the same development measurements that Chess has, so that MSH-187 (Shogi Strength Round) can accept or reject changes on evidence. Propose an external opponent pool for John's decision.

## Approach
- Extend the shared tools in `tools/strength` for Shogi. Do not build a second framework.
- Benchmark: fixed SFEN positions (opening, middlegame, endgame, positions with pieces in hand) with fixed depths, reporting nodes per second and time to depth.
- Tactical suite: positions with independently established answers. Good sources are published tsume (mate) problems with known solutions and checked positions from real games. Record source and license for each set. Keep exposed development positions separate from any later held-out set.
- Openings: a balanced opening set from established opening lines or professional game prefixes, with source and license recorded, used in pairs with colors reversed.
- Game test: add a clock mode (base + increment) for Shogi, as for Chess, using the clock and session support of MSH-179 (Clock and Session for All Games). Keep the sequential stopping rule of the Chess harness.
- Texel data: a reproducible way to make labeled Shogi positions (for example from self-play games of the current engine at a fast control), so that MSH-187 can tune evaluation weights.
- Record the baseline of current main with all of these tools.
- Opponent pool proposal (no rating games in this task): research USI engines that can play the accepted profile `mshogi.shogi.csa36.v1` (fourth-occurrence repetition, perpetual-check loss, entering-king declaration). Candidates include Fairy-Stockfish and YaneuraOu (also at reduced settings, for example classical evaluation or node limits) and other open engines. For each: license, version, source, profile compatibility, settings, approximate strength and fit with the existing USI adapter (`experiment-external usi`). Present a short recommendation with a bounded baseline protocol and budget for John's approval. External engines stay measurement tools only, never rules oracles or build/CI dependencies.

## Completion
Tools, data, baseline numbers and the opponent proposal are documented in docs/m5 (link from docs/m5/strength-harness.md and docs/m5/strength-roadmap.md). Independent review covers the tools and the data sources. Verification level 1 (tool and test changes only; no production rule or search change).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Shogi benchmark, tactical suite and opening set exist in tools/strength/data with recorded sources and licenses, and the strength CLI runs each one for Shogi.
- [ ] #2 A Shogi clock-mode predecessor game test with the sequential stopping rule runs end to end; native rules decide and replay every scored game.
- [ ] #3 A reproducible procedure makes labeled Shogi positions for Texel tuning, with documented size and generation conditions.
- [ ] #4 Baseline results of current main for benchmark, tactical suite and depth at a stated time control are recorded with revision and environment.
- [ ] #5 A written opponent-pool proposal (engines, versions, licenses, csa36 compatibility, settings, protocol and budget) is presented to John; no external rating games run in this task.
- [ ] #6 Independent review accepts the tools and data sources.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
