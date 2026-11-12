# MSH-161 (Competitive Gameplay Diagnosis): interim gameplay assessment

This is an **exploratory assessment of the 20 fully analyzed local Mac games in main batch 01**, not the conclusion of the intended 200-game study. The evidence repeatedly shows missed tactical resources, loss of an advantage through the wrong continuation, and missed active defenses. It also shows successful attacks, recoveries and long conversions. Those observations justify broader observation, not a final weakness ranking, a rating claim, or an engine change.

**Local execution is now stopped after batch 02**, including its existing games,500k screening and5M confirmations. There are40complete Mac main games; the quantitative findings below retain their original20-game batch-01 denominator and selected20M evidence. The separate batch-02 addendum records additional provisional5M observations without pooling those evidence levels. The20earlier qualification games remain separate. No localbatch03, new20M or automatic retry is authorized. The primary200-game study will replay all original planned pairs on validated standardized cloud workers; Mac evidence stays an exploratory hardware stratum. See the [repository cloud handoff](evidence/msh161/migration/cloud-study-handoff.md).

## Evidence base and limits

Batch 01 contains nine prepared opening pairs spanning seven broad frozen family labels, and one unaided start pair, all against fixed Oxidation with five seconds requested per move. Native outcomes were **10 wins, 2 draws and 8 losses** for mShogi: prepared games 8W/2D/8L over 18 games, unaided games 2W over two games. The shared, unpinned Mac host and actual timing receipts are documented in the [independent verification review](evidence/msh161/main/batch-01/verification-review.md). These outcomes neither measure a rating nor establish results under the future cloud conditions.

All **2,494 played decisions by both players** have recorded dispositions; 288 prepared-prefix decisions are excluded from competitor-error attribution. Initial 500k-node reference screening was followed by matched 5M queries for the preregistered follow-up/audit panel and selected 20M adjudication. Independent reviewers examined complete games, legal explanatory continuations, successes and opponent responses. The [first-half 20M overlay](evidence/msh161/main/batch-01/adjudicated20m-00-04.json) and [second-half overlay](evidence/msh161/main/batch-01/adjudicated20m-05-09.json) supersede earlier judgments at their queried roots. Earlier records remain preserved.

Across the **73 selected decisions**, the overlays contain 49 supported finite deteriorations across both players, four subthreshold comparisons, 14 unresolved comparisons, four stable singleton controls and two judgments whose actual-result attribution depends on claim policy. These are selected-decision dispositions, **not independent error samples or population frequencies**. Several moves belong to one advantage-loss/recovery episode; some describe the reference player. A later error in an already poor position is not another independent explanation for the defeat.

“Supported” means a legal, stable finite reference comparison satisfying the frozen method, including observed-range sensitivity. A lone stale-final-report flag is permitted and retained; directional-bound conflicts, instability and other defects are not waived. WDL expectations are Stockfish's mover-relative model outputs, not observed win probabilities. Neither finite support nor a plausible PV proves a game-theoretic result. The initial audit found no stable missed-important-error threshold crossing in its sampled comparison stratum; that does not prove absence of undiscovered mistakes. See the [method](evidence/msh161/main-method.md) and verification review for exact coverage and limits.

## Observed frequency, severity and phase

The candidate made **1,248 played choices** in these 20 games. The targeted 20M panel selected **49 candidate decisions**, of which **31 have supported finite deterioration**, occurring in **18 distinct games**. The selected decisions are not random: they emphasize consequential and uncertain cases. Thus 31/49 describes the adjudicated panel, not the error rate among all choices. Nor are 31 moves 31 independent episodes. Claims with policy-dependent result attribution and unresolved comparisons are excluded from these supported counts.

Severity uses the conservative observed-range difference: significant .05–<.10, serious .10–<.20, major ≥.20. These are changes in the reference WDL expectation, not empirical loss probabilities.

| Severity of supported candidate comparison | Decisions | Distinct affected games, out of 20 |
|---|---:|---:|
| Major | 24 | 15 |
| Serious | 1 | 1 |
| Significant | 6 | 6 |
| Total | **31** | **18 unique games** |

Phases below use the **frozen per-root phase tags**, not a new narrative classification. Prepared moves do not enter the denominators. Some prepared starts already leave little or no candidate opening play.

| Frozen phase | Candidate played choices | Selected candidate 20M decisions | Supported decisions: major / serious / significant | Distinct affected games / games with candidate choices in phase |
|---|---:|---:|---:|---:|
| Opening | 45 | 4 | 1 / 0 / 2 | 3 / 14 |
| Middlegame | 638 | 38 | 17 / 1 / 4 | 14 / 20 |
| Endgame | 565 | 7 | 6 / 0 / 0 | 5 / 19 |
| Total | **1,248** | **49** | **24 / 1 / 6** | **18 distinct affected games overall** |

Games can appear in several severity and phase bins; the game counts must not be added. The small, targeted endgame and opening panels do not establish their relative difficulty or prevalence. These counts are separate from the overlapping behavioral example groups below. Exact event membership, frozen phase hashes and denominators are retained in the [frequency audit](evidence/msh161/main/batch-01/interim-frequency-audit.json).

## Recurring behavior, with conservative denominators

The following are **descriptive, overlapping example groups**, not ranked causes. Their numerators count distinct reviewed games with the specified supported 20M candidate examples, at most once per group per game. The denominator is the 20 analyzed games—not 200 intended games, 2,494 moves, qualification games, or unfinished batch 02. These lower-bound example counts are not exhaustive prevalence estimates: relevant positions are not equally common across games, and many judgments remain unresolved. A game appearing in several groups does not supply independent observations of several mechanisms. The [independent denominator audit](evidence/msh161/main/batch-01/interim-frequency-audit.json) verifies the exact example members: these three groups overlap and cover 15 distinct games in union.

| Behavioral question | Conservative recurrence in these 20 games | Representative supported evidence |
|---|---:|---|
| Recognizing a concrete threat or tactical consequence before committing | At least 5/20 games across Sicilian, Slav/Semi-Slav, Nimzo-Indian and Queen's Indian/Bogo families | 02/0 p52 g3 allows ...Bg5/...Rxf2/...Be3; 03/1 p21 ...b4 permits forcing central captures; 04/0 p38 Nxf7 permits ...Rxa7/...Ra1+ and a passer; 06/0 p82 Ke1 loses the defense available with Be6; 07/0 p32 b3 neglects the threatened bishop and queen-tempo sequence. |
| Preserving a favorable position through the next continuation | At least 8/20 games across five prepared family groups | 00/0 p84 Re5 and p92 h5 belong to one repeated attacking-opportunity episode; 00/1 p119 ...Kg8 permits recovery; 01/1 p81 ...b3 advances before the stronger ...c4; 03/0 p30 g4 loses pressure relative to Nb4; 04/1 p35 ...Nxd5 misses ...Ra5; 06/1 p81 ...c4 misses ...b4; 08/0 p34 Qd6 releases an attack; 08/1 p63 ...e4 loses the advantage preserved by ...Re8. |
| Finding active defensive resources or choosing a viable ending transition | At least 5/20 games across English, Sicilian, Slav/Semi-Slav and Queen's Indian/Bogo families | 01/0 p106 Rd1 misses active Rd8 defense; 02/1 p87 ...Rc6 exchanges into the inferior ending instead of ...Kf7; 03/1 p61 ...g6 misses the exchange defense ...Re8; 06/1 p89 ...Kxh6 misses ...b4; 07/0 p108 Ra7 and p126 Kf5 miss distinct late recovery chances. |

References use shortened pair indices, **game 0/1 and zero-based ply**. For example 02/0 p52 means `main-01-02`, game 0, ply 52. The [pairs 00–04 review](evidence/msh161/main/batch-01/review20m-00-04.md) and [pairs 05–08/unaided review](evidence/msh161/main/batch-01/review20m-05-09.md) link these comparisons to exact histories and hashed raw receipts. The [category taxonomy](evidence/msh161/main/category-taxonomy.json) preserves original themes under descriptive parents; it is not a causal model.

The phase span matters. Some supported differences arise in early development and central choices: pair 05/0 p16 Bxf6 versus f4 and 06/0 p20 Bd2 versus e4 are modest contextual losses. Larger examples occur in middlegame tactics and attacking continuations, and others arise after simplification when rook activity, king activity or passed-pawn timing matters. There is no evidence here for reducing the problem to a single opening defect. Conversely, two unaided wins do not prove opening competence generally: the unaided p2 e4 and p3 ...e6 comparisons show modest finite opportunity costs, without establishing that those moves are unsound or too unusual.

## Opportunities, recovery and contrary successes

Both players' errors and responses matter. In 01/1, candidate ...b3 p81 permits a drawing defense, but reference axb3 p84 instead restores the dangerous passer; candidate subsequently promotes and wins. In 04/1, candidate correctly finds ...Nd4 after reference cxd5, later fails to retain all the advantage with ...Nxd5, then exploits renewed rook-entry opportunities. Its later ...Qh2+ followed by ...Nf3+ is an actual successful queen-fork sequence. These are multi-step episodes, not proof that a win means all preceding decisions were good.

The candidate also demonstrates substantial constructive play. Pair 05/0 exploits ...Nb4 with Qg3+ and converts an attack. Pair 07/1 finds ...Ra2 and coordinated rook invasion after reference b4. Pair 08/0 correctly uses c4, misses a continuation with Qd6, then finds Qf6 when the opponent offers a second chance. The unaided White game develops, exploits the reference's ...Qe8 with Bxf6 and sustains a mating attack. Pair 00/0 contains missed attacking chances but also actual draw defense through a long liquidation. These contrary examples prevent a blanket assertion that the candidate cannot attack, coordinate rooks, defend, or convert.

Deeper work also removes apparent failures. Pair 05/0 Ndb5 p18 becomes subthreshold, and f5 p30 retains a very large favorable estimate rather than a supported preservation loss. Pair 07/1 ...f6 p29 is subthreshold. Pair 01/0 g4 p18 changes from an earlier significant classification to a .0465 conservative difference at 20M: above a .04 sensitivity threshold, below the primary .05 and .06 thresholds. Such reversals must remain in the record; they are not inconvenient exceptions to discard.

## Uncertainty and the native claim-policy distinction

Fourteen selected comparisons remain unresolved, plus earlier unqueried or linked uncertainties. Among them are 00/0 p28 despite the attractive c5 tactical line, 02/1 p61 despite the ...Nc5 resource, 03/0's rook sacrifice/defense sequence, and 08/1 p71 ...Qxd1+. The last must not retain its previous 5M “largest confirmed error” label. Two stability cases sit exactly on the decimal .025 expectation-grid boundary but trigger the frozen binary-arithmetic check; their flags remain explicit, not misrepresented as large swings. There are no unsupported absolute first/largest-error claims when earlier or linked uncertainty remains.

Pair **06/1** provides a particularly important distinction. After actual 50.Bf3 Bd5 (p98/p99), the native controller makes White's valid threefold claim with intended **Bf3–g4**. That intended move is **not played**. Matched 20M analysis assigns favorable expectation to both Bf3 and the discovered alternative Bf5; it does not establish a comparative Bf3 error. A player in ordinary Chess may decline a claim, whereas the preregistered controller automatically claims. Therefore Stockfish's winning continuations cannot be translated directly into an actual-result recovery/failure label for p97/p98. The native draw is authoritative; this is a study-policy qualification, not evidence of a rules defect. The exact native claim and remaining counterfactual limitation are documented in the [second-half 20M review](evidence/msh161/main/batch-01/review20m-05-09.md).

## What remains a hypothesis

The observed behavior motivates questions about threatened-piece defense, comparing forcing with quiet continuations, active defense, and assessing the consequences of exchanges and pawn breaks. **It does not establish whether search admission, horizon, selectivity, move ordering, evaluation, or interactions caused those decisions.** No interior-search evidence was collected for that purpose in this study. The same behavioral failure can arise from different mechanisms, and the successful examples show that presence or absence of a technique cannot be inferred from one move.

The following are possible **future engineering research questions**, not diagnoses, authorized corrections or a priority order. Each investigation would need a separately scoped task and the original histories; this interim report authorizes no production changes.

| Observed behavior | Possible causes to distinguish—not established causes | Future discriminating investigation |
|---|---|---|
| A concrete threat or forcing continuation is missed | Insufficient effective depth; selective exclusion of a critical line; ordering or evaluation of the resulting position | Trace the actual interior admission, windows and returned values for played and alternative continuations. Under matched budgets, compare one controlled search-policy change at a time with independently specified tactical consequences. |
| An attack or favorable position is released by a quiet move, exchange or pawn break | Horizon limitations; undervaluing sustained pressure or counterplay; failure to compare the strongest defensive reply | Rank meaningful intermediate positions and examine plausible defenses before and after the exchange/break. Separate evaluation-only comparisons from search-policy experiments; do not force a preferred move. |
| Active defense is missed or an inferior ending is chosen | Inaccurate endgame judgment; insufficient coverage of checking/blockade resources; poor valuation of activity relative to material | Establish defensible continuation and ending outcomes, then compare king/rook activity and pawn-race consequences with legal full-history analysis. Use exact small-ending evidence only where available and compatible with claim policy. |
| A strong first tactical move is found but its continuation fails | Different branch coverage at subsequent roots; unstable advantage estimates; inadequate consequence valuation | Follow the whole opportunity-response episode across actual roots, recording what was searched and valued. Include successful episodes as controls so the explanation predicts both success and failure. |
| Native outcomes differ from favorable reference continuations around repetition | Automatic-claim semantics rather than playing judgment; history-sensitive reference limitations | Replay the native claim event and compare explicitly defined claim-policy counterfactuals. Keep the actual result authoritative and avoid treating an intended claim move as a played move. |

Preserve this assessment, all superseded evidence and resource receipts as the exploratory Mac handoff. Batch 02 has completed, including its 500k screening and 5M confirmations, and local execution is stopped; no batch 03 was launched. Its provisional addendum remains separate from batch 01’s 20M quantitative findings. The primary cloud study needs its own concrete frozen execution/provenance record; it should not silently merge hardware/time-condition strata or relabel these exploratory observations as a completed 200-game diagnosis. No final ranking, engineering change or strength conclusion is proposed here.


## Completed batch02 addendum — provisional5M evidence,12 November2026

The [independent offline audit](evidence/msh161/main/batch-02/verification-review.md)
verified20additional complete native games and8,243successful reference queries:
6,040initial queries cover2,732played and288prepared-prefix decisions;2,203singleton
5Mqueries cover all1,249flagged followups and202fixed-hash audit decisions after
same-choice deduplication. No stable significant missed-error audit crossing was
found. This is completed computational coverage, not proof that all judgments are
resolved. No20Mqueries were run forbatch02, and no final episode adjudication or
absolute first/largest labels are claimed.

Batch02 scored11W/3D/6L (prepared9W/3D/6L over18games,unaided2W). Combined Mac
main outcomes are21W/5D/14L over40games; prepared17W/5D/14L over36,unaided4W.
These hardware-specific exploratory outcomes are neither a rating nor40completed
observations in the future standardized cloudprimary200. Across both main batches,
15,965reference attempts are preserved, including batch01's144at20M; qualification
resources and outcomes remain separate.

Two independent reviewers examined full histories and existing analysis for
[pairs00–04](evidence/msh161/main/batch-02/migration-review-00-04.md) and
[pairs05–08/unaided](evidence/msh161/main/batch-02/migration-review-05-09.md).
The additional observations resemble earlier behavioral questions, but remain
**5Mprovisional**, with flags, alternative moves and related episodes preserved:

- Opportunity and attack preservation: batch02 pair00/game0 p44/p46 occurs within
  one opportunity sequence before an actual win; pair08/game0 p26 Nxb7 versus h4
  raises another attack-versus-pawn-collection question. Neither extends the earlier
  eight-of20 count or establishes an evaluation mechanism.
- Active defense and ending choices: pair04/game0 p90 Rc5 versus Rc8 and p108 Re8
  versus Rh8 are separate potential recovery episodes; pair05/game0 p64 Rxe5 versus
  fxe5 and p98 Qc3+ versus a4 involve two pawn-ending transitions separated by an
  opponent-created reprieve. Do not count downstream moves as independent causes.
- Concrete defensive resources: pair07/game0 p76 Bxb5 allows the actual
  ...Qc1+ Bf1 f3 sequence; pair07/game1 p31 ...Bxh3 versus ...Nf6 raises a defense
  and counterplay question. Both await consequential adjudication; these are not
  newly certified20M mistakes.
- Contrary successes: both Sicilian games in pair01 show tactical responses and
  long conversions; pair03/game1 finds ...b4 and central-passer play; pair06/game0
  eventually uses king activity and converts; both unaided games win. A win or a
  panel with no eligible deterioration does not certify flawless play.

The three native draws comprise an Italian dead-position draw and two threefold
claims, including pair05/game1's intended **unplayed Qd2–e2** after218played plies.
Automatic-claim policy again limits direct translation of favorable Stockfish
continuations into actual-result conversion failures. Preserve the native outcomes
and full claim histories. Deferred linked questions are recorded in the reviewers'
JSON ledgers; they are **not executable search authorizations**.

The exact stopping observation found the recorded wrapper/child/native PIDs absent,
the execution lock free, successful settled notification and no batch03. All native
and query cleanup receipts passed audit. No search was interrupted and no new engine
process was launched for this offline review. The completed batch02 archive/manifest,
local executable copies, original freezes and cloud integration gates are indexed
in the [evidence README](evidence/msh161/README.md). Further computation requires the
validated cloud transition; uncertainty is carried forward rather than cleared by
new local searches. Final engineering priorities remain unselected.
