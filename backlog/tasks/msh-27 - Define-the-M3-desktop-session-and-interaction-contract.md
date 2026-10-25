---
id: MSH-27
title: Define the M3 desktop session and interaction contract
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T12:50:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-25
documentation:
  - docs/desktop/m3-design.md
ordinal: 27000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Desktop work needs one durable session, engine and interaction contract, with an explicit first-batch boundary and complete remaining milestone coverage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The M3 design maps all eleven required areas to native tickets with dependencies and explicit first-batch versus later implementation boundaries; M2 and existing task evidence remain intact.
- [x] #2 The contract specifies native authority, exact search context and process generation, optional claims, distinct outcomes/failures, cancellation, untimed defaults, mode preservation and unsupported restoration, and bounded Full execution.
- [x] #3 Interaction, historical-view separation, future timed suspension policy gate, real telemetry, synthetic-fixture labeling and three-platform packaged verification are specified and independently reviewed.
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
Reconcile existing milestones and task identities; record the researched native authority, lossless process interface and interaction contract; create dependency-scoped tickets covering all eleven areas; independently review and integrate the planning change before implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @codex, branch codex/m3-desktop, reusable checkout .claude/worktrees/next-task. Backlog CLI 1.55.1 matches repository baseline. No preexisting M3 tasks; MSH-27–39 added without altering M2.

Independent documentation review by /root/m3_contract_review assessed AGENTS, design and all MSH-27–39 against the eleven areas and bounded batch; no actionable findings. Existing M2 milestone and task evidence are unchanged. Official Tauri prerequisites and sidecar documentation consulted.

Integrated e8dafdf into codex/rewrite by fast-forward and normal push. Documentation-only contract closure; no native behavior changed, so existing M2 native evidence is reused.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Recorded the authoritative desktop/session/engine contract and mapped all eleven M3 areas into MSH-27–39. The first batch is limited to 27–32; remaining Ready tickets depend on their actual deliverables and M3 remains open. Independent documentation review found no actionable issues.
<!-- SECTION:FINAL_SUMMARY:END -->
