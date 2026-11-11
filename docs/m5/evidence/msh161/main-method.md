# MSH-161 main procedure v1

Freeze this procedure and source manifest before the first main game. Qualification
is separate:20 games, two opponents, four prepared roots plus starts. Independent
`qualification-final-review.md` selects unchanged Oxidation0.6.0: it beat mShogi
3–1 with4draws on prepared positions, punishing errors across3families/both colors;
Goldfish won none. Combined start+prepared Oxidation score is level, so superiority
is provisional and distribution-dependent, not statistically proved or a rating.
Keep this opponent fixed throughout the main study, including unaided starts.

## Sample and execution

90 source-cleared master histories×both colors=180prepared games;10colorpairs from
the normal start=20unaided games. No book for either engine after supplied prefix.
Separate strata in all reports. Repeated starts/trajectories are correlated.
Exclude all4qualification roots from main selection. The source final-admission
ledger freezes90roots, each actual source/history and all balance rejections;
22families have≥4roots, with≥12exact pawn-symmetric,≥10opposite-castled,
≥15closed and≥15open centers under preserved source definitions. Fixed seed SHA
orders roots independently of engine outcomes. Each of10batches has9prepared
pairs+1unaided pair,20games. Complete all200 before ranking priorities; extend
only for a documented remaining coverage/uncertainty question.

Source balance uses aligned nonbound Stockfish reports, |cp|≤60, White E∈[.40,.60],
last-two-distinct-depth E spread≤.025, no directional final same-move bound conflict,
valid history/PV and no truncation. A final-report selection flag remains visible
and alone does not disqualify an otherwise stable finite estimate. Stronger5M
source evidence supersedes500k for a queried root; retain both. This is finite
balance assessment, not proof of soundness. Main roots must meet every quota;
selection stops rather than relaxing thresholds or substituting known failures.

Use frozen77101fe candidate/native runner and verified Oxidation binary from
build.json; one worker, default recorded memory, no pondering/strength handicap/TB.
Both receive go movetime5000, native auto-claim adjudication, fresh processes/game,
512total-ply cap(unscored if reached),15s reply and6000s pair watchdogs. Native
replay must pass. All failures/caps/retries remain visible. No concurrent native
matches and reference analysis; shared OS lock enforces serial engine work.
No production changes. Wrapper queues owning chat on success/failure with normal
local access; no timed watcher or guessed wake. Notifications and cleanup failures
have separate durable receipts. Never restart over an inflight marker.

## All-move reference and confirmation

Fixed Stockfish19/Threads1/Hash64/UCI_ShowWDL, clear hash/fresh process per query,
full history. All supplied and played moves from both sides get500knode unrestricted
and singleton-played searches. Prepared prefixes are context, never charged to
competitors. Preserve exact score kind/perspective, WDL, bounds, PV, nodes/depth/time,
rawUCI and independent legalPV checks. Native records remain game/outcome authority.
E=(W+D/2)/1000 is a Stockfish self-play expected-score proxy, not calibrated chance
against Oxidation. Nonbound reports are finite opinions, not proved values or
necessarily completed iterations. No unique-best claim from move disagreement.

Keep v0 summaries and add reviewed screen_v1: expose earlier aligned reports with
all original flags, last-two-depth ranges, and directional final-bound conflicts
(>.025E or>20cp). Never erase negative losses or silently drop uncertain rows.
At each completed batch automatically request matched5M singleton comparisons for
played and initial alternative on all v1followup rows(rawloss≥.05,uncertain,
negative or unavailable), deduplicating identical moves. Missing alternatives stay
explicitly unresolved pending5M unrestricted discovery. Additionally fixed-hash
sample≥10% of otherwise stable subthreshold played decisions within each
game/player/phase cell(at least1 if nonempty). Add proposed strong-play examples
and favorable-episode boundary decisions during batch review if not already tested.

Stable finite support requires legal aligned nonbound reports, no transport/hash/
truncation defect, last-two E spread≤.025 and no conflicting final bound. Retain
stale-report flags. Compare minimum alternative E to maximum played E across those
observed reports as sensitivity, not a statistical lower bound. Severity bands:
.05significant/.10serious/.20major; report .04/.06significance sensitivity.
20Mmatched checks are required for unresolved/threshold-straddling consequential
claims and final-report first/largest mistakes, plus legal explanatory continuations
and plausible defenses. Persisting ambiguity remains unresolved; never repeat until
a desired label appears. Tablebase exactness is separate with halfmove/history/
claim caveats. No giant TB download is authorized by this method.

Every played decision gets a disposition: supported finite event, supported
subthreshold, same-choice control, already-lost secondary, or unresolved. Initial
followup and audit cannot by themselves certify events. If audit reveals missed
important errors, expand that stratum and record a versioned method change before
ranking. An earliest confirmed error is not called first if earlier uncertainty
remains. A supplied disadvantage is recorded independently at its baseline root.

## Phases, events and categories

Use phase_tags on full accepted histories. Opening boundary: first full-move
boundary with≤1original minor still on home square per side and both kings disposed
(castled in actual history, no castling rights, or both queens absent), persisting
at the following2full-move boundaries. Captured originals no longer await development;
returned originals count as home. Imported unknown history leaves opening unresolved.
Endgame material≤26points(Q9/R5/B3/N3,total bothsides) overrides other phase tags;
otherwise after opening boundary is middlegame. Preserve mechanical indicators,
unresolved boundary and reasoned alternative tags; report boundary sensitivity.
This convention is descriptive, not a claim about safety or evaluation.

Link events by game/ply/fullhistory and related episode. Confirm opponent errors
and candidate responses before naming opportunity, missed chance or recovery;
recovery requires opponent-created renewed chances. Adjacent-root swings alone
are insufficient. Favorable episodes: candidate E≥.75 on2successive candidate
turns with stable evidence; track until lost/end/unresolved. E≤.10stable is an
operational already-lost marker, not proof. Later errors in such episodes are
secondary until an opponent error restores chances. Do not double-count overlapping
themes or correlated moves as independent causes. Actual results measure conversion;
strong finite advantage is distinct from demonstrably winning exact evidence.

Candidate categories: development/opening choices; king safety; tactical calculation;
quiet defense; coordination/activity; pawn structures/breaks; planning; exchanges;
opportunity response; advantage preservation/conversion; endgame decisions. Analysts
may add behavior categories with explicit examples; no code mechanism follows from
an error. Record frequency by affectedgames AND moves, severity, phases/families/
colors/results, representative legal continuations, contrary successes and uncertainty.

## Review, inference and deliverable

Independent gameplay analysts inspect completed batches without earlier engineering
explanations. Review after each20games: quality/fairness, category discovery and
cumulative affectedgames/severity/families, uncertainty and successes. Do not stop
at plausible early themes. Rankings use recurrent consequential episodes, severity
and breadth separately; no fabricated universal frequency×severity metric.
Use prepared-opening pairs as minimum inferential cluster; stratified pair bootstrap
within families plus family sensitivity and unresolved-case sensitivity. Purposive
source coverage is not a random Chess population. Keep unaided-repeat statistics
separate. Report actualW/D/L and paired uncertainty without Elo claims.

Final independent challenge must separate observed behavior, possible technical
cause, discriminating investigation and potential general correction. Preserve full
games/source/reference evidence and resource totals, reuse benchmark only as support,
update roadmap and stop for John before engineering. Routine studyanalysisLevel1;
actual local process/notification checksLevel2macOS. No new GUI/Linux/Windows claim.
Compute estimates are planning only: games roughly28–45h depending length, plus
potentially substantial allmove/confirmation work. Owner authorized the necessary
study; record actuals, use perprocess bounds and do not truncate scientific coverage.
