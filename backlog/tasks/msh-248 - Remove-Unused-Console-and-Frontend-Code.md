---
id: MSH-248
title: Remove Unused Console and Frontend Code
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T17:41:00Z'
labels:
  - cleanup
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 3500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-209 found code that has no product caller or cannot run (checked 2026-11-25; check the callers again before you remove an item):
- Console: the 'stop cancelled|resource|engine|work' command (only tests call it); 'position record-file|record-json' in tori_session.rs, chu_session.rs and tenjiku_session.rs, which repeats 'record load-file|load-json'; the line-length check at the start of each session execute function (main.rs bounded_line already refuses longer lines); the ended flag in xiangqi_uci.rs, which is never set to true.
- Frontend: desktop/src/product/ToriBirdPiece.tsx and tori-birds.css (marked dormant; only their test imports them); about 30 CSS classes with no use in desktop/src (for example session-badges, resign-dialog, event-timeline, technical-analysis, analysis-card, history-card, native-history-row, shogi-technical, game-mark, app-footer in style.css, shell.css, shogi.css and xiangqi.css).
- Scripts: retired markers (native_contract_*, 'MSH-72 DEVELOPMENT CONTRACT') in the forbidden lists of desktop/scripts/package-smoke.py and desktop/scripts/build-frontend.mjs, which no source contains.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each listed item is removed, or kept with a current caller or a recorded decision of John's recorded in the notes; tests that only exercise removed code are removed, and documents that name removed console commands are updated.
- [ ] #2 Workspace, frontend and desktop script tests pass; production line counts before and after are recorded.
- [ ] #3 The ignored one-time measurement test record_timed_native_games in crates/mshogi-engine/tests/all_games_timing.rs and its now unused imports are removed; the other tests in that file stay.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.

John, 2026-11-25: agreed to remove the listed console commands.

2026-11-25, John: delete that one timing test in this task. It collected one-time evidence for MSH-179 (Clock and Session for All Games). Do this after MSH-246 (One Chess Judging Mode), which can edit the same file.

2026-11-25, claimed on branch claude/msh-248-remove-unused. Keep desktop/src/product/ToriBirdPiece.tsx, tori-birds.css, its test and every CSS class that they use: John decided on 2026-11-14 to keep this bird artwork (MSH-170 (Remove unused code) notes, docs/design.md "Pieces", docs/m4/tori/piece-artwork.md). This task was written without that decision. The coordinator asked John on 2026-11-25 whether he wants to change it. Criterion 1 now also permits an item that a recorded decision of John's keeps.

John, 2026-11-25 (answer relayed by the coordinator), confirmed: "No, keep it" about the removal of the Tori bird artwork. ToriBirdPiece.tsx, tori-birds.css, its test and the CSS classes that they use stay.
<!-- SECTION:NOTES:END -->
