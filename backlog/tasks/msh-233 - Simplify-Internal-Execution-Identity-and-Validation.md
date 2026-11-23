---
id: MSH-233
title: Simplify Internal Execution Identity and Validation
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-23T11:05:00Z'
updated_date: '2026-11-23T17:44:00Z'
labels:
  - simplification
  - engine
milestone: m-6
dependencies: []
priority: medium
ordinal: 240000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Split from MSH-229: ExecutionContext echoes complete records and has request_id, process_generation and session_version; desktop assigns request_id and process_generation the same value. Analysis targets repeat version/turn/event facts, and adapters replay complete result lines and native histories at several boundaries. Reduce redundant runtime identity, context copies and repeated validation while preserving exact pending-request admission, native legality of applied commands, bounded child communication and honest analysis. Recorded experiment reports and measured incremental Shogi/Tori sessions remain supported.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Trace each identity/context field and validation pass to its consumer; remove redundant runtime fields/copies or checks with measured evidence, in small reviewed steps
- [ ] #2 Stale, cancelled, cross-game and cross-profile results cannot apply moves; invalid results do not create outcomes; accepted PV/proof presentation remains truthful and bounded
- [ ] #3 Recorded reports still replay, retained sessions still work, and required Rust/frontend/process checks pass; record bytes, timings and line counts
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main9ec1d05. Trace runtime identity and validation consumers, measure current bytes/replay work, then remove demonstrated redundancy in reviewed steps. Preserve recorded report replay, retained Shogi/Tori sessions, exact stale-result checks and native command admission. Branch codex/msh-233-execution-validation.
<!-- SECTION:NOTES:END -->
