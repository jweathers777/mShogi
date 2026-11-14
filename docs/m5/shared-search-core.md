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
| `tori/position.rs` | Tori Shogi search position: 49-bit bitboards, hands, drops, compulsory promotion, the Swallow drop rules, repetition on the path | Tori |
| `tori.rs` | Adapter between a native Tori `Game` and the core; material and placement evaluation | Tori |

The core contains no type of a specific game. `alphabeta/tests.rs` runs it on a
subtraction game, and Chess and Tori Shogi use it through the same trait.

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

## Tori Shogi on the core

MSH-165 (Tori Search Core Adoption) moved Tori Shogi to the core and removed
its private full-width minimax. The trait and the core did not change.

- The search position agrees with the native Tori rules on perft counts and
  on random games: legal moves and drops, resulting boards and hands, check
  status, hash and move encoding (`tori/position_tests.rs`).
- Native authority: a concluded root returns its native conclusion. The chosen
  action must be in `Game::legal_actions`, which excludes the departures that
  the repetition rule forbids. The reported line is replayed in a private
  native game and cut where that game refuses an action or concludes.
- Repetition: the native rule makes the side to move leave a position at its
  third occurrence and gives no draw. The search scores a position that
  occurs again as level (zero). This is an approximation: the scan looks back
  only to the last capture or drop, and in a drop game a position can also
  come back through a capture and a drop. The forbidden departures of the root
  come from the native game and are removed from the root move list.
- Limits: depth 1 to 64 and at most 5,000,000 nodes. The desktop plays with
  300,000 nodes. A request has no time budget, so each search starts with an
  empty table and equal requests give equal results. The reported line has at
  most eight actions, because a saved document keeps and replays each line.
- Observations: the new search has the identities `mshogi.tori.shared-core.v1`
  and `mshogi.tori.material-placement.v1`. Its score is not the static value
  of the end of its line, so the earlier exact score check does not apply to
  it. Saved observations of the earlier identities stay valid and keep their
  exact score check (`mshogi-engine/src/tori.rs`).

Result (Apple M1 Pro, engine binaries through the internal protocol, native
Tori game as controller, eight seeded openings with both colors for each
seed; files and commands in `evidence/msh165/verification.json`):

| Measure | Earlier search | Shared core |
|---|---|---|
| Median completed depth at the desktop setting | 2 | 10 (300,000 nodes) |
| Median time for one move, equal-time test | 92 ms (depth 2, 2,000 nodes) | 89 ms (150,000 nodes) |
| Game test, equal time | — | 40 wins, 0 losses, 0 draws; 8 games not scored |
| Game test against the largest earlier setting (depth 3, 10,000 nodes) | — | 26 wins, 0 losses, 0 draws; 22 games not scored (175 ms against 674 ms for one move) |

The game tests show that the new search is stronger than the earlier one. They
are not a rating. A game is not scored when the report of its run reaches the
size limit of the experiment runner: the runner then stops the current game
and does not start the later games of that run. This depends on the order and
length of the games, not on their results.

## What Tori showed, and what the other games need

Findings from the second game, recorded for the later tasks of Shogi, Xiangqi,
Chu Shogi and Tenjiku Shogi. Nothing here is built yet.

**The trait was sufficient.** Drops are ordinary moves of the quiet stage.
A loss without a legal move is `no_move_score`. A game without draw claims
returns false from `draw_claimable`. A history rule that restricts the root
(the Tori forbidden departures) fits in the position: it filters its own move
list at ply zero.

**Points to change in the core before a larger game uses it:**

- *Continuation history memory.* The table has `HISTORY_SLOTS` squared
  entries of four bytes. With slots of side, piece type and target square this
  is 2 MB for Chess and 3 MB for Tori, about 20 MB for Shogi and 6 MB for
  Xiangqi, but about 430 MB for Chu Shogi and more than 2 GB for Tenjiku
  Shogi. Before Chu Shogi, give the continuation table its own smaller index
  (a method of the trait) or a fixed-size hashed table.
- *Score scale.* The margins of the core (futility 150, 300 and 450,
  aspiration 30, quiescence 200) assume that the smallest piece is worth about
  100. Each game must use that scale, or the margins must come from the trait.
- *Checks in quiescence.* Quiescence searches the tactical stage only. In
  the drop games a quiet drop can give check or mate. Tori relies on the check
  extension of the main search. Shogi will probably need a third stage for
  checking moves at the first quiescence ply.
- *Exchange test.* Tori uses a one-step estimate in `exchange_at_least`. A
  full exchange evaluation needs a list of attackers of a square for each game.
- *Root restrictions.* If a second game needs a root filter from its native
  rules, move the filter from the position to the core.

**Needs of each game:**

- *Shogi (81 squares).* A 128-bit square set. Drops with the Pawn file,
  last-rank and drop-mate rules, as Tori has for the Swallow. Optional
  promotion: two moves for one source and target. Repetition: the fourth
  occurrence is a draw, but a loss for a side that gave check each time; the
  search can score a path repetition as a draw (an approximation: the
  capture-and-drop limit of the Tori scan does not hold in a drop game) and treat perpetual check as a
  loss, with the native rules as the authority at the root. Entering-king
  declarations are root commands from the native rules, as Chess claims are.
- *Xiangqi (90 squares).* A 128-bit square set. No drops and no promotion.
  Blocked Horse and Elephant moves, the Cannon screen, the facing-Generals
  rule in the legality test. The perpetual check and perpetual chase rules
  depend on history; the search needs an approximation in `rule_score`
  (repetition with check is a loss for the checking side) and the native
  rules for the root. Draw claims of the selected profile come from the
  native rules at the root.
- *Chu Shogi (144 squares).* A multi-word square set or an array board with
  piece lists; measure before the choice. Lion moves have two steps and can
  capture two pieces or return to the source (igui, pass): the move needs a
  wider encoding than 16 bits (`encode` already gives 32). A real pass exists,
  so `null_move_safe` and the null move need care. The Lion capture rules and
  optional promotion with its history condition belong to the position.
  Repetition is forbidden for the side that repeats, as in Tori. The
  continuation history change above is necessary.
- *Tenjiku Shogi (256 squares).* As Chu Shogi, with range-jumping pieces and
  the Fire Demon burn, which removes several pieces in one move: make and
  unmake need a variable-length capture record, and the evaluation of a move's
  gain must include burned pieces. Two rule profiles exist; the position takes
  the profile from the native game.

**The same work for each adapter.** The engine protocol of each game checks a
saved observation against its search identity (`mshogi-engine/src/<game>.rs`).
A game that moves to the core needs the same split as Tori: the earlier
identity keeps its exact checks, and the shared-core identity has checks that
fit a selective search. Put that split in one shared helper when the second
of these games moves, not in a copy for each game.

## Present limits

- Selectivity is the set that the earlier search had, in standard form.
  MSH-166 (Search Selectivity Completion) adds and tests the missing techniques.
- Time use is one soft and one hard limit per move. MSH-167 (Engine Time
  Management) owns time allocation.
- The evaluation has the same arithmetic as before. MSH-168 (Classical Endgame
  and King Evaluation) owns its improvement.
- Only Chess has a time budget and state that lasts between moves. Tori and
  the other games search with fixed node limits. MSH-167 (Engine Time
  Management) is the place to give the other games a time budget.
- The search is single-threaded.
