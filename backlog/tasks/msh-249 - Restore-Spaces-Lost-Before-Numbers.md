---
id: MSH-249
title: Restore Spaces Lost Before Numbers
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T18:00:00Z'
labels:
  - cleanup
  - model:small
  - effort:low
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
- [ ] #2 Tests that check changed messages are updated; workspace and frontend checks pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.
<!-- SECTION:NOTES:END -->
