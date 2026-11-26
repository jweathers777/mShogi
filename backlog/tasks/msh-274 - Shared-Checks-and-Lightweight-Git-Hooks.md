---
id: MSH-274
title: Shared Checks and Lightweight Git Hooks
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-26T12:57:00Z'
updated_date: '2026-11-26T13:02:00Z'
labels:
  - tooling
  - ci
  - model:medium
  - effort:high
dependencies: []
priority: medium
ordinal: 248000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Centralize mechanical check selection and commands so local work and CI use the same policy. Install lightweight Git hooks that check staged changes without builds or tests, and replace duplicated agent instructions with the shared entry point.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A shared check command selects relevant checks from changed paths, handles the root workspace and excluded Tauri package, and skips product checks for documentation-only changes.
- [ ] #2 Tracked hooks and an explicit installer work with partial staging and worktrees, never alter staged or working files, and run no builds or tests on commit or push.
- [ ] #3 CI reuses shared check selection and commands without dropping existing verification; automated tests cover selection, hook behavior and failure handling on affected operating systems.
- [ ] #4 AGENTS.md and development setup document the shared command and hook installation; required checks pass and changes are committed and pushed.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extend the existing CI path policy with a shared executable check command. 2. Add staged-snapshot formatting hooks and setup, with isolated Git tests. 3. Reuse commands in CI and document setup and scope. 4. Run focused tooling tests and cross-platform CI, integrate, record results and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint: shared command, staged-snapshot pre-commit hook, installer, CI command reuse and shorter AGENTS.md implemented on codex/msh-274-shared-checks. Focused local policy and real Git tests pass (including partial staging, initial commits, worktrees and actual rustfmt without product builds). Next: validate CI on Linux/macOS/Windows, fix findings, install hook in this clone, integrate and clean up. No pre-push hook; pushes never trigger local builds/tests.
<!-- SECTION:NOTES:END -->
