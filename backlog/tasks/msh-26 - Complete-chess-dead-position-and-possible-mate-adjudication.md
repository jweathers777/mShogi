---
id: MSH-26
title: Complete chess dead-position and possible-mate adjudication
status: Done
assignee:
  - '@completeness-builder'
created_date: '2026-10-24T20:48:00Z'
updated_date: '2026-10-24T23:37:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-17
  - MSH-18
documentation:
  - docs/rules/chess-coverage.md
ordinal: 26000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John approved a sound incomplete M2 recognizer on 2026-10-24. Full dead-position detection and side-specific possible-mate adjudication remain chess-completeness obligations; unknown cannot justify a timeout/resignation result.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Full dead-position and side-specific possible-mate behavior has primary-rule conformance evidence, including blocked positions outside the M2 material subset.
- [x] #2 Timeout and resignation unknown cases are resolved only with justified evidence; ledger, profile/version migration and regression fixtures describe the remaining completeness honestly.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Implement exact cooperative Article-3 legal reachability for side-specific mate, with exact visited identities, positive witnesses, exhaustive negative proofs and explicit resource/cancellation uncertainty. 2. Supply complete/unlimited mode plus operational work limits, and integrate sound results into full dead-position/live and timeout/resignation paths with recorded configuration and compatible version migration. 3. Verify blocked positions beyond the M2 subset, forced-capture and witness cases, graph exhaustion/reference traversal and resource limits; document correctness/termination and practical limitations, obtain independent review and native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Dependency-ready after MSH-17/18 completion. Delegate owns isolated core completeness implementation and its tests/docs while parent owns search/protocol integration. Full detection must not be relabelled from a bounded heuristic. Primary research confirms exact finite cooperative reachability is a valid decision procedure; unfinished budgeted searches remain Unknown. Full completeness does not imply a practical runtime guarantee. Preserve M2 default behavior and old records through explicit configuration/version handling.

Implemented exact finite cooperative Article-3 reachability with exact board/turn/rights/legal-EP keys, normalized counters, sorted legal actions, checked witnesses and exhaustive negative proofs. Full/unlimited mode is complete given sufficient resources, without a practical arbitrary-position guarantee; bounded/cancelled work remains Unknown. Recorded opt-in live/match mode caches dead analysis, resolves timeout/resignation only from proof, and preserves defaults/legacy schema1 through explicit schema2 migration. Independent wall expectation 1152 vertices/6240 transitions agrees with independent DFS, plus forced-capture, live move/undo, budget boundaries, generated match evidence and corruption tests. On macOS27 arm64 Rust1.89, 36 focused core tests and Clippy passed; console mode-dropping rejection and coordinator mode-identity regressions passed. Independent review_profile found no material findings and independently reran all38 focused checks against pinned FIDE laws. docs/rules/chess-completeness.md records correctness, termination, migration and resource limitations. Parent retains Backlog/integration ownership.

Integrated normally into origin/codex/rewrite at source 94a7f45026cb07b6b9d092a9e9bd60d6343eb8ea. Native CI https://github.com/jweathers777/mShogi/actions/runs/40143374953 passed Rust (ubuntu-24.04) (job 121019557914), Rust (macos-15) (job 121019558099), Rust (windows-2022) (job 121019558145); formatting, Clippy, locked workspace tests/build and console smoke all passed. Closure bookkeeping reuses this exact source evidence. Existing deep/oracle evidence is retained for unchanged rules.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added opt-in exact Article-3 mating reachability and live/match adjudication with verified witnesses, exhaustive negative proofs and explicit resource uncertainty. Blocked-wall and migration/integration regressions, independent review and Linux/macOS/Windows CI passed. Default M2 subset remains unchanged; unlimited completeness assumes sufficient resources, and bounded unknown timeout/resignation stays unscored.
<!-- SECTION:FINAL_SUMMARY:END -->
