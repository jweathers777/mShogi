---
id: MSH-233
title: Simplify Internal Execution Identity and Validation
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-23T11:05:00Z'
updated_date: '2026-11-23T17:58:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Trace runtime, report and retained-session consumers; measure full-response bytes/decode cost and native validation on short/long histories. 2. Use one compact response contract bound to the exact encoded request, reusing the submitted context instead of returning and replaying its record; preserve full recorded observations and retained Shogi/Tori sessions. Remove duplicate runtime identity checks where one monotonic request identity already supplies the same authority. 3. Reuse admitted native state during result validation and command application instead of replaying it at adjacent boundaries; keep native PV/proof checks and atomic candidate installation. 4. Fresh review of bounded changes, compatibility/stale/cancellation tests, required Rust/frontend and affected-platform process checks, final measurements, integration and cleanup.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main9ec1d05. Trace runtime identity and validation consumers, measure current bytes/replay work, then remove demonstrated redundancy in reviewed steps. Preserve recorded report replay, retained Shogi/Tori sessions, exact stale-result checks and native command admission. Branch codex/msh-233-execution-validation.

Consumer trace: bridge State keeps last_id and last_generation, both advance per request; desktop assigns both the same next_request. Full response decode reparses/replays the echoed CanonicalRecord, then Client.decode and bridge exchange both call matches. Incremental replies already omit the record but full/first replies use a separate ReadyResponse shape. Observation schema1 and runner decode_step preserve process_generation and full target provenance in recorded reports; avoid rewriting that saved evidence merely to simplify the live wire. Native worker reconstructs response.context.game even for an already admitted live target, then document.with_command replays it and state.game replays the resulting record again. Chu/Tenjiku PV validation also encodes/replays an already admitted root. These are the initial measured candidates; no rule/search changes planned.

Checkpoint: baseline benchmark added (pending commit), branch codex/msh-233-execution-validation. Release median-of-five baseline in runs/msh-233/baseline: Chu at 128 events response 18359 bytes, decode 301755 us, context clone 0.375 us, native result validation 298706 us. Tori 16861 bytes/3878 us decode; Tenjiku profiles about 28 KB/1.9 ms decode. Synthetic legal one-move PV measures admission, not strength. No product changes yet. Next: compact request-bound replies, review, then adjacent native validation reuse and required checks. MSH-235 tracks unrelated stale native-application documentation.
<!-- SECTION:NOTES:END -->
