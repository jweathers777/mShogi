---
id: MSH-173
title: Frozen Legacy Opponent
status: Ready
assignee: []
created_date: '2026-11-14T14:14:00Z'
labels:
  - handle:frozen-legacy-opponent
milestone: m-5
dependencies: []
references:
  - docs/design.md
  - docs/m5/strength-roadmap.md
  - >-
    backlog/tasks/msh-52 -
    Establish-and-meet-measured-chu-shogi-engine-strength-criteria.md
priority: medium
type: task
ordinal: 177000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Frozen Legacy Opponent.

**Problem**
The original C++ Chu engine is a useful potential baseline, but the current harness cannot identify and reproduce a build of it. Its speed and strength relative to the rewrite are unmeasured.

**Objective**
Provide a reproducible, headless frozen opponent from original revision 941cd4dc37e6e6210d4f993c96a553753e228b19, without changing its search, evaluation or rules to improve compatibility or strength.

**Approach**
Keep historical source and modern product code separate. Record any minimum compiler/build portability changes as an explicit patch over the frozen source. Build only what the console/engine needs; GUI support is not a prerequisite. Retain licenses and source/build provenance. This supplies MSH-52 (Chu Engine Strength); it does not authorize a rating study, substantial compute campaign or product dependency on the legacy engine.

**Completion**
A documented build and bounded smoke run identify the original source, patch, compiler, platform, binary hash and selected search settings. Large binaries and build output stay outside Git. Record risk-based verification and independent review appropriate to build/process changes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A documented clean build produces a headless Chu opponent from the pinned original revision, with required dependencies and exact build commands recorded.
- [ ] #2 The source revision, explicit compatibility patch, compiler/options, platform, executable hash and chosen legacy search settings are retained; search, evaluation and rules are unchanged and the build is reproducible without a private local checkout.
- [ ] #3 A bounded smoke run starts Chu, accepts a legal ordinary move, produces an actual search move and terminates with stdout/stderr, exit status and process cleanup evidence.
- [ ] #4 Legacy artifacts are optional experiment inputs, not product, ordinary CI or conformance dependencies; original history and required notices are preserved.
- [ ] #5 Actual successful and failed build/run checks and limits are recorded at the tested revision, with proportionate independent review; untested platforms and playing strength are not claimed.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
