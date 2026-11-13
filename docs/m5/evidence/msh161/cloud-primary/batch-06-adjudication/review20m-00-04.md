# MSH-161 (Competitive Gameplay Diagnosis): Batch 06, pairs 00–04, 20M review

This independent review covers all 301 selected roots and 500 completed queries. It uses the frozen method, own initial gameplay review, original native histories and cached evidence. All 500 selected PV instances were replayed legally. No engine, retrieval, cloud job, repeated query or earlier engineering explanation was used. Native results remain two candidate wins, three losses and five original automatic threefold draws.

## Normalized dispositions

| Root status | Candidate | Opponent | Total |
| --- | ---: | ---: | ---: |
| Supported finite event | 35 | 33 | 68 |
| Supported finite subthreshold | 55 | 11 | 66 |
| Same-choice control | 81 | 2 | 83 |
| Unresolved | 48 | 31 | 79 |
| Claim-policy-limited terminal root | 5 | 0 | 5 |

One finite event is secondary: candidate h4 at pair 04 game 0 ply 56. It is not a fresh primary cause. Thus there are 67 primary finite event roots, 34 candidate and 33 opponent. Twenty-six roots across statuses carry secondary tags. Counts describe selected roots, not independent episodes or all-game error rates.

The ledger records exact source/query hashes, candidate identity, played/alternative moves, conservative margin, severity, separate secondary tags, threshold sensitivity, legal lines and retained earlier uncertainty. Margin is alternative minimum minus played maximum across the last two distinct depths. These observed ranges are not statistical intervals. Stable finite support is not exactness. Bound conflicts and unstable estimates remain unresolved regardless of apparent magnitude.

Finite comparison support counts at 0.04 / 0.05 / 0.06 are 68 / 68 / 64. At 0.05, 79 roots retain report or discovery uncertainty; no otherwise stable range straddles that threshold. Terminal claim controls and secondary comparisons remain separate from primary event counts. Same-choice controls do not establish unique best play. Earlier unselected and unresolved evidence prevents unqualified first-error or largest-error claims.

## Discovery and terminal treatment

All three discoveries were inspected. Pair 02 game 0 ply 68 returns played Rca1, but both finite reports are unstable and the unrestricted report also has a bound conflict. Pair 04 game 1 ply 29 returns played ...exf4, but its unrestricted estimate is unstable and differs from the singleton. Both roots remain unresolved.

Pair 02 game 0 ply 69 discovers ...Kg8, distinct from played ...Rd7. Its unrestricted point E is 0.457 versus the unstable played singleton's 0.447. This requires a new singleton before comparison; the small point difference and best-move label alone prove nothing.

All five native terminal claim roots are explicitly `claim_policy_limited`: pair 00 game 1 ply 131; pair 02 game 0 ply 80; pair 02 game 1 ply 75; pair 03 game 0 ply 96; pair 03 game 1 ply 109. Four have same-choice finite controls and the last has a subthreshold comparison. Their finite evidence is retained separately. Any reference continuation beyond the native automatic claim is not observed gameplay and cannot establish a primary terminal error. The draw itself is not weakness, and no policy work is proposed.

## Game-level findings

Plies are zero-based. “Supported” means stable finite support under the frozen rules, not a forced result or technical mechanism.

### Pair 00

Game 0 is a candidate White win. Candidate 28 h3 and 30 f5, and opponent 29 ...Be6 and 33 ...Rd6 remain unresolved. The early sequence therefore cannot be described as a fully confirmed exchange of chances. Candidate 34 Bc5 has supported major loss, margin 0.344. Opponent 35 ...e6 has supported significant loss, and 41 ...Rd7 supported significant loss. Candidate 44 Rg1 is supported serious rather than major at the conservative margin. New Ne2, ...Rd7, Ne2 and ...b6 alternatives at plies 30, 35, 36 and 37 are bounded linked dependencies from already completed played PVs.

Opponent 53 ...Rc6 and 55 ...Bg2 have supported major losses. Candidate 64 Rf5 is supported major, while opponent 65 ...h6 remains unresolved. Opponent 73 ...Nh4 and candidate 74 Bxh4 both have supported major losses. The d4 comparison at 74 starts d4 Ng6 d5 Rxd5 Nc7+; the capture line starts Bxh4 gxh4 d4. These legally replayed alternatives support a concrete exchange and passed-pawn decision.

Candidate 84 Kd3 now has supported major loss, but opponent 85 ...Bg5, candidate 86 Nd4 and opponent 87 ...Be7 remain unresolved. Opponent 89 ...Kc7 has supported major loss; ...Bf6 is a new linked alternative needing a singleton. Candidate's subsequent activity, promotion and actual mate remain contrary success. Unresolved opponent moves limit any complete recovery account.

Game 1 is an automatic threefold draw. Candidate 17 ...Nh6 has supported serious loss. Candidate 51 ...Ne2 and opponent 52 f5 both have supported major losses. Candidate 101 ...Nxc4 and opponent 102 Ra7 also have supported major losses in the ending. This supports two linked mutual-error episodes, not a continuous candidate decline. The final ...Kd5 is a separate terminal claim control. The native draw and conversion limits remain unchanged.

### Pair 01

Game 0 is a candidate White win. Opponent 21 ...Kh8 is supported serious; 23 ...Rac8, candidate 24 Be3 and opponent 25 ...c6 are supported major. Candidate 22 h3 and 26 b3 remain unresolved. Opponent 29 ...b5 is supported major, and the candidate's immediate capture and activity controls remain contrary success. Candidate 38 Nd2 is supported major, while 40 a5 remains unresolved. Opponent 47 ...Nf6 and candidate 48 a6 remain unresolved. Opponent 51 ...d5, candidate 54 Nxc4 and opponent 55 ...Ra7 have supported major losses. The alternative b5 line at 54 starts b5 c3 Qxc3 Nd5; the played capture line permits ...Rxa6 and exchanges. The actual candidate converts the queenside passed-pawn sequence and mates. Nearby events are linked and must not become separate causal counts.

Game 1 is a candidate Black loss. Opponent 24 Be3 remains unresolved. Opponent 32 Qd2 is supported serious; candidate 37 ...e4 and 43 ...b5 are supported major. Opponent 44 b3 remains unresolved despite a near-half-point range. New Nd2 at opponent 28 and ...Qf5 at candidate 29 are needed for bounded linked responses.

In the late defensive episode, candidate 99 ...Be5 and opponent 102 Qf3 remain unresolved. The initial large comparison at opponent 106 Rb3 is now stable subthreshold against the existing Rb4 alternative, conservative margin 0.031. Do not retain its earlier major label. A distinct Rc5 alternative from the preceding played PV is still pending. Candidate 107 ...Ba1 has supported major loss; opponent 108 Ke6 and candidate 109 ...Be5 remain unresolved. ...Ra7 at 107 and Rb8 at 108 require singleton comparisons for the linked alternatives. The native opponent mate remains the outcome, not proof that every apparent defense was inferior.

### Pair 02

Both games are automatic threefold draws. In game 0, opponent 17 ...O-O remains unresolved despite a large finite margin. The prior played PV proposes ...dxe4, distinct from the existing ...cxd4 comparison, and needs a singleton. Candidate 20 Ndf3 and opponent 21 ...c4 have supported major losses. The candidate's attacking controls remain contrary success without proving a forced win.

Candidate 58 Rab1 remains unresolved despite a 0.5 margin. Candidate 62 Ra1 and opponent 65 ...Kh7 have supported major losses. ...Qg8 at opponent 67 is a new linked alternative. Discoveries at 68 and 69 retain the limits stated above; Rb8 at candidate 70 also needs a singleton before that linked PV can be used.

Opponent 71 ...Kxh6 is supported serious, smaller than the initial major estimate. Opponent 73 ...Nc7 and candidate 76 Ra8 are supported major. Candidate 78 R8a7 has supported major finite loss with margin 1.0 against Rxc8. The played line starts R8a7 Rf7 Qb1 Qd8; the alternative starts Rxc8 Qxc8 Re1. The extreme finite E endpoints are not game-theoretic proof. Opponent 79 ...Nb5 gives back supported major value. Terminal 80 Ra8 ends the native game through AutoClaim and is not counted as another cause.

In game 1, opponent 18 Qb3, candidate 25 ...b5, opponent 32 Ne5, candidate 41 ...Qf8, opponent 48 f5 and candidate 57 ...a5 have supported major losses. Qe6 at opponent 32 is a new linked alternative; the existing supported comparison remains valid while it is pending. The candidate's intervening tactical and active-piece controls remain contrary evidence. Terminal ...Ng4 remains claim-policy-limited rather than a failed-conversion label.

### Pair 03

Both games are automatic threefold draws. In game 0, opponent 23 ...Qd7 and candidate 24 Be3 are supported significant; opponent 25 ...Rc8 and candidate 26 a3 are supported serious. In the queen/rook ending, opponent 75 ...Kh6, candidate 76 Qh4+ and opponent 77 ...Kg6 now have supported major losses. Candidate 78 Rxg7+ remains unresolved because the reference support conditions still fail despite a half-point margin. The actual rook exchange and queen checks lead to terminal 96 Qe4+ and AutoClaim. Do not replace that draw with a reference attack or count the terminal check as a primary cause.

In game 1, opponent 24 f3 and candidate 25 ...Qe5 have supported major losses. Opponent 28 g5 remains unresolved. Candidate 31 ...Qxf6, opponent 32 Be3 and candidate 33 ...O-O have supported major losses. The ...Bxf6 alternative at 31 starts ...Bxf6 Bxg6+ hxg6 Qe1 Qxe1; the queen line starts ...Qxf6 Kh1 Qe5 Bb2. These support a concrete recapture comparison, not a code explanation. Candidate 41 ...Ba5 is supported major. Later low-value context is preserved. The final ...Kh8 is a terminal claim control, even though the finite reference favors the opponent strongly before termination.

### Pair 04

Both games are candidate losses. In game 0, opponent 19 ...dxc3 and candidate 20 Bxd7+ are supported significant. Candidate 32 Rb3 and opponent 33 ...a5 remain unresolved. Candidate 34 Rb5 is supported significant and opponent 35 ...Bd7 supported serious. Opponent 55 ...f6 is supported serious. Candidate 56 h4 has supported significant finite loss but the alternative's E remains under the low-value marker; retain it as secondary rather than a fresh primary cause. The material ending and eventual opponent promotion remain the native continuation.

In game 1, opponent 22 a5 remains unresolved. Candidate 25 ...O-O and 27 ...Be6 have supported significant losses. Candidate 37 ...Nd8, opponent 42 b6, candidate 43 ...Nc6 and opponent 44 Kh1 remain unresolved. Re1 at 44 is a new linked alternative, with no existing forced receipt.

Candidate 51 ...Kh8 and 53 ...Nd8 have supported major losses; opponent 52 Bh3 remains unresolved despite an apparent half-point difference. The ...Bxh3 alternative at 53 starts ...Bxh3 Rxe8 Raxe8 Nxd4 Bd7, compared with ...Nd8 Qe2 Bd7 Qxe8. Opponent 54 Ng5 has supported serious loss while retaining a high finite position; it does not establish that the opponent gave away the whole advantage. The actual opponent converts passed pawns and mates. These observations do not identify an implementation mechanism.

## Sixteen bounded singleton obligations

Each requested alternative is legal in the exact native history, differs from all existing forced 20M moves at that root, and reuses a unique played receipt. One is unrestricted discovery; fifteen are necessary linked replies from already completed played PVs.

| Pair/game/ply | Played | New alternative | Supporting root |
| --- | --- | --- | --- |
| 00 / 0 / 30 | f5 | Ne2 | 29 |
| 00 / 0 / 35 | ...e6 | ...Rd7 | 34 |
| 00 / 0 / 36 | fxe6 | Ne2 | 35 |
| 00 / 0 / 37 | ...fxe6 | ...b6 | 36 |
| 00 / 0 / 89 | ...Kc7 | ...Bf6 | 88 |
| 01 / 1 / 28 | Rac1 | Nd2 | 27 |
| 01 / 1 / 29 | ...b6 | ...Qf5 | 28 |
| 01 / 1 / 106 | Rb3 | Rc5 | 105 |
| 01 / 1 / 107 | ...Ba1 | ...Ra7 | 106 |
| 01 / 1 / 108 | Ke6 | Rb8 | 107 |
| 02 / 0 / 17 | ...O-O | ...dxe4 | 16 |
| 02 / 0 / 67 | ...Rc6 | ...Qg8 | 66 |
| 02 / 0 / 69 | ...Rd7 | ...Kg8 | Discovery |
| 02 / 0 / 70 | Qf5+ | Rb8 | 69 |
| 02 / 1 / 32 | Ne5 | Qe6 | 31 |
| 04 / 1 / 44 | Kh1 | Re1 | 43 |

A preceding PV or adjacent score change identifies the question, not its answer. This is one bounded set, not a recursive PV chain. Existing unstable choices are not searched again. Remaining ambiguity will stay unresolved rather than trigger repeated queries for a preferred label.

`followup-selection-00-04.json` and the pre-follow-up ledger are frozen with exact hashes. Sixteen query obligations remain before linked-comparison closure. Original claims, earlier uncertainty, secondary context and contrary successes remain explicit. No first/largest claim, final weakness ranking, engineering mechanism or production recommendation is made.
