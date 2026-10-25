---
id: MSH-33
title: Add bounded desktop analysis and explicit Full-mode execution
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T15:50:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-32
  - MSH-34
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Extend the internal protocol to version2 with required purpose and exact native target provenance; keep engine-turn acceptance separate from read-only ordinary/Full analysis acceptance. Use one supervised bridge with explicit play pause, analysis cancel/replace and Resume ownership. Run advanced cooperative mating reachability and all reconstruction in the child under recorded position/transition caps plus watchdog. Add native/actual-process and frontend isolation/limits/stale-result tests, readable evaluation/provenance UI, actual packaged cancellation/replacement/Full journeys, independent review, normal integration/push and applicable CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner clarification 2026-10-25: implement after MSH-34 in this batch. Ordinary bounded analysis is read-only with explicit request purpose and separate acceptance; it must never call the handler that applies an engine move/claim. Bind exact target record/history/selection, request/session/process identity, modes and limits. Document one supervised execution resource: analysis pauses/retires turn work, replacement retires old analysis, cancellation leaves play paused, explicit Resume retires analysis and starts a fresh turn. Full cooperative mating reachability is a distinct advanced supervised operation; expensive reconstruction/replay/adjudication stays off host interaction path. Cancellation/resource exhaustion yields unknown/incomplete, no chess result. Preserve modes/limits and reject unsupported restoration combinations. Show understandable pawn/mate-distance scores with perspective/analyzed position; raw identifiers/encoding in details; final actual telemetry sufficient. Existing AC preserved. See M3 design second-batch clarifications.

Dependency list now explicitly includes MSH-34 as well as MSH-32 to enforce the owner-requested batch order; MSH-35 retains both MSH-33 and MSH-34. No acceptance criteria were changed.

Owner @codex on reusable codex/m3-desktop checkout .claude/worktrees/next-task; MSH-32 and MSH-34 are Done. Isolated native builder /root/desktop_engine owns engine/session boundaries; /root/desktop_foundation owns frontend; primary owns console integration/package verification/docs/Backlog/host integration. Research API agreed before activation; no33implementation preceded34completion. Starting analysis pauses play until explicit Resume, which retires analysis and starts fresh eligible turn work. Full operation does not change live M2 adjudication; unsupported Full live restoration remains an explicit refusal.
<!-- SECTION:NOTES:END -->
