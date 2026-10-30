# Current internal execution and durable observations (MSH-116 / R1)

This is the sole supported internal engine boundary. Development transports
v1/v2/v3 are retired. External UCI, USI and Xiangqi interoperability are separate,
unchanged functional contracts. See [R1 evidence](../review/execution-migration-116.md).

## Ownership and encoding

`mshogi-engine` exports one family:

- `ExecutionContext`: `purpose`, `request_id`, `session_version`,
  `process_generation`, `game: CanonicalRecord`, and `limits: WorkLimits`.
- `ExecutionRequest`: `{ protocol: 4, context: ExecutionContext }`.
- `ExecutionObservation`: `{ schema_version: 1, context: ExecutionContext,
  result: ExecutionResult }`. This is durable semantic evidence, with no wire
  version or pending-execution meaning.
- `ExecutionResponse`: `{ protocol: 4, observation: ExecutionObservation }`.

One `EngineBridge::submit` and `poll` serve all three games. The one-shot
`mshogi internal-search` child parses this request and emits this response.
There is no version negotiation, legacy-shape fallback, Chess transport adapter,
paired polling API or second rules/search model. Strict typed parsing preserves
raw canonical payload bytes for their authoritative native parsers; duplicate,
unknown, missing and conflicting fields reject. Unsupported versions never
execute. A parseable current-shaped request with an unsupported version returns
an explicit unscored InvalidRequest response; an old flat shape is malformed.

`CanonicalRecord` explicitly carries game, immutable profile/edition, native
schema and native root/provenance/complete known history. This does not flatten
native states, sides, actions, rule-dependent commands or score meanings.

## Purpose and native results

`ExecutionPurpose` distinguishes live `engine_turn`, read-only `move_analysis`
and Chess-only `chess_full_reachability`. Analysis binds `ExecutionTarget`:
selection identity, complete native `event_index`, known `board_ply`, session
version, native label and game-qualified side. Same board placement is not a
substitute for exact canonical context/history.

The result tag selects native `ChessResult`, `ShogiResult` or `XiangqiResult`,
or a bounded unscored `Failure`. Chess retains move versus intended/current
claim, concluded root, incomplete work, actual telemetry and Full cooperative
proof/Unknown. Shogi retains complete move/drop/promotion, formal declaration,
conclusion and native incomplete work. Xiangqi retains Red/Black semantics,
complete WXF history, warning/claim state, sound-fast-only search and unresolved
adjudication without a fabricated score. Chess Full is not forced-win search or
a Shogi/Xiangqi analysis capability. Native Xiangqi in-process exact evidence
retains its separate explicit bounded operation and original provenance.

The server executes Chess directly from its native canonical record, rejecting
unsupported Full live restoration before replay. It never creates a retired
Chess request. The authoritative desktop owner checks exact current context,
purpose and native command admission. Ordinary analysis uses a separate read-only
acceptance handler and cannot issue a live move or claim.

## Execution safety and bounds

Request and response equality includes full history, selection/purpose, request,
session/process identities and actual limits. Replacement/cancellation retires
pending and already published work; semantic undo and historical selection do
not restore execution authority. Current document reopening allocates fresh
runtime identities and leaves work stopped until explicit Resume.

The existing one-worker/one-replaceable-pending/one-result supervision model,
watchdogs, bounded pipes, cleanup and diagnostic handling remain. Encoded caps
are unchanged: request1MiB, response2MiB, retained diagnostic4096UTF-8bytes.
Admission measures actual current framing, not an inner payload estimate.
Cancellation/Unknown/incomplete work/failure never invents a fallback move,
score or game outcome. Cancellation remains recorded in native desktop controls;
absence of a final observation does not invent search telemetry.

## Current-stage persistence and next boundaries

Chess `ArchivedObservation` and native `PresentedObservation` contain an
`observation: ExecutionObservation`. Validators replay authoritative native
history/prefixes and verify native result/variation/proof facts, identities,
limits and perspective. Stored observations are historical evidence, never
resumable pending requests. Take back retains abandoned histories and their
observations; Xiangqi exact evidence is not discarded or turned into an outcome.

R1 changes observation contents in the current-stage desktop codecs; it does
**not** promise to read development-era saves. Such old observation encodings
reject strictly, without falling back to a retired engine parser. Current-stage
Save/Open, atomic file failures, fresh stopped restoration, capacity admission
and reserved stop/close headroom remain tested. No archive/resource constant is
raised. The Xiangqi payload now records `observation_schema_version: 1` instead
of coupling durable data to an engine protocol number.

R2/MSH-117 owns the new single current outer desktop envelope and deletion of
old desktop version/read branches. R3 owns native runtime/document ownership;
R4 owns the single frontend session and removal of the remaining Chess product
projection. R1 introduces no temporary transport adapter and does not start any
of those tasks. Native Chess presentation types remain meaningful domain data,
not an alternate engine execution interface.
