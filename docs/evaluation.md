# Chess static evaluation

## Active conventional evaluator

The Chess coordinator's conventional search uses
`chess_conventional_eval::evaluate`, identity
`mshogi.chess.conventional-tapered.v1`. `Evaluation` exposes each color's material
and tapered piece-placement, pawn-structure, mobility/activity, king-safety,
threat and endgame-conversion contributions. `score_for(Color)` subtracts the
opponent's contribution; positive centipawns favor the requested color.
The [conventional engine checkpoint](m5/chess-conventional.md) records the associated
engineering evidence. This evaluator has no trained parameters or copied
piece-square tables, and its coefficients are not measured exchange values.

Material retains the values below. Original arithmetic placement rewards central
minor pieces and development, pawn advancement, appropriate king placement and
endgame king activity. Pawn terms cover isolated, doubled, connected, backward and
passed pawns, including a simple king-escort estimate. Activity includes bishop
pairs, rook files/seventh rank and geometric mobility excluding friendly occupancy
and enemy pawn attacks. King safety combines pawn shelter/open files with weighted
opposing attacks into the king ring. Threats estimate attacked undefended or
higher-valued pieces; a major-material conversion term encourages king proximity
and driving the opposing king to the edge. Each coefficient is inspectable in the
implementation.

Remaining non-pawn, non-king material controls the middle/endgame blend, capped at
the initial 6400cp total; raw phase can rise above that after promotion and is
retained exactly for undo. Material, arithmetic piece placement and raw phase form
a reversible incremental base in `EvalState`. `push(before, after)` compares native
position snapshots and updates changed cells; `pop()` restores the saved base.
Pawn, activity, safety, threat and conversion terms recompute from current native
geometry. Hand-derived arithmetic, color/rank reflection and rebuild-equivalence
checks over native opening, castling, en-passant, capture, promotion and undo
sequences test the decomposition and state restoration.

`Position::attacks_from` supplies geometric attacks, including a slider's first
occupied blocker. Mobility, defended-piece and pressure features are heuristics:
they do not filter pinned pieces or count legal actions. The evaluator owns no
move generator, effect resolution, outcome recognizer or history policy. Native
outcomes and claims take precedence in search. Static estimates remain below the
100,000cp conservative bound, far below the 1,000,000 mate-score band. A zero score
proves no draw, a large score proves no win, and positional tests or call timings
make no playing-strength claim.

## Historical positional candidate

The retained `chess_candidate::evaluate` API has identity
`mshogi.chess.tapered-position.v1`. It combines the material values below with
original untrained arithmetic placement: knight/bishop development, pawn
advancement and isolated/doubled/passed pawns, rook files/seventh rank, bishop
pairs, middle-game king shelter/castling squares and endgame king centrality,
with a small early queen-development penalty. The middle/endgame blend is capped
at the initial 6400cp of non-pawn material. This historical reference has no trained
weights or copied piece-square tables. Its arithmetic, reflection and native
terminal/claim separation checks remain in the suite; the owner-stopped
[earlier study](m5/chess-strength.md) is historical evidence, not an active
playing-strength requirement.

## Frozen material baseline

MSH-19 adds the native `mshogi-search` crate. `chess::evaluate(&Position)` returns White and Black material contributions; `Evaluation::score_for(Color)` subtracts the opponent's contribution. Positive scores favor the requested side. For a side-to-move score, pass `position.side_to_move()` explicitly. Identity: `mshogi.chess.material.v1`.

Values in centipawns: pawn 100, knight 320, bishop 330, rook 500, queen 900, king 0. These are transparent untrained parameters, not measured exchange values. There are no positional, tempo, mobility or learned components. Contributions and `piece_value` are inspectable, sums fit i32 under validated position bounds, and repeated evaluation is deterministic without mutating the position. Moving side, rights, raw EP, counters and history do not alter material contributions. Color swap plus rank reflection swaps contributions and negates White's score.

This is a **static evaluator**, not game or search adjudication. It accepts mate and stalemate diagrams and returns their material values. A zero score does not conclude a draw, and a large score proves neither mate nor mating possibility. MSH-20 must consume rules-owned game history, outcomes and claim policy before evaluating live leaves, assign terminal/mate scores there and preserve caller state. This deliberate separation keeps MSH-19 independently executable; it does not depend on MSH-16/17 or introduce a second outcome classifier.

Verification: `cargo test -p mshogi-search --locked` checks hand-derived values, both score perspectives, color/rank symmetry, state/counter independence, terminal diagrams receiving raw values and a promotion changing a pawn's contribution to a queen's, plus a public API doctest. No playing-strength claim or timing measurement is made. Search, cancellation, UCI and matches remain separate M2 tasks.
