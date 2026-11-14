# Chess search

The Chess engine runs on the shared search core of MSH-164 (Shared Search Core).
[Shared search core](m5/shared-search-core.md) describes the structure, the
authority of the native rules, the state that lasts for a game, and how to add a
game. This page gives the public Chess search contract.

## Components

| Item (`crates/mshogi-search/src`) | Content |
|---|---|
| `search.rs` | Types that the engine and its consumers share: `SearchLimits`, `SearchChoice`, `SearchDecision`, `SearchStatus`, `SearchResult`, `SearchError`, `MATE_SCORE`, `MAX_DEPTH` |
| `chess_engine.rs` | `ChessEngine`: adapter between an `AdjudicatedGame` and the core. Search identity `mshogi.chess.shared-core.v1` |
| `chess_eval.rs` | Static evaluation, see [evaluation](evaluation.md) |
| `coordination.rs` | `SearchCoordinator`: one worker thread that owns the `ChessEngine`, request identity and stale-result checks. See [search coordination](search-coordination.md) |

UCI, the native engine process and the desktop use the coordinator.

## Request

`ChessEngine::search(game, limits, options, stop)` searches the current position
of `game` and does not change it.

- `SearchLimits { depth, nodes }`: a nominal depth of 1 to `MAX_DEPTH` (128)
  plies, and an optional total node budget. A depth outside that range gives
  `SearchError::InvalidDepth`.
- `SearchOptions::time`: an optional `TimeBudget`. The search does not start a
  new iteration after the soft time, and stops at the hard time.
- `SearchOptions::allow_draw_claims`: true if the consumer can make an optional
  draw claim for the engine.
- `stop`: a cheap callback. The search polls it and stops when it returns true.

## Result

- A concluded root gives `SearchDecision::Finished` with the native outcome.
  No search runs.
- Otherwise the decision is a move or a draw claim (`SearchChoice`). Under
  `AutoClaim` an available claim is returned at once. An optional claim is
  preferred to a move whose score is not above zero.
- Each returned move is in the native legal actions. A native representation
  error, for example a move counter overflow, gives
  `SearchError::Transition`.
- Scores are centipawns from the side to move at the root. Draws are zero. A
  mate is `±(MATE_SCORE - plies)`.
- Only a completed iteration supplies the decision, score and principal
  variation. If the search stops before the first iteration completes, there
  is no decision. The engine does not invent a result. If the core proposes a
  move that the native rules do not list (a defect; debug builds stop), a
  release build returns the first native legal move with no score.
- `completed_depth` is the last completed nominal depth. `EngineResult` also
  gives the selective depth and the table fill (`hashfull`).

## Adjudication mode

Live and supervised consumers search only M2 games. UCI creates only M2 games.
The native engine process, the desktop session and the experiment runner refuse
a record in Full mode before they search. Full mode is the opt-in
exact mate-reachability check of [chess completeness](rules/chess-completeness.md).

## Correctness tests

- `chess_bitboard::tests`: published perft counts, and random walks that compare
  legal moves, positions, hashes and incremental state with the native rules.
- `chess_engine::tests`: forced mates with exact distance, concluded roots, claim
  policy, repetition choice, limits and native legality of each move.
- `alphabeta::tests`: the core on a subtraction game whose theory is known.

These tests establish correctness. They do not establish playing strength.
[Strength harness](m5/strength-harness.md) gives the acceptance test for engine
changes.
