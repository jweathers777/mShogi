---
id: MSH-248
title: Remove Unused Console and Frontend Code
status: Ready
assignee: []
created_date: '2026-11-25T12:21:00Z'
labels:
  - cleanup
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 6500
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
- [ ] #1 Each listed item is removed, or kept with a current caller recorded in the notes; tests that only exercise removed code are removed, and documents that name removed console commands are updated.
- [ ] #2 Workspace, frontend and desktop script tests pass; production line counts before and after are recorded.
<!-- AC:END -->
