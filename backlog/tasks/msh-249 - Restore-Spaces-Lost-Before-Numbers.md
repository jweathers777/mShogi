---
id: MSH-249
title: Restore Spaces Lost Before Numbers
status: Done
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T18:36:00Z'
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
- [x] #1 Comments, messages and test text in crates/, desktop/ and tools/ have a space between a word and a number; the damaged chu/game.rs comments say what they mean.
- [x] #2 Tests that check changed messages are updated; workspace and frontend checks pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.

Run result: cargo fmt, clippy -D warnings and cargo test --workspace passed. Desktop npm ci (Node 24.22.3), tsc --noEmit and vitest (43 files, 589 tests) passed. Python: desktop/scripts/test_gui_smoke.py (130) and tools/strength tests passed. tools/xiangqi/test_reference.py: 8 of 15 tests fail because an external reference binary under /private/tmp/msh91-reference is missing on this machine; this is environmental and was not changed. Left unchanged on purpose: identifiers and keys (repetition3, seed12, pair0012, UCI0-9, rule60/check10, MSH44, belongs77 in conformance assertions.rs), notation (square and coordinate names), the Tenjiku artwork message (all44faces, fixture-linked in shared-tenjiku-*-all-faces.json), the chu_bench and chess_study_roots selection strings, and the 0heldout text in texel_data.py. Question for John if he wants these changed: the Tenjiku artwork message needs its fixtures regenerated; belongs77 needs the task reference confirmed.

2026-11-25: Reopened. The first result did not meet acceptance criterion 1: about 60 comment lines and 20 message or test strings in crates/, desktop/ and tools/ still had a lost space. The agent level changed from model:small, effort:low to model:medium, effort:medium for this reason. Handoff: branch claude/msh-249-finish-spaces; next step is a full search, then the fixes.

2026-11-25, second pass (branch claude/msh-249-finish-spaces, commit 861a75d).

Search: extracted all comments, string literals, JSX text and text files in crates/, desktop/ and tools/, then scanned for letters joined to digits, lost spaces after punctuation and before format placeholders, continued strings, strings with no space, joined words (system dictionary and repository vocabulary), and bare task numbers. The history has no single bulk edit in these directories to compare with.

Fixed: comments, help text, error and test messages, README files and the font NOTICE. Restored task references from the task records and docs/testing/conformance-policy.md: belongs77 means MSH-77, to127 means MSH-127, component105 means MSH-105, and bare 127/128/129 in the Tori comments mean MSH-127/128/129. Changed seed12 and pair0012 too: the first pass listed them as identifiers, but they are prose. Changed the Tenjiku artwork and Tori deadline messages. Regenerated their frontend fixtures and the Chu and Tenjiku fixture manifests with the generators; in the copied files only these strings differ. All .mshogi files are unchanged.

Left on purpose (the scan still finds these):
1. Saved records and version strings: "MSH44 packaged native source fixture ... Pawn id5" (write_tenjiku_gui_fixtures.rs and the .mshogi files); the editions MSH82-2026-11-02, MSH83-2026-11-04 and MSH208-MODERN-2026-11-20.
2. Text in frozen evidence: the selection strings in chess_study_roots.rs (MSH-49 catalogs have recorded SHA-256 values and were reproduced byte for byte) and chu_bench.rs (frozen MSH-201 benchmark input); data files in tools/strength/data (shogi-tactics-v1, shogi-bench-v1, shogi openings); public-diagnostics provenance (Linux241).
3. Text that an experiment config_sha256 covers: protocol and provenance strings in experiment_pilot.rs, chu_match_config.rs and tenjiku_match_config.rs.
4. Not damage: identifiers, file names and paths (msh64, msh91, round4/match100k, repetition8), protocol and dialect names (UCI0-9, xiangqi-uci09, WXF2018, rule60/check10), units such as 16MiB, move and square notation, FEN and SFEN strings, and format strings that join a word to an optional suffix ("{} to move{}", "{}mate").
Outside scope: docs/ and the root tests/fixtures directory, which has similar compact notation.

Checks: cargo fmt, clippy -D warnings and cargo test --workspace --locked passed (1171 passed, 0 failed, 10 ignored). Node 24.22.3: tsc --noEmit passed; vitest 43 files and 589 tests passed. Python: desktop/scripts test_gui*.py 166 passed, test_macos_verification.py 11 passed, tools/strength/tests 62 run with 1 skipped. tools/xiangqi/test_reference.py: 8 of 15 fail because the external binary under /private/tmp/msh91-reference is absent. tools/shogi/reference.py: parse check only. main did not change after these runs. No cloud job or spending.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Restored the spaces and the lost MSH- task references that the first pass missed in comments, messages and test text in crates/, desktop/ and tools/, including the strings that the first pass left for a follow-up where they are prose. Regenerated the affected fixtures with their generators. Saved records, version strings and text in hashed or frozen experiment evidence are unchanged and listed in the notes. fmt, clippy, workspace tests, frontend checks and the touched Python tests passed; the Xiangqi reference tests need an absent external binary.
<!-- SECTION:FINAL_SUMMARY:END -->
