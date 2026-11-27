---
id: MSH-241
title: Replace Unclear Uses of Native in Names
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T10:40:00Z'
updated_date: '2026-11-27T02:02:00Z'
labels:
  - cleanup
  - model:medium
  - effort:high
dependencies:
  - MSH-243
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inventory names and OS-specific uses. Record the shared terminology in the design.
2. Rename APIs, files, output and fixtures together. Regenerate changed producer outputs and verify independently authored expectations still express the same rules.
3. Run change-selected workspace, frontend, host and Python checks plus focused CI. Obtain a fresh review, then integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Consolidation order approved by John, 2026-11-26: MSH-243 completes the structural cleanup prerequisites. MSH-266 follows this task for the final all-six-game fixture regeneration and reproducibility check. This task still updates current callers and regenerates changed fixtures as its criteria require.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Handoff: codex/msh-241-clear-names is claimed. Inventory and implementation plan are complete. Next: apply the terminology map and regenerate fixtures.

Handoff: codex/msh-241-clear-names carries the terminology changes, glossary, generated fixture updates to date and GUI review fixes. Build, fixture regeneration, local checks and independent review are in progress. Next: verify remaining results, dispatch affected-platform CI, and integrate. Approval review blocked staging the AGENTS.md and CLAUDE.md wording, so their changes remain local while authorization is clarified.

Retained native uses refer to OS windows, menus, dialogs, filename entry, platform UI input and captures, host builds and packages, platform CI, platform attachments and drivers, external driver flags, and the pinned upstream thread_native.h source filename. The original Linux diagnostic capture revision and source hash are retained in the reader-fixture provenance.
<!-- SECTION:NOTES:END -->
