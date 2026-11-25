---
id: MSH-241
title: Replace Unclear Uses of Native in Names
status: Ready
assignee: []
created_date: '2026-11-25T10:40:00Z'
labels:
  - cleanup
  - model:medium
  - effort:high
dependencies: []
priority: medium
ordinal: 7950
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John (2026-11-25): rename things so that unclear uses of "native" are gone. The word is used for mShogi's own rules library and its types, records and actions (for example native rules, NativeGame, NativeSession, the mshogi-core native module), about 12,000 lines in 750 files outside Backlog. Use plain names that say what the thing is, for example "rules library" or "game record". Run after the other cleanup tasks, so that code which they delete is not renamed first. Saved formats and protocols may change (A-09).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Code identifiers, module and file names, user-visible text, fixtures and documents no longer use "native" for mShogi's own rules, types or data; it remains only in its standard platform meaning (native application, native platform or code), and each kept kind of use is listed in the task notes.
- [ ] #2 The wording in AGENTS.md, CLAUDE.md files and docs/design.md is updated without any change to a process rule, game rule or product decision; Backlog task records are left unchanged.
- [ ] #3 Names follow one glossary recorded in docs/design.md; workspace, frontend and Python checks pass and changed fixtures are regenerated with the existing generators.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->
