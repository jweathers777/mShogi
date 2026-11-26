---
id: MSH-274
title: Shared Checks and Lightweight Git Hooks
status: Done
assignee:
  - '@codex'
created_date: '2026-11-26T12:57:00Z'
updated_date: '2026-11-26T13:12:00Z'
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
- [x] #1 A shared check command selects relevant checks from changed paths, handles the root workspace and excluded Tauri package, and skips product checks for documentation-only changes.
- [x] #2 Tracked hooks and an explicit installer work with partial staging and worktrees, never alter staged or working files, and run no builds or tests on commit or push.
- [x] #3 CI reuses shared check selection and commands without dropping existing verification; automated tests cover selection, hook behavior and failure handling on affected operating systems.
- [x] #4 AGENTS.md and development setup document the shared command and hook installation; required checks pass and changes are committed and pushed.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extend the existing CI path policy with a shared executable check command. 2. Add staged-snapshot formatting hooks and setup, with isolated Git tests. 3. Reuse commands in CI and document setup and scope. 4. Run focused tooling tests and cross-platform CI, integrate, record results and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint: shared command, staged-snapshot pre-commit hook, installer, CI command reuse and shorter AGENTS.md implemented on codex/msh-274-shared-checks. Focused local policy and real Git tests pass (including partial staging, initial commits, worktrees and actual rustfmt without product builds). Next: validate CI on Linux/macOS/Windows, fix findings, install hook in this clone, integrate and clean up. No pre-push hook; pushes never trigger local builds/tests.

Verified and integrated on main through 2d1392f. Local: 8 existing policy tests plus 16 check/hook tests passed; shell syntax and git diff --check passed. CI run 45826384573 passed routine selection and all three tooling jobs (Ubuntu 24.04, macOS 15, Windows 2022): https://github.com/jweathers777/mShogi/actions/runs/45826384573 . Earlier run 45825339554 passed root workspace formatting/Clippy, release search regressions and frontend TypeScript/tests; its Windows test-fixture CRLF failure was fixed with explicit LF fixtures and a tracked LF attribute for hooks. Remaining superseded runs were canceled after the corrected three-platform run passed. Full workspace tests and packaged GUI checks were not run: no product Rust/frontend behavior changed. Hook installed locally via core.hooksPath=.githooks and exercised on real commits, without builds/tests/downloads. No GCP jobs or new worktrees were created.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added tools/checks.py as the shared local/CI entry point and retained the existing CI path policy. Pre-commit checks staged whitespace, Python syntax and affected-package Rust formatting in an offline temporary snapshot; there is no pre-push hook. CI reuses named suites and Python selection. Simplified AGENTS.md and documented setup. All 24 local tooling tests and Linux/macOS/Windows CI tooling checks passed after fixing Windows line endings. Integrated and pushed to main; no follow-up implementation is required.
<!-- SECTION:FINAL_SUMMARY:END -->
