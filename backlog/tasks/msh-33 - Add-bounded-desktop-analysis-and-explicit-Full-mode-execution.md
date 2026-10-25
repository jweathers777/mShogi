---
id: MSH-33
title: Add bounded desktop analysis and explicit Full-mode execution
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-32
documentation:
  - docs/desktop/m3-design.md
ordinal: 33000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Analysis is a separate interaction from committing an engine turn and must not run unbounded adjudication on the host event path.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Users can request explicitly bounded analysis without changing the live game, inspect real result depth/nodes/score/PV and cancel or replace the work safely.
- [ ] #2 Full adjudication has documented bounded or independently terminable execution and preserves its mode/limits; restoration rejects unsupported combinations without downgrading.
- [ ] #3 Native and interaction tests cover analysis isolation, limits, cancellation and stale results; any live telemetry has a bounded native reporting implementation and real observation evidence.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
