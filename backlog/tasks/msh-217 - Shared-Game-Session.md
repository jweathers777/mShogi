---
id: MSH-217
title: Shared Game Session
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-20T23:25:00Z'
updated_date: '2026-11-21T22:41:00Z'
labels:
  - desktop
  - refactor
milestone: m-6
dependencies: []
priority: medium
ordinal: 224000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: replace the copied per-game modules of crates/mshogi-desktop-session with one generic session implementation behind a per-game trait, so that each game supplies only its genuine differences (rules types, records, presentation, notation).

Finding (2026-11-20 review): the crate has about 25k production lines, more than the search crate for all six games. After replacing game names with one placeholder, shared_chu.rs and shared_tenjiku.rs differ in 149 of about 1,690 lines, chu_document.rs and tenjiku_document.rs in 362 of about 2,070, and the presentation modules in 363 of about 870; Tori and Xiangqi follow the same pattern with more divergence. The copies have drifted: Tenjiku restore and browse use stop-aware calls (to_game_with_stop, record_prefix_with_stop, legal_actions_with_stop) and Chu does not, so a Chu restore cannot be interrupted. The per-game test files repeat the same structure.

Approach: measure the per-game differences first, design the trait from the real differences (A-10; generalize only shared semantics), migrate one game pair (Chu and Tenjiku) to prove the boundary, then Tori, Xiangqi, Shogi and Chess where they fit. Native rules, saved-document format and session behavior do not change (A-09 allows changing internal APIs).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic session/document implementation serves the games; per-game code contains only game-specific behavior, and the production line count before and after is recorded
- [ ] #2 Behavior is unchanged: existing desktop-session tests pass (re-expressed once against the generic code instead of copied per game), saved documents of every game load and save identically, and the desktop host tests pass
- [ ] #3 Stop/cancellation behavior is the same for every game (the Chu restore and browse paths become interruptible like Tenjiku)
- [ ] #4 One review by a fresh agent session of the trait boundary and the migration
<!-- AC:END -->
