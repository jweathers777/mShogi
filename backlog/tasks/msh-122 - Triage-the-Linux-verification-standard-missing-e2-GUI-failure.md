---
id: MSH-122
title: Triage the Linux verification-standard missing-e2 GUI failure
status: Ready
assignee: []
created_date: '2026-10-30T12:06:00Z'
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
