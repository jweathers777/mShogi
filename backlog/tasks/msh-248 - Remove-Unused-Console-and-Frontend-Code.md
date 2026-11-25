---
id: MSH-248
title: Remove Unused Console and Frontend Code
status: Done
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T17:54:00Z'
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
- [x] #1 Each listed item is removed, or kept with a current caller or a recorded decision of John's recorded in the notes; tests that only exercise removed code are removed, and documents that name removed console commands are updated.
- [x] #2 Workspace, frontend and desktop script tests pass; production line counts before and after are recorded.
- [x] #3 The ignored one-time measurement test record_timed_native_games in crates/mshogi-engine/tests/all_games_timing.rs and its now unused imports are removed; the other tests in that file stay.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Check the callers of every listed item again.
2. Remove the console items, their tests and their documentation.
3. Remove CSS selectors for classes with no use, the retired package markers and the one-time timing test.
4. Run all checks, count production lines, integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.

John, 2026-11-25: agreed to remove the listed console commands.

2026-11-25, John: delete that one timing test in this task. It collected one-time evidence for MSH-179 (Clock and Session for All Games). Do this after MSH-246 (One Chess Judging Mode), which can edit the same file.

2026-11-25, claimed on branch claude/msh-248-remove-unused. Keep desktop/src/product/ToriBirdPiece.tsx, tori-birds.css, its test and every CSS class that they use: John decided on 2026-11-14 to keep this bird artwork (MSH-170 (Remove unused code) notes, docs/design.md "Pieces", docs/m4/tori/piece-artwork.md). This task was written without that decision. The coordinator asked John on 2026-11-25 whether he wants to change it. Criterion 1 now also permits an item that a recorded decision of John's keeps.

John, 2026-11-25 (answer relayed by the coordinator), confirmed: "No, keep it" about the removal of the Tori bird artwork. ToriBirdPiece.tsx, tori-birds.css, its test and the CSS classes that they use stay.

Caller check, 2026-11-25 (all items had no product caller):
- Console 'stop': only tori_cli, chu_cli and tenjiku_cli tests called it. Removed the command, OperationalStop, the 'stopped' session state and the 'incomplete ...' status output. Deleted the test that only exercised stop; removed stop steps from four other tests and kept their checks.
- 'position record-file|record-json' (Tori, Chu, Tenjiku): only console tests called it. Tests now use 'record load-file|load-json'. 'position startpos' stays.
- Line-length check in six session execute functions: the only caller is main.rs shell, behind bounded_line. The bounded_line check and its test stay.
- Xiangqi UCI 'ended': never true. Removed the field and its unreachable 'gameover' error.
- CSS: removed 93 selectors for 39 classes with no use (checked as literals in desktop/src, scripts, Tauri host and crates; run-time names completed-action-${kind}, tenjiku-face-${n}, shell-${game}, .piece.white|black are in use and stay). A script compared the rule sets: all other selectors and declarations are identical. Classes: analysis-card analysis-fields analysis-form analysis-selection analysis-suggestion app-footer archived-observation cancel-analysis document-actions document-details empty-history engine-actions engine-choice event-timeline failed failure-detail footer-details game-choices game-mark guard-message history-card history-scroll io-failure native-history-row new-game new-game-controls new-game-note played-actions resign-dialog resign-stale session-badges shogi-history-banner shogi-technical technical-analysis telemetry thinking unresolved variation xiangqi.
- Package markers: native_contract_snapshot, native_contract_command, native_contract_file and 'MSH-72 DEVELOPMENT CONTRACT' were in no source; removed. All other markers are still in some source and stay.
- record_timed_native_games and its imports removed; the other tests in all_games_timing.rs stay. No script, workflow or document named it.
- Kept: ToriBirdPiece.tsx, tori-birds.css, its test (John's decision, above).
- Documents updated: docs/m4/tori, chu and tenjiku notation.md; the current launch section of docs/m4/desktop-session.md. Other leftovers are in MSH-271 (Remove Remaining Retired Desktop References).

Production lines (physical), main af1ef09 to this task: 8 changed console source files 3704 to 3579; 4 changed CSS files 1195 to 871; 2 changed desktop scripts 230 to 229. All crates/*/src files without 'test' in the path: 74438 to 74313.

Checks on commit 05d8335, macOS arm64: cargo fmt --check pass; clippy --workspace --all-targets -D warnings pass; cargo test --workspace --locked 1171 passed, 0 failed, 10 ignored. Node 24.22.3: npm ci, npm run check, npm test (43 files, 589 tests), npm run build and build:verification pass. python3 -m unittest discover -s desktop/scripts -p 'test_*.py': 177 passed. tools/ci_checks.py --self-test pass. Not run: packaged desktop build and package-smoke.py (release check), GUI checks, other operating systems (the change is not platform-sensitive). No separate review (deletion; no rules, saved format or large refactor).

GitHub Actions run 45668966251 (Rust workspace, Ubuntu, pinned Node) on 05d8335: success.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed code that had no product caller (commit 05d8335): the console 'stop' command and its session state, 'position record-file|record-json' for Tori, Chu and Tenjiku (use 'record load-file|load-json'), the second line-length check in six session execute functions, the Xiangqi UCI 'ended' flag, 39 unused CSS classes, four retired package markers and the one-time test record_timed_native_games. Tests that only exercised removed code are gone; checks of kept behavior stay. Documents are updated. Kept: the Tori bird artwork (ToriBirdPiece.tsx, tori-birds.css and its test), by John's decision of 2026-11-14, confirmed 2026-11-25 ("No, keep it"). Production lines in the changed files: console 3704 to 3579, CSS 1195 to 871, scripts 230 to 229. Workspace tests (1171 passed, 10 ignored), frontend (589), desktop script tests (177), builds and branch CI pass. Packaged build and GUI checks did not run. Follow-up: MSH-271 (Remove Remaining Retired Desktop References).
<!-- SECTION:FINAL_SUMMARY:END -->
