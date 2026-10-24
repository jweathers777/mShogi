# Exact chess mating reachability and full adjudication

MSH-26 adds a complete finite-state decision procedure for cooperative Article 3 mating reachability, including dead positions outside the M2 material subset. It does **not** claim a practical runtime or memory bound for arbitrary positions. Existing constructors remain on the sound M2 subset. Full mode is an explicit recorded choice, with optional operational work limits; interrupted work remains unknown and cannot invent a chess score.

## Rules interpretation and authority

The pinned authority is the [FIDE Laws effective 2023-01-01](https://handbook.fide.com/chapter/E012023): Article 3.10.1 defines legal moves through Articles 3.1–3.9; Article 5.2.2 asks whether either side has any legal mating continuation; Articles 5.1.2 and 6.9 ask the same question for the opponent of the resigning/flagged player. Both players cooperate in this existence question. A forced-win engine score, adversarial draw, failure to find a mate, and material advantage are not answers.

This implementation follows the Article 3 interpretation already documented and tested in MSH-18: hypothetical witnesses disregard future optional claims, selected auto-claim policy, clocks, and 50/75-move or repetition outcomes. Actual previously concluded results still take precedence. Counter storage limits do not affect hypothetical legality. Castling permissions, promotion choices, legal en passant, king safety, checkmate and stalemate do affect it. This is not a historical-reachability proof for imported FENs; the existing validated local-position contract remains authoritative.

[A Practical Algorithm for Chess Unwinnability, Ambrona, FUN 2022](https://drops.dagstuhl.de/storage/00lipics/lipics-vol226-fun2022/LIPIcs.FUN.2022.2/LIPIcs.FUN.2022.2.pdf) supplies useful primary research: exhaustive cooperative search can decide unwinnability; structural mobility overapproximations can soundly accelerate blocked positions. Its completeness argument requires growing search bounds, distinct from the constant per-iteration cap used experimentally. mShogi implements its own simple exact traversal over its own rules, without importing the paper's more involved structural analysis or another move generator. The author's [current implementation description](https://chasolver.org/how-it-works) explicitly separates empirical coverage from a theoretical completeness guarantee. Those external performance numbers are not mShogi evidence.

## Solver and proof contract

`Position::mate_reachability(target, limits, cancelled)` uses breadth-first traversal. `MateTarget::Side(color)` asks about that side; `Either` asks about either side and therefore establishes global deadness when impossible. Actions are ordered by ascending coordinate strings. A collision-checked HashMap of `RepetitionIdentity` retains exact board, side, castling rights and legally available EP. It is an index with full equality, never a hash-only proof. Counters are normalized to halfmove zero/fullmove one before each hypothetical transition.

`MateReachability` has three disjoint results:

- `Possible { witness, work }`: a complete legal action sequence ending in checkmate delivered by the requested side, or either side for `Either`. It may be empty only when the starting position is already the requested checkmate. Intended witnesses never change the live board.
- `Impossible { work }`: the entire reachable graph was exhausted without the requested target. The only pruned leaves use existing sound M2 global-dead proofs or the requested side having only a bare king. Checkmate/stalemate are terminal vertices; no moves follow them.
- `Unknown { reason, work }`: cancellation, position limit, transition limit, or a detected allocation/work-counter resource limit prevented a decision. No negative chess conclusion follows.

The negative proof is reproducible graph exhaustion, not a separately serialized graph certificate. Work observations count discovered exact identities, expanded vertices, examined legal transitions and static-proof leaves. Counts alone are not trusted proof: live construction/match replay executes the solver again under the recorded configuration. Positive witnesses can also be checked independently through `mating_feasibility`.

Correctness follows by induction: every queued vertex is reachable by its parent action; every unpruned reachable vertex's legal outgoing actions are enumerated; merging identical keys preserves Article 3 successor behavior; only proven impossible leaves are pruned. Therefore a target found is possible, and exhausting the queue without one proves impossible. Early resource exits explicitly cannot enter the impossible branch.

For termination, each board square has finitely many piece labels, the side/rights/EP domains are finite, and captures/promotions never create an unbounded number of occupied squares. Counters and historical claims are outside this hypothetical relation. Exact visited-state merging adds each finite vertex at most once. Unlimited traversal therefore terminates with an exact answer **given sufficient resources**. The theoretical graph is enormous; the finite-state argument is not a usability guarantee or evidence that all legal positions were enumerated. The implementation may report a detected platform resource limit, and an external OS out-of-memory kill cannot be converted into an in-process result.

## Live-game and match integration

`AdjudicationMode::M2` preserves existing behavior. `Full { limits }` selects the full algorithm; `ReachabilityLimits { max_positions: None, max_transitions: None }` requests unlimited work. Finite values bound distinct retained vertices including the root and examined transitions. Zero budgets may return unknown even for readily recognizable positions. Limits are work limits, not wall-clock time promises.

`AdjudicatedGame::from_fen_with_mode` and `from_history_with_mode` select the mode. History reconstruction validates the complete live path under it. Full global-dead analysis is cached at construction and refreshed on each board action/analysis undo; `outcome()` reads do not rerun the solver. A full impossible result concludes a dead-position draw, before fivefold/75-move outcomes but after checkmate/stalemate. Full unknown retains the existing sound M2/history conclusions and creates no additional result. Claims remain optional and unavailable after any conclusion. `dead_analysis()` exposes the cached proof/work observations.

`ChessMatch` uses the live game's mode. With no supplied witness, full-mode resignation/flag fall runs side-specific reachability. Generated witnesses, exhaustion or unfinished work are retained in the corresponding reply's `feasibility_evidence`. Possible yields a justified win, impossible a draw, and unknown an explicitly unresolved/unscored termination. An explicit supplied certificate continues through the existing checker. Clock ordering and prior-outcome precedence remain unchanged. A full dead position concludes before a later resignation or flag request can replace it.

The standalone solver checks a caller's cancellation callback before exploration, between vertices and between transitions; it never mutates the input. Cached live/match calls are synchronous and use their recorded work limits, not an unserializable cancellation callback. Configure finite budgets for latency-sensitive callers. Unlimited mode must be a deliberate offline/resource decision; neither default search nodes nor legacy matches acquire it implicitly. No playing-strength claim follows.

## Record migration and compatibility

The rules profile remains `mshogi.chess.match.v1`: this implements its specified reachability more fully rather than changing the underlying laws. Encoding versions change where configuration must be preserved:

| Record | Legacy | Full mode |
| --- | --- | --- |
| Game history/live record | Schema 1, original fields/bytes, implicit M2 | Schema 2, required `adjudication_mode` containing `full` and limits |
| Match envelope | Schema 1, M2 initial history, no generated solver evidence | Schema 2, embedded full-mode history; generated evidence in replies |

Schema/configuration mismatches are rejected. Schema 1 cannot silently contain a full configuration, and a schema 2 game cannot omit it. Legacy constructors and record outputs remain unchanged; old records replay under M2 rather than being reinterpreted as full games. Match replay recomputes all observations, including generated witnesses and work counts, and rejects discrepancies. Full-mode live replay also refuses historical play after a dead conclusion that the M2 subset would have missed.

`GameRecord::replay_history()` remains diagnostic action reconstruction and does not run adjudication. Its `adjudication_mode()` getter lets adapters detect configuration they cannot restore. An adapter must use live replay or refuse a full-mode record, rather than silently dropping the mode. Full record replay can be expensive: decoding metadata is separate from explicitly executing replay.

## Focused evidence and remaining limitations

`cargo test -p mshogi-core --test completeness --test adjudication --test matchplay --test game_record --test game_history --locked` covers the new boundary and legacy behavior. `cargo clippy -p mshogi-core --all-targets --locked -- -D warnings` checks production/tests/examples. Parent integration records the actual revision, environment, independent review and native CI; a local run alone establishes no other platform result.

The main independent negative fixture is `8/7k/8/p1p1p1p1/P1P1P1P1/8/K7/8 w - - 0 1`. Locked pawn pairs and their permanent attacks confine each king to a separate 3×8 rectangle. All 24×24×2 = **1152** vertices are reachable; each directed rectangle has 42 horizontal + 32 vertical + 56 diagonal = 130 arcs, giving **6240** total transitions. The full solver and an independent DFS using FEN string keys/no production identity or pruning both match those counts. This tests a real cyclic closed graph beyond the material subset.

The forced-capture fixture `8/8/8/8/2b5/2k5/8/Kr6 w - - 0 1` has exactly `Kxb1`, then KB versus K: the root is beyond M2 but full traversal proves death. Other checks cover an actual move closing the pawn wall, live conclusion/undo/replay, generated resignation/timeout witnesses, one-sided impossibility, checkmate/stalemate, maximal imported counters, exact budget boundaries, cancellation, corrupted evidence and schema migration. Existing independent M1 identity/action fixtures continue to cover castling, EP and promotion legality.

Remaining practical limitations are explicit: breadth-first space can become prohibitive; difficult full-board positions can exhaust any usable budget; no structural blocked-region acceleration, general tablebase or universal practical latency guarantee is provided. Operational unknown is retained as unscored where an exact result is required. The complete unlimited algorithm exists; a bounded run is never advertised as full coverage or a completed proof.
