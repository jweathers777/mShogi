# Native shogi state boundary (MSH-65)

The native `mshogi_core::shogi` module supplies local state and complete board
choices for the approved `mshogi.shogi.csa36.v1` profile. This evidence covers
MSH-65 state, notation and structural transitions. Full legal generation,
king safety and pawn-drop mate belong to MSH-66; history, match conclusions,
protocols, engine and UI delivery remain separate work.

## API and representation

`Side` owns Sente/Gote direction. `Square::new(file, rank)` uses **one-based**
numeric files/ranks 1–9; display and parsing use USI `1a`–`9i`. Board arrays and
`Square::all()` run rank a–i, each rank file 9–1, matching SFEN order.
`Piece` retains owner, base kind and promotion independently. `Hand` holds only
unpromoted base resources, with kings excluded. Read-only `Position` queries
expose board, square occupant, each hand, side, next-ply, implied completed
actions and root provenance. Existing chess state/action contracts are unchanged.

`Action::Move { from, to, promote }` and `Action::Drop { kind, to }` encode one
complete choice. Captures, demotion, ownership transfer and resource changes
come from native state. USI parsing/display covers board actions only; match
commands such as `resign` or `win` are not board actions. Parsing does not claim
that an action is legal in a particular position.

The crate-private `apply_structural` checks geometry, intervening occupancy,
owner, destination, king-capture exclusion, promotion availability/necessity,
drop resources, nifu and dead drop ranks. It deliberately omits self-check and
uchifuzume. Its private visibility prevents clients from treating this limited
boundary as the legal-action API. Two narrowly documented `dead_code` allows
retain these private entry points until MSH-66 consumes them.

Successful transitions save complete before/after snapshots. The crate-private
`undo_structural` restores every field only when the complete current position
equals the saved post-state. An out-of-order/wrong post-state is rejected
atomically. Equal state in another game context is not distinguished; undo
records are private-layout local restoration records, not canonical history.
There are no incremental caches in this representation.

## Local imports and counter provenance

`Position::from_sfen` accepts a bounded ASCII four-field SFEN with mandatory
positive decimal `u32` next-ply. Fields have one separating space, rank runs
use single digits 1–9, and duplicate hand kinds for an owner are rejected.
Hand counts of one omit the numeric prefix; leading zeroes, explicit zero,
promoted hand pieces, kings in hand and overflow are rejected. Hand input may
use another owner/kind order; output canonicalizes to Sente then Gote, each
in RBGSNLP order. Move numbers have no leading zeroes.

SFEN and `from_diagram` validate exactly one king per owner, legal promotion
states, no dead unpromoted piece, no same-owner unpromoted pawn duplication on
a file, and combined board/both-hand base inventory caps. Sparse diagrams are
permitted; missing nonroyal material is not reconstructed. Prior-side check and
king adjacency are not rejected by this structural boundary. Local acceptance
does not establish reachability or king safety.

Strict SFEN records `HistoryOrigin::Diagram` with the supplied root counter.
`from_diagram(..., None)` explicitly defaults to one and records `Defaulted`;
`Some(n)` records `Supplied`. `Position::initial()` instead records `Initial`.
Actions increment the current next-ply while preserving the exact root origin.
The implied completed count is `next_ply - 1`; no prior moves, checks or
repetition occurrences are inferred. An analysis diagram at or beyond next-ply
321 is accepted without silently resetting the counter or adjudicating a match.
An increment beyond `u32::MAX` fails atomically as a representation limit.

SFEN serialization preserves local board/hands/side/counter and canonicalizes
notation. SFEN has no field for root provenance: canonical game records must
preserve `history_origin()` separately. Reimporting initial SFEN creates an
imported root, distinct from the known standard initial state.

## Verification evidence

Checked on 2026-10-26 in branch `codex/shogi-board`, next-task worktree based on
`f63f2d2`, against the uncommitted MSH-65 implementation. Environment:
Darwin 27.0.0 arm64, rustc 1.89.0 (`29483883e`), Cargo 1.89.0 (`c24e10642`).

| Command | Result |
| --- | --- |
| `cargo test -p mshogi-core shogi::tests` | Passed, 11 test groups; other integration tests filtered out. |
| `cargo clippy -p mshogi-core --all-targets -- -D warnings` | Passed. |
| `rustfmt --edition 2024 --check crates/mshogi-core/src/shogi/mod.rs crates/mshogi-core/src/shogi/action.rs crates/mshogi-core/src/shogi/sfen.rs crates/mshogi-core/src/shogi/tests.rs` | Passed. |
| `git diff --check` | Passed for tracked changes; new source also passed focused rustfmt checks. |

The 11 groups cover independently specified initial/custom placement and
notation, all-square USI roundtrips, malformed and local-invariant rejection,
sparse diagram scope, opening capture/drop sequences with material conservation
and nested exact restoration, capture/demotion of every promotable base kind,
required/optional/leaving-camp promotion for both sides, geometric movement and
blocked rays, invalid-action/counter-overflow atomicity and wrong-context undo,
and both-side nifu/dead-rank/resource drop checks. A hand-derived expected
opening SFEN initially misplaced the vacated gote silver square; the expected
row was corrected to the explicit action's `3a` source before the passing run.

These checks provide local macOS state evidence, not Windows/Linux execution,
full legal conformance, history adjudication or complete shogi delivery.
Independent review and integration are tracked by the primary implementer in
MSH-65.
