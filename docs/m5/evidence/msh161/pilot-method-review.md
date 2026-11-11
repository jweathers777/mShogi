# Pilot method review — MSH-161 (Competitive Gameplay Diagnosis)

Independent reviewer: execution_review; 2026-11-11. This is a proposal for the
method freeze, not a completed gameplay diagnosis. No engines ran for this review.
Read both completed qualification summaries, selected raw query reports and the
source-screen coverage. The initial four unaided games gave mShogi three wins and
one draw. Neither reference is established as stronger at five seconds. The fixed
extension on four contrasting balanced master roots, both references and both
colors (16 games), is an appropriate limited next qualification. Keep it separate
from the main study; choose a reference only after reviewing resistance and real
opportunities, and preserve uncertainty if neither proves suitable.

## Screening must retain the missing half

The initial640 decisions have328 usable v0 comparisons and312 uncertain ones.
637 decisions nevertheless have aligned earlier nonbound numerical reports.
Their raw differences cross.05 in59 cases: only8 appear in v0's usable screens;
**51 more are uncertain**. Restricting review to v0 flags would select on analysis
behavior and could miss most consequential events. All640 decisions stay in the
denominator. Do not describe a game with no usable flag as error-free.

`screen_v1.py` creates a separate, non-overwriting interpretation of existing
receipts. It changes no analysis inputs, raw outputs or v0 summary. It exposes
aligned earlier nonbound estimates with every original uncertainty flag, the last
two distinct-depth reports, their expected-score range, and directional conflict
with the final same-move bound. Every raw loss>=.05, every negative difference,
and every uncertain/missing comparison enters follow-up. No row is confirmed.
Initial qualification outputs contain59 raw threshold crossings and60 negative
raw differences; negatives are analysis inconsistency, not improvement credit.

Nonbound UCI reports do not prove completed iterations. The last-two-report
spread<=.025 is a reproducible stability heuristic, **not** a confidence interval.
A final same-move upper bound materially below the older estimate, or lower bound
above it, contradicts numerical stability even if two earlier reports agree.
The script flags differences>.025 in expected score or>20cp; differing mate/score
kinds require review. These are depth-evidence conflicts, not mathematical bounds
on an earlier depth's value. Missing aligned reports stay unresolved. Earlier
estimates may screen a root but never erase the recorded final bounds.

Offline checks passed on both completed analysis directories: all640 decisions
remain, all threshold/negative rows require follow-up, and all rows are unconfirmed.
A synthetic stable-earlier-report/final-conflicting-bound case correctly retains
both directional conflicts and its stale-report flag. No new engine computation.

## Confirmation gates before ranked findings

Freeze these rules before using the study to rank weaknesses:

1. **Discovery:** for each queued decision preserve the played move, existing
   unrestricted alternatives, full root/history, score perspective and selection
   reason. Use a5M-node unrestricted discovery query if existing alternatives are
   insufficient or the apparent best move changed. The current fixed pilot panel
   tests this method; it does not discharge every uncertain study decision.
2. **Matched comparisons:** independently cleared hash, same fixed Stockfish,
   options/history and5M-node singleton searches for played and candidate moves.
   When both are the same move, record that agreement; it does not prove global
   optimality. Record actual work/depth rather than pretending every node budget
   completed the same depth.
3. **Finite support:** require legal continuations, aligned nonbound WDL and
   score reports for each compared move, and no transport/truncation/identity
   defect. Last-two-report spread<=.025 and no directional final-bound conflict
   permit a stable finite opinion even when the stale-report flag remains. Keep
   that flag visible. Use the minimum alternative estimate minus maximum played
   estimate over the two-report ranges as a conservative sensitivity check:
   if this is>=.05, the significant-deterioration screen is stable under those
   observed reports. This is not a statistical or game-theoretic lower bound.
4. **Escalation:**20M-node matched comparisons for unstable/conflicting values,
   threshold-straddling cases, alleged result-changing errors and the report's
   first/largest consequential mistakes. Discover/check an additional plausible
   defense where needed; a short preferred PV alone does not explain the cost.
   If uncertainty persists, retain an unresolved event. Never repeat until a
   preferred label appears. Exact tablebase evidence is separate and must retain
   halfmove/history/claim qualifications.
5. **Coverage:** every uncertain initial row receives explicit follow-up
   disposition: stable finite comparison, confirmed event, below-threshold finite
   evidence, already-lost secondary diagnostic, or unresolved. Do not silently
   discard uncertain or negative rows. Prioritize large raw swings and their
   surrounding opponent-response sequences; same-history duplicate queries can
   reuse verified results. A below-threshold label is not proof of best play.
6. **Nonflagged audit:** before freeze, select by fixed hash at least10% of otherwise
   stable subthreshold played decisions, stratified by player and phase with at
   least one per game, for the same matched5M procedure. Add every proposed
   strong-play example and every transition into/out of a favorable episode.
   For an alleged first error, all earlier candidate decisions must have a
   supported below-threshold disposition or the report must say **earliest
   confirmed error; earlier uncertainty remains**. If audit discoveries show
   material missed events, expand that stratum's confirmation and preserve the
   method revision before ranking. The audit is not a license to ignore the
   uncertain half.

Preserve operational severity bands.05/.10/.20 expected-score loss; also report
sensitivity at.04/.06 for significant events and retain cp/mate evidence. Stable
finite opinions are not proofs or calibrated probabilities against the opponent.
No unique-best-move claim follows merely from preference disagreement.

## Phase rule to freeze and pilot-check

Use a board/history rule rather than a universal move number. One workable rule:
opening ends at the first position where each side has at most one original minor
piece still on its original home square, and each king has a resolved disposition
(castled, no remaining castling rights, or both queens gone). Require the condition
at the next two full-move boundaries to avoid temporary excursions; record the
first qualifying boundary. Captured original minors count as no longer awaiting
development; a returned original minor on its home square still counts as home.
This rule describes development, not safety or soundness. Record indicators and
review ambiguous uncastled/blocked-development cases without seeing severity
rankings. Preserve both mechanical boundary and any reasoned alternative, and
report boundary-sensitive events separately rather than moving boundaries to fit
an explanation.

For a reproducible endgame tag, use total nonpawn, nonking material<=26 conventional
points (Q9/R5/B3/N3); report material subclasses as well. This is a descriptive
phase convention, not an evaluation feature. Endgame takes precedence over opening
when early simplification meets it. Otherwise positions after the opening boundary
are middlegame. Pilot-check examples before adopting either rule. Prepared prefixes
are supplied context, never candidate decisions; baseline advantage is measured
at the supplied root even when that root remains in the opening phase.

## Linked events and consequences

Give each event a game/ply/history identity and each related sequence an episode
ID. Confirm both players' choices before attributing an opportunity. An opponent
error must improve the candidate's available outcome; candidate response analysis
then determines retained gain, missed opportunity or recovery. Do not use an
adjacent-root score change alone: work, root normalization and horizon changed.
Confirm the opponent's played/alternative moves and the candidate's actual/better
responses with their proper full histories. A recovery requires this opponent-error
sequence; it is not merely an increasing candidate evaluation.

Use provisional favorable episodes when candidate-perspective E>=.75 on two
successive candidate turns with stable finite evidence. Track every subsequent
candidate decision until advantage disappears, the game ends, or evidence becomes
unresolved. E<=.10 with stable evidence is an operational already-lost marker,
not proof of a forced loss. Keep later mistakes as secondary diagnostics unless
an opponent error genuinely restored chances. Only exact/proved outcomes justify
"demonstrably winning"; otherwise say "strongly favorable finite assessment".
Actual win/draw/loss determines the episode's observed conversion outcome, and
continued resistance/opponent mistakes qualify its interpretation. Link repeated
errors within one episode; do not sum overlapping theme damage into several
independent causes of defeat.

## Ranking and stopping

For each weakness report affected games/all games, affected moves/all relevant
moves, unresolved games/moves, severity distribution, consequential episodes,
phase/color/family/result coverage, representative continuations and contrary
successes. Separate frequency from severity; do not impose a fabricated universal
product of the two. Prioritize recurring consequential behavior across families,
while reporting rare catastrophic cases separately. Keep observed behavior,
possible mechanisms and discriminating technical investigations distinct.

Prepared pairs are the minimum inferential cluster; family dependence also matters.
Use stratified opening-pair bootstrap within families for descriptive uncertainty
and a family-level sensitivity analysis. This purposive sample is not a random
population, so intervals do not confer universal prevalence. Unaided repeats are
separate: report unique trajectories, repeated-start dependence and run-to-run
variation; do not pool them as20 independent opening families. Show unresolved-case
sensitivity, including how plausible reassignment could change priority tiers.

Review after20–30 main games with a cumulative category-discovery/affected-game/
severity/family table. Complete the planned broad coverage despite early plausible
weaknesses. Extend only for a named coverage or unstable-ranking question, with a
specified additional sample and stopping condition. The current pilot/source work
has not completed the study or selected an engineering priority.
