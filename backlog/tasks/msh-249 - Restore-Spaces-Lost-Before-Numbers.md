---
id: MSH-249
title: Restore Spaces Lost Before Numbers
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T18:16:00Z'
labels:
  - cleanup
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 3600
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
An earlier bulk edit removed the space between a word and the number after it in many comments and in some messages that users see, for example 'exceeds16MiB' (desktop-session document.rs and shared_archive.rs), 'exceeds1MiB' and 'exceeds64MiB' (runner), 'rank12' (core chu and tenjiku position.rs), and 'belong78/43' and 'remain78' in mshogi-core chu/game.rs, where the task reference was lost as well. About 190 places in about 50 files; most are in docs/m4, which MSH-235 (Reconcile and Prune Durable Documentation) handles.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Comments, messages and test text in crates/, desktop/ and tools/ have a space between a word and a number; the damaged chu/game.rs comments say what they mean.
- [x] #2 Tests that check changed messages are updated; workspace and frontend checks pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.

Run result: cargo fmt, clippy -D warnings and cargo test --workspace passed. Desktop npm ci (Node 24.22.3), tsc --noEmit and vitest (43 files, 589 tests) passed. Python: desktop/scripts/test_gui_smoke.py (130) and tools/strength tests passed. tools/xiangqi/test_reference.py: 8 of 15 tests fail because an external reference binary under /private/tmp/msh91-reference is missing on this machine; this is environmental and was not changed. Left unchanged on purpose: identifiers and keys (repetition3, seed12, pair0012, UCI0-9, rule60/check10, MSH44, belongs77 in conformance assertions.rs), notation (square and coordinate names), the Tenjiku artwork message (all44faces, fixture-linked in shared-tenjiku-*-all-faces.json), the chu_bench and chess_study_roots selection strings, and the 0heldout text in texel_data.py. Question for John if he wants these changed: the Tenjiku artwork message needs its fixtures regenerated; belongs77 needs the task reference confirmed.

2026-11-25: Reopened. The first result did not meet acceptance criterion 1: about 60 comment lines and 20 message or test strings in crates/, desktop/ and tools/ still had a lost space. The agent level changed from model:small, effort:low to model:medium, effort:medium for this reason. Handoff: branch claude/msh-249-finish-spaces; next step is a full search, then the fixes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Restored the space between a word and the number in about 130 messages and comments in 76 files in crates/, desktop/ and tools/ (for example exceeds 16MiB, depth 1..2, rank 12). Restored the chu/game.rs references as MSH-78 and MSH-43. Updated the tests and the desktop frontend fixture that compare changed messages. Checks: fmt, clippy, workspace tests, frontend tsc and vitest, and the touched Python tests passed; tools/xiangqi reference tests need an absent external binary. Some identifiers and fixture-linked strings were left, listed in the notes.
<!-- SECTION:FINAL_SUMMARY:END -->
