# Chess static evaluation

## MSH-49 positional candidate

The active Chess coordinator uses `chess_candidate::evaluate`, identity
`mshogi.chess.tapered-position.v1`. It retains the material values below and
returns separate material and positional contributions for each color. All
parameters are original arithmetic heuristics, without trained weights or copied
piece-square tables. Remaining non-pawn material determines a middle/endgame
blend capped at the initial6400cp total.

Terms cover knight/bishop central development, pawn advancement, isolated and
doubled pawns, passed pawns, open/semi-open rook files, seventh-rank rooks, bishop
pairs, middle-game king shelter/castling and endgame king centrality. Early queen
development has a small penalty. The implementation documents each coefficient.
These are static estimates only: native history, outcomes and claim policy take
precedence in search. Color/rank reflection, hand-derived arithmetic, unchanged
state and native terminal/claim regressions verify this separation. They do not
measure strength; [MSH-49](m5/chess-strength.md) owns that evidence.

## Frozen material baseline

MSH-19 adds the native `mshogi-search` crate. `chess::evaluate(&Position)` returns White and Black material contributions; `Evaluation::score_for(Color)` subtracts the opponent's contribution. Positive scores favor the requested side. For a side-to-move score, pass `position.side_to_move()` explicitly. Identity: `mshogi.chess.material.v1`.

Values in centipawns: pawn 100, knight 320, bishop 330, rook 500, queen 900, king 0. These are transparent untrained parameters, not measured exchange values. There are no positional, tempo, mobility or learned components. Contributions and `piece_value` are inspectable, sums fit i32 under validated position bounds, and repeated evaluation is deterministic without mutating the position. Moving side, rights, raw EP, counters and history do not alter material contributions. Color swap plus rank reflection swaps contributions and negates White's score.

This is a **static evaluator**, not game or search adjudication. It accepts mate and stalemate diagrams and returns their material values. A zero score does not conclude a draw, and a large score proves neither mate nor mating possibility. MSH-20 must consume rules-owned game history, outcomes and claim policy before evaluating live leaves, assign terminal/mate scores there and preserve caller state. This deliberate separation keeps MSH-19 independently executable; it does not depend on MSH-16/17 or introduce a second outcome classifier.

Verification: `cargo test -p mshogi-search --locked` checks hand-derived values, both score perspectives, color/rank symmetry, state/counter independence, terminal diagrams receiving raw values and a promotion changing a pawn's contribution to a queen's, plus a public API doctest. No playing-strength claim or timing measurement is made. Search, cancellation, UCI and matches remain separate M2 tasks.
