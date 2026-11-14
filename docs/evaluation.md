# Chess static evaluation

The Chess engine uses `chess_eval::evaluate` in `crates/mshogi-search`. Its
identity is `mshogi.chess.conventional-tapered.v1`. It evaluates the
search-internal bitboard position (`chess_bitboard::ChessPosition`), not the
native rules position. [Chess search](search.md) describes the engine.

## Score

`evaluate` gives centipawns from the side to move. `decompose` gives the full
breakdown as an `Evaluation`: the `Terms` of each color, the phase and a
specialized ending if one applies. `Evaluation::white_score` gives the score
from White.

## Terms

Each color has these terms. Each term except material has a middle-game and an
endgame value (`Tapered`).

| Term | Content |
|---|---|
| Material | Pawn 100, knight 320, bishop 330, rook 500, queen 900, king 0 |
| Placement | Value of each piece on each square |
| Pawns | Doubled, isolated, backward, connected and passed pawns, with a race estimate for a passed pawn |
| Mobility | Attacked squares, bishop pair and rook activity |
| King safety | Pawn shelter, pawn storms and attack pressure near the king |
| Threats | Attacked pieces that are not defended or that have a higher value than the attacker |
| Endgame | A preference for conversion when one side has a large material lead |

The phase is the remaining non-pawn material, capped at 6400 (`MIDDLE_PHASE`).
The score of a color is its material plus the blend of the middle-game and
endgame values by the phase. The position keeps material, placement and phase
incrementally when it makes and unmakes a move. The other terms use the current
attacks.

## Specialized endings

Some endings replace the general terms (`EndgameKind`):

- King and pawn against king: the win/draw bitbase in `chess_kpk.rs`.
  `examples/generate_chess_kpk.rs` generates its table from the native legal
  actions.
- Wrong-colored bishop with a rook pawn.
- Bishop and knight against a bare king.
- Two knights against a bare king.
- Queen against an advanced rook-file or bishop-file pawn.
- Rook and pawn against rook with a third-rank defence.

## Limits

All coefficients are original, transparent and untrained. They are not
measured exchange values. A static score never decides a game: the native rules
and the search handle outcomes and claims first. A score of zero does not prove
a draw, and a large score does not prove a win.

The tests in `chess_eval/tests.rs` check the term arithmetic, the perspective,
color reflection symmetry, the specialized endings and the passed-pawn race. They
establish correctness, not playing strength.
