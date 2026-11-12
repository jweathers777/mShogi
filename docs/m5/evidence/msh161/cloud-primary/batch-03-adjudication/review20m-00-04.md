# MSH-161 (Competitive Gameplay Diagnosis): Batch 03, pairs 00–04, 20M review

This independent review covers all 167 selected roots and 306 completed queries in this half of Batch 03. It uses the frozen method, original native histories, the initial independent review, and cached 20M artifacts. No engine, retrieval, cloud job, or production change was run. Every selected PV was independently replayed as legal: 306 checks. Native results remain five candidate wins, four losses and one automatic threefold draw.

## Dispositions

| Root disposition | Candidate | Opponent | Total |
| --- | ---: | ---: | ---: |
| Supported finite event at the 0.05 threshold | 32 | 30 | 62 |
| Supported finite subthreshold comparison | 17 | 8 | 25 |
| Same-choice control | 24 | 2 | 26 |
| Finite already-lost secondary decision | 5 | 5 | 10 |
| Unresolved report stability or bound conflict | 23 | 17 | 40 |
| Unresolved same-choice report | 2 | 0 | 2 |
| Unresolved significance threshold | 1 | 1 | 2 |

These are selected-root counts, not all-game error rates or independent causal episodes. Multiple roots can describe one exchange of chances. Same-choice controls do not establish unique best moves. The finite already-lost marker uses alternative E at most 0.10; it is not an exact loss or proof that every legal move loses.

For each comparison, `dispositions20m-00-04.json` gives point loss, observed last-two-depth sensitivity range, conservative severity, separate 0.04/0.05/0.06 sensitivity, original 5M loss, all flags, artifact paths and legal SAN continuations. Stable support requires both reports to pass the frozen finite-support rules. Stale-report flags remain visible. A range is not a statistical interval. Missing stability or conflicting bounds keep a claim unresolved regardless of apparent size. No source flag was cleared merely because the node count increased.

Two stable comparisons straddle the 0.05 significance threshold: pair 01 game 0 ply 23 ...O-O versus ...Ne7 has a range of 0.0435–0.068; pair 01 game 1 ply 17 ...Qc7 versus ...Nxd4 has a range of 0.0455–0.051. Both support the 0.04 sensitivity threshold but not the primary 0.05 claim. The second remains below 0.06; the first straddles it.

## Game-level changes and linked evidence

All plies are zero-based. “Supported” below means stable finite support, not exactness or a proved engine defect. Earlier unresolved and unselected roots prevent an unqualified first-error claim.

### Pair 00

Game 0 is still a candidate White win. Opponent 25 ...Qc7 and candidate 26 Bd5 have stable major losses of about 0.506 and 0.492. This supports a linked missed response: the candidate did not preserve the chance that the opponent had just given. Opponent 31 ...Qb6, candidate 36 Rac1 and opponent 39 ...Bd7 also have stable major losses, with legal continuations retained. Candidate 32 b3, 34 Be3, 40 Kf2, 42 Rhd1 and the later 106 Rxe6 remain selected contrary-success evidence. The actual game ends with h-pawn promotion mate.

Do not confirm the earlier 23 ...a5 or 24 Qb3 comparison: the former has depth instability and the latter a final expected-score bound conflict. This also limits an early-opportunity narrative.

Game 1 is still a candidate Black loss. Candidate 23 ...a5 remains unstable, and 29 ...Qc8 retains a bound conflict despite a point loss near 0.252. Candidate 31 ...Bc4 is stable but secondary under the finite already-lost marker: the alternative ...Rd8 has E about 0.081. It must not become the “first confirmed cause” of the loss by ignoring the unresolved earlier decisions.

### Pair 01

Game 0 is still a candidate White loss. Candidate 24 b4 is now a stable significant event; 26 h3, 28 exf6 and 30 g3 have stable major losses against Ne2. Opponent 29 ...Rxf6 has a stable major loss against ...Bxf6, followed by the candidate's 30 g3 error. The proposed recovery involving opponent 27 ...f6 remains unresolved because its alternative has instability and a bound conflict. Opponent 43 ...Qxc2 also remains unstable, so renewed-chance attribution after this move remains qualified. In the later minor-piece ending, candidate 58 Kg3 is a stable serious event, and 64 g5 is a stable major event against a4. The smaller 62 Bc3 comparison does not retain significant support.

Game 1 is still a candidate Black win after 258 plies. Opponent 28 Na4, candidate 31 ...f5, and candidate 33 ...Qc7 retain stable large comparison losses; opponent 32 a5 remains unstable. The 20M PV after 32 a5 proposes ...Qd8, distinct from the already compared ...Qc6. This is a follow-up dependency before making a complete response claim at 33.

Candidate 67 ...Bf8, 75 ...Rb8, 79 ...Qa4 and 97 ...Bxa3 now have stable major losses. Opponent 68 Rb2, 72 Qd2 and 82 Rc1 also have stable consequential losses. The sequence is therefore a series of linked exchanges of chances, not continuous failure by one player. Candidate 81 ...Ba7 and opponent 76 Rb1 and 80 Qd2 remain unresolved. The previous large estimate at 93 ...Rc5 does **not** survive: its stable 20M loss is only 0.0185. Likewise 131 ...Rc2 and 141 ...Ra3 are now stable subthreshold comparisons. These withdrawn event labels must not survive in later counts.

In the rook-and-bishop ending, candidate 133 ...Rc4 and 137 ...Rc3 have stable major losses against ...g5. Opponent 132 Rc8, 136 Rc8 and 138 Rc6 also have stable major losses. This supports the linked chance-exchange account. Candidate 157 ...Bc5 has a stable major loss against ...Kg8. Candidate 183 ...Kxf6 has only a stable serious loss of 0.111 against ...Bb6, smaller than the 5M estimate. The candidate still eventually converts. The proposed 161 ...Be3 and 185 ...Bf4 comparisons have negative point losses plus bound/depth issues; neither is an error claim. New ...Kh7 alternatives at 67 and 69 need singleton support before the preceding PV can support those response claims.

### Pair 02

Game 0 is still a candidate White win. Opponent 17 ...O-O has stable major loss, but candidate 18 Nxf6+ and opponent 19 ...Kh8 remain unresolved. The played 20M PV at 17 suggests Nf3 at 18, which differs from the existing a3 comparison; a singleton is required. Candidate 22 Bc4 and 30 Ne3 have stable major losses, linked to opponent errors at 27 ...Nd5 and 29 ...exd5. Candidate 24 Qc3 remains unresolved because of a bound conflict.

The initially largest candidate error, 32 f4, is still **unresolved** despite a point loss near 0.767 and a large sensitivity range. The played report has a bound conflict and instability. Opponent 31 ...Ra6, opponent 33 ...Qd6, and candidate 34 g3 also remain unresolved. The PV after 32 f4 proposes ...Rb6 at 33, which has not yet received a matched singleton. It must not be treated as proved punishment. Opponent 35 ...f5 has a stable major loss against ...Rb6; candidate 36 Nf3 and subsequent attacking controls support contrary success in the actual win.

Game 1 is still a candidate Black loss. Opponent 18 Nxf6+ has a stable major loss. Candidate's 19 ...Kf8 and advancing queenside-pawn controls preserve the strong finite values. Candidate 31 ...f5 then has a stable major loss against ...f6 (point E 0 versus 1). Opponent 32 Bxb5 gives back a stable major chance against Qh5; candidate 33 ...Qa5 has a stable major loss against ...Nxa2. This is a supported linked mutual-error episode. Candidate 35 ...cxb5 remains unstable and is not confirmed. The later opponent 48 Qh5 versus Qh6+ and candidate 49 ...Qc7 versus ...Ra7 comparisons each have stable loss 0.5. The latter is a supported missed defensive opportunity, not merely an adjacent-score inference.

### Pair 03

Game 0 is still a candidate White loss. Candidate 20 a3 and 22 Re1 have stable losses against Bb2, while opponent 21 ...Be7 has a stable major loss against ...Bxd2. Candidate 26 Ra2 remains unresolved. Opponent 39 ...Qd7 is still unstable, so the beginning of the ensuing favorable episode remains qualified. Candidate 44 h5 is now a stable major event against Nb3. The smaller 46 a4 comparison is unresolved and negative at the selected point; do not retain its old error label.

Opponent 51 ...g5 and 53 ...hxg6 have stable major losses. Candidate 52 hxg6 remains unstable. Candidate 60 Ng4 and opponent 61 ...Bxb5 each have stable major losses; this supports their linked tactical episode. Candidate 78 fxg4 and 80 Rd2 now have stable major losses against Kg2. The latter retains finite range 0.471–0.496 and is no longer rejected for instability. The actual ending remains an opponent promotion mate. Candidate 82 Kh1 is a zero-point-loss comparison with a cp-bound conflict and stays unresolved, rather than adding a late cause.

Game 1 is still a candidate Black win. Candidate 23 ...a6 has a stable major loss against ...dxc4, but opponent 24 b5 remains unstable. Opponent 44 Qa2 has a stable major loss against Qxc2; candidate 45 ...Qe4 and the following attacking controls retain strong finite values. Opponent 46 Rfc1 is secondary under the already-lost marker. The actual attack, survival of checks and passed-pawn conversion remain contrary success. The early uncertainty prevents a flawless-play claim.

### Pair 04

Game 0 is still a candidate White win. Opponent 17 ...g5, candidate 18 h3 and opponent 19 ...Kh8 all have stable major losses in their matched comparisons. Candidate 20 a3 and subsequent attacking controls retain high finite values. Thus the candidate's early missed chance and later successful conversion can both be retained. The game ends with 66 Rg8 mate.

Game 1 is still an original AutoClaim threefold draw. Candidate 21 ...Bd7 now has a stable major loss against ...b5. Candidate 23 ...Qe7 remains unresolved because of an alternative expected-score bound conflict. Opponent controls 24 h4, 26 g3 and 34 Qxf4 retain E 1 in the finite reference. Candidate controls later in the game remain E 0 or secondary in the fixed reference. These facts do **not** authorize replacing the native draw, declaring failed conversion from the draw itself, or pooling this as an unrestricted conversion observation. Original automatic claims limit what the game outcome can show.

## Legal explanatory continuations

These are legal finite reference lines, not forced proofs. Complete selected lines are in the JSON, including plausible alternative defenses.

- Pair 02 game 1 ply 31: ...f5 Qh5 Qe8 Qh6+ Kg8 c3; comparison ...f6 Nxc6 Nxc6 Bxb5 Qa5 a4. This shows the concrete king-pressure and knight-pressure alternatives at the actual root.
- Pair 02 game 1 ply 49: ...Ra7 Qh6+ Ke8 Qh5+ is the short defensive repetition line. The actual ...Qc7 line permits Ne2, ...Qe7, Nxf4. Native outcome authority and claim semantics remain separate from this finite line.
- Pair 03 game 0 ply 80: Rd2 Rf4 Qc3 Rcf8 Kg2 Bc7, compared with Kg2 Bc7 Qc3 Rf4 Qd3 Rcf8. The different order of king defense and rook placement is a board-level observation, not a code explanation.
- Pair 01 game 1 ply 133: ...g5 hxg6+ Kxg6 Rd8 h5 Re8, compared with ...Rc4 Rd8 Rc2 Kg1 d3+ Kh1. The matched comparison supports the decision difference; it does not establish every later move as forced.
- Pair 04 game 0 ply 18: h3 Nb4 Qe2 dxc4 h4 Nd3+, compared with g4 Kg7 a3 Rb8 Ne2 e5. Both alternatives and their replies were replayed legally.

## New alternatives and remaining dependencies

Six new singleton alternatives are requested in `followup-selection-00-04.json`. Each root already has a unique played 20M receipt. The preparer must reuse it and search only the new alternative. No discovery search or duplicate existing move is requested.

| Pair | Game | Ply | Played | New alternative | Supporting prior root |
| --- | ---: | ---: | --- | --- | ---: |
| 01 | 1 | 33 | ...Qc7 | ...Qd8 | 32 |
| 01 | 1 | 67 | ...Bf8 | ...Kh7 | 66 |
| 01 | 1 | 69 | ...Be7 | ...Kh7 | 68 |
| 02 | 0 | 18 | Nxf6+ | Nf3 | 17 |
| 02 | 0 | 21 | ...a5 | ...b5 | 20 |
| 02 | 0 | 33 | ...Qd6 | ...Rb6 | 32 |

These are distinct legal replies from a preceding played 20M PV at the next actual full-history root. The adjacent finite value change only identifies the dependency; it does not certify the new move. Six added singletons cannot clear all 44 unresolved selected roots. Persistent uncertainty is retained without repeated searches to obtain a preferred label.

The two uncertain audit roots from the initial review remain unresolved and unselected here: pair 01 game 0 ply 35 and pair 04 game 1 ply 31. Neither was a stable audit expansion trigger. Unselected earlier decisions keep their original evidence; this review does not silently upgrade them. No first-error claim, full favorable-episode census, independent-error rate, final weakness ranking, or engineering priority is made.
