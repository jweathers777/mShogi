# Chess coverage ledger

Contract: [`mshogi.chess.position.v1`](chess-position-v1.md). Established by MSH-2 on 2026-10-23 and maintained through the M1 tasks MSH-3 to MSH-14 (see [the M1 closure review](../review/m1-closure.md)).

State values:

- **Implemented**: implemented and verified by the cited evidence.
- **Deferred**: a requirement that remains, with a named owner.
- **Unresolved**: a design question that needs an owner decision before implementation.
- **Outside scope**: not a software rules obligation of this profile.

| Obligation | State | Present state and evidence | Owner / follow-up |
| --- | --- | --- | --- |
| Position fields, strict FEN and local analysis validation | Implemented | All 68 independent inputs tested for acceptance/rejection/canonical state, plus targeted boundary tests; no history/reachability proof | MSH-4 |
| Ordinary moves, captures, promotions, rights/counters and exact undo | Implemented | Checked execution and full-state undo: 49 independent sequences / 74 exact after-states plus rejection/overflow tests. Pseudo-legal ordinary candidates match 17 independent exact sets / 274 candidates, and the candidate contract holds for every bounded action | MSH-5, MSH-7 |
| Occupancy-aware attacks, pinned attackers, king safety | Implemented | Geometric attack queries for every piece kind and both colors: 29 independent diagrams / 155 exact attacker sets plus complete per-color maps; used by the legal list | MSH-6, MSH-9 |
| Orthodox castling and en passant | Implemented | Checked execution through `Position::apply` with full-snapshot undo, plus special candidates: 21 independent positions, 6 sequences / 25 exact after-states, 18 expected rejections | MSH-8 |
| Legal actions; check, checkmate, stalemate (position-level) | Implemented | `legal_actions`, `in_check`, `classify`. Exact legal sets and classifications for all 35 corpus positions, 8 classification diagrams and 21 special-action positions. Legality ignores the u32 counter domain (owner decision 2026-10-23); `CounterOverflow` is a representation failure. This is the limited M1 classifier, not a game result | MSH-9 |
| Perft/divide | Implemented | Explicit depth semantics, deterministic order, counter-independent traversal. 38 corpus perft totals and 14 ordered divides match; three Stockfish depth-5 totals reproduce in the opt-in release baseline. A speed baseline is recorded, not a strength claim | MSH-10 |
| Headless inspection console | Implemented | `mshogi shell`: load/inspect, legal list, atomic apply/undo, classification, perft/divide, with documented output, errors and exit status. Process-tested and smoke-run in native CI. Diagnostic only | MSH-11 |
| State properties, sequence and differential checks | Implemented | Fixed-seed sequences from 92 start positions check every ply (6,720 plies in CI, 316,880 deep). Replayable regression family. Optional differential with pinned python-chess: bounded agreement, not proof. No derived caches exist yet | MSH-12 |
| Broader complete-action semantics | Implemented (review) | [Action-boundary review](../review/m1-action-boundary.md). Chess boundaries are intentionally chess-specific; a synthetic test-only compound-action fixture is mapped to future adapters. No A-02/A-03 conflict | MSH-13 |
| Headless kernel on Windows, macOS and Linux | Implemented | Native CI build, test and console smoke on `ubuntu-24.04`, `macos-15` and `windows-2022` for every behavior task; the integrated-revision run is recorded in MSH-14 | MSH-3 to MSH-14 |
| Packaged desktop application on three systems | Deferred | Not started | M3 |
| Rules-owned game history and record (profile ID, initial state, complete actions, repetition identity, history-restoring undo) | Deferred | Position is local only; the console's undo stack is adapter state | M2 |
| Dead position (FIDE 5.2.2) | Deferred | The obligation is impossibility of mate by **any** legal sequence; a material heuristic cannot close it | M2 (its extent needs an owner decision; see below) |
| Resignation and its possible-mate exception (5.1.2) | Deferred | Match command, not a board action | M2 |
| Agreed draws (5.2.3, 9.1) | Deferred | Event restrictions and offer/accept state are outside position classification | M2 |
| Claimable threefold repetition (9.2) | Deferred | Current- and intended-move claims; rule-equivalent positions need history, rights and legal EP distinctions | M2 |
| Claimable fifty-move draw (9.3) | Deferred | Current- and intended-move claims; not an automatic terminal test at halfmove 100 | M2 |
| Automatic fivefold / 75-move draw (9.6) | Deferred | History and halfmove handling, including last-move checkmate precedence | M2 |
| Time loss with possible-mate exception (6.9), clock policies | Deferred | A counter or FEN cannot settle these results | M2 (match runner) |
| Search, evaluation and competitive strength | Deferred | Not started; perft and diagnostics are not search evidence | M2 baseline, M5 strength |
| External engine protocols (UCI and others) | Deferred | The console is a human diagnostic session, not a machine protocol | M2 (first adapter) |
| Learned evaluation and neural infrastructure | Deferred | Not started | M5 |
| Other five promised games | Deferred | Requirements remain; none implemented or certified by this corpus | M4 |
| History/claim API shape, intended-move claim handling, automatic-result ordering | Unresolved | Design questions recorded in M1 | Owner decision in M2 |
| Extent of dead-position recognition | Unresolved | Full FIDE 5.2.2 versus a documented, explicitly incomplete subset | Owner decision in M2 |
| Software match/event policy; all-six-game save schema; search-cache identity | Unresolved | Deliberately not frozen in M1 | Owner decisions in M2 to M4 |
| Physical touch-move, arbiter penalties, illegal-move forfeits, scoresheets and event procedures | Outside scope | Over-the-board procedures; software must not claim tournament compliance | Refine the concrete software match scope before release |

Primary rule reference: [FIDE Laws effective 2023-01-01](https://handbook.fide.com/chapter/E012023), Articles 3, 5, 6.9 and 9. The accepted M1 exclusions come from [the design](../design.md); this ledger keeps them as visible obligations rather than relabelling them implemented.

**Milestone ownership.** Following the design's M2 outcome ("complete the needed chess match semantics"), M2 owns every Deferred FIDE match-semantics row above. Any item that M2 explicitly postpones needs an owner decision recorded in its task and this ledger. It then moves to the M4 chess-completeness work, so nothing is left unowned.

## Questions and decisions

There is no unresolved orthodox movement-rule ambiguity blocking this profile. MSH-2 chose a documented **local analysis** boundary rather than promising historical reachability. Strict six-field FEN, the counter domain, canonical rights order, local EP checks and preservation of the raw target are project parsing decisions, not invented FIDE rules. They are resolved for v1 and the corpus exercises them.

Owner decision (2026-10-23, MSH-9): chess legality and the M1 classifier are independent of counter representability. Perft and divide never skip legal branches or treat overflow as a leaf.

Known oracle differences:

- python-chess accepts some inputs this profile rejects, such as omitted FEN fields or fullmove zero.
- Its default FEN may suppress an uncapturable target.
- Its validity status can reject impossible-check diagrams beyond the specified local boundary.

Neither its parser nor `outcome()` defines this profile. The optional Python checker evaluates oracle position and accepted-FEN expectations and deliberately skips rejection-policy execution; the Rust MSH-4 suite executes those rejection contracts against the product parser.
