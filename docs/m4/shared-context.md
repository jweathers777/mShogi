# Shared native context and routing (MSH-69)

The library supports two explicit native games through `mshogi_core::native`.
`CanonicalRecord` holds either the existing chess `GameRecord` or the approved
shogi `GameRecord`. It does not merge board storage, actions, rules, sides,
outcomes or scores. Native payload encodings and historical evidence are unchanged.

## Identities and strict records

The JSON envelope has six required fields: `envelope_version` (1), `game_kind`,
`rules_profile`, `profile_edition`, `native_schema_version` and `payload`.
Metadata is derived from the typed native record when writing. Reading checks
every identity and routes raw payload bytes to exactly one native parser. There
is no fallback to another game. Unknown, duplicate, missing and conflicting
fields reject. `RawValue` captures both the outer envelope and payload; an
intermediate object map must never precede strict native parsing.

| Game | Rules profile | Immutable edition | Native schema |
| --- | --- | --- | --- |
| Chess | `mshogi.chess.match.v1` | `FIDE-2023-01-01+mshogi.chess.match.v1` | 1 (M2) or 2 (Full, with its limits) |
| Shogi | `mshogi.shogi.csa36.v1` | `JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026` | 1 |

The chess edition names the already accepted FIDE source edition and software
profile, without changing chess behavior. The shogi edition is the existing
MSH-67 native record edition, now exported as a constant. Imported roots retain
their provenance and declared counters; known board actions and event history
are separate quantities. A terminal command can change the record without
changing board ply. No earlier repetition history is inferred.

Chess history/observation records are not complete executable `MatchRecord`s.
Observations remain inert data. Live chess reconstruction retains its existing
rejection of non-claim match observations. Complete command admission uses an
existing `ChessMatch`; the shared dispatcher never fabricates one from history.
Shogi retains its complete accepted command ledger and native replay validation.
This is transport support, not the desktop archive feature owned by MSH-73.

## Commands and native ownership

`NativeCommandTarget` borrows a chess `ChessMatch` or shogi `Game`.
`dispatch_command` checks only that the native command tag matches its owner,
then delegates the unmodified native request and returns its native reply.
Native owners decide version, actor, turn, legal action, claim/declaration and
terminal semantics. There is no shared side-to-move validator. In particular,
shogi declaration eligibility is an advisory native query; a formal declaration
is a committed native command and an ineligible out-of-turn declaration produces
the selected profile's false-declaration loss. Resignation also retains native
terminal-command admission. Wrong-game commands reject before mutation.

## Transport and compatibility

The console's existing `internal-search` entry point accepts both:

* **v2:** unchanged `EngineRequest`, `EngineResponse`, purposes and native chess
  JSON. Existing desktop consumers and saved historical observations remain v2.
* **v3:** `SharedRequest`, `SharedResponse`, `SharedPurpose`, `SharedResult` and
  the record envelope above. V3 chess execution adapts into the existing native
  executor, then echoes the original v3 context. No old record is rewritten.

A version header is inspected from raw bytes, followed by a strict typed parse
of the original bytes. Known v3 parse failure never falls through to v2. An
unsupported version with the exact old chess shape retains the existing v2
`InvalidRequest` diagnostic and cannot execute: its version is rejected before
reconstruction. Unsupported versions with a shared envelope are malformed input,
not a reason to guess a game. Untrustworthy malformed contexts produce a bounded
transport error rather than an invented echoed request.

V3 purposes are live `engine_turn`, read-only `move_analysis`, and the explicitly
chess-specific `chess_full_reachability`. The entire context is accepted by
equality, including native root, mode, policy, complete known event history,
purpose, request identity, host session version, process generation and all work
limits. Analysis also binds a monotonic selection identity, complete event-prefix
index, known board-action count, target session version, label and native side.
`NativeSide` keeps White/Black and Sente/Gote distinct. Chess replay verifies the
target's actual side and ply; shogi is never replayed as chess.

V3 chess results wrap the existing native distinctions: complete move, claim,
finished game, incomplete work, failure, or bounded cooperative Full proof result.
Scores retain the existing chess root-side perspective and mate-distance meaning.
Full record restoration remains unsupported before live replay; the separate
bounded Full operation on an M2 context remains available. Analysis results are
observations, never commands implicitly applied by this layer.

`execution_capabilities(Shogi)` is empty until MSH-70 supplies a real engine
consumer. Every well-formed shogi execution request, including a chess Full
operation, receives explicit unscored `UnsupportedMode`. It receives no fabricated
move, telemetry, score, game outcome or pretend tsume result.

## Execution lifetime and bounds

Both wire versions share the existing single `EngineBridge` supervisor, one
current request, one pending slot and one final-result slot. Request identities
and process generations increase across both versions and all games. Replacement,
cancellation and shutdown retire publication through the same current-owner
check. Native undo and historical semantic equality do not reset that ledger.
Hosts must issue fresh session and selection identities and cancel obsolete work;
restoring a native position is not restoring a runtime request. Calling the wrong
version's poll leaves the other version's result untouched.

The existing child watchdog, bounded pipes/stderr, kill/reap cleanup and bounded
diagnostics remain in use. Encoded limits are unchanged: request 1,048,576 bytes,
response 2,097,152 bytes (excluding newline), retained diagnostics 4,096 bytes.
The bridge checks the **whole encoded request**, including envelope and purpose;
the server checks the **whole encoded response**, including echoed context and
result. A native record fitting its own cap is not proof that an enclosing frame
fits. Oversized work fails explicitly without a game action or score.

The canonical wrapper itself is limited to 1,048,576 encoded bytes. Shared chess
context accepts at most 4,096 events, matching the existing desktop archive event
boundary; this is a transport admission cap, not a change to native chess record
semantics. Shogi retains its native bound of at most 320 board actions, or 319
plus a terminal command (the parser's conservative event cap is 321), and its
native record byte cap. No constants were raised. Focused tests measure actual
encoded envelopes and full request/response frames, including maximum retained
shogi history and a large chess observation ledger.

Measured by the composition test (UTF-8 bytes, newline excluded):

| Actual context | Native payload | Envelope | Request | Echoed response |
| --- | ---: | ---: | ---: | ---: |
| Shogi deterministic 320-event capacity walk | 79,733 | 79,914 | 80,062 | 80,328 |
| Chess 4,096 clock observations with u64-max counters | — | 573,826 | 573,974 | 574,126 |

The chess observation ledger is transportable data; native live replay correctly
rejects it for search. These are concrete measurements, not worst-case proofs
for every chess observation type. Whole-frame admission remains decisive. The
test also accepts an exactly 1 MiB request and rejects one additional input byte.

## Evidence ownership and remaining work

`native_envelope` tests cover real payloads, provenance, same-board histories,
same-ply terminal commands, native command admission and strict nested parsing.
Engine `shared` tests cover exact-purpose acceptance, real chess search/Full,
unadvertised shogi execution, identity disagreement and parser failures.
Console `shared_internal` tests exercise the actual executable and cross-game,
cross-version cancellation/replacement, forged echoes and irreversible request
retirement. Its helper is separately labeled test instrumentation and is not
packaged. Existing M2/M3 protocol, process, archive and desktop tests remain
applicable; no MSH-67 rule fixture or mutual-check expectation changed.

MSH-68 owns shogi console diagnostics; MSH-70 owns real shogi engine execution.
MSH-73 owns desktop archive integration. The complete-delivery owners MSH-41–47
remain open; this boundary does not deliver those consumers or a six-game framework.
