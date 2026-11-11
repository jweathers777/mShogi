---
id: MSH-162
title: Parallel Experiment Runner
status: In Progress
assignee:
  - '@codex-cloud-runner'
created_date: '2026-11-11T18:35:00Z'
updated_date: '2026-11-11T18:52:00Z'
labels:
  - handle:parallel-experiment-runner
dependencies: []
ordinal: 166000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Problem
MSH-161 (Competitive Gameplay Diagnosis) serial execution is too slow.

## Objective
Handle: Parallel Experiment Runner
Deliver reusable local and GCP Batch execution over native match/replay and frozen analysis.

## Approach
Versioned immutable jobs, bounded workers, dependency scheduling and verified artifacts. Preserve frozen study and coordinator ownership.

## Completion
Reviewed isolated branch handoff. Cloud pilot requires project/access/spending approval; coordinator owns integration.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Game pair, all-move screening and confirmation jobs reuse native and frozen analysis authorities.
- [ ] #2 Parallel local/GCP jobs support dependencies, integrity, retries, immutable success, status/retrieve/cancel and resources.
- [ ] #3 Targeted macOS/Linux tests and independent source/execution review cover replay, failure and restart.
- [ ] #4 Authorized cloud pilot proves complete pair and analysis, recovery and eight-worker throughput/cost.
- [ ] #5 Reviewed operating and integration handoff preserves frozen inputs.
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
1. Wrap frozen pair/query/extraction functions with portable manifests and stable identities.
2. Implement immutable stores, local dependency execution and GCP Batch one task per VM with GCS conditional writes.
3. Provide CLI and fixed Linux packaging.
4. Level 2 macOS/Linux process/storage tests and independent review; actual cloud gate after owner authorization. No desktop campaign.
5. Preserve isolated branch for coordinator integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner codex-cloud-runner; branch codex/parallel-experiment-runner; worktree /Users/johnweathers/.codex/worktrees/parallel-experiment-runner/mShogi; base c01d54bf. Cached CLI 1.55.1 matches baseline; PATH 1.55.0 will not mutate records.

Implementation ready for source checkpoint: versioned immutable local/GCS jobs, native pair/replay and frozen screening/confirmation adapter, Linux physical-core process isolation, Batch one task per VM, status/recovery/retrieve/cancel/resources and event-driven Pub/Sub exit notification. Independent reviews found and resolved dependency substitution, missing derived confirmations, signal cleanup, notification accounting, runtime/store identity and pipeline coverage defects. Targeted Mac/Linux ARM tests and native replay performed; exact final-source record follows. No new study searches/matches, cloud resources, integration or coordinator communication. Pending owner GCP project/region/service-account/bucket/registry and spending authorization, fixed Linux x86 artifacts and real pilot.
<!-- SECTION:NOTES:END -->
