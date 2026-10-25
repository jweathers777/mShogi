---
id: MSH-40
title: Correct the remaining roadmap representation without starting implementation
status: In Review
assignee:
  - '@codex-roadmap'
created_date: '2026-10-25T15:36:00Z'
updated_date: '2026-10-25T15:42:00Z'
labels: []
dependencies: []
documentation:
  - docs/design.md
ordinal: 40000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The milestone board treats M4 as complete because only the completed MSH-26 is assigned; the five promised games and known M5/M6 deliverables have no tasks. John requested a bounded planning correction on 2026-10-25. Preserve existing records and the M3 batch; this task authorizes planning records and documentation only.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Five scope-level To Do game tickets, shared M4 integration ownership and a complete dependent M4 closure gate represent the accepted product scope while MSH-26 and all existing evidence remain unchanged.
- [ ] #2 Scope-level To Do M5 and M6 tickets preserve six-game strength obligations, required product delivery, optional research and unresolved design questions without speculative plans or estimates.
- [ ] #3 Roadmap documentation distinguishes task-count completion from evidence-based milestone closure; CLI board, dependency and preservation checks verify the correction, which is committed and pushed through codex/rewrite.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Compare accepted design and existing tasks using Backlog.md 1.55.1 in the isolated codex/roadmap-backlog-correction worktree. 2. Add unassigned To Do scope tickets and closure dependencies using only the CLI; document ownership and deferred questions. 3. Update docs/design.md with task mapping and closure policy. 4. Verify board, dependency integrity and unchanged existing records, then commit, integrate and push normally.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Planning correction implemented in isolated codex/roadmap-backlog-correction worktree from 47209e379cf505276145b961056c6436b15e586d. The default Homebrew CLI was 1.55.0; all mutations used the cached project-baseline 1.55.1 native CLI at /private/tmp/mshogi-bootstrap-npm-cache/_npx/749414256ca1c44f/node_modules/backlog.md-darwin-arm64/backlog. No application/rules/search implementation was started.

Created MSH-41–47 for five complete game scopes, single shared integration owner and M4 closure; MSH-48–56 for shared measurements, six game-specific strength obligations, initial learned evaluation and M5 closure; MSH-57/58/60 for required M6 product/release delivery and closure, with optional-research MSH-59 excluded from closure dependencies. Deferred D-07/D-08/D-09/D-10 questions remain in the relevant scopes. docs/design.md section 5 maps ownership and distinguishes counts from closure verdicts.

Verification on macOS, 2026-10-25: CLI --version = 1.55.1; milestone list --show-completed --plain shows M4 active 1/8, M5 0/9, M6 0/4; task list by milestone --json verifies status and counts. Python byte comparisons against git show 47209e3:<path> passed for every previously tracked file other than docs/design.md, including MSH-26 and all M3 records/evidence. All 20 future scopes are unassigned To Do with unchecked acceptance and no implementation plan/notes/final summary. All dependency IDs resolve; exact M4/M5/M6 closure sets checked, optional research absent from required M6 dependencies. backlog doctor reports no duplicate IDs, self-dependencies or cycles; git diff --check passes. CLI board export contains every new scope and retained MSH-26. The first export using an absolute filename failed because this CLI resolves it below cwd; retry with an os.path.relpath temporary destination succeeded. This was an artifact-path error, not a failed backlog verification.

Documentation/record-only change: no Rust, frontend or packaged-application tests run or newly claimed. Existing code/platform evidence is preserved. Bounded independent scope review requested from review_planning; integration and push still pending. Concurrent unrelated M3 work advanced codex/rewrite while this correction was in progress; integration will preserve it and verify this correction's delta separately.
<!-- SECTION:NOTES:END -->
