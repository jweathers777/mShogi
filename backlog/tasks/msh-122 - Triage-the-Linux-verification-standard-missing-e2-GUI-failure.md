---
id: MSH-122
title: Triage the Linux verification-standard missing-e2 GUI failure
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-30T13:26:00Z'
labels:
  - gui-verification
  - planning
milestone: m-4
dependencies:
  - MSH-106
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-health-106.md
  - docs/review/linux-gui-triage-122.md
priority: high
type: bug
ordinal: 122000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-106 V01/P9 captured an unexplained Linux verification-standard missing-e2 selector failure; MSH-111 pooling diagnosis does not establish its cause and MSH-112 concerns another native runner race. This separate bounded triage is intended before substantial architecture migration to establish a reliable GUI baseline. Creation is planning-only, with no active investigator or execution in this assignment.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Inspect original run40994147366 at2f17510e4c5b8c3b92a051adfa070b4d5d834b21, exact package/driver/WebKit/environment, phase screenshot/DOM/host/transport evidence and selector/aggregate code. Retain missing-e2 failure as failed; classify observations without assuming driver reset, timing or product cause.
- [ ] #2 Controlled focused reproduction or independently reviewed evidence-grounded disposition distinguishes fixture/selector/observation timing, transport and actual native/UI behavior. Any correction remains bounded to the diagnosed issue; rerun success alone, weakened assertions or raised timeouts are not a diagnosis/fix.
- [ ] #3 Mutations/input/lifecycle are never automatically replayed after ambiguous delivery. Preserve meaningful phase assertions, actual native host/engine exit proof before teardown, bounded observation retries and normal-package instrumentation exclusion.
- [ ] #4 Record an actual current installed/packaged Linux baseline covering all required existing phases and native file/quit journeys, with exact source/package/runtime/proxy identities and failures. If material failures prevent a reliable baseline, record the concrete blocker and leave acceptance unsupported; no inference of Xiangqi/other-platform GUI success.
- [ ] #5 Focused boundary verification, independent source/evidence review and applicable CI support the disposition; MSH-111 Done driver history and MSH-112 runner classification remain distinct, unchanged records. No architecture refactor or variant implementation is bundled into triage.
- [ ] #6 Execute this separately before substantial R1-R6 refactoring as the intended regression-baseline assignment. No blanket P9 dependency is added to unrelated tasks; any newly demonstrated verification prerequisite must be recorded concretely. Stop after normal integration/push and evidence handoff.
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
Inspect retained original Linux artifact and full phase traceback/DOM/package identities; reproduce the concrete boundary in isolation and apply only a demonstrated narrow correction. Verify observation/mutation and lifecycle invariants with focused checks, obtain independent source review, then run actual packaged Linux baseline and applicable CI. Independently reconcile evidence and finalize only MSH-122, preserving historical failures and MSH-111/112 ownership.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup 2026-10-30: primary builder Codex owns MSH-122 in primary checkout on codex/rewrite at 311ea53e152c71c5eadd2c428427325b30a42b29. Checkout clean; fetched origin and confirmed 0/0 divergence. Existing unrelated worktrees preserved. Pinned Backlog.md 1.55.1 used instead of installed 1.55.0; overview/execution guides read. MSH-106 dependency Done; no competing active task identified. Original Linux artifact12950842851 remains available, digest sha256:7b1be5e0ba05c047bc870dda6f7ff125064005ae2d61985841f787f474f3b3c0. Investigation does not assume driver, timing or product cause.

Original artifact downloaded and retained selected bytes/full report under docs/review/evidence/msh122/original. Full traceback identifies first e2 precondition before any pointer input; original null conflates absent and disabled, delayed capture shows healthy initial Chess. Controlled App/native snapshot ordering reproduces document-card present with64 squares/e2 disabled before command connection; original exact instantaneous branch remains unknown. Verifier-only correction polls specific present/enabled/visible/hit-testable target within unchanged25s budget, then dispatches pointer once. No product/driver/lifecycle changes. Local Node24.19.0/Python3.14.7:41 GUI boundary tests,9 App tests,TS check,diff check pass. Development-only test setup failures preserved in triage doc. Independent original-artifact/source reviewer found no material defect, pending final diff and actual Linux package evidence.
<!-- SECTION:NOTES:END -->
