# Chess game history and record v1

MSH-16 implements the history/record portion of [`mshogi.chess.match.v1`](chess-match-v1.md) over the unchanged [`mshogi.chess.position.v1`](chess-position-v1.md). `Game` is a rules-library type. Its current coverage is action history and repetition identity/counts. MSH-17 now adds a separate [live adjudicated boundary](chess-adjudication.md); this document retains the diagnostic history/observation contract. MSH-18 now supplies match-command/clock semantics in its separate [match envelope](chess-match-api.md). Consumers must not describe `Game` as complete adjudication yet.

## Game boundary

`Game::new(ClaimPolicy)` starts at `STANDARD_START_FEN` with `HistoryOrigin::StandardStart`. `Game::from_fen(fen, policy)` always records `ImportedFen`, meaning prior history is unknown, including when the FEN equals the standard diagram. Both count the initial state once. Imported counters and raw EP are preserved; fullmove numbers never create imaginary occurrences. The policy is `ExplicitOnly` or `AutoClaim`; history reconstruction stores it without exercising claims.

`position()` exposes only a shared reference. `apply(Action)` checks the complete action on a private position before publishing position, action history and the occurrence table. Invalid actions and counter overflow preserve the entire game. `undo()` owns the LIFO restoration and returns the undone action (None at the beginning). It restores all saved fields, history, current identity and exact occurrence counts, removing abandoned entries. Snapshots favor clarity, consistent with the existing position undo; they can be optimized later with equivalent checks. `clone()` supports private analysis. `ply_count()` is an action count, **not** a unique search-request version.

`Position::repetition_identity()` and `Game::repetition_identity()` describe the exact board, side, all castling permissions and only **legally available** en passant. They exclude counters and uncapturable raw targets. A pinned EP action does not affect identity; a legal one does. Blocked/attacked castling permissions still matter. This typed identity is separate from `Position::Eq` and defines no search cache semantics. HashMap indexing always resolves collisions using equality of the full identity. `occurrences(identity)` counts known occurrences, and `current_occurrences()` queries the current one.

## Canonical chess record

`Game::record()` exports the current analysis branch. `GameRecord::to_json()` uses compact UTF-8 JSON without a trailing newline; fields and event arrays have a fixed order. FEN and coordinates are canonical. `from_json()` accepts JSON formatting whitespace but rejects unsupported metadata, unknown/duplicate fields, malformed numbers/colors/actions, and a standard-start origin on any other initial state. Parsed/built events remain untrusted until replay validates played actions and their contexts.

The record is intentionally chess-only and independent of private undo/repetition storage. It contains:

| Field | Contract |
| --- | --- |
| `schema_version` | `1`, the encoding version, separate from the rules version |
| `rules_profile` | `mshogi.chess.match.v1` |
| `initial_fen` | All six initial position fields, canonicalized without discarding counters/raw EP |
| `history_origin` | `standard_start` or `imported_fen`; the latter records unknown earlier history |
| `claim_policy` | `explicit_only` or `auto_claim` |
| `events` | Ordered `{ "ply": n, "event": payload }` entries; `ply` is the number of played actions **before** the event |

Payloads are tagged with `kind`: `move` carries a complete coordinate `action`; `command` carries `actor` (`white`/`black`) and a typed command; `clock` carries `at_ns`, `remaining_ns` (White then Black, unsigned u64 nanoseconds), and nullable `active`. Commands are `resign`, `offer_draw`, `accept_draw`, `decline_draw`, or `claim_draw` with `reason` (`threefold`/`fifty_move`) and nullable coordinate `intended`. Unit commands have no other fields. An intended claim action is command data, **never** a played move. No effects/victim lists, private undo, trusted outcomes or engine-specific hashes are accepted.

`GameRecord::push_event` supports assembling untrusted data. `replay_history()` verifies every event's declared ply and applies every played action through the rules library. It returns the rebuilt `Game` and `ContextualObservation` entries with one-based **event sequence**, action ply, full position at that context and the original observation. Multiple observations can share a ply; event sequence keeps their total order. A bad action/context returns an error with event index and no partial game. JSON order and full typed state, rather than hash iteration, determine record output and replay.

This is **history and observation replay**, not full match replay. An observed request may be invalid; an observed clock may be inconsistent. MSH-16 preserves both honestly and does not silently apply, approve or discard them. The separate live/match APIs (MSH-17/18) validate their rule/clock semantics and establish outcomes. Engine configuration, proofs, match conclusions/failures and further runner metadata belong to a versioned match envelope when those consumers arrive. They can embed this history without serializing private undo. The future full replay obligation remains MSH-23/MSH-25.

## Real adapter and verification

The diagnostic shell now holds a `Game`, with no separate position/undo stack. `history` reports action count, origin, stored policy and current repetition count. `record` prints `record <JSON>`; `record load <JSON>` restores action history atomically, including policy and undo. The shell explicitly refuses observation-bearing records because it cannot restore their match semantics; failure preserves its prior game. Standard position loads reset history; FEN loads mark imported history. Existing shell line/depth limits remain; records longer than its 4096-byte input limit use the library API. The shell stores auto-claim policy without acting as an unattended runner.

Run `cargo test -p mshogi-core --test game_history --test game_record --locked` and `cargo test -p mshogi-console --test cli --locked`. Tests consume the independent MSH-15 history/identity fixtures and M1 special-action after-states, verify exact complete-game restoration over 933 fixed-seed plies from the 35 corpus positions, and check canonical encoding, malformed records, same-ply observations, intended claims and fresh-process console reload/undo. These tests establish history/transport behavior, not unimplemented adjudication.

Dependency boundary: the codec uses exact `serde = 1.0.229` with derive and `serde_json = 1.0.145`, already represented in the lockfile; JSON moves from test-only to production for this real record consumer. Local registry manifests and MIT license texts were inspected; both are `MIT OR Apache-2.0`, with supported minimum Rust versions below the pinned Rust 1.89. Transitive package versions/checksums stay unchanged. JSON handles strict typed decoding/escaping without a bespoke parser; the rules library still has no console/filesystem/network I/O or Python/UI dependency. The deferred release licensing decision remains unchanged.
