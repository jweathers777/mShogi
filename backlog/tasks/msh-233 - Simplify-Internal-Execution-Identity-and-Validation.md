---
id: MSH-233
title: Simplify Internal Execution Identity and Validation
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-23T11:05:00Z'
updated_date: '2026-11-23T18:20:00Z'
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

Checkpoint: compact protocol5 reply implemented; request SHA-256 binds exact bytes, one Client decoder attaches retained submitted context, old full/ReadyResponse wire removed, duplicate bridge matches removed. Recorded Observation schema1 remains unchanged. Fresh wire review found no product defects after fixture/test migrations; no tests run by reviewer. Native reuse now implemented: selected admitted candidate validates result; native command applies once before document serialization; Chu/Tenjiku explicit fork copies admitted history with fresh runtime identity for PV validation. Added replay-differential/token isolation test. Removed redundant bridge last_generation counter and retained-session stamp generation/version/event copies; request ID is monotonic authority. These latest identity/native changes need review. Full debug workspace build/tests in progress; prior intermediate release engine run failed on old test decoders now migrated. Branch codex/msh-233-execution-validation; not integrated.

Fresh native/identity review complete: no product findings; updated Chu/Tenjiku accounting tests to charge only actual PV transitions (the reviewer findings). Fmt and full workspace clippy passed. Required debug workspace test run stopped at the same four known MSH-199 Chu process/controller watchdog failures (~30.39 s); two Chu tests passed. No rule expectations loosened. Full release workspace run and frontend checks are in progress. Protocol smoke consumer updated to verify exact SHA-256 binding before reconstructing semantic report observations. Remaining identity fields in ExecutionContext/targets stay for durable report provenance and target consistency; runtime now keeps only monotonic request_id, with the pending digest binding all fields. Removed unused observation-target replay trait method.

Checkpoint b5dd08d (preceded by f05da67 and consumer fixes 0a13793): both required fresh reviews complete, frontend TypeScript and all600 tests pass. main merged (already current). Final identity regression proves reused historical generation does not prevent retained suffix execution; exact monotonic request ID remains mandatory. Host ExecutionResponse no longer implements Deserialize, so typed consumers cannot silently decode child bytes as saved context. Targeted process workflow45235109958 running Linux/macOS/Windows at f05da67. Release workspace still compiling with2 workers; later verification-helper migrations need final build. Pending: all release results, final fmt/clippy/debug after main merge, benchmark after numbers, reports compatibility confirmation, upload run evidence, integrate and clean branch/target/runs. Branch codex/msh-233-execution-validation.

Measured9ec1d05 (benchmark a9526ec) versus1c13156 on Apple M1 Pro, release, median5 same deterministic legal one-move PV, encode prepared outside decode timer, no concurrent compiler. At128 events: Tori reply16861->561 bytes, decode3878->14.3 us, validation28.5->31.2 us (unchanged path); Chu18359->529 bytes, decode301755->6.17 us, validation298706->241.7 us; Tenjiku Adams28408->573 bytes, decode1936->10.4 us, validation1791->43.5 us; Muller28192->609 bytes, decode1976->10.6 us, validation1763->41.5 us. Context copies were sub-microsecond except Tori3 us, so retained for recorded observations. Request bytes unchanged. Synthetic admission workload, not search/strength or end-to-end latency. All0/16/128 samples and source line counts in runs/msh-233/measurements.json. No GCP compute, $0. Final-source fmt/clippy pass; debug again stops at same fourMSH-199 failures30.39s. Final release workspace and platform workflow45236400013 at1c13156 running.

Additional verifier check: package-smoke query sends UTF-8 bytes, avoiding Windows newline translation in the exact request digest. Extracted the unchanged query function and exercised the actual release engine for Chess and both Tenjiku profiles: all three passed. No packaged GUI/release campaign was run. Tracked .rs/.ts/.tsx/.py/.mjs source lines: 171583 -> 172009 before this one-line verifier clarification; the increase includes the benchmark and regression coverage. Runtime simplifies three reply paths to one; no claim that total source shrank.
<!-- SECTION:NOTES:END -->
