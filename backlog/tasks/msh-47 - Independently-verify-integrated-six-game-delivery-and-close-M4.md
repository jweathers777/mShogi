---
id: MSH-47
title: Independently verify integrated six-game delivery and close M4
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-26
  - MSH-39
  - MSH-41
  - MSH-42
  - MSH-43
  - MSH-44
  - MSH-45
  - MSH-46
documentation:
  - docs/design.md
ordinal: 47000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M4 was shown as complete from one completed ticket while five promised games were absent. This is the evidence-based closure gate for the entire M4 scope, including preserved chess completeness and required shared integration. A task-count percentage is only a count of currently assigned tasks. P-01/P-03/P-04 and the accepted roadmap determine closure. This scope does not authorize future implementation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 MSH-26 remains Done with its milestone and evidence preserved. All five complete game deliveries MSH-41 through MSH-45 and shared integration MSH-46 satisfy their full acceptance criteria and definition of done; M3 prerequisite closure MSH-39 is established.
- [ ] #2 An independent review traces all six games through sourced complete profiles, authoritative state/actions/history/adjudication, independent conformance verification, reusable library, standalone console, functional native mShogi engine, canonical saved records and actual desktop behavior. Review explicitly distinguishes rule/search correctness, functional play, performance and M5 measured strength.
- [ ] #3 Integrated revision and actual Windows/macOS/Linux packaged application and engine evidence are reproducible. Required integrations, unresolved rules, missing promised games, failed checks and material findings prevent closure; none may be hidden by task aggregation, archiving or a 100% board count.
- [ ] #4 The durable M4 closure verdict records evidence, resolved findings and honest limitations, reconciles any refined child tasks with the original promised scope, and identifies M5/M6 follow-ups without transferring unfinished M4 obligations. No M4 closure occurs while any required game or integration remains incomplete.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
