---
id: MSH-176
title: Legacy Runner Adapter
status: Ready
assignee: []
created_date: '2026-11-14T14:14:00Z'
labels:
  - handle:legacy-runner-adapter
milestone: m-5
dependencies:
  - MSH-174
  - MSH-175
references:
  - crates/mshogi-runner/src/experiments/mod.rs
  - crates/mshogi-runner/src/experiments/run.rs
  - docs/m4/chu/engine-v1.md
  - docs/m5/strength-roadmap.md
priority: medium
type: task
ordinal: 180000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Legacy Runner Adapter.

**Problem**
The shared six-game experiment runner already owns native Chu state, paired schedules and replayable evidence, but its participants expect the rewrite internal-search interface. The old C++ engine cannot participate directly. The Chess-specific UCI strength harness is not a Chu runner.

**Objective**
Connect the frozen legacy Chu opponent to the existing shared runner for bounded, reproducible comparisons against the current Chu engine.

**Approach**
Reuse shared scheduling, native authority, supervision and reporting. Translate complete moves through the legacy interface and apply the shared opponent adjudication policy. Preserve distinct native and legacy states and detect disagreements. Record original binary and wrapper/adapter identities, settings and actual compute. Compare equal elapsed-time conditions where supported; do not present equal depth/node budgets across different engines as equal computing work. Unsupported time controls require explicit refusal or a clearly labeled alternative. No GUI integration, legacy-engine product dependency, old-rule emulation in the core or new strength target is required.

**Completion**
A bounded smoke schedule runs both side assignments against the real current and wrapped legacy engines, retains complete native records and failure evidence, and replays without either binary. This prepares MSH-52 (Chu Engine Strength). A substantial campaign or rating claim remains separate work with explicit authorization; smoke evidence alone establishes neither engine superiority nor speed superiority.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Runner configuration can select the wrapped legacy Chu participant and current native Chu participant, with exact source/binary/wrapper identities, search settings, profile and effective resource bounds recorded.
- [ ] #2 Complete native history and complete legacy choices are translated without loss of Lion routes or promotion; every accepted move passes the native authority and observed legacy rejection/state disagreement follows the shared policy.
- [ ] #3 The existing shared schedule runs a finite real-engine Chu smoke in both side assignments; work caps, per-search watchdogs, whole-run limits, stderr/stdout bounds and process cleanup are effective.
- [ ] #4 Canonical reports replay the accepted history and validate outcomes/dispositions without the opponent binaries. Native results, experimental forfeits, capped games, mismatches and process failures stay separate in summaries.
- [ ] #5 Focused adapter tests cover ordinary play, promotion, double capture/return, disagreement and execution failure; shared/existing protocol paths receive relevant regressions and independent review at the exact tested revision.
- [ ] #6 Documentation provides reproducible build/run/replay commands, fair-comparison conditions, actual compute and known limits; no speed/strength superiority or formal rating is inferred from the smoke test.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
