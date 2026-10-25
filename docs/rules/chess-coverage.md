# Chess coverage ledger

Contract: [`mshogi.chess.position.v1`](chess-position-v1.md). Established by MSH-2 on 2026-10-23 and maintained through the M1 tasks MSH-3 to MSH-14 (see [the M1 closure review](../review/m1-closure.md)).

State values:

- **Implemented**: implemented and verified by the cited evidence.
- **Deferred**: a requirement that remains, with a named owner.
- **Unresolved**: a material behavior/scope question that needs an owner decision before implementation; routine API choices remain implementation work.
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
| Rules-owned game history and record (profile ID, initial state, complete actions, repetition identity, history-restoring undo) | Implemented | [Game/history record contract](chess-history-record-v1.md): exact known occurrences, imported-history provenance, policy, owned undo; ordered command/clock observations replay with action context. Console reloads move-only M2 records and refuses mode loss. Live/match replay adjudicates commands/clocks; diagnostic replay alone does not | MSH-16; MSH-17/18/23 semantic replay, MSH-25 integration closure |
| Dead position (FIDE 5.2.2), approved M2 subset | Implemented (subset) | [Adjudication](chess-adjudication.md): sound kings/minor/same-color-bishops proofs; other cases remain Unknown in the default M2 mode | MSH-17 subset; MSH-26 opt-in exact mode below |
| Resignation and its possible-mate exception (5.1.2) | Implemented (bounded) | [Match API](chess-match-api.md): sound impossibility/verified witness, otherwise unresolved and unscored | MSH-18; MSH-26 full feasibility |
| Agreed draws (5.2.3, 9.1) | Implemented | Rules-owned offer/accept/decline with both recorded first moves; deterministic replay tests | MSH-18 |
| Claimable threefold repetition (9.2) | Implemented | Current/intended optional claims, separate outcomes, policy selection and live replay; independent profile fixtures | MSH-17 |
| Claimable fifty-move draw (9.3) | Implemented | Optional current/intended claims; halfmove 100 alone does not conclude a game | MSH-17 |
| Automatic fivefold / 75-move draw (9.6) | Implemented | Rules-owned live boundary, known history and checkmate precedence; profile/undo/replay tests | MSH-17 |
| Time loss with possible-mate exception (6.9), clock policies | Implemented (bounded) | Injected untimed/increment clocks, deadline/increment ordering, sound proof/witness or unscored uncertainty, semantic match replay | MSH-18; MSH-23 runner, MSH-26 full feasibility |
| Transparent static evaluation | Implemented | [Material baseline](../evaluation.md), independent score/perspective/symmetry tests; no outcomes or strength claim | MSH-19 |
| Full-width search baseline | Implemented | [Search](../search.md): independently solved trees/chess, full game history and claim policy, private state, deterministic work limits; measured release baseline | MSH-20 |
| Competitive strength | Deferred | No playing-strength claim from the baseline search | M5 |
| UCI chess engine protocol | Implemented (subset) | [UCI adapter](../uci.md): separate machine mode, private history-sensitive search, depth/node/basic clock limits, cancellation and protocol-only stdout. Real child-process tests and recorded complete Stockfish19 matches | MSH-22/23 |
| Self-play and independent-engine experiments | Implemented | [Runner](../match-runner.md): authoritative rules, paired openings, bounded UCI supervision, executable/model/config identities, explicit unscored failures and complete canonical match replay. Actual standard-start/forced-ending self-play and Stockfish19 games, conditional work-limited reruns and timed observations | MSH-23; MSH-25 integration closure |
| Other engine protocols | Deferred | USI, CECP/XBoard and UCCI remain available when their game/opponent requires them | Future variant/opponent tasks |
| Learned evaluation and neural infrastructure | Deferred | Not started | M5 |
| Other five promised games | Deferred | Requirements remain; none implemented or certified by this corpus | M4 |
| History, claims and automatic-result policy | Implemented | Known history and live outcomes/claims follow [match v1](chess-match-v1.md); imported origin and optional claims remain explicit | MSH-16, MSH-17 |
| Full dead recognition and side-specific possible mate | Implemented (exact algorithm, opt-in) | [Completeness contract](chess-completeness.md): finite cooperative Article-3 graph exploration, verified positive continuations and exhaustive negative proofs, including blocked pawn walls outside the M2 subset. Unlimited mode is complete given sufficient resources; bounded or cancelled work remains Unknown. No practical runtime guarantee for arbitrary chess positions. Full live/match mode and schema 2 preserve the selected limits; default M2/schema 1 is unchanged | MSH-26; resource uncertainty still leaves timeout/resignation unscored |
| Software match/event policy | Implemented | Untimed/increment clocks, optional core claims, recorded unattended auto-claims, command/clock replay and failure separation follow [match v1](chess-match-v1.md). Watchdog and clock boundaries retain actual arrival/deadline evidence | MSH-18/23 |
| All-six-game save schema and search-cache representation | Deferred | Not required to settle in M2; ordinary representation choices belong to implementers within A-03/A-06 | Actual consumers in M2–M4; no prerequisite universal design |
| Physical touch-move, arbiter penalties, illegal-move forfeits, scoresheets and event procedures | Outside scope | Over-the-board procedures; software must not claim tournament compliance | Refine the concrete software match scope before release |

Primary rule reference: [FIDE Laws effective 2023-01-01](https://handbook.fide.com/chapter/E012023), Articles 3, 5, 6.9 and 9. The accepted M1 exclusions come from [the design](../design.md); this ledger keeps them as visible obligations rather than relabelling them implemented.

**Milestone ownership.** Following the design's M2 outcome ("complete the needed chess match semantics"), M2 owns every Deferred FIDE match-semantics row above. Any item that M2 explicitly postpones needs an owner decision recorded in its task and this ledger. The approved 2026-10-24 exception is full dead-position/possible-mate detection in MSH-26 (M4); M2 implements the sound subset and explicit unknown handling. MSH-25 verifies integrated M2 closure without repeating the M1 audit.

## Questions and decisions

There is no unresolved orthodox movement-rule ambiguity blocking this profile. MSH-2 chose a documented **local analysis** boundary rather than promising historical reachability. Strict six-field FEN, the counter domain, canonical rights order, local EP checks and preservation of the raw target are project parsing decisions, not invented FIDE rules. They are resolved for v1 and the corpus exercises them.

Owner decision (2026-10-23, MSH-9): chess legality and the M1 classifier are independent of counter representability. Perft and divide never skip legal branches or treat overflow as a leaf.

Known oracle differences:

- python-chess accepts some inputs this profile rejects, such as omitted FEN fields or fullmove zero.
- Its default FEN may suppress an uncapturable target.
- Its validity status can reject impossible-check diagrams beyond the specified local boundary.

Neither its parser nor `outcome()` defines this profile. The optional Python checker evaluates oracle position and accepted-FEN expectations and deliberately skips rejection-policy execution; the Rust MSH-4 suite executes those rejection contracts against the product parser.
