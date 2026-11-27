# Current internal execution and session observations

This is the sole supported internal engine boundary. External UCI, USI and
Xiangqi interoperability are separate functional contracts.

## Ownership and encoding

`mshogi-engine` exports one family:

- `ExecutionContext`: `purpose`, `request_id`, `session_version`,
  `game: CanonicalRecord`, and `limits: WorkLimits`.
- `ExecutionRequest`: `{ protocol: 7, context: ExecutionContext }`.
- `ExecutionObservation`: `{ schema_version: 1, context: ExecutionContext,
  result: ExecutionResult }`. This is semantic evidence, with no wire
  version or pending-execution meaning.
- Process reply: `{ protocol: 7, request_digest: [32 SHA-256 bytes],
  endpoint: null | { request_id }, result: ExecutionResult }`.
- `ExecutionResponse`: the host's admitted `{ protocol: 7, observation }`.
  This is not the child wire shape.

The digest covers the exact encoded request bytes, excluding one framing newline.
The process client retains that digest and the immutable submitted request. It
checks the bounded, duplicate-free reply against the pending digest, protocol,
endpoint and engine result family/shape, then attaches its own submitted context.
A reply consumes the pending request once. Invalid replies do not advance the
retained endpoint. There is no echoed record to deserialize or replay.
The digest binds history, profile, purpose, target, limits and identity; it is not
a signature or a substitute for engine result validation.

One `EngineBridge::submit` and `poll` serve all six games and seven immutable
profiles. The
`mshogi internal-search` child reads one request line for each search and
writes one response line, until its input ends. The bridge keeps one child
between searches and stops it on cancellation, replacement, failure, timeout
or shutdown; see "Engine session" below.
There is no version negotiation, legacy-shape fallback, Chess transport adapter,
paired polling API or second rules/search model. Strict typed parsing preserves
raw canonical payload bytes for their authoritative record parsers; duplicate,
unknown, missing and conflicting fields reject. Unsupported versions never
execute. A parseable current-shaped request with an unsupported version returns
an explicit unscored InvalidRequest response; an old flat shape is malformed.

`WorkLimits` has
mandatory `depth` and `nodes`, with optional `time_ms` (0–60,000 milliseconds)
for ordinary engine turns or move analysis in all six games. The exact time
limit is part of context equality, request replacement and observations.
Fixed work omits it. The host does not retry
with downgraded limits or a different protocol. Bundled host/child are built
together. The process watchdog is separate and must accommodate the requested
thinking interval plus transport and cleanup; it is never a game clock result.
Chess preserves its existing fixed search-time semantics. The other adapters
charge setup against the submitted budget and reserve up to 50 ms for
engine result validation. All use the shared soft/hard search limits. Clock
callers must retain transport and scheduling reserve; cooperative search and
atomic rules validation are not a hard real-time wall-clock guarantee.

`CanonicalRecord` explicitly carries game, immutable profile and edition, record
schema and game root/provenance/complete known history. This does not flatten
game states, sides, actions, rule-dependent commands or score meanings.

## Purpose and engine results

`ExecutionPurpose` distinguishes live `engine_turn` and read-only
`move_analysis`. Analysis binds `ExecutionTarget`:
selection identity, complete `event_index`, known `board_ply`, session
version, label and game-qualified side. Same board placement is not a
substitute for exact canonical context/history.

The result tag selects `ChessResult`, the result of one of the five games on
the shared search core, or a bounded unscored `Failure`. Chess retains move
versus intended/current claim, concluded root, incomplete work and actual
telemetry. Shogi, Xiangqi, Tori, Chu and Tenjiku use one generic adapter
(`mshogi-engine` `adapter.rs`; each game's types and calls are in `game.rs`):
a `choice` (a board move, a Shogi declaration or a Xiangqi count claim), a
`finished` conclusion, `incomplete` work, or for Xiangqi an `unresolved`
required adjudication without a fabricated score. Their telemetry has the same
fields in every game; one admission check (`matches`) and one line replay
(`validate`) serve all five. Each game keeps its own score unit and its terminal
distance unit. A timed request can return the first legal move without a
completed iteration and without a score.

The server executes Chess directly from its canonical record. It never
creates a retired Chess request. The authoritative desktop owner checks exact current context,
purpose and game command admission. Ordinary analysis uses a separate read-only
acceptance handler and cannot issue a live move or claim.

Chess's conventional search reports nominal `completed_depth`, all attempted
`nodes` and `selective_depth`: the deepest actually visited board ply
including extensions and quiescence. A completed iteration's PV can therefore
exceed its nominal depth, bounded by the recorded selective depth and the
128-ply path cap. Every complete action and terminal claim is still re-admitted
by the rules owner; a claim's intended move is not played. A terminal root or root claim can have selective
depth0, while a first incomplete iteration can have nonzero attempted depth.
Neither selective depth nor a search score establishes a game outcome.

The coordinator carries the exact optional soft/hard allocation with its request.
Soft time stops new iterations; hard time and cancellation are polled during
search. Only the last completed iteration can supply a choice. Zero thinking
time remains honestly incomplete, with no fallback action or invented score.
UCI and supervised Chess use this same timed search. Current live desktop
defaults are a128 nominal depth ceiling,20,000 visits and1,000ms hard time; analysis
defaults and all other game budgets are unchanged. Documents open stopped and preserve their game records. Explicit Resume
selects the current defaults for future live work; opening or resuming never
rewrites the original file.

## Execution safety and bounds

Completed Tori, Chu and Tenjiku delivery uses this same family for real bounded
engine turns and read-only move analysis. Complete ordered routes,
stationary captures, promotion/refusal rights, derived effects and conclusions
remain game/profile-owned. Both Tenjiku whole profiles retain exact identity at
every node and observation; cancellation, cross-game or cross-profile replacement
cannot reuse a request. See the current shared integration map
and linked game contracts for scoring, limits and accepted evidence.
The earlier Chess/Shogi/Xiangqi result descriptions are examples, not an exclusion
of the three subsequently delivered games.

Request binding includes full history, selection/purpose, request,
session/process metadata and actual limits. The host also checks the observation
against the current pending request before game admission. Replacement/cancellation retires
pending and already published work; semantic undo and historical selection do
not restore execution authority. Current document reopening allocates fresh
runtime identities and leaves work stopped until explicit Resume.

The existing one-worker/one-replaceable-pending/one-result supervision model,
watchdogs, pipe workers, cleanup and diagnostic handling remain. Requests and
responses have no byte limit (MSH-268); retained diagnostics stay at 4096 UTF-8
bytes.
Cancellation/Unknown/incomplete work/failure never invents a fallback move,
score or game outcome. Cancellation retires the active runtime request;
absence of a final observation does not invent search telemetry.

## Persistence

The shared document saves the game, human side and settings (MSH-228).
Engine observations stay in session memory; the list keeps its newest 256
results. They are not part of Save/Open. Take back
cuts the game command list and keeps the open session's results as historical
observations. Opening restores the live game with fresh runtime identities,
empty results and stopped execution. It does not restore a history selection.

Chess uses the same minimal document and session lifecycle (MSH-225). Its
record retains accepted player commands through the match API. No
development-era document compatibility is promised; see design A-09. Atomic file failures preserve the
accepted document and save path. The document has no byte limit.

Subsequent MSH-117/R2 delivered the sole current outer desktop envelope and
deleted old desktop version/read branches. MSH-118/R3 and MSH-119/R4 delivered
the runtime/document owner and single frontend session, deleting the
finite installation/forwarding stages and remaining Chess product projection.
See the [current document](document-v1.md) and [native application](native-application.md)
contracts. R1 introduced no permanent transport adapter. Chess
presentation types remain meaningful domain data, not an alternate engine
execution interface.

## Engine session

MSH-169 (Engine Session Persistence) and MSH-179 (Clock and Session for All Games).
The session retention policy remains the same under protocol 7.

- One child serves successive requests. A response is accepted when its line is
  complete and matches the request; the bridge does not wait for the child to
  exit.
- The bridge stops the child on cancellation or replacement of a running
  request, on each typed failure, on the watchdog and on shutdown. The next
  request starts a new child. A kept child that ended normally while it was
  idle is replaced once, and the request runs on the new child. A kept child
  that ends with a failure status during a request is an `UnexpectedExit`
  failure with the diagnostic text of that request.
- Memory: the child retains state for one game kind at a time: Chess uses a
  64 MB table by default; the other adapters use 16 MiB tables plus shared
  move statistics. A request without `time_ms` uses fresh state for its duration.
- Each game: a request with `time_ms` uses the transposition table and move
  statistics that the child kept, when the game of the last timed request is an
  exact prefix of the game of this request. A new, loaded or taken-back game
  starts with empty state. Explicit host cancellation retires an idle child as
  well as active work, so an equal loaded history also starts empty. A request
  without `time_ms` always runs on new
  state and does not change the kept state, so depth and node limits stay
  reproducible.
- All six games use the same session retention rule. Game history prefix
  comparison includes the initial state and immutable profile. A timed switch
  of game kind drops the previous game state. Desktop defaults are unchanged.
- The bridge requires strictly increasing nonzero `request_id` values.
- Timed engine turns and move analysis for Shogi, Xiangqi, Tori, Chu and
  both Tenjiku profiles retain the game. A suffix names the exact
  acknowledged request ID. The child checks the resulting game events and
  the selected analysis target. Chess sends its full adjudicated match record.
  Every reply uses the same compact shape,
  including the initial full request and fixed-work requests.

## Engine result admission

The desktop worker restores one private candidate and checks the result against
its exact selected game record. Commands apply to that admitted candidate once;
serialization still enforces the document bound. Failure or cancellation discards
the candidate, and publication still checks the captured host context. Historical
analysis validates its selected prefix without issuing a live command.

Chu and Tenjiku PV validation fork the admitted game history with a fresh runtime
identity instead of replaying it. Reversal tokens cannot cross that boundary.
Every PV command still passes game admission, terminal-score checks and line
bounds. Reported validation work counts actual PV transitions; it does not charge
for a history replay that no longer occurs. Saved experiment observation schema1
and its complete provenance remain unchanged.
