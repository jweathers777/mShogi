# Chess coverage ledger

Contract: [`mshogi.chess.position.v1`](chess-position-v1.md). Updated by MSH-2 on 2026-10-23. **Specified** means required of future work, not implemented or verified by the current prototype. No M1 product behavior is certified by MSH-2.

| Obligation | M1 commitment / present state | Owner / follow-up |
| --- | --- | --- |
| Position fields, strict FEN and local analysis validation | Specified; acceptance/rejection data curated | MSH-4 |
| Ordinary moves, captures, promotions, rights/counters and exact undo | Specified; independent move sets curated; no mShogi conformance run | MSH-5, MSH-7 |
| Occupancy-aware attacks, pinned attackers, king safety | Specified; pin, x-ray, interposition and double-check cases curated | MSH-6, MSH-9 |
| Orthodox castling and en passant | Specified; both colors and rejection/king-safety cases curated | MSH-8 |
| Check, checkmate, stalemate | Specified position classification; data covers both colors | MSH-9 |
| Perft/divide and inspection console | Specified; six reference positions with depth 0..3 expectations plus terminal cases | MSH-10, MSH-11 |
| State properties, fuzzing/sequence/differential checks | Curated oracle comparison only; product sequence verification not run | MSH-12; focused tests begin in earlier implementation tasks |
| Broader complete-action semantics | Chess adapter does not establish compound-action support | MSH-13, then MSH-14 review |
| All three desktop operating systems | No product build/test/smoke evidence in MSH-2 | MSH-3 CI; MSH-14 requires actual platform evidence |
| Dead position (FIDE 5.2.2) | Deferred beyond M1: impossibility of mate by **any** legal sequence is the obligation, not just a material heuristic | M2 match-semantics refinement before claiming complete chess |
| Resignation and its possible-mate exception (5.1.2) | Deferred; match command, not board action | M2 match-semantics refinement |
| Agreed draws (5.2.3, 9.1) | Deferred; event restrictions and offer/accept state are outside position classification | M2 match-semantics refinement |
| Claimable threefold repetition (9.2) | Deferred; current/intended-move claims and rule-equivalent positions require history, rights and legal EP distinctions | M2 match-semantics refinement |
| Claimable fifty-move draw (9.3) | Deferred; current/intended-move claims are not an automatic terminal test at halfmove 100 | M2 match-semantics refinement |
| Automatic fivefold / 75-move draw (9.6) | Deferred; history and halfmove handling, including last-move checkmate precedence, remain obligations | M2 match-semantics refinement |
| Time loss with possible-mate exception (6.9), clock policies | Deferred; a counter/FEN cannot settle these results | M2 match semantics, match runner |
| Physical touch-move, arbiter penalties, illegal-move forfeits, scoresheets and event procedures | Outside M1 position API; software cannot silently claim full over-the-board tournament compliance | Refine concrete software match scope before release |
| Complete chess, competitive strength and GUI | Not claimed; M1 is deliberately narrower | M2/M3 and later release evidence |
| Other five promised games | Requirements remain; none implemented or certified by this chess corpus | M4 and the six-game design |

Primary rule reference: [FIDE Laws effective 2023-01-01](https://handbook.fide.com/chapter/E012023), Articles 3, 5, 6.9 and 9. The accepted M1 exclusions come from [the design](../design.md); this ledger retains them as visible obligations rather than relabeling them implemented.

## Questions and decisions

There is no unresolved orthodox movement-rule ambiguity blocking this profile. MSH-2 chooses a documented **local analysis** boundary rather than promising historical reachability. Strict six-field FEN, counter domain, canonical rights order, local EP checks and preservation of raw target are project parsing decisions, not invented FIDE rules. They are resolved for v1; the corpus exercises them.

Before M2 work, refine the history/claim API, intended-move claim handling, automatic-result ordering, the extent of dead-position recognition, and software match/event policy. Those questions are deliberately unresolved outside M1; a heuristic insufficient-material detector cannot close the dead-position obligation. All-six-game public save schemas and cache identity remain deferred design questions. No new task IDs or speculative M2 implementation plans are created here; these are roadmap refinement inputs.

Known oracle differences: python-chess accepts some inputs this profile rejects (e.g. omitted FEN fields or fullmove zero), its default FEN may suppress an uncapturable target, and its validity status can reject impossible-check diagrams beyond the specified local boundary. Neither its parser nor `outcome()` defines this profile. The corpus checker independently evaluates position expectations only, and reports parser rejection cases as future contracts rather than passing product parser tests.
