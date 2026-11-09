---
id: MSH-138
title: >-
  Calibrate responsive desktop Chess play limits for the refined conventional
  engine
status: Ready
assignee: []
created_date: '2026-11-08T23:48:00Z'
updated_date: '2026-11-09T00:50:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-137
documentation:
  - docs/m5/chess-classical-refinement.md
priority: high
ordinal: 143000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-137 improves engine work substantially, but normal desktop Chess retains its accepted 20000-visit and 1000ms hard search limits. Faster fixed-visit play can finish well before one second; full-second UCI/timed-profile capability does not establish GUI default strength. Assess the ordinary play budget using short native execution/desktop responsiveness checks. Preserve request/limit identities, old-document behavior, cancellation and existing resource admission ceilings. This is product-default calibration, not an engine architecture blocker or a new strength study. No additional Chess opponent studies are authorized.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measure whether accepted desktop defaults terminate predominantly at visits or time on representative positions and distinguish search limits from process-supervision watchdogs.
- [ ] #2 Retain or revise defaults with proportionate latency/work evidence within existing admission ceilings; preserve loaded observations, explicit Resume semantics, identities, cancellation and stale-result rejection.
- [ ] #3 Relevant controller/session and actual execution checks plus independent review support the chosen behavior; document limitations without Elo or GUI maximum-strength claims.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Next bounded Chess product checkpoint after137, before50/54 transfer. Review supports engine architecture readiness; calibrate ordinary play defaults without increasing admission ceilings or conducting new opponent studies.

MSH-137 is Done after independent final acceptance and required actual3OSCI42538719298 at2a2ff7c. This task is now dependency-ready, unassigned and next in the Chess-first sequence. Use current codex/rewrite and its source/evidence; no additional Chess opponent studies. Do not reinterpret engine-readiness evidence as GUI-default maximum strength.
<!-- SECTION:NOTES:END -->
