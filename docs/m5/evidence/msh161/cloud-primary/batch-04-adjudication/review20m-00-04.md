# MSH-161 (Competitive Gameplay Diagnosis): Batch 04, pairs 00–04, 20M review

This independent review covers 195 selected roots and 333 queries. It uses the frozen method, original native histories, the initial gameplay review and cached 20M artifacts. All 333 selected PV instances were replayed legally. No engine, retrieval, cloud job or production change was run. Native results remain four candidate wins, four losses, one dead-position draw and one original AutoClaim threefold draw.

## Selected-root dispositions

| Status | Candidate | Opponent | Total |
| --- | ---: | ---: | ---: |
| Supported finite event | 35 | 32 | 67 |
| Supported finite subthreshold | 21 | 14 | 35 |
| Same-choice control | 44 | 3 | 47 |
| Unresolved | 25 | 21 | 46 |

None of the 67 supported finite events has a secondary tag. Nine other selected roots have a finite already-low-value secondary tag. Comparison support and secondary context are separate fields. These are selected-root counts, not all-game error rates or independent causal episodes.

The normalized ledger has one row per root under `decisions`. It records candidate identity, played move, compared alternative, status, conservative margin, severity, secondary tag, exact query hashes, legal continuations, earlier uncertainty and nested comparisons. Margin is minimum alternative E minus maximum played E across the last two distinct depths. Stable finite support is not exactness. Ranges are not statistical confidence intervals. All stale-report and conflict flags remain available.

At thresholds 0.04 / 0.05 / 0.06, supported counts are 69 / 67 / 64. At 0.05, two otherwise stable comparisons straddle the threshold: pair 00 game 1 ply 24 and pair 01 game 0 ply 72. Forty-four further roots have report or same-choice uncertainty. Same-choice controls do not prove unique best moves. Earlier unselected and unresolved decisions prevent unqualified first-error and largest-error claims.

## Game-level findings

Plies are zero-based. “Supported” means stable finite evidence under the frozen method. Opponent errors and candidate responses remain linked rather than being counted as independent causes.

### Pair 00

Game 0 is a candidate White win. Candidate 24 f4 against Bg5 has supported major loss, with conservative margin 0.414. Opponent 27 ...Nxe2+ against ...Nh5 also has supported major loss (0.435). Their linked sequence supports the early error-and-reply account, without clearing earlier uncertainty.

A consequential late event now has strong support: opponent 63 ...gxh4 against ...Ra6 has conservative margin 0.494. The 500k screen had flagged this comparison, but matched 5M gave loss only 0.0005. The disputed comparison was preserved and selected; this is not a missed stable audit control or a reason to rewrite the screening record. Its legal line ...gxh4 Nf4 Rc7 Bxb7 Rcxb7 Rxd7 supports the concrete exchange sequence. Candidate success controls include the ensuing captures and eventual promotion conversion. The actual mate remains contrary success, not proof of flawless play.

Game 1 is a candidate Black loss. Opponent 18 Na4 is supported significant; 20 c4 stays unresolved. Candidate 25 ...Nd4 is supported major and opponent 32 g4 is supported serious. Candidate 39 ...Nc6 and opponent 40 Nxb4 both have supported major losses. Candidate 41 ...cxb4 remains unresolved despite a conservative finite margin near 0.495, because its reports fail the support rules. Do not turn the full three-ply sequence into a confirmed chain or call the final recapture the established cause of the mate.

### Pair 01

Game 0 is a candidate White win. Candidate 20 O-O is supported significant; 28 Rfd1 and 36 h4 are supported major. Candidate 32 a4 and 34 Ra3 remain unresolved. Opponent 31 ...Ra6 also remains unresolved, but 33 ...Kh8, 43 ...Qe7 and 45 ...Qd6 have supported major losses. The linked chance-exchange account therefore has partial support, not uniform confirmation.

Candidate 50 Na2 and 60 Rc4 have supported major losses; opponent 55 ...b5 and 61 ...Bb5 have supported major losses. Opponent 63 ...Bd7 now has supported serious loss with margin 0.175, so its former low-value interpretation must not become a permanent secondary label. Candidate 64 gxf4 is supported significant, 66 Nc3 is unresolved, and 72 Qf2 straddles the primary significance threshold. The discovery at opponent 67 ...c4 returns the actual move but retains disagreement and report uncertainty. No new alternative follows from that discovery. Candidate success controls and the actual passed-pawn conversion remain valid contrary evidence.

Game 1 is a candidate Black loss. Opponent 22 b4 and candidate 25 ...Na4 have supported major losses. Candidate 31 ...Ng4 and 43 ...Re2 are also supported major; 39 ...Bf6 is supported serious. Opponent 28 b5 and 40 f3 remain unresolved. Opponent 42 c5 has supported major loss, so its response must remain part of the Re2 episode.

Opponent 56 g3, candidate 57 ...Bh3, candidate 63 ...Bf2, 75 ...Qh3 and 87 ...Bf5 have supported major losses. Opponent 58 Re1 has supported significant loss against Rfb1 with margin 0.0505; it was a disputed screen/5M comparison, not an audit omission. It falls below the 0.06 sensitivity threshold. The actual bishop capture after ...Bf5 and eventual opponent promotion remain the native continuation. This supports concrete missed defenses and repeated releases of finite advantage, not a code mechanism or final study priority.

### Pair 02

Game 0 is a dead-position draw. Opponent 31 ...Rb6 has supported significant loss. Opponent 73 ...Rc6 and candidate 80 Rxc7 have supported major losses, with margins near 0.5. Candidate 74 Nd4, 76 Nxb5 and 78 Rc2 are contrary success controls between them. The legal Kg2 alternative at 80 starts Kg2 f5 Kf3 Rf6 gxf5+ Kxf5 Ke3. The played capture line starts Rxc7 Re4 Kg3 Rxb4. These support a specific missed continuation in the actual ending. The draw alone is not the evidence. Native dead-position termination is unchanged.

Game 1 is a candidate Black loss. Opponent 20 Rac1 has supported major loss, while candidate 21 ...f6 remains unresolved. Candidate 27 ...b5 is supported significant. Opponent 34 f3 has supported major loss; the candidate's subsequent tactical and favorable controls remain contrary successes. Candidate 69 ...Nxf5+ against ...Kh8 and 143 ...Rb3 against ...Nxc6 have supported major losses. The latter alternative has finite drawing value, so the late mistake is not classified as already-lost secondary merely because the actual game later loses.

The discovery at opponent 26 Bb3 returns Bb3, but finite estimates disagree and the played report is unstable. The prior played PV proposes Nh4 at this exact root. A singleton Nh4 comparison is requested to assess that bounded linked question; the existing played receipt must be reused.

### Pair 03

Game 0 is a candidate White win. Opponent 21 ...b5 has supported major loss against ...b6. Candidate's subsequent development, exchanges and rook-ending controls preserve the contrary-success account. The actual promotion mate does not justify inventing a candidate weakness in this game.

Game 1 is a candidate Black loss. Candidate 17 ...Nd5 is supported serious, opponent 20 Nc3 is supported serious, and candidate 21 ...Rg8 is supported major. Discovery at 19 ...Qa5+ returns the same move, but singleton and unrestricted E differ by about 0.0985. That decision remains unresolved; same best move is not enough to erase the finite disagreement.

Opponent 46 Bb7 remains unresolved; candidate 47 ...Nd6 has supported serious loss and opponent 48 Rxa7 remains unresolved. Candidate 53 ...f6 has supported major loss against ...Ra7. The alleged recovery through the preceding opponent moves stays qualified. A distinct Ba4 reply at 46, proposed in the preceding played PV, needs a singleton comparison before the linked defense can be used. No repeated search of the unstable existing choices is requested.

### Pair 04

Game 0 is a candidate White win. Candidate 18 Qb3 is supported significant, 24 Bd2 is supported major, and opponent 25 ...Rac8 is supported major. Opponent 41 ...Nc3 is supported significant; 45 ...Bxb4 is supported major. Candidate response, tactical exchange and conversion controls retain contrary-success evidence. Earlier smaller comparisons are retained individually in the ledger rather than being promoted into a broad opening diagnosis.

Game 1 is an original automatic threefold draw. Opponent 22 Qd1 is supported significant, candidate 23 ...Bd6 is supported serious and 31 ...a6 is supported significant. Opponent 36 a4 remains unresolved; 38 Qb3 has supported major loss. Candidate 41 ...b5, opponent 44 Nh4, candidate 53 ...Qh3, opponent 54 Ng2, candidate 55 ...g5, opponent 56 f3, candidate 57 ...Kh7 and opponent 58 g4 have supported major losses in their existing comparisons. Candidate 59 ...f6 is supported serious and 63 ...Rxd3 is supported major. These are linked exchanges of chances in the same game, not independent causes.

Distinct replies from preceding played PVs remain untested at 44 Rc5, 55 ...Qg4 and 59 ...Nf8. They receive bounded singleton follow-up before those particular linked alternatives can support explanatory claims. Existing supported alternatives remain valid while those new comparisons are pending.

The native automatic threefold result remains unchanged. Decision comparisons before the claim can have finite support, while conversion observation is still limited by AutoClaim. Do not replace the result with a reference score, infer weakness from the draw itself, or start policy work.

## Five bounded follow-up obligations

All three unrestricted discoveries returned the actual move; none discovered a new alternative. Five distinct next-root replies from preceding played PVs require singleton comparisons for the linked claims. They are legal, have no existing forced 20M receipt, and reuse each root's completed played evidence.

| Pair/game/ply | Played | New alternative | Supporting root |
| --- | --- | --- | --- |
| 02 / 1 / 26 | Bb3 | Nh4 | 25 |
| 03 / 1 / 46 | Bb7 | Ba4 | 45 |
| 04 / 1 / 44 | Nh4 | Rc5 | 43 |
| 04 / 1 / 55 | ...g5 | ...Qg4 | 54 |
| 04 / 1 / 59 | ...f6 | ...Nf8 | 58 |

Adjacent score changes only identify these dependencies; they do not establish the new move's value. This is one bounded set, not a recursive PV investigation. Existing unstable queries are not repeated. Further unresolved evidence will remain unresolved rather than being searched until a preferred label appears.

`followup-selection-00-04.json` is frozen and compatible with the follow-up preparer. `dispositions20m-00-04.json` is the pre-follow-up ledger with exact source and query hashes; preserve it before later overlays. Five query obligations remain before linked-comparison closure. Broader report uncertainty is explicitly retained. No first/largest claim, final weakness ranking, engineering cause or production recommendation is made.
