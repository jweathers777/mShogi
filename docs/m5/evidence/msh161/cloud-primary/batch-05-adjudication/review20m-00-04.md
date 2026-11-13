# MSH-161 (Competitive Gameplay Diagnosis): Batch 05, pairs 00–04, 20M review

This independent review covers all 219 selected roots and 351 queries. It uses the frozen method, original native histories, the initial gameplay review and cached completed 20M evidence. All 351 selected PV instances were replayed legally. No engine, retrieval, cloud job, production change or earlier engineering explanation was used. Native results remain two candidate wins, five losses, two automatic threefold draws and one dead-position draw.

## Normalized selected-root ledger

| Status | Candidate | Opponent | Total |
| --- | ---: | ---: | ---: |
| Supported finite event | 29 | 21 | 50 |
| Claim-policy-limited root | 1 | 0 | 1 |
| Supported finite subthreshold | 28 | 10 | 38 |
| Same-choice control | 69 | 10 | 79 |
| Unresolved | 29 | 22 | 51 |

No supported finite event is tagged secondary. The terminal claimed root at pair 03 game 0 ply 84 is excluded from these 50 events; its finite comparison remains separately recorded. Eleven other selected roots have finite secondary tags. This separates low-value context from comparison threshold support. Counts refer to selected roots, not all-game error rates or independent episodes. Several roots belong to one exchange of chances.

`dispositions20m-00-04.json` has one normalized row per root, exact source and query hashes, candidate identity, played and alternative moves, conservative margin, severity, secondary tag, threshold sensitivity and legal continuations. Conservative margin is alternative minimum minus played maximum across the last two distinct depths. It is a finite sensitivity measure, not a statistical interval. Stable finite evidence does not establish exactness. Flags remain visible; no disagreement is cleared merely by increasing nodes.

At thresholds 0.04 / 0.05 / 0.06, finite comparison support counts are 56 / 51 / 49. These include the one terminal claim-policy-limited root; they are not the root event counts. Five otherwise stable comparisons straddle the primary 0.05 threshold: pair 00 game 0 ply 19; pair 00 game 1 ply 21; pair 01 game 0 ply 65; pair 03 game 0 ply 66; pair 04 game 1 ply 37. Forty-six further roots retain report, discovery or comparison uncertainty. Earlier unselected and unresolved evidence remains unchanged, preventing unqualified first-error or largest-error claims.

## Five discovery results

- Pair 00 game 0 ply 28: unrestricted discovery returns Ba3, distinct from played Rd1. The point values differ in direction from a simple best-move label: played singleton E is 0.435, unrestricted Ba3 E is 0.4035. Ba3 needs a singleton at equal work before comparison. Do not infer an error from the label alone.
- Pair 01 game 0 ply 56: discovery returns played g6. The two finite estimates agree closely and provide a same-choice control.
- Pair 01 game 0 ply 88: discovery returns played Rg1, with E 1 in both queries. This is a same-choice control, not proof of unique optimality.
- Pair 03 game 0 ply 23: discovery returns played ...Rad8, but the played report has a bound conflict and the unrestricted report is unstable. The finite disagreement remains unresolved.
- Pair 04 game 0 ply 69: discovery returns played ...Kd5. Both finite estimates are close to zero. This is a same-choice control in low-value context, not an error or recovery claim.

## Game-level findings

Plies are zero-based. “Supported” below means stable finite support under the frozen method. It does not identify a technical mechanism or establish that every later move in an illustrative line is forced.

### Pair 00

Game 0 is a candidate White win. The initial opponent 19 ...e5 comparison now straddles significance, so the earlier serious-error impression is not confirmed. Candidate 24 Qd3 against Qb3 has supported major loss, conservative margin 0.400. Candidate 26 Rb1 against Nxc8 has supported serious loss, margin 0.132. The legal Qb3 line starts Qb3 e4 Nxe4 Rb8 a4; the played Qd3 line starts Qd3 Rd8 Bg5 Rd7. Discovery at 28 remains pending its new Ba3 singleton.

Opponent 29 ...Nd4 has supported major loss, margin 0.574. Candidate's immediate captures, rook activity and later conversion controls retain contrary-success evidence. The actual candidate eventually promotes and mates. Earlier uncertainty prevents a complete causal account from being reduced to this one opponent decision.

Game 1 is an original automatic threefold draw. Candidate 21 ...Rd7 straddles significance. Opponent 26 Rad1 has supported significant loss against Bd2, margin 0.094, but the preceding played PV proposes Rfd1 as another reply; that new singleton is required for the linked claim. The later exchanges and repetition controls remain separate from any mistake claim. Native draw authority and AutoClaim conversion limits are unchanged.

### Pair 01

Game 0 is a candidate White win. Candidate 30 Bxh7, 44 g4, 52 g5, 58 h3, 60 Rbd2 and 66 Qg4 have supported major losses. Opponent 53 ...Bc4, 55 ...Rh5, 59 ...a5, 73 ...Bc8, 81 ...Qe6+ and 87 ...Qxc3 also have supported major losses. These are linked attack-and-defense episodes with mistakes by both players, not independent causes or unbroken candidate failure.

Opponent 39 ...Be8, 43 ...Rh6 and 51 ...Rg7 remain unresolved; candidate 82 Kg2 and opponent 83 ...Qc4 also remain unresolved. These gaps limit recovery and opportunity narratives. Discovery controls at candidate 56 g6 and 88 Rg1 support concrete successful replies without proving unique best moves. The actual game ultimately converts to mate. Two new linked alternatives remain: Kh2 at candidate 60 and ...Bf7 at opponent 65. The existing supported comparison at 60 remains valid while the new comparison is pending.

Game 1 is a native dead-position draw. Opponent 34 Rc1 now has supported major loss and candidate 35 ...e5 supported serious loss. Opponent 36 f4 and 44 h4 remain unresolved. The preceding played PV proposes Kf1 at 36, which needs a new singleton. Candidate 49 ...Rxe1+ no longer supports a consequential error: its stable margin is only 0.0245. Preserve this withdrawal rather than carrying the larger 5M label forward. Candidate 51 ...Re8+ and opponent 52 Kf1 remain unresolved.

In the bishop ending, candidate 81 ...Be5 remains unstable/conflicting. Opponent 84 Be3 and candidate 85 ...g5 have supported major losses. The ...Ke5 comparison at 85 starts ...Ke5 Bb6 Kd5 f4 Kc4. Opponent 98 Kg3 also has supported major loss; candidate 99 ...Bc3 retains an alternative cp-bound conflict and remains unresolved. Candidate 121 ...Bb6 remains unstable. A distinct Bc7 reply at 84 needs singleton support before the preceding PV can establish that alternative. The dead-position draw does not itself prove failed conversion. Intermediate matched decisions provide the evidence; exact ending claims remain separate.

### Pair 02

Both games are candidate losses. In game 0, candidate 24 Nfg5 remains unstable/conflicting. Candidate 28 Qe2 has supported major loss, margin 0.375, and opponent 29 ...h6 has supported major loss, margin 0.457. Opponent 33 ...Ba6 now has supported significant loss at the primary threshold. Candidate 36 Nd4 remains unresolved despite a large conservative difference, so its earlier apparent major event must not be counted as confirmed. Later low-value decisions remain secondary unless renewed chances are supported.

In game 1, candidate 25 ...Bxd4 against ...Nxd4 now has supported major loss, margin 0.498. The earlier cp-bound conflict no longer prevents finite support at this work level. The actual-root bishop line starts ...Bxd4 Rad1 e5 Bh6 Qh4 Bxf8. The knight line starts ...Nxd4 Qxb7 a5 Rad1 a4. These legally replayed alternatives support a concrete recapture and coordination difference. They do not prove a search defect. The opponent's later activation, material gains and promotion remain the native continuation.

### Pair 03

Game 0 is an original automatic threefold draw. Opponent 19 ...Bg4 remains unresolved, while candidate 20 Nxc5 has supported serious loss. Opponent 21 ...Na5 remains a linked uncertainty with a new ...Qc8 alternative proposed by the preceding played PV. Discovery at 23 ...Rad8 remains unresolved. Candidate 24 Qc2 and opponent 25 ...Qb6 have supported major losses. Candidate 28 Bxb7 and opponent 29 ...Rd4 remain unresolved.

Candidate 36 f3 is still unresolved despite a margin near 0.491; the large finite difference is not sufficient to override failed support conditions. Later candidate 68 Kf2 is supported major and 70 Rb1 supported serious. Opponent 71 ...Raa3 remains unresolved despite a near-half-point comparison. Candidate 84 Rd2 has a supported major comparison against Rxc3 in the finite reference, but its root disposition is claim-policy-limited and it is excluded from primary error causes. The legal line Rd2 Rxc7 is not an observed continuation: the actual native game ends through AutoClaim. Do not replace that draw with the reference line or infer conversion weakness from the result itself.

Game 1 is a candidate Black loss. Opponent 20 Bf4 has supported major loss, and candidate 23 ...Bxc4 against ...Bh3 has supported major loss, margin 0.475. This supports a concrete missed response after an opponent concession, but earlier uncertainty remains. The actual Qa4 tactic and later opponent conversion are preserved. The result does not justify fresh late error counts in already-low-value positions.

### Pair 04

Both games are candidate losses. In game 0, candidate 20 Rb1 is supported serious and 22 a4 supported significant. Candidate 46 Nf5 and opponent 47 ...Bc7 both have supported major losses. Opponent 65 ...Nd2+ has supported major loss; candidate 66 Ke3 and 68 Kf4 remain successful finite controls. Discovery at 69 ...Kd5 is a same-choice low-value control. Candidate 70 Kf3 against g5 now has supported major loss, unlike the earlier unstable alternative evidence.

Opponent 81 ...h5 remains unresolved, so the origin of the later favorable episode stays qualified. Candidate 84 h6 has supported major loss against Ng3. Candidate 104 Be3, opponent 105 ...Kf7 and candidate 106 Bxc5 each have supported major losses. They belong to one linked defensive episode and must not be counted as three independent strategic causes. The actual opponent later promotes and mates.

In game 1, opponent 26 Ne5 and candidate 27 ...Bxe5 have supported significant losses. Opponent 32 Nb5 remains unresolved despite a large apparent difference. Candidate's subsequent high-valued controls remain contrary success, with renewed-chance attribution limited by that uncertainty. New alternatives Nxa7 at opponent 34 and Rfd1 at opponent 40 need bounded singleton checks for linked replies; neither has an existing forced 20M receipt.

Candidate 87 ...g5, opponent 90 Rxg5+, candidate 97 ...Kh8, 99 ...Qf8, opponent 102 Bd2 and candidate 105 ...c3 have supported major losses. The actual late attack therefore contains supported errors by both sides. The ...Rf8 alternative at 105 starts ...Rf8 Rf1 Rxf1 Kxf1 Qf8+; the played ...c3 line starts ...c3 Rf1 Qa7 Bg5. New Bb4 at opponent 98 and ...Qe8 at candidate 103 need singleton comparison before their particular linked PV claims can be used. Existing supported events do not depend on assuming that those new alternatives are already confirmed.

## Eleven bounded singleton obligations

`followup-selection-00-04.json` contains one discovery alternative and ten linked alternatives. Each is legal, distinct from every already forced 20M move at its root, and reuses a unique completed played receipt. No unstable existing query is repeated.

| Pair/game/ply | Played | New alternative | Origin |
| --- | --- | --- | --- |
| 00 / 0 / 28 | Rd1 | Ba3 | Unrestricted discovery |
| 00 / 1 / 26 | Rad1 | Rfd1 | Prior played PV at 25 |
| 01 / 0 / 60 | Rbd2 | Kh2 | Prior played PV at 59 |
| 01 / 0 / 65 | ...a4 | ...Bf7 | Prior played PV at 64 |
| 01 / 1 / 36 | f4 | Kf1 | Prior played PV at 35 |
| 01 / 1 / 84 | Be3 | Bc7 | Prior played PV at 83 |
| 03 / 0 / 21 | ...Na5 | ...Qc8 | Prior played PV at 20 |
| 04 / 1 / 34 | Bb7 | Nxa7 | Prior played PV at 33 |
| 04 / 1 / 40 | Rad1 | Rfd1 | Prior played PV at 39 |
| 04 / 1 / 98 | Bc3 | Bb4 | Prior played PV at 97 |
| 04 / 1 / 103 | ...Qc5+ | ...Qe8 | Prior played PV at 102 |

Adjacent values and PV inclusion identify a dependency; they do not establish the new move's value. This is one bounded follow-up set, not a recursive PV investigation. Persistent report uncertainty will remain unresolved rather than triggering repeated searches for a preferred label.

The selection and pre-follow-up ledger are frozen with source hashes. Eleven query obligations remain before linked-comparison closure. Broader report uncertainty, earlier unselected decisions, secondary contexts and original claim limits remain explicit. No first/largest claim, final weakness ranking, technical mechanism or engineering priority is proposed.
