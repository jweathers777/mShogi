---
id: MSH-215
title: Docs Trim
status: Done
assignee:
  - '@claude'
created_date: '2026-11-20T23:19:00Z'
updated_date: '2026-11-20T23:19:00Z'
labels:
  - docs
milestone: m-6
dependencies: []
ordinal: 222000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: trim docs/ to durable knowledge (design, rules profiles and derivations, protocols and contracts, how-to guides, data that code reads), as AGENTS.md requires. Rewrite strength-roadmap.md and development.md to their current content. Delete old task reports, reviews, validations and plans; Git history keeps them.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 strength-roadmap.md holds only the current program, game-test rules, status by game and clock policy
- [x] #2 development.md holds only setup, workspace, dependency, CI and rules-tool instructions
- [x] #3 Old reports are deleted; every file that code, tests or validators read remains; dead Markdown links are removed
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Kept 109 docs files (design, rules, contracts, how-to, fixtures, validator inputs, the mutual-check proof supplement); deleted 284 reports, reviews, validations, plans and the bootstrap START-HERE.md and PACKAGE-VALIDATION.md. strength-roadmap.md 523 -> about 140 lines: the speed step (MSH-200/201/202) replaces the deep-dive paragraph; the harness no longer proposes external opponents and rounds no longer require a fresh-session review, following AGENTS.md. development.md 176 -> about 90 lines. Dead Markdown links became plain text (40 files); conformance-policy.md now points to AGENTS.md checks. CI push trigger widened from main/codex/** to all branches so <agent-name>/ branches get CI. Checks (macOS): cargo test --workspace --locked --no-run OK; mshogi-console common_console 9 passed; Chu and Tenjiku validate.py and test_validate OK; tools/ci_checks.py --self-test 8 OK.

Correction to counts: docs/ has 115 tracked files after the change (279 deleted), about 87k lines, most of them fixture JSON. The Tenjiku research folder keeps its .gitignore so the local originals/ source copies stay out of Git.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
docs/ went from 393 files (about 219k lines) to 115 durable files (about 87k lines, mostly fixtures); strength-roadmap.md (523 -> 125 lines) and development.md (176 -> 94 lines) rewritten to current content; dead links removed; CI now runs on all branches. Old reports remain in Git history.
<!-- SECTION:FINAL_SUMMARY:END -->
