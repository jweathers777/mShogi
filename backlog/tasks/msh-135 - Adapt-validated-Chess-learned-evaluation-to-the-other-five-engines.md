---
id: MSH-135
title: Adapt validated Chess learned evaluation to the other five engines
status: To Do
assignee: []
created_date: '2026-11-08T13:07:00Z'
labels: []
dependencies:
  - MSH-55
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
priority: high
ordinal: 137000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner direction2026-11-08 sequences learned evaluation after established conventional engines: prove the complete new approach in Chess first (MSH-55), then extend the research to Shogi, Xiangqi, Tori, Chu and both accepted Tenjiku profiles. This follow-on preserves that requested scope without silently making all-game learned delivery a new MSH-56 closure dependency. Milestone placement/game increments are refined from the actual Chess result; no shared neural architecture, compute budget or improvement result is predetermined. Routine correctness/tactical tests and bounded profiling accompany coherent increments. Aggregate routine local engine-match work is limited to15minutes per task unless John explicitly authorizes more; no per-technique substantial statistical studies or multi-hour benchmarks. Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The validated Chess learned approach and relevant conventional game baselines inform a concrete per-game research scope, feature/action/history requirements, data provenance and regeneration/compatibility plan for all five remaining games including both Tenjiku profiles.
- [ ] #2 Game-specific learned implementations and native inference are adapted and independently verified in coherent increments, with truthful model/profile identities, dataset separation and preservation of rules, execution, cancellation and saved records; no automatic transfer of Chess features or architecture is assumed.
- [ ] #3 Major new-evaluation studies run only with explicit owner-approved resources/protocols. Actual improvement, regressions or limitations are independently assessed and documented per game; loss curves or small development matches never substitute for formal strength claims.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
