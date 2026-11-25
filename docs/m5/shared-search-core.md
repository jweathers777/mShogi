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
| `alphabeta/mod.rs` | `SearchPosition` trait, `Searcher`: iterative deepening, aspiration, principal-variation alpha-beta, quiescence, null move, late-move reductions, check extension, shallow futility, move ordering, limits; margins scaled by the game's `MARGIN_UNIT`; optional first-ply quiescence check stage and root mate search | No |
| `alphabeta/mate.rs` | Mate search by continuous checks: df-pn over game-supplied checks and evasions, bucketed proof table, acyclic proof lines | No |
| `texel.rs`, `examples/support/texel.rs` | Shared Texel model trait and fitting tool (corpus format, K calibration, coordinate descent, validation and held-out checks) | No |
| `alphabeta/table.rs` | Transposition table: 16-byte entries, buckets of four, size in megabytes, generation aging | No |
| `chess_bitboard/` | Chess search position: bitboards, magic slider attacks, make/unmake in place, incremental hash, material, placement and phase, staged pseudo-legal generation, static exchange evaluation, repetition and fifty-move detection on the path | Chess |
| `chess_eval.rs` | Classical tapered evaluation on the bitboard position | Chess |
| `chess_engine.rs` | Adapter between an `AdjudicatedGame` and the core; owns the state that lasts for a game | Chess |
| `coordination.rs` | One worker thread that owns the `ChessEngine` between requests | Chess |
| `tori/position.rs` | Tori Shogi search position: 49-bit bitboards, hands, drops, compulsory promotion, the Swallow drop rules, repetition on the path | Tori |
| `tori/eval.rs` | Tori evaluation weights (Texel-fitted), feature layout and Texel model | Tori |
| `tori.rs` | Adapter between a native Tori `Game` and the core | Tori |
| `shogi/position.rs` | Shogi position, hands, drops, promotion, history and classical evaluation | Shogi |
| `shogi.rs` | Native Shogi root/declaration and continuation adapter | Shogi |
| `xiangqi/position.rs` | Xiangqi board, movement, private history and classical evaluation | Xiangqi |
| `xiangqi.rs` | Native Xiangqi outcome, count-claim and continuation adapter | Xiangqi |
| `chu/position.rs`, `chu.rs` | Chu board, compound moves, Lion rights and native adapter | Chu Shogi |
| `tenjiku/position.rs`, `tenjiku.rs` | Tenjiku board, ranked jumps, Demon effects, both profiles and native adapter | Tenjiku Shogi |

The core contains no type of a specific game. `alphabeta/tests.rs` runs it on a
subtraction game, and all six target games use it through the same trait.

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
  state of the earlier moves of the same game. Explicit cancellation stops
  active or idle processes, and the next search starts with empty state.
- A fresh engine gives the same result for the same depth or node limit. A
  warm table changes node counts; that is intended.

MSH-179 (Clock and Session for All Games) extends this policy to all six games
and both Tenjiku profiles. Each adapter exposes caller-owned search state; one
server slot mechanism retains it only for timed, exact history continuations.
Fixed-limit requests use fresh state and leave the retained state unchanged.
The core owns clock allocation and iteration timing. Non-Chess adapters use
shared `RequestTime` accounting for setup and native result work. Chess keeps
its earlier fixed search-time semantics. See the [protocol contract](../desktop/internal-execution-v4.md)
and verification. Desktop defaults do not change.

## Bounded ordering history

MSH-178 (Search Core History Scaling) caps the basic history at 65,536 `i32`
entries (256 KiB) and continuation history at 8,388,608 `i32` entries (32 MiB).
Their combined payload is at most **32.25 MiB**, independent of board size and
piece count. Vector descriptors and indexing metadata add a fixed amount.
This bound excludes the separately configured transposition table and move/PV
buffers. Killers, previous-move slots, static evaluations and excluded moves
have `MAX_PLY + 2` entries; they do not grow with the square of the slot count.

Tables that fit retain their complete dense index and the same update arithmetic.
This includes Chess, Tori, Shogi and Xiangqi. Chu uses the capped table. Above the cap, continuation history
mixes the previous and current slots separately into a fixed table. Basic history
uses the low 16 bits above its cap. Collisions share ordering statistics; they
cannot admit moves or decide native outcomes. Bonus, penalty, clear and basic
history decay remain unchanged. Continuation history is still kept between
searches and cleared at a new game. Sizing uses checked multiplication, so even
an extreme slot count cannot cause a quadratic allocation or integer overflow.
The trait requires a nonzero slot count and in-range move slots.

Payload for current adapters:

| Game | Slots | Basic bytes | Continuation bytes | Total bytes |
| --- | ---: | ---: | ---: | ---: |
| Chess | 768 | 3,072 | 2,359,296 | 2,362,368 |
| Tori Shogi | 882 | 3,528 | 3,111,696 | 3,115,224 |
| Shogi | 2,268 | 9,072 | 20,575,296 | 20,584,368 |
| Xiangqi | 1,260 | 5,040 | 6,350,400 | 6,355,440 |
| Chu Shogi | 12,096 | 48,384 | 33,554,432 | 33,602,816 |
| Tenjiku Shogi | 22,528 | 90,112 | 33,554,432 | 33,644,544 |

Chu uses side × 42 original-identity/promotion states × 144 destinations,
so original identity remains distinct even when movement faces agree. The
MSH-178 estimate used 29 movement faces; the cap accommodates the implemented
index without a core change. Tenjiku uses side × 44 movement faces × 256
destinations. The cap needs no game-specific
core branch or new trait method. See history verification.

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
  300,000 nodes. Requests without a time budget start with an empty table
  and equal requests give equal results. Timed requests use the shared session
  policy above. The reported line has at
  most eight actions, because a saved document keeps and replays each line.
- Observations: the new search has the identities `mshogi.tori.shared-core.v1`
  and `mshogi.tori.material-placement.v1`. Its score is not the static value
  of the end of its line, so no exact score check applies to it. Observation
  admission accepts only the current identities (`mshogi-engine/src/adapter.rs`).
- Evaluation (MSH-206, Tori Evaluation Round) is linear in 148 Texel-tuned
  weights (`tori/eval.rs`): board and hand values, rank and column tables for
  each piece, Phoenix safety (own and attacked neighbors, enemy pieces near
  it, drop threats), Eagle and Quail mobility, and tempo. Board, hand and
  table terms are kept incrementally; Phoenix safety and mobility are computed
  per node. `INITIAL` is the earlier evaluation. The corpus is cloud self-play
  of the predecessor (6,400 pairs at 2 s + 0.05 s, about 778,000 quiet
  positions, split by opening). Only about 38% of Tori game positions are
  quiet, so a corpus needs about twice the games of Xiangqi. The static fit
  won about +255 Elo against the predecessor; the per-node terms, fitted
  separately and tested against the static fit, won about +99 Elo more. Unlike
  Xiangqi, the per-node terms are kept. Exchange (SEE) and ordering values
  still use the earlier fixed piece values. The evaluator identity is
  unchanged.

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

## Shogi on the core

MSH-180 (Shogi Search Core Adoption) replaces the private full-width Shogi
search. The core and its trait do not change. The private position uses an
81-square array with u128 piece sets, precomputed step attacks, ray attacks,
incremental material and board/hand hash, and reversible in-place updates.
All promotion choices and legal drops are generated, including Pawn-file,
dead-rank and Pawn-drop-mate restrictions. Perft and randomized differential
tests compare legal actions, resulting boards/hands, hashes and restoration
against the native rules.

The adapter checks native root outcomes and entering-king declarations, admits
only a native legal chosen action, and replays the reported line through a
private native game. A refused or concluded continuation ends the reported
line. Native rules remain the authority for actual play and saved history.

Private analysis uses these approximations:

- The table is keyed by board, hands and turn, without history or move count.
- A repeated position on the search path scores as a draw, or a loss for a
  side that checked on every move in the cycle. Pre-root history requires four
  occurrences. The scan crosses captures and drops; a null move sets a barrier.
- At 320 completed actions, the position is drawn unless it has no legal move,
  which takes priority. Null moves do not advance this count.
- Entering-king declarations are checked only at the root, not inside the tree.
- Quiescence includes captures and promotions; in check it searches all evasions.
  Since MSH-190 (Shogi Selectivity Round) the first quiescence ply also tries
  at most eight quiet checks that do not lose the moving piece; the stage does
  not recur. The rejected MSH-187 candidate had an unbounded version.
- The exchange test is a full swap-list SEE with board and hand values
  (MSH-190). Margins use `MARGIN_UNIT = 200`. Quiet ordering adds bonuses for
  checks and drops near the enemy king and a penalty for declining a Pawn,
  Bishop or Rook promotion. In check, only interposing drops are generated.
- A root df-pn mate search uses at most 1/8 of the budget; the alpha-beta
  result is preferred when its mate is no longer. Mate probes inside the tree
  were measured and rejected. See Shogi selectivity round.
- Evaluation (MSH-188, Shogi Evaluation Round) is linear in 133 Texel-tuned
  weights: board and hand values, rank tables, King placement, slider mobility
  and King safety. Static terms are incremental; mobility and King safety are
  computed per node. Fitting uses the shared Texel tool
  (`examples/support/texel.rs`, trait `texel::TexelModel`), which Chess also
  uses. See Shogi evaluation round.

Limits are depth 1–64 and at most 5,000,000 nodes. Desktop play and bare USI
`go` use 1,000 ms with those depth/node safety ceilings; a reported line has at
most eight actions. Explicit fixed-limit requests start with empty tables and
statistics. Timed internal and USI requests use the shared session policy above.
USI accepts bounded `movetime`; full game clocks, ponder and tsume remain
unsupported. New game resets the retained state, and incompatible history
prefixes reset it through the shared slot policy. Exact comparison of validated
Shogi records avoids replay solely to establish cache-prefix identity; native
request admission still validates the history. Earlier saved play
observations remain readable.

MSH-189 (Shogi Request Overhead) adds an incremental native session below the
search slot. `native::incremental::Session<R>` owns a rules-authoritative game;
the record adapter reconstructs a full reset and admits a typed event suffix
atomically. `incremental::SessionServer<A>` owns the shared transport lifecycle.
The adapter supplies native types, supported limits and target checks. Shogi is
the first adapter; other games use these same lifecycle implementations.

Each child first receives an ordinary full request. A full response can advertise
an exact prepared endpoint. Only then does the bridge send compact suffixes,
bound to the acknowledged request, generation, host version and event count.
Older engines without the capability continue to receive full requests. A count
alone is never history identity. The host checks the complete validated prefix;
the child applies each new command through native rules and checks every derived
event and final result. Invalid suffixes roll back. A wrong base requires a full
reset. Takeback, imported or incompatible roots reset; process retirement drops
both the native endpoint and search state. Fixed-work probes leave timed state
alone. Compact responses echo the exact metadata and endpoint; only after those
checks does the bridge attach its original immutable context. Saved observations
and native record formats remain unchanged.

Timed Shogi requests also keep one native legal root-order move available through
the shared `search_with_timed_fallback` entry point. If time or the node safety
limit stops the first iteration, the result has a legal choice, depth zero, no
score, actual visited nodes and a one-move unsearched continuation. This includes
zero-node timed requests. Explicit cancellation suppresses fallback. Already
concluded roots return the native result; declaration choices remain distinct.
Mandatory native root checks can exceed a minimum time budget. Fixed-work search
keeps its prior incomplete-result semantics. No evaluation or selectivity policy
changes are part of this task.

The identities are `mshogi.shogi.shared-core.v1` and
`mshogi.shogi.material-placement.v1`. Observation admission accepts only the
current identities; selective lines can extend beyond nominal depth.

Verification and the predecessor comparison are recorded in
evidence/msh180/README.md. The later search round's
before/after probes, game-test decision and limits are in
Shogi strength round.

## Xiangqi on the core

MSH-181 (Xiangqi Search Core Adoption) removes Xiangqi's private full-width
search. The core and trait are unchanged. A 90-square array and u128 piece
sets support blocked Horse legs, Elephant eyes and river restrictions, palace
moves, Cannon screens, Soldier river movement and facing Generals. Hash,
material and placement are updated in place. Existing independently specified
board fixtures and perft, randomized native comparisons and history tests
verify the boundary.

Native authority stays at the root and the application boundary. A concluded
root reports its conclusion. A root with pending adjudication stays unscored.
The result carries the native sound fast root assessment; an existential win
witness is not a forced-win score. A valid count
claim is chosen after a completed search if the move score is no better than
zero. The adapter never claims for the nonmoving player. The selected action
must be locally legal and must pass native submission in a private copy. The
reported line is replayed natively and ends at conclusion, pending adjudication
or a refused action. Native resource failures on the chosen action remain errors.

Private approximations are explicit:

- The table uses board and turn, without conduct history or count-claim state.
- A search-path repetition is scored at its second occurrence; earlier game
  history needs three occurrences. A continuous checker loses; other cycles,
  including chase cycles, score as a draw. This does not replace the selected
  native WXF warning, chase, exchange or fourth-occurrence rules.
- Captures and null moves bound the repetition scan. Non-board native events
  are not turns. Full native board history initializes the private position.
- No-legal-action loss, including unchecked stalemate, is tested before draw
  approximations. Bare Generals draw. No other no-win proof runs inside the
  tree. Count claims are native root options only.
- The exchange test is a one-step estimate. Evaluation uses material, Soldier
  advancement and centralization/development placement with a tempo bonus.
  It is not a rules adjudication or a learned evaluator.

Depth is 1–64 and nodes are bounded at 5,000,000. Default UCI 0–9 and desktop
play use depth 64 and 300,000 nodes. The reported line has at most eight actions.
Fixed-limit requests start with empty tables; timed requests use the shared
session policy above. The identities are `mshogi.xiangqi.shared-core.v1` and
`mshogi.xiangqi.material-placement.v1`.

See verification and predecessor evidence.

## Chu Shogi on the core

MSH-182 (Chu Search Core Adoption) replaces Chu's private full-width loop.
The shared trait adds `generate_search`, which defaults to complete `generate`.
The core uses this hook at its two staged generation points. A 144-square byte
array and three-word occupancy sets support complete single- and double-landing moves. A compact
32-bit move retains both landings and optional promotion. In-place updates keep
material, placement, original identity, promotion, royal sets, Lion protection
and hash; a small undo record restores up to two captured pieces.

The position implements native movement geometry and complete-action rules:
Lion jumps, double captures, igui and quiet returns; Soaring Eagle and Horned
Falcon special routes; optional promotion opportunities; defended-distance Lion
capture, qualifying intermediate victims, and next-turn Lion protection.
Public native movement descriptors supply geometry facts. The private position
performs its own route generation and state updates. Native fixtures, perft and
randomized action/result/hash/undo comparisons verify that boundary.

The adapter supplies the native legal root list, including must-vary restrictions.
Native conclusions are reported exactly; a live root without board actions stays
unscored. A selected action must pass native submission. The reported line is
replayed natively and stops at a conclusion or refusal; a refusal of its first
action is an error. A native one-action conclusion replaces a private score.

Private analysis remains separate from native adjudication:

- The hash includes original identity, promotion, turn and active Lion rights,
  but not the complete known history or counters.
- Path repetition at the second occurrence scores zero; pre-root repetition
  requires a third occurrence. Captures and promotion bound the scan. This is
  not the native same-sequence must-vary algorithm, and never declares a real
  draw. Native root admission and PV replay enforce that algorithm.
- Real compound passes remain legal complete moves. Null-move pruning is
  disabled. No Chess self-check filter is added.
- A threat means one admitted complete action can capture all surviving royals.
  Separate attacks on King and Prince do not become a one-action loss.
- Actual last-royal loss is terminal. The full native finite immediate-mate
  procedure is not called at each private node. Native conclusions retain
  priority at the application boundary.
- A locally admitted complete action that can capture every opposing royal
  scores a win one action ahead. This cuts known losing replies without removing
  any legal move. The reported selective line can stop before that capture;
  an actual win still requires native submission. Two separate royal attacks
  do not qualify for this shortcut.
- The private C08 material award counts only the narrow three-piece case,
  excluding dead unpromoted Pawn/Lance from that count while retaining their
  occupancy. The bare side can make an immediate royal-capture or equalizing
  reply. These reply probes use local admission, without full native must-vary
  obligations, and do not run native immediate-mate precedence. Native root
  conclusions and line replay remain authoritative. No general bare-royal draw
  or material-exhaustion draw is added.
- An internal position with no move scores zero as a private approximation;
  the native root still returns an unscored error, not a draw.
- Classical evaluation uses unchanged material values plus placement and tempo.
  Capture ordering accounts for both victims. It is not a native outcome.
- The exchange filter uses a bounded local material-recapture model. It counts
  both victims and promotion gain, and follows each capturing piece to its
  actual final square, including Lion return routes. Complete local admission
  enforces Lion rights. Each side may stop the exchange. This is not a full
  tactical proof: future checks are outside the exchange model, so a checking
  sacrifice can be undervalued. Actual royal loss, narrow material endings and
  unresolved work limits retain the move. The core bypasses the exchange filter
  when the current node is threatened. Equal recapture successors require equal board,
  turn, Lion rights and next target, not merely an equal hash. The filter changes
  private selective search only; full legal generation and native admission stay
  unchanged.
- Chu limits quiescence to eight additional plies, counted from each tactical
  search entry. The shared trait's default retains the existing absolute-ply
  limit for other games. Royal/material terminals and the one-action royal
  capture proof run before this horizon. Threatened nodes search all quiet
  evasions and do not stand pat before the horizon. At the horizon, a threatened
  position receives an uncertain zero estimate; other positions use static
  evaluation. This follows the existing absolute-ply horizon convention and
  never declares a native draw. Long tactical chains can be missed.
- Bounded quiescence stores negative elapsed tactical depth in the table.
  A near-horizon result cannot replace a fresh tactical query: score cutoffs
  require sufficient stored depth. Quiescence entries do not replace a current
  main-search entry. Other games retain zero-depth quiescence entries.
- Search generation keeps one admitted complete route for each equal successor
  within a stage. Equality includes all piece identities and promotions, turn
  and active Lion rights. Native root restrictions apply before this reduction.
  Full `generate`, legal actions and perft retain every route. This avoids
  repeated analysis of equivalent Lion routes without changing native actions.

Nominal depth is 1–64, with at most 5,000,000 nodes. Default play uses depth 64
and 300,000 nodes. A reported line contains at most eight complete actions.
Fixed-limit requests start a fresh searcher with a 16 MiB transposition table
and bounded history tables. Timed requests use the shared session policy above.
Current identities are `mshogi.chu.shared-core.v1` and
`mshogi.chu.material-placement.v1`.

## Tenjiku Shogi on the core

MSH-183 (Tenjiku Search Core Adoption) replaces the private full-width loop
with the same shared searcher. No Tenjiku branch is added to the core. Chu
and Tenjiku share a small fixed-word square-set implementation. Tenjiku uses
four words for 256 squares, with cached owner, rank, royal and Fire Demon sets.
The board retains original identities, base pieces, promotion and refusal
rights. A 32-bit move retains up to three landings or a stationary igui action.
An undo record has a count and space for twelve changed squares: the source,
three landings and eight burn neighbours. It restores captures, mover death,
promotion rights, material, placement and both hashes.

Both native profiles supply the immutable rules identity. Private geometry
implements all 44 faces, including ranked jumps, profile-specific post-jump
victims, area routes, compound captures and Heavenly Tetrarch igui. Complete
effects apply direct captures, Demon hazards and burns, mover death, then
surviving promotion. Adams burns before a moving Demon dies; Muller suppresses
that burn. A newly promoted Demon does not burn on its promotion action.
Tests compare full actions and states with native rules in both profiles.

The adapter obtains exact native root conclusions and legal actions, including
repetition admission. It checks the selected action and replays the reported
line through native submission. A refused first action is an error; a later
refusal clips the line. A native first-action conclusion replaces the private
score.

Private analysis has these limits:

- The search hash includes profile, turn, original identities, promotion and
  refusal rights, but omits full history and counters. The repetition hash
  omits physical piece IDs and refusal rights, but retains original kind and
  promotion, as native board repetition does. Known board actions are replayed
  once to seed that history. A private
  repeated board scores zero; it does not declare a draw. Native Tenjiku
  forbids the first known recurrence, not Chu's same-sequence must-vary rule.
- Complete loss of both royal sets scores zero. A one-action win shortcut
  requires a complete local action that removes every enemy royal while an
  own royal survives. Separate attacks on two royals do not suffice.
- The bare-royal calculation uses Tenjiku's local pending reply: capture all
  enemy royals or equalize to two bare royals. It does not use Chu's narrow
  material award or dead-piece exclusion. These private replies do not apply
  full native history admission. Native root and replay remain authoritative.
- No Chess self-check filter or Chu protected-Lion rule is added. Null-move
  pruning is disabled. An internal position with no move scores zero; a live
  native root without board actions remains an unscored error.
- Material values are unchanged. Placement and tempo supplement them. Move
  gain includes all direct and burn victims, promotion and mover suicide.
  The exchange filter follows complete local recaptures of the last mover,
  including off-target Demon burns. It permits either side to stop. This is
  a material heuristic, not proof against future checks or other sacrifices.
  It retains uncertain royal/material endings and exhausted work. Each query
  allows eight recapture levels, 63 projections, 2,048 generated routes and
  4,096 geometry probes. The core bypasses this filter at threatened nodes.
- Quiescence uses a four-ply bound and the shared signed tactical table depth.
  Before the horizon it searches quiet evasions when threatened. At the
  horizon a threatened position receives an uncertain zero estimate; other
  positions use static evaluation. This does not establish a native result.
  Longer exchanges, repeated threats and delayed burn effects can be missed.
  The initial eight-ply setting exhausted even 100,000 nodes before completing
  an iteration on an ordinary game position. The shorter bound controls that
  cost; its playing effect is assessed separately through predecessor games.
- Search generation retains one route for each exactly equal successor in
  each stage. Equality includes every cell's original identity and rights;
  turn and profile are fixed within a stage. Native root admission precedes
  this reduction. Full legal generation and perft retain all complete routes.

Nominal depth is 1–64, with at most 5,000,000 nodes and eight reported actions.
Default play uses depth 64 and 50,000 nodes, selected from fixed-root cost
and completion measurements in both profiles.
Fixed-limit requests create a fresh 16 MiB transposition table. The 22,528 history
slots use 90,112 bytes of basic history and the capped 32 MiB continuation
table. Timed requests use the shared session policy above. Current identities are `mshogi.tenjiku.shared-core.v1` and
`mshogi.tenjiku.material-placement.v1`.

## What Tori showed, and what the other games need

Findings from the second game, recorded for the later tasks of Shogi, Xiangqi,
Chu Shogi and Tenjiku Shogi. These are the findings at the Tori checkpoint;
the Shogi and Xiangqi implementations above record the later decisions.

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
  Shogi. MSH-178 (Search Core History Scaling) resolves this limit with the capped
  dense/hashed table documented above; these are the earlier rough estimates.
- *Score scale.* The margins of the core (futility 150, 300 and 450,
  aspiration 30, quiescence 200) assume that the smallest piece is worth about
  100. Each game must use that scale, or the margins must come from the trait.
- *Checks in quiescence.* Quiescence searches the tactical stage only. In
  the drop games a quiet drop can give check or mate. Tori relies on the check
  extension of the main search. The first Shogi checking-stage experiment in
  MSH-187 (Shogi Strength Round) increased first-iteration work enough to leave
  a request incomplete. It was rejected; future work must measure that cost.
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
  bounded continuation history described above is now available.
- *Tenjiku Shogi (256 squares).* As Chu Shogi, with the Fire Demon burn, which
  removes several pieces in one move: make and unmake need a variable-length
  capture record, and the evaluation of a move's gain must include burned
  pieces. The four jumping generals (Great General rank 3, Vice General rank 2,
  Rook and Bishop General rank 1; royals rank 4) jump along their lines over
  any number of lower-ranked pieces of either side. Only a piece of equal or
  higher rank stops them, and after a jump they cannot capture a royal piece.
  Move generation, attack detection and exchange estimates therefore need
  occupancy by rank. Lower-ranked occupancy still affects quiet paths and
  whether a capture is before or after a jump. Two
  rule profiles exist, and they differ in what a general can capture after a
  jump; the position takes the profile from the native game.

**The same work for each adapter.** The engine protocol of each game checks a
saved observation against its search identity (`mshogi-engine/src/<game>.rs`).
A game that moves to the core needs the same split as Tori: the earlier
identity keeps its exact checks, and the shared-core identity has checks that
fit a selective search. Put that split in one shared helper when the second
of these games moves, not in a copy for each game.

## Present limits

- MSH-166 (Search Selectivity Completion) adds reverse futility, razoring,
  late-move pruning, internal iterative reduction, singular extensions and an
  improving modifier to the shared core for Chess and Tori. The combined set
  passed its sequential predecessor test (+40 -3 =17) and improved the development
  blunder result from 86 to 98 of 146. See evidence.
  Static pruning remains a private approximation; native roots and real outcomes
  retain their authority. Low-material LMR remains enabled.
- MSH-167 (Engine Time Management) adds shared clock allocation and adaptive
  soft targets, with a hard response budget. Chess UCI uses it; fixed move-time
  diagnostics and desktop defaults retain their policy. See the
  bounded comparison.
- The evaluation has the same arithmetic as before. MSH-168 (Classical Endgame
  and King Evaluation) owns its improvement.
- All six games support time budgets and retained state through the internal
  protocol. Desktop choices remain in MSH-140 (Player Thinking Time); existing
  defaults retain their fixed node limits outside Chess.
- MSH-190 (Shogi Selectivity Round) makes the margins game-scaled
  (`MARGIN_UNIT`, 100 for Chess, Tori, Xiangqi, Chu and Tenjiku, so their
  search is unchanged) and adds two opt-in core features: a bounded quiescence
  check stage (`QUIESCENCE_CHECKS`, `generate_checks`) and a root mate search
  (`MATE_SEARCH`). Only Shogi enables them so far; another game enables them
  after its own game test. See Shogi selectivity round.
- The search is single-threaded.
