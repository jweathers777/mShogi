---
id: MSH-156
title: Independent Chess Benchmark
status: In Progress
assignee:
  - '@codex-coordinator'
created_date: '2026-11-10T12:57:00Z'
updated_date: '2026-11-10T13:09:00Z'
labels:
  - handle:independent-chess-benchmark
dependencies:
  - MSH-155
ordinal: 161000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Independent Chess Benchmark.

## Problem
The eight-root regression panel cannot establish general decision improvement.

## Objective
Freeze a manageable broad public-source benchmark with independent development and held-out collections.

## Approach
Verify sources and rights; select diverse difficulties and categories without engine-result selection; preserve provenance, available history and exact versus opinion expectations. Reuse existing experiment tools.

## Completion
Reviewed frozen data, leakage controls, practical-time measurement protocol and baseline enable the authorized engineering round.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A few hundred diverse sourced positions have verified licensing/provenance, preserved state/history availability and independently justified frozen expectation tiers.
- [ ] #2 Development and held-out groups prevent same-game/problem-family leakage; old eight roots are regression-only; promotion/replacement policy is explicit.
- [ ] #3 Existing infrastructure records equal-time decision quality and search telemetry with five seconds primary; no simple reference agreement is claimed as general strength.
- [ ] #4 Level 1 local validation and independent review support a reproducible baseline and coherent-round handoff.
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
1. Research source licenses/formats and bounded retrieval; inspect existing native frontier infrastructure.
2. Freeze approximately 200-300 diverse roots and grouped splits before baseline search; keep heldout details away from implementation.
3. Validate legal state/history/expectation semantics and freeze hashes; independently review design.
4. Run bounded five-second development baseline, preserve heldout evaluation for frozen candidate checkpoint, use one/fifteen seconds only where discriminating.
5. Level 1 local data/harness checks and review; no GUI/platform campaign or tournament. Integrate with next authorized coherent round.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Coordinator owns integration in existing assessment worktree, branch codex/chess-adversarial-assessment; base fb87ca2, production d53f344. Pinned Backlog 1.55.1. Independent source researcher and read-only coverage investigator active; no production edits before benchmark freeze.

Corpus frozen before baseline: 240 roots (160 development/80 heldout), 141 full source PGN histories; 48 exact TB maps incl8defensive draws and3 exhaustive mate-in-one objectives. Development SHA256 4c2dc1459b7be62201f04cb58cd7cbddc61483d8a90d9584fd16f3220bc00389; heldout b83342ce6944596fc4416e90a836a1f94360ddeb693a4871cf6e936d1a35d623. All-field replay, legal expectations and grouped known-line/nearboard/old-regression exclusions passed. One near-related draw pair regrouped before freeze. STS redistributed as retrieval references only due upstream rights uncertainty. Protocol docs/m5/independent-chess-benchmark.md. Native runner reviewed independently;18 offline fixtures passed; real100ms smoke passed. Five-second development baseline underway with byte-frozen predecessor55b38b74, unchanged96 production-source hashes. Global3300 active-search ceiling includes reference160/profile60 reservations; no matches.
<!-- SECTION:NOTES:END -->
