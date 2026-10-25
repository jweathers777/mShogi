---
id: MSH-33
title: Add bounded desktop analysis and explicit Full-mode execution
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T16:11:00Z'
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

Implementation/evidence ready: protocol2 requires purpose and exact native target; separate analysis acceptance never invokes live move/claim application. Single bridge ownership pauses play, cancel/replacement retire old work and explicit Resume starts fresh eligible work. Full cooperative solver/reconstruction runs only in bounded/watchdog child, never changes live M2 mode; unsupported Full live records refused before replay. Focused checks passed:22session all-feature,2engine unit+8protocol,2publiccontroller actualanalysis+4play+5internalprocess tests,35frontend tests, strictTS/Vite, native/host format+Clippy/tests, final normal package build and all four packaged interfaces/operations. Actual normal macOS GUI: depth2/20k analysis442nodes suggests a2a3 without playing; depth8/5m cancellation no final counts; active replacement with depth1/1k yields21nodes; Full one-position/transition yields Unknown with actual1/1work, unchanged0plyboard/M2mode; explicitResume then e2e4/a7a5 actualreply642nodes. Screenshots and exact package identities/limits in docs/desktop/second-batch-evidence.md and evidence/analysis-local-macos.json. Independent /root/m3_contract_review clean on native/frontend/purpose/resource semantics and focused resignation retirement delta. Integration and exact-source native/package CI remain before Done.

Final independent evidence review /root/m3_contract_review inspected all four normal-app screenshots, retained report and MSH-33 documentation: read-only442node suggestion, cancellation without accepted counts, FullUnknown1position/1transition, and explicit-resume642node real reply are visible/readable and honestly scoped. No findings. Exact-source integration and native/package CI remain completion gates.
<!-- SECTION:NOTES:END -->
