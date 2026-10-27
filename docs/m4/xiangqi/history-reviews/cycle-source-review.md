# G-CYCLE independent source/fixture review (MSH-94)

2026-10-27. Research worker only; no product, Backlog or repository changes. No comparator history helper was inspected or used as authority. Proposed outcomes require separate independent review before implementation acceptance.

## Governing primary evidence

Read official cached WXF2018 English/Chinese PDFs identified by repository `source-manifest.json`: EN SHA256 `2b45534953bbfcca42dc2c52e06be48ff44ebe3bf7237248b12e39576c494cf3`, ZH `f61f5c82ddaf2abb45eb25c24d58f2b278d2c150439f391d413c32c4ce275b3e`. Critical original pages visually inspected using cached renders; OCR used only for locating text. No rulebook text has been added to the repository.

- **EN3.1C PDF14/printed6; ZH3.1C PDF10/printed9:** prohibited repetition is followed by arbiter requirement to vary, then refusal produces loss. Brief exact EN wording: "if he still refused to change his moves". The approved software third/fourth boundary already reconciles this with the draw provision; this research does not reopen it.
- **EN3.2B/C PDF15/printed7; ZH3.2B/C PDF10/printed9:** position repetition without unilateral prohibited responsibility is a draw; equal prohibited conduct is a draw. Exact EN count phrase: "repeated four times". There is no statement that choosing a different legal return path erases board occurrences.
- **EN19.8 PDF44/printed36; ZH19.8 PDF28/printed27:** continuous checking and return of the position define perpetual check. Exact EN continuity phrase: "for every move made". Exact short ZH phrase: `连续不停照将`. Excluding intervening quiet moves to cherry-pick separated checking subsequences is incompatible with this continuity.
- **EN19.10 PDF44/printed36; ZH19.10 PDF28/printed27:** pursuit is successive attacks on the same identified enemy piece, with each reply moving it away or directly resolving the attack. Victim identity is not an occupied square and must follow its moves.
- **EN19.16 PDF45/printed37; ZH19.16 PDF28/printed27:** mixed checking, threat, chasing and idle conduct is distinguished from all-check/all-chase. EN20.2 PDF46/printed38 and ZH20.2 PDF29/printed28 allow the named mixtures. This does not authorize forgetting earlier moves in a classified window merely because the last return has a pure category.
- **EN20.1 PDF46/printed38; ZH20.1 PDF29/printed28:** perpetual checking remains prohibited irrespective of circumstances, except mutual all-check draw. Brief EN emphasis: "Under ALL circumstances". Changing literal action sequence while still checking on every move does not cure perpetual check.
- **EN20.3/6/7 PDF46–47/printed38–39; ZH20.3/6/7 PDF29/printed28:** same-victim chasing, real roots, and same-type exceptions establish the new T fixture below. Its target is an unprotected Cannon chased by a Rook, so neither same-type nor rooted-target exception applies.
- **EN chapter5 diagram3 PDF49/printed41:** a longer multi-rook cycle is still perpetual check. Diagram4 on that page supplies mutual cross-check, not a general window-selection algorithm. Diagrams1/2 PDF48/printed40 retain their original incomplete/end-state discrepancies; no invented completion is used here.

The exact text does not uniquely name an implementation data structure or express first/last-anchor tie-breaking. We should not claim it does. The already approved software rule explicitly fixes **third and fourth board occurrences**, which supplies the remaining deterministic occurrence boundary without further product approval.

## Deterministic interpretation within existing owner authorization

1. A repetition key is the complete board plus side to move, independent of counters/events. Maintain chronological occurrence plies for each key starting at the known root; a bare imported FEN cannot supply earlier ones.
2. At a key's **first third occurrence**, let its first three chronological occurrence boundaries be `p0 < p1 < p2`. The witness is the **contiguous** legal action history `(p0,p2]`, preserving both return subintervals `(p0,p1]` and `(p1,p2]`, each move's actor, actual conduct, and persistent piece identities. Classify every relevant action; do not substitute just the latest return or a noncontiguous subsequence. Record unilateral obligation if and only if that whole witness establishes it.
3. At this key's first fourth occurrence `p3`, use the warning's exact history witness and the contiguous continuation `(p2,p3]`. An unbroken same responsibility produces the selected loss. A literal action-word change does not itself break responsibility. A non-check move breaks all-check; the historical warning event remains in the record but cannot independently justify a later all-check loss.
4. If the fourth occurrence has no continued one-sided obligation under that witness, apply the appropriate selected WXF draw category. Do not silently restart an episode and postpone the owner-approved fourth occurrence because a return path changed.
5. A different board key matures only at the event boundary where that key is the current position. Two different complete board keys cannot newly mature at the same board-action boundary. Earlier authoritative conclusions stop play; shifted anchors cannot later overturn them. Warnings for shifted keys are history observations, not independent authority to discard intervening conduct or retroactively change an earlier event.
6. Keep classification/cycle witnessing and terminal-event ordering explicit in the game layer. Local moves remain legal even when accepting them causes a repetition conclusion.

This is a minimal deterministic completion of the current owner interpretation, **not a claim of an official WXF erratum**. I found no concrete outcome-changing alternative that is both supported by the source and compatible with the current literal occurrence policy. Restart-on-exact-action-cycle variation would change that explicit policy. Latest-return-only classification would call Q+C all-check by ignoring Q, despite Q being inside the third-occurrence witness. No additional owner question is recommended on those alternatives. The separate reviewer should challenge this reading using concrete histories; any actual contradiction must be retained.

## Independently constructed exact histories

### Existing F002 root and cycles

Root `4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1`: Red General d0, Red Rook e9, Black General f8; Red moves.

- C = `e9f9 f8e8 f9e9 e8f8` (all Red checks, no Black checks).
- Q = `e9a9 f8f7 a9e9 f7f8` (non-check permitted moves, no non-General material victims; not asserted Idle without separately excluding mating-threat labels).
- **D** = `e9e8 f8f7 e8e7 f7f8 e7e8 f8f9 e8e9 f9f8` (longer eight-ply all-check return).

All have original complete board and side at the end. Counters intentionally differ. D's board after its first ply equals the board after its fifth ply, so a four-ply candidate interval overlaps its longer eight-ply return. It is not sound to assume the shortest identical action word defines the whole classification.

| Exact history | Root occurrence plies | Independently expected boundary |
|---|---|---|
| C C |0,4,8| Red all-check warning at8, no conclusion |
| C C C |0,4,8,12| Red loses12; Black's final move does not make Black offender |
| Q Q Q |0,4,8,12| Draw12 |
| C Q C |0,4,8,12| No warning8; draw12, no noncontiguous C cherry-pick |
| Q C C |0,4,8,12| No warning8 because full0→8 includes Q; draw12 under fixed fourth occurrence policy |
| C C Q |0,4,8,12| Warning8; Red's non-check move9 breaks all-check; draw12; warning8 remains historical |
| **C C D** |0,4,8,16| Warning8; Red keeps checking despite different path; Red loses16, not12 |

For C C, other anchors of the same cycle lie at plies1/5,2/6,3/7. They would reach their third occurrences at9/10/11 **only if** C continues. In C C Q, the action9 produces a different position; those old shifted candidates cease to mature, whereas the original root remains reachable by the new Q route. Thus variation retires a candidate without erasing all old board occurrences or all possible new cycles.

### New check/chase overlap witness

Root `4R4/5k3/c8/9/9/9/9/9/9/3K5 w - - 0 1`: same Generals/Rook plus Black Cannon a7.

- C remains legal and all-check, leaving the Cannon unmoved.
- **T** = `e9a9 a7e7 a9e9 e7a7`.
- After each Red T move, the Rook has a legal capture of the **same** Black Cannon down its file, with no screen/obstruction; the Cannon moves horizontally away on each Black reply. It is not protected by Black General f8; there is no immediate legal recapture after capture. The victim is not a General, Soldier, or same-type piece. T is unilateral prohibited same-Cannon chase when repeated.

| Exact history | Root occurrence plies | Independent expectation |
|---|---|---|
| T T T |0,4,8,12| Warning Red at8; Red loses12 for same-Cannon pursuit |
| C T C |0,4,8,12| Full0→8 mixes checking/pursuit; no one-sided all-check/all-chase warning; draw12 |
| T C C |0,4,8,12| Full0→8 includes chase; do not cherry-pick latest C; draw12 |
| C C T |0,4,8,12| Check warning8; first T move9 breaks all-check. Latest T alone is chase but does not retroactively become the third-occurrence witness; draw12 |

The four-ply latest T interval is pure chase whereas the containing eight-ply C+T interval combines check/chase. This is a concrete overlap with different classifications, not merely an integer index tie. All listed actions are complete local actions; no partial or synthetic game actions are used.

## Expected facts versus observations

`proposed-cycle-fixtures.json` preserves independently reasoned expected outcomes and exact complete histories. It is a proposal for review, not a passing implementation suite. `native-board-prefix-observations.json` separately records **143** actual accepted-board-probe prefix replays across **11** histories, all success and exact undo. These observations establish local legality only. They do not validate the proposed history outcome and are not differential/comparator authority.

Executed command: `python3 /private/tmp/msh94-research/build_cycle_research.py`, using accepted existing developer binary `target/debug/examples/xiangqi_probe`; no native history adjudicator exists in this probe. All source and board reasoning above preceded this execution. No Pikafish history/outcome routine was called.

### Independent review note

Reviewer `xq_history_reviewer` independently traced C/D checks and T chase geometry/legal recapture. They correctly noted that Q must not be asserted Idle without excluding every19.2 mating-threat case. Q is now labeled only non-check/no non-General victim; either Idle or an allowed mating-threat mixture has the same relevant no-unilateral-check/no-chase outcome here. This precision prevents a test from pretending it has independently proved a stronger classification than needed.
