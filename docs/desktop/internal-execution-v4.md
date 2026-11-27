# Internal engine execution

`mshogi-engine` provides one supervised execution interface for six games and
seven rules profiles. The child executable runs `mshogi internal-search`. It
reads one JSON request per line, writes one final JSON reply per request and
flushes the reply before reading again. UCI, USI and Xiangqi's external UCI
adapter have separate console contracts.

## Protocol 7

A full request is `{ protocol: 7, context }`. `ExecutionContext` contains
`purpose`, `request_id`, `session_version`, `game` and `limits`. The game is
a canonical record with the exact rules profile, record schema, root and known
history. `purpose` is `engine_turn` or `move_analysis`. An analysis target
has a selection ID, event index, known board ply, session version, label and
game-qualified side. Context equality includes the full history and limits.

Timed Shogi, Xiangqi, Tori, Chu and Tenjiku start with a full request. Once
the child acknowledges an endpoint, subsequent timed requests can use
`{ protocol: 7, session, update }`. The session carries the same request
metadata without the game. An `advance` update names the acknowledged request
ID as its base and submits the game-tagged suffix. The child admits the suffix
through game replay and validates any analysis target. A `reset` update
supplies a full record when the earlier history cannot be extended. Chess and
all fixed-work requests use the full request form.

The reply is `{ protocol: 7, request_digest, endpoint, result }`. The digest
is the SHA-256 of the exact encoded request bytes, excluding the framing
newline. The endpoint is null or `{ request_id }` for an acknowledged timed
session. The host checks the reply's shape, version, digest, endpoint and
result against its pending request. A valid reply consumes that request once.
The host attaches its own submitted context to make an `ExecutionObservation`
(`{ schema_version: 1, context, result }`) and wraps it with the protocol
version in an `ExecutionResponse`. The process sends only the reply above.

Typed fields and game record parsing enforce the current shapes. A
current-shaped request with an unsupported version returns an unscored
`InvalidRequest` failure. Malformed envelopes or absent line framing end the
child exchange. Request and reply lines have no byte limit. Retained child
stderr is bounded to 4,096 UTF-8 bytes.

## Search and result admission

`WorkLimits` contains `depth` and `nodes`, and may contain `time_ms` from
0 to 60,000 milliseconds. The budget belongs to one search. The host process
watchdog separately covers the exchange and cleanup. Fixed work omits
`time_ms` and uses fresh search state. Each game validates its search limits
and defines its own scoring and terminal-distance units.

The result is game-tagged or a typed unscored failure. Chess can return a
move, a current or intended-move claim, a concluded root, or incomplete work
with actual telemetry. An intended-move claim does not play the move. The
other adapters can return a game choice, a conclusion or incomplete work.
Xiangqi can also report unresolved rule adjudication. A timed adapter can
report a legal choice without a completed iteration or score if its search
supports that result. See the [shared search core](../m5/shared-search-core.md).

A live choice must pass the game rules under the current pending context before
the desktop commits the command. Analysis validates the exact selected prefix
and presents its result read-only. Chess telemetry can include selective depth
and a principal variation longer than the nominal iteration depth. Search
scores, cancellation and engine failure do not decide the game outcome. An
incomplete or cancelled search can leave no choice to admit.

## Child lifecycle

The bridge has one worker, one replaceable pending request and one result
slot. It requires strictly increasing nonzero request IDs. It can keep an
idle child after a valid reply. Cancellation, replacement of running work,
failure, watchdog expiry and shutdown retire the child. A child that exited
normally while idle can be replaced once. Unexpected exit during work is
reported with bounded diagnostics.

Timed requests can reuse search state for exact same-game, same-profile
descendants. A timed game switch or incompatible history starts fresh.
Fixed work does not advance the retained timed endpoint. Desktop New game,
Load and Take back retire the preceding execution, including idle child
state. Runtime authority is never recovered from saved history. The
[desktop session contract](native-application.md) describes observation
retention, and the [document format](document-v1.md) describes Save and Load.
