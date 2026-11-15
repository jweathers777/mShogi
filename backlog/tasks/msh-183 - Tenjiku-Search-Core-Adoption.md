---
id: MSH-183
title: Tenjiku Search Core Adoption
status: Ready
assignee: []
created_date: '2026-11-15T22:31:00Z'
updated_date: '2026-11-15T22:31:00Z'
labels:
  - handle:tenjiku-search-core-adoption
milestone: m-5
dependencies:
  - MSH-178
  - MSH-182
priority: medium
ordinal: 190000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tenjiku Search Core Adoption.

**Problem:** Tenjiku Shogi still uses the first full-width search (`mshogi.tenjiku.full-width.v1`): material only, at most depth 2 and 2,000 nodes. The shared alpha-beta core gave Chess and Tori large gains (MSH-164 (Shared Search Core), MSH-165 (Tori Search Core Adoption)). Project decision 2026-11-15: Shogi and Xiangqi move to the core first, then Chu Shogi and Tenjiku Shogi.

**Objective:** Tenjiku searches through the shared core and is much stronger than its present engine.

**Approach:** Follow the path of MSH-165 and "How to add a game" in `docs/m5/shared-search-core.md`: a fast search position for Tenjiku, a classical evaluation, and an adapter that keeps the native rules as the authority for the root, the chosen move and the reported line. If the core lacks something Tenjiku needs, add it to the core through the trait; do not add a search loop for one game. Tenjiku needs a 256-square board, range-jumping pieces, and the Fire Demon burn, so make and unmake need a variable-length capture record and the tactical gain must include burned pieces. Both rule profiles (Adams and Muller) come from the native game. Reuse what the Chu adoption built for large boards and Lion moves. Saved observations of the present search identity must stay valid; the identity split of saved observations belongs in one shared helper, not in a copy for each game. Follow "Game tests in this direction" in `docs/m5/strength-roadmap.md` for every game test. Use the time and session mechanism of MSH-179 (Clock and Session for All Games) if it is complete; it is not a dependency. Level 1, with local checks of all engine, console, runner and desktop consumers of Tenjiku.

**Completion:** Tenjiku plays through the shared core, its private search is removed, and the game test shows it stronger than the present engine.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Tenjiku searches through the shared core with no game-specific branch inside the core
- [ ] #2 The Tenjiku search position agrees with the native rules on perft counts and on a randomized differential test of legal actions, resulting positions and hashes
- [ ] #3 The private Tenjiku search is removed, the engine, console, runner and desktop paths of Tenjiku still work, and saved observations of the earlier search identity stay valid
- [ ] #4 Tenjiku with the shared core is stronger than its present engine in a game test with a sequential decision
- [ ] #5 Approximations of the native rules inside the search, and any change to the trait or core, are recorded in docs/m5/shared-search-core.md
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
