# MSH-161 (Competitive Gameplay Diagnosis): scientific synthesis

Status: scientific synthesis from all 200 closed games, with corrected category and threshold summaries bound in the structured companion. Independent challenge accepted the substantive judgments and the bounded favorable segment. Final delivery must retain the review of these exact files. This document recommends investigations for John's review. It does not authorize implementation.

The requested analyst configuration was `gpt-6-astra`, reasoning effort `high`. This records the requested configuration, not an independently observable backend identity. See `model-assignment.json`. This analysis used completed repository evidence and cached reports only. It did not run an engine, retrieve evidence, or change production code.

## Main finding

mShogi can attack, coordinate pieces, promote pawns, defend long endings and complete difficult mates. Its observed weakness is inconsistent choice at consequential transitions. In several distinct positions, it takes material or advances a pawn before resolving a specific exchange, defensive task or coordination requirement. In other positions, it misses an active defensive move or delays a concrete resource while moving pieces elsewhere. These are recurring decision behaviors. They do not establish a common defect in search or evaluation.

The most useful next investigation should join exact continuation coverage with the judgment of the resulting positions. A broad instruction to increase activity, avoid pawn moves, or prefer captures would contradict the evidence. Some errors require a quiet move; others require an immediate capture or pawn break. The common question is whether the engine compares the consequences of the available actions correctly.

Endgame coordination and the timing of passed-pawn actions deserve a second, distinct investigation. The study contains concrete missed promotion and king-activation resources. It also contains successful promotions, defensive liquidation and bishop-and-knight mate. It does not support a general claim that mShogi cannot convert an advantage, nor an exact conversion failure rate.

## What the completed sample establishes

The native records give 82 wins, 58 draws and 60 losses for mShogi. Prepared games give 67 wins, 55 draws and 58 losses, a score of 52.5%. The approved whole-pair, family-stratified bootstrap gives a conditional 95% interval of 47.22–57.78%. Leaving out one observed family gives scores from 51.45% to 53.53%. These are conditional summaries of the selected master-supported openings, not a random sample of all Chess.

Oxidation was a useful fixed practical reference. It won 58 prepared games and provided resistance as well as exploitable errors. The final evidence does **not** establish that it was stronger. The original aim to use a demonstrably stronger reference was not achieved by the final result. This limits claims about performance against stronger opposition. It does not invalidate supported local decisions, since those are separate matched Stockfish comparisons. No rating or Elo estimate follows.

The unaided result was 15 wins, three draws and two losses. All ten candidate Black games were wins; candidate White scored five wins, three draws and two losses. This asymmetry is a property of these repeated starts and this opponent. All ten Black games share the first 16 plies. Their complete trajectories are distinct, but they are not ten independent opening families. The 82.5% unaided score must not strengthen the prepared confidence interval or imply broadly superior opening play.

The coverage record reconciles all 24,939 played decisions and keeps 2,880 supplied moves separate. Both players received reference analysis. There are 6,141 selected roots and 589 supported candidate comparisons in 175 games. The selected candidate dispositions also contain 1,781 same-choice controls, 1,013 subthreshold comparisons, 657 unresolved roots and 20 claim-limited roots. These numbers describe the final selected ledger. They are not an unbiased all-move error rate. Screening-only roots remain unresolved; the 9,711 screening-only decisions are not certified non-errors.

The supported comparison severity counts are 106 significant, 64 serious and 419 major under the frozen E thresholds. E is Stockfish's self-play expected-score proxy. A margin of 0.5 or 1.0 is not a measured loss of that probability against Oxidation. Saturated WDL estimates, selected consequential roots and correlated decisions help explain the large major-event count. Do not add margins or treat 419 major comparisons as 419 independent lost games.

Base phases are 105 opening, 366 middlegame and 118 endgame supported candidate roots. Endgame counts become 97 or 121 with material cutoffs 24 or 28. Opening counts become 88 or 126 when the boundary moves by two plies. These shifts change descriptive grouping, not support for the underlying move comparisons. They do not establish that the middlegame is intrinsically more error-prone: the exposure and selection processes differ.

Source packets use different attribution thresholds: the first leaves 82 of 240 supported roots unknown, compared with three of 151 and one of 198 in the later packets. This difference is not a batch effect. The aggregate applies all 11 additive correction rows from the two challenge records; immutable packets remain unchanged. The corrections include wording changes as well as tag changes. Unknowns remain unknown. The following counts therefore describe the corrected coding, not a precise defect-frequency ranking or a comparison of rates between packets.

| Descriptive category | Supported roots | Affected games | Prepared / unaided games | Prepared families | Major roots |
|---|---:|---:|---:|---:|---:|
| Coordination/activity | 252 | 117 | 102 / 15 | 22 | 180 |
| Quiet defense | 154 | 86 | 82 / 4 | 22 | 136 |
| Tactical calculation | 127 | 83 | 70 / 13 | 22 | 106 |
| Endgame decisions | 118 | 62 | 59 / 3 | 21 | 110 |
| Pawn structures/breaks | 110 | 78 | 70 / 8 | 22 | 75 |
| Exchanges | 100 | 80 | 68 / 12 | 22 | 71 |
| Opening/development | 71 | 52 | 36 / 16 | 20 | 19 |
| King safety | 40 | 32 | 30 / 2 | 19 | 35 |
| Opportunity response | 37 | 23 | 20 / 3 | 11 | 33 |
| Advantage preservation/conversion | 3 | 1 | 1 / 0 | 1 | 3 |
| Planning | 0 | 0 | 0 / 0 | 0 | 0 |
| Unknown behavior | 86 | 50 | 45 / 5 | 19 | 47 |

Tags overlap. “Opportunity response” describes a concrete available resource unless a supported opponent contribution is separately shown. The three conversion-tagged roots are the repeated promotion motif described below, not an episode count. Zero planning tags mean that this coding did not establish a separate planning category. They do not prove the absence of planning errors. The detailed family, phase, color and result tables are in `behavior-aggregate.json`.

The broad activity, defense, tactical and exchange tags occur in both colors and in wins, draws and losses. Quiet-defense tags include 92 roots in losses, 38 in draws and 24 in wins; activity tags include 75, 78 and 99 respectively. Those are roots grouped by their game's result, not counts of lost games caused by each behavior. The larger Black quiet-defense count, 91 versus 63 White roots, is also descriptive. Different game trajectories and selection prevent a color-specific defect-rate claim.

The broad observations are not confined to one opening family. After leaving out any one prepared family, activity remains coded in 94–101 prepared games, quiet defense in 76–81, tactical calculation in 65–69, exchanges in 63–67 and endgame decisions in 53–59. These are affected-game counts among the remaining 170 or 172 prepared games. They establish observed breadth under this coding, not population prevalence. By contrast, removing the one family containing the promotion motif removes all three conversion tags.

Threshold sensitivity also leaves the main broad observations present. Of the originally supported coded roots, 236 activity, 152 quiet-defense, 124 tactical and 116 endgame roots remain supported at .06. Opening/development falls from 71 to 56. Across all selected candidate roots, the corrected literal range calculation gives 614, 589 and 555 supported at .04, .05 and .06. A separate uniform 1e-9 arithmetic tolerance gives 615, 590 and 556. This is sensitivity analysis, not an upgrade of source dispositions; the supported coding population remains 589. The earlier incomplete threshold summary must not be used. Stable straddlers and unresolved roots remain separate; the 657 original unresolved candidate roots are not allocated to invented behavioral categories.

## Recurring behaviors and exact examples

Root identifiers use zero-based ply before the move. The structured companion binds each example to its exact packet, native history and final ledger. The continuations below are legal retained reference lines. They illustrate a concrete consequence; they are not forced lines or game-theoretic proofs. The observed native continuation is stated separately where it matters.

### 1. Exchange order and piece placement can fail together

At `main-06-08/g0/p44`, White played **Qxe2** after ...Rxe2. The native game continued **...Ne5 Nxe5 Bxe2 Nxe2**. The retained alternative starts **Nxd4 Re4 Nf3 Rxc4 Qd3**. The matched margin is 0.5095, major. This is a concrete recapture-order problem: the immediate queen recapture permits an intervening knight move and a different exchange sequence. It is not evidence that all captures are overvalued. The game ended in a candidate loss; earlier uncertainty prevents a first-cause claim.

At `main-08-01/g0/p28`, White played **Qxh7**. Both the native course and the tested line show **...Rdd8**, then **...Rh8** after d4, restricting the queen. The tested line continues **Qg7 Rdg8 Qxh8 Rxh8**. The alternative **Ng3 Rg8 Kh2 b3 Qd2** keeps different defensive resources. The matched margin is 0.5835, major, and the native result was a loss. Here the concrete problem is queen mobility after a material capture, not a generic penalty for exposed queens.

At `main-10-01/g0/p72`, White played **Bxf5**, followed in the native game by **...Bg6 Bxc8 Rxf3**. The alternative starts **Nd7 Rd6 Kf1 Rc7 Nc5**. The matched margin is 1.0. The supported judgment survives .04/.05/.06 thresholds, but it remains a finite judgment. The game later ended in a threefold claim. This example links a quiet knight move to the exchange and queen-safety consequences that the immediate capture failed to preserve. A draw does not erase the local error; the error does not make the draw an observed lost win.

These three anchors occur in different prepared families, include two losses and a draw, and involve different pieces. Related Black-side cases include `main-08-03/g1/p69`, where **Qxh3+** was inferior to **Rf8**, with a retained defensive line **Rf8 Nxd4 Nf4 Rxc6 Nxe2 Ne6+**. Its native continuation **Qxh3+ Nxh3 Rxh3+** led into further play and eventually a loss. The common behavior is a failure to retain a consequential alternative. “Tactical calculation” is a descriptive tag here; it is not a diagnosis of pruning, depth or evaluation.

### 2. Quiet defense and useful activity require specific obligations

The broad activity and defense categories overlap. A move may activate one piece while removing a necessary defender or allowing an attack. Their counts must not be added as separate causes.

At `main-01-00/g0/p48`, **h4** is inferior to **Rfd1**, margin 0.571. The played reference line starts **h4 Rg8 Rab1 Bc6 Kg2 Rg4**; the alternative starts **Rfd1 Bd5 Rab1 Kg7 Rd2**. In the actual game, **h4 Bd5 Qe2 Rh6 Rfd1 Bxh4** led to a different attacking course. Both support concern about defensive coordination and king access, but the reference rook lift was not the exact native continuation. The native result was a loss.

At `main-03-02/g1/p31`, Black's **f5** was inferior to **f6**, margin 1.0. The exact retained line begins **f5 Qh5 Qe8 Qh6+ Kg8**; the alternative begins **f6 Nxc6 Nxc6 Bxb5**. This is a specific king-access and central-tempo contrast. The packet's prose says “Qh5+”; the legal SAN is **Qh5**, without check. This synthesis uses the legal line and does not change the immutable packet.

At `main-05-00/g0/p24`, **Qd3** versus **Qb3** is a large supported comparison, but its current evidence only warrants a descriptive queen-placement claim. The challenge removes a tactical tag. This is a useful boundary: a large reference margin and a legal PV do not by themselves identify a forced tactic or a missing search branch. The candidate won this game. A severe local disagreement can coexist with a successful native result.

### 3. Endgame actions need king, piece and passed-pawn coordination

At `main-05-01/g1/p85`, Black played **g5** instead of **Ke5**. The alternative **Ke5 Bb6 Kd5 f4 Kc4 Kf3** activates the king toward the queenside. The retained continuation later captures a4 and promotes the a-pawn. The played line allows a different bishop defense. The margin is 0.5; the native result is a dead-position draw. This is evidence for a local king-versus-pawn priority error, not proof that the native draw was avoidable against every defense.

At `main-05-04/g0/p84`, White played **h6**, and the native game continued **...Nxf5 h7 Nxh4+**. The alternative **Ng3** protects f5 and h5 and permits **Bf8 Kg4** in the retained line. The margin is 0.5 and the native result was a loss. The challenge correctly replaces the generic pawn-structure tag with coordination. The problem is the loss of support during the pawn advance.

The opposite action is sometimes required. At `main-05-04/g0/p70`, **g5** is superior to **Kf3**, with **g5 Bd6+ Kf3 Ke5 g6 hxg6 fxg6** in the retained line, margin 0.5. These two roots in one ending refute a rule such as “activate the king before pushing a pawn.” The engine must compare the concrete pawn route and the opponent's replies.

At `main-09-01/g0/p132`, **e6** permits the actual sequence **...Rxg3+ Ke4 Rxe3+ Nxe3**. The alternative **Kf4 Rg7 Kf5 Bxe5 Kxe5** preserves a different ending, margin 0.5. At `main-10-08/g1/p117`, Black's **Ra7** is inferior to **Be7**, whose retained line **Be7 Re5 Bf8 Re4 Rc8 Rc4 Rxc4 Bxc4** uses bishop coverage and liquidation against an advanced f-pawn. These examples extend the coordination problem to rook and minor-piece endings and to both colors.

### 4. Opening choices are part of the same decision problem

The supplied master prefixes are context, not candidate errors. The first played choices, development and central breaks still contain supported failures. At `main-10-04/g0/p22`, **Bh6** is inferior to **e5**, margin 0.6085. The retained alternative starts **e5 Ne8 Ne4 f6 exf6 Nxf6 Bxf6 Rxf6 Nxf6+ Bxf6**. The actual **Bh6 Ne8 Bd3 Kh8 Bg5** does not obtain that same central course, and the candidate later lost.

This example supports investigating the timing of central play and piece deployment. It does not show that an opening book is the necessary correction. Tiny family/color samples, selected roots and the strong but dependent unaided result make a family blacklist or a book-first priority unjustified. MSH-160 (Chess Opening Quality Assessment) should use these same records. There is no need for a separate competing corpus.

## Linked opportunities, preservation and outcomes

Three bounded narratives help separate errors, opponent contributions and native results. They are illustrative episodes, not an exhaustive episode census.

**A supported favorable segment followed by a reversal: `main-06-02/g0`, plies 73–80.** At p73 the reference played ...Nc7, with stable mover E=0 versus [.4955,.4975] for ...Rg7. White then played Rb8 at p74, E=[.999,.9995], and Ra8 at p76, E=[.7785,.7785]. These are successive candidate turns with stable played E above .75, so this bounded segment meets the frozen two-turn favorable criterion. The p76 comparison already prefers Rxc8 by .2215, but the played estimate still remains favorable. At p78, R8a7 has E=0 versus E=1 for Rxc8. The alternative starts **Rxc8 Qxc8 Re1 Qe8 Re3 Rf7 Qh3**, retaining the attack.

The reference then played ...Nb5 at p79, E=.5 versus E=1 for ...Rf7. White played Ra8 at p80. Black then claimed threefold repetition with intended ...Nc7; that intended move was not played. This sequence records a reference-created opportunity, initial preservation, loss of the finite favorable segment, and a later reference concession to a drawing course. It does not imply that the candidate independently recovered a lost position, that its p76 and p78 margins should be added, or that the claim concealed a forced mate. Earlier p67–72 uncertainty remains. The segment is bounded from the demonstrated qualifying turns; no earlier onset is asserted.

**Repeated promotion choices: `main-02-00/g1`, plies 81, 91 and 113.** Black chose Ke5, Kc4 and Ra2 while **b1=Q** was a supported alternative at each root. Each retained comparison gives played E=.5 and alternative E=1. The line at p81 starts **b1=Q Rxb1 Rxb1 Rxa3 Rb3**. The actual game contains a long checking sequence and ends by threefold claim at 127 played plies. Plies 91 and 113 have the same board arrangement but different clocks and histories. Thus these are three supported local choices within a repeated motif, not three independent conversion failures. They show a concrete failure to use a promotion resource. They do not supply a complete favorable-episode denominator or a counterfactual no-claim result.

**Missed attacking resources followed by a win: `main-02-05/g0`, plies 46–62.** The retained alternative Bxh7 is supported at six selected candidate roots. The sequence also contains supported reference errors at p45, p51, p53, p55 and p61, plus unresolved roots. At p46 the actual move was Nf1, whereas the tested alternative begins **Bxh7 Be5 Be4 Na4 Bxe5 Qxe5**. The candidate eventually promoted two pawns and delivered **Qbc7#** at p204. This is repeated missed local opportunity inside an eventual win. It cannot be counted as six lost wins or a clean uninterrupted favorable episode. The opponent's contributions and the candidate's later successful play are both part of the explanation.

## Contrary evidence and limits on the diagnosis

The native ending of `main-01-08/g0` contains **Kc2 Ka1 Kb3 Kb1 Na3+ Ka1 Bc3#**. This is a completed bishop-and-knight mate, not merely a favorable reference score. In `main-09-02/g0`, the candidate's actual sequence includes **e6, e7, e8=Q**, then **Rc7+**, **Qb5+** and **Qb3#**. The new Bb1 alternative in that game was stably worse than played Bd3. It must remain contrary evidence, not be recast as another error.

Other native successes in the coding packets include passed-pawn wins, active rook defenses, and dead-position draws reached by stopping or capturing promoted pawns. These facts constrain any proposed correction. A change that rewards activity but damages blockade, forces exchanges that damage attacks, or advances passed pawns without their supporting pieces would not address the observed problem coherently.

Good local choices also occur in losses. In `main-08-00/g1`, the actual **...Nxg3** at p45 has a supported subthreshold comparison against ...Nc3, with margin .0005, and **...Bxh3** at p55 is a same-choice control. These are useful local controls for the attacking sequence. The earlier active **...Ra2** at p43 remains unresolved; its attractive appearance is not certified success. None of these controls proves the whole attack sound.

All 200 games used the original AutoClaim policy. Of 58 draws, 43 were threefold claims and four were fifty-move claims. Intended claim moves were not played. Automatic claims censor observation of play beyond those endpoints. They do not justify assigning wins, counting all draws as conversion failures, or blaming search for a runner policy. Dead-position draws are a different native outcome. This synthesis estimates no alternate-policy result.

Unresolved roots can change the account of how a game reached an illustrative position. For example, the large Ne2 comparison at `main-10-01/g0/p56` remains unresolved, and several nearby transitions are also unresolved. They cannot be silently set to zero or promoted to errors. Removing all broad activity labels would weaken claims about category prevalence, but it would leave the exact exchange-order, queen-restriction, pawn-support and promotion examples above. Conversely, plausible unresolved cases may add behaviors; without supported labels and boundaries, they do not give a numerical upper bound or a stable new ranking.

The primary audit includes admitted mixed-budget expansion. Full played-root coverage is complete, but query depth and selected populations are not uniform. No phase, family or engine error-rate comparison should use the selected counts as if each played decision had the same confirmation budget.

## Recommended investigations for John's decision

The order below is a practical order of investigation. It is not a proven frequency ranking or a choice of production architecture. It follows exact consequential examples and the need to distinguish explanations before correction.

| Order | Observed behavior | Possible mechanisms, not findings | Discriminating investigation | Potential general correction, only if supported |
|---|---|---|---|---|
| 1 | Exchange order, queen mobility, quiet defense and attacking preparation are sometimes compared incorrectly at a critical transition. | A relevant continuation is not admitted or not completed; selective search loses it; the returned bound is misused; the continuation is searched but its resulting position is misjudged. | Trace the actual interior search for contrasted failure and success families. Record legal alternative admission, windows, TT use, reductions, frontier replies and returned values. Check the concrete consequence at matched work and five-second conditions. A separate child search or a displayed PV is insufficient. | Repair demonstrated general search/value propagation defects, or narrowly justified classical threat/coordination judgment. Do not select a blanket activity bonus or pruning change from this report. |
| 2 | Pawn advances, king activity, piece coverage and liquidation can be timed incorrectly in endings. | Pawn-race horizon; incomplete treatment of checking defenses or promotion transitions; incorrect ranking of king routes, blockers or exchanges. | Contrast cases requiring immediate promotion/push with cases requiring support or king activation. Preserve full history and claims. Test intermediate-position rankings and concrete legal consequences; separate continuation admission from evaluation of reached states. | General passed-pawn/blockade/king-activity or exchange judgment, or focused continuation coverage, according to the result. A tablebase project is not established as the necessary remedy. |
| 3 | Central play and development choices can miss stronger continuations after prepared prefixes. | The same continuation/valuation problems occur early; a separate development or central-tension weakness may remain. | Reuse the opening roots and matched success cases from MSH-161 (Competitive Gameplay Diagnosis), coordinated with MSH-160 (Chess Opening Quality Assessment). Test consequence quality before proposing a new opening subsystem. | A general central-tension/development correction only if it explains remaining cases and preserves good unaided and prepared play. |

Before any such work, freeze a small behavior-defined failure/success panel from this evidence, with full histories and independent expected consequences. Keep families together when separating development and evaluation material. These 200 games are exposed evidence; do not relabel their positions as an untouched held-out test. Preserve the separate existing benchmark's exposure rules. This is a recommendation for a later bounded task, not a request for another broad assessment or new corpus now.

For the first investigation, require an explanation that predicts an observable distinction: whether a critical continuation is absent, cut off, returned incorrectly, or reached and valued incorrectly. Accept a correction only when it improves that consequence across more than one family, preserves the success controls and does not depend on a position-specific exception. If distinct reasonable trials fail, stop with the exact unresolved distinction. Do not convert speed, depth or an attractive PV into a decision-quality claim.

After a meaningful coherent correction, propose a controlled earlier/current comparison under matched practical conditions. Any substantial competitive campaign needs separate approval. This study does not authorize learned evaluation, Shogi/Xiangqi work, a new opponent campaign, or automatic implementation.

Retain the final independent review with this synthesis and stop for John's review. No Batch 11, new search, or production correction is part of this synthesis.
