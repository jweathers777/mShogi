# Shared search core

MSH-164 (Shared Search Core) replaced the Chess search with a game-independent
classical search core and a search-internal Chess position. This document
describes the structure, the boundary with the native rules, and how to add a
game. Results and evidence are in `evidence/msh164/`.

## Result

Measured on an Apple M1 Pro. The predecessor is the frozen engine 65887fb0.
Exact revisions, commands and files are in `evidence/msh164/verification.json`.

| Measure | Predecessor | Shared core |
|---|---|---|
| Nodes per second (benchmark median) | 0.30 million | 1.38 million |
| Nodes for the fixed benchmark depths | 5.43 million | 0.86 million |
| Depth at five seconds, middlegame | 8 to 9 | 14 to 16 |
| Depth at five seconds, endgame | 8 to 13 | 20 to 24 |
| Blunder suite at 1000 ms, avoided of 146 | 47 | 85 |
| Game test at 200 ms per move | — | 55 wins, 0 losses, 5 draws against the predecessor |

The game test is self-play against the predecessor with exposed openings. It
shows that the new engine is stronger than the old one. It is not a rating and
says nothing about other opponents.

## Structure

| Module (`crates/mshogi-search/src`) | Content | Game-specific |
|---|---|---|
| `alphabeta/mod.rs` | `SearchPosition` trait, `Searcher`: iterative deepening, aspiration, principal-variation alpha-beta, quiescence, null move, late-move reductions, check extension, shallow futility, move ordering, limits | No |
| `alphabeta/table.rs` | Transposition table: 16-byte entries, buckets of four, size in megabytes, generation aging | No |
| `chess_bitboard/` | Chess search position: bitboards, magic slider attacks, make/unmake in place, incremental hash, material, placement and phase, staged pseudo-legal generation, static exchange evaluation, repetition and fifty-move detection on the path | Chess |
| `chess_eval.rs` | Classical tapered evaluation on the bitboard position | Chess |
| `chess_engine.rs` | Adapter between an `AdjudicatedGame` and the core; owns the state that lasts for a game | Chess |
| `coordination.rs` | One worker thread that owns the `ChessEngine` between requests | Chess |

The core contains no Chess type. `alphabeta/tests.rs` runs it on a subtraction
game to prove that.

## Authority of the native rules

The search is private analysis. The native rules in `mshogi_core` decide the
real game:

- A concluded root returns its native outcome. No search runs.
- Draw claims at the root come from `AdjudicatedGame::available_claims`.
  `AutoClaim` takes a claim at once. An optional claim is preferred to a move
  whose score is not above zero. A consumer that cannot claim always gets a move.
- The adapter returns a move only if the native legal actions contain it, and
  it reports a native representation error (for example a move counter overflow)
  before it searches.
- A line with score zero is reported with the native claim that ends it, when
  the native rules make a claim available there.

Inside the tree the search uses approximations, as classical engines do
(design A-03, owner clarification of 2026-11-13):

- The transposition table key is the board state: pieces, side to move,
  castling rights, and the en passant file only when a capture is possible. An
  entry can serve a position that was reached through a different history.
- A position that occurs again on the search path is scored as a draw. A
  position from the game before the root is scored as a draw at its third
  occurrence.
- A halfmove counter of 100 is a draw unless the side to move is mated. At 99
  the side to move can claim with an intended quiet move, so its score is at
  least a draw.
- King against king, and king and one minor piece against king, are draws.

These approximations apply also when the consumer cannot claim (UCI), because
the opponent or the controller can claim.

## State that lasts for a game

`ChessEngine` holds the transposition table and the history statistics. The
coordinator's worker thread owns it for the life of the worker.

- `SearchCoordinator::new_game()` clears it. UCI calls this on `ucinewgame`.
- `SearchCoordinator::set_table_megabytes(n)` sets the table size. UCI exposes
  it as `setoption name Hash value N` (1 to 4096, default 64).
- The desktop keeps one engine process (MSH-169 (Engine Session Persistence),
  `docs/desktop/internal-execution-v4.md`). A timed search continues with the
  state of the earlier moves of the same game. Cancellation of a running
  search stops the process, and the next search starts with empty state.
- A fresh engine gives the same result for the same depth or node limit. A
  warm table changes node counts; that is intended.

## Acceptance of engine changes

Use the harness of MSH-163 (Engine Strength Test Harness), see
`strength-harness.md`. A change does not have to keep search output, node
counts or PVs. It must keep:

- `chess_bitboard::tests`: published perft counts, and random walks that compare
  legal moves, resulting positions, hashes and incremental state with the
  native rules.
- `chess_engine::tests`: forced mates with exact distance, concluded roots,
  claim policy, repetition choice, limits and native legality of each move.

## How to add a game

1. Write a search position for the game: the fastest board representation that
   fits it (bitboards for 64 squares, a 128-bit set for 81 or 90 squares, a
   multi-word set or an array board for larger boards).
2. Implement `SearchPosition` for it. The position owns its hash history, so it
   applies the game's own repetition and move-count rules in `rule_score` and
   `draw_claimable`. Drops are ordinary moves of the quiet or tactical stage.
3. Prove it against the game's native rules with perft counts and random-walk
   comparison, as `chess_bitboard::tests` does.
4. Write an adapter like `chess_engine.rs` that takes root outcomes and special
   commands from the native rules and checks the chosen move against them.

Do not add a search loop for a game. If a game needs a search behavior that the
core does not have, add it to the core behind a method of the trait.

MSH-165 (Tori Search Core Adoption) is the first use of this path for a second
game and records what the trait needs for Shogi, Xiangqi, Chu Shogi and
Tenjiku Shogi.

## Present limits

- Selectivity is the set that the earlier search had, in standard form.
  MSH-166 (Search Selectivity Completion) adds and tests the missing techniques.
- Time use is one soft and one hard limit per move. MSH-167 (Engine Time
  Management) owns time allocation.
- The evaluation has the same arithmetic as before. MSH-168 (Classical Endgame
  and King Evaluation) owns its improvement.
- The search is single-threaded.
