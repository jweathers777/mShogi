---
id: MSH-112
title: Investigate early-exit runner failure classification race
status: To Do
assignee: []
created_date: '2026-10-29T12:53:00Z'
updated_date: '2026-10-30T21:40:00Z'
labels:
  - native-ci
  - runner
milestone: m-4
dependencies: []
references:
  - https://github.com/jweathers777/mShogi/actions/runs/40874033735
documentation:
  - docs/m4/evidence/msh102/completion-2026-10-29/README.md
priority: medium
type: bug
ordinal: 112000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Intermediate MSH-102 Linux CI40874033735 at6dec5ab failed process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored at crates/mshogi-runner/tests/runner.rs:172. Fixture exit expected UnexpectedExit but observed MissingResponse, engine0 handshake, Broken pipe (os error32), no exit status. Runner source was unchanged across095f27a through finala1f8234; final three-platform CI passed, which does not establish a fix. Preserve MSH-25 historical evidence and its earlier isolated-awaited-engine correction; this observation differs from its idle-other-engine race. Future bounded runner diagnosis only; no desktop/rules change, M2 reopening or gate on unaffected Xiangqi work. Implementation is not started.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Establish a controlled reproduction and identify early write-error, EOF and process-exit observation ordering; retain failed CI evidence and prior MSH-25 rationale.
- [ ] #2 Resolve the product or fixture defect under the documented bounded failure contract. Preserve meaningful UnexpectedExit versus MissingResponse distinctions, established exit diagnostics, bounded cleanup and failures as unscored; do not merely accept either category or raise timeouts to mask the mismatch.
- [ ] #3 Focused controlled process tests and relevant native regressions pass with independent review and actual platform CI; record remaining nondeterminism honestly.
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
Recurrence observed during MSH-119 final verification at eb027be7cec0f3206094c477a31f0cd9b9f73f6f: Linux Rust CI41131548408 job124323454077 workspaceTest runner.rs172 expectedUnexpectedExit but observedMissingResponse(engine0 handshake, Brokenpipe32, exitNone, at_ns2229415). Runner/engine/config sources unchanged from82a8ae8; initial291Linux behavior suite passed before separate fixture-diff gate. Focused unchanged test passes once on local macOS (cargo test --locked -p mshogi-runner --test runner process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored -- --exact;1PASS,0.59s). This does not establish a fix or Linux success. Original failure retained in MSH-119 evidence; normal focusedCI retry pending workflow completion. No assertion, timeout, productionrunner or ownership/status change; MSH-112 remains To Do/unstarted.
<!-- SECTION:NOTES:END -->
