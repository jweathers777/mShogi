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

One `EngineBridge::submit` and `poll` serve all six games and seven immutable
profiles. The one-shot
`mshogi internal-search` child parses this request and emits this response.
There is no version negotiation, legacy-shape fallback, Chess transport adapter,
paired polling API or second rules/search model. Strict typed parsing preserves
raw canonical payload bytes for their authoritative native parsers; duplicate,
unknown, missing and conflicting fields reject. Unsupported versions never
execute. A parseable current-shaped request with an unsupported version returns
an explicit unscored InvalidRequest response; an old flat shape is malformed.

MSH-49.1 adds optional Chess fields within this same family. `WorkLimits` keeps
mandatory `depth` and `nodes`, with optional `time_ms` (0–60,000 milliseconds)
for ordinary Chess engine turns or move analysis. The exact time limit is part
of context equality, request replacement and durable observations. Other games
and Chess Full refuse this field. Omission serializes exactly as before, so
existing deterministic requests and observation digests retain their encoding.
An older strict child may reject the new timed field; the host does not retry
with downgraded limits or a different protocol. Bundled host/child are built
together. The process watchdog is separate and must accommodate the requested
thinking interval plus transport and cleanup; it is never a chess clock result.

`CanonicalRecord` explicitly carries game, immutable profile/edition, native
schema and native root/provenance/complete known history. This does not flatten
native states, sides, actions, rule-dependent commands or score meanings.

## Purpose and native results

`ExecutionPurpose` distinguishes live `engine_turn`, read-only `move_analysis`
and Chess-only `chess_full_reachability`. Analysis binds `ExecutionTarget`:
selection identity, complete native `event_index`, known `board_ply`, session
version, native label and game-qualified side. Same board placement is not a
substitute for exact canonical context/history.

The result tag selects native `ChessResult`, `ShogiResult`, `XiangqiResult`,
`ToriResult`, `ChuResult` or `TenjikuResult`,
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

Chess's conventional search reports nominal `completed_depth`, all attempted
`nodes` and optional `selective_depth`: the deepest actually visited board ply
including extensions and quiescence. A completed iteration's PV can therefore
exceed its nominal depth, bounded by the recorded selective depth and the native
128-ply path cap. Every complete action and terminal claim is still re-admitted
by the native owner; a claim's intended move is not played. Old observations
without selective metadata keep their nominal-depth board-PV bound, plus a
possible final claim command. A terminal root or root claim can have selective
depth0, while a first incomplete iteration can have nonzero attempted depth.
Neither selective depth nor a search score establishes a native game outcome.

The coordinator carries the exact optional soft/hard allocation with its request.
Soft time stops new iterations; hard time and cancellation are polled during
search. Only the last completed iteration can supply a choice. Zero thinking
time remains honestly incomplete, with no fallback action or invented score.
UCI and supervised Chess use this same native timed search. Current live desktop
defaults are a128 nominal depth ceiling,20,000 visits and1,000ms hard time; analysis
defaults and all other game budgets are unchanged. Current-stage documents with
the prior fixed depth2/20,000-visit untimed defaults still open stopped, preserving
their records and observations. Explicit Resume selects the new defaults for
future live work; opening or resuming never rewrites the original file.

## Execution safety and bounds

Completed Tori, Chu and Tenjiku delivery uses this same family for real bounded
engine turns and read-only move analysis. Native complete ordered routes,
stationary captures, promotion/refusal rights, derived effects and conclusions
remain game/profile-owned. Both Tenjiku whole profiles retain exact identity at
every node and observation; cancellation, cross-game or cross-profile replacement
cannot reuse a request. See the [current shared integration map](../m4/shared-integration/README.md)
and linked game contracts for native scoring, limits and accepted evidence.
The earlier Chess/Shogi/Xiangqi result descriptions are examples, not an exclusion
of the three subsequently delivered games.

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

Subsequent MSH-117/R2 delivered the sole current outer desktop envelope and
deleted old desktop version/read branches. MSH-118/R3 and MSH-119/R4 delivered
the native runtime/document owner and single frontend session, deleting the
finite installation/forwarding stages and remaining Chess product projection.
See the [current document](document-v1.md) and [native application](native-application.md)
contracts. R1 introduced no permanent transport adapter. Native Chess
presentation types remain meaningful domain data, not an alternate engine
execution interface.
