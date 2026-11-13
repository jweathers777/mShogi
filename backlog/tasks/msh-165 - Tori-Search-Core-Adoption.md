---
id: MSH-165
title: Tori Search Core Adoption
status: To Do
assignee: []
created_date: '2026-11-13T21:29:00Z'
labels:
  - handle:tori-search-core-adoption
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/evidence/msh161/cloud-primary/independent-review-fable/review.md
priority: medium
ordinal: 169000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Search Core Adoption.

**Problem:** A search core built with Chess only can hold Chess-only assumptions. The project goal is reuse across chess-like games and easy addition of new games.

**Objective:** Prove the shared core boundary with a second, different game early. Tori Shogi is small (7x7) and has drops and its own repetition rule.

**Approach:** Give Tori Shogi a search position for the shared core and remove its private minimax copy. Change the trait where Tori shows a Chess-only assumption; do not add a Tori-only path in the core. Record what Shogi, Xiangqi, Chu Shogi and Tenjiku Shogi will need (board width, drops, lion moves, repetition rules) as notes for their later tasks, without building it now.

**Completion:** Tori plays through the shared core, its old search copy is removed, and the trait changes are recorded.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Tori Shogi searches through the shared core with no game-specific branch inside the core
- [ ] #2 The Tori search position agrees with the native Tori rules on a differential test of legal actions, including drops
- [ ] #3 The private Tori minimax copy is removed and Tori engine and desktop paths still work
- [ ] #4 Tori with the shared core is stronger than its predecessor in a game test
- [ ] #5 Requirements for the four other games on the trait are recorded for their later tasks
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
