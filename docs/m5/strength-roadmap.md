<a id="current-direction--classical-search-engineering-owner-decision-2026-11-13"></a>

# Current direction — classical search engineering (project decision, 2026-11-13)

Review of MSH-161 (Competitive Gameplay Diagnosis) and its
[independent review](evidence/msh161/cloud-primary/independent-review-fable/review.md)
established this direction. It replaces the investigation order in the next section,
which stays as history.

The main limit is search depth (median 8–9 at five seconds), caused by the search
infrastructure. Work in this order:

1. MSH-163 (Engine Strength Test Harness): speed benchmark, development blunder
   suite, and game tests of candidate against predecessor. Done; see the
   [harness and baseline](strength-harness.md).
2. MSH-164 (Shared Search Core): game-independent core, fast Chess search position,
   large board-keyed transposition table kept for the full game.
   Implemented 2026-11-14, see [shared-search-core.md](shared-search-core.md): depth at
   five seconds 14–16 in the middlegame (before: 8–9); +55 −0 =5 against the predecessor.
3. MSH-165 (Tori Search Core Adoption): prove the shared boundary with a second game.
   Done 2026-11-14: Tori uses the core unchanged; see shared-search-core.md.
4. MSH-166 (Search Selectivity Completion). The combined set reached H1 after
   30 pairs against task-start main (+40 -3 =17), with blunders avoided improving
   from 86 to 98 of 146. See [combined evidence](evidence/msh166/combined/README.md).
   The earlier individual tests were too small to decide; they were not rejections.
5. MSH-167 (Engine Time Management). Done 2026-11-15.
6. MSH-168 (Classical Endgame and King Evaluation). The jointly tuned classical
   evaluation reached H1 (H1=10) after 597 pairs against task-start main
   (+424 -340 =430). Blunders avoided rose from 100 to 112 of 146, including
   31 to 38 of 47 endgames. See [Texel evidence](evidence/msh168/texel/README.md).

Accept engine changes on correctness checks, the blunder suite and game tests, not
on identical search output or single positions.

## Game tests in this direction

These rules apply to each engine-strength task above. Tasks refer to them and do
not repeat them.

- **A game test must be able to decide.** Use the sequential test of the harness
  (`strength-harness.md`) with a maximum that a gain of the expected size can reach:
  the default of 600 pairs, or more. A test that reaches its maximum without a
  decision is "no decision", not a rejection. Record it, then enlarge the test or
  decide on other recorded grounds.
- **Choose H1 before the test starts.** Set H1 near the expected gain: about
  10 Elo for evaluation changes, and 15 Elo when a larger gain is expected, for
  example search changes. A target above the likely gain can leave a useful
  improvement without a decision within the test bound. In MSH-168 (Classical
  Endgame and King Evaluation), the revised set was estimated at about +12.5 Elo
  after 600 pairs and +7 Elo after 1,000 pairs. With H1 = 15, it reached 1,000
  pairs without a decision. Do not change H1 during a test.
- **Cost.** The standing authorization covers these tests: fast time control,
  sequential stopping rule, recorded compute. At 50 ms per move with four workers,
  60 pairs take about three minutes, so 600 pairs take about 30 minutes. The
  earlier cumulative 15-minute match allowance, which the historical sections of
  this file still mention, no longer applies. A campaign of several hours, or a
  rating study against external engines, still needs approval.
- **Time control.** Choose a time control at which the change can act. A
  technique that starts at a depth of 7 needs a search that reaches that depth
  often.
- **Coherent sets first.** Test a set of standard techniques or terms together
  against the frozen predecessor. Test the removal of one part only when the set
  result is weak, or when there is a specific reason to doubt that part. Single
  standard techniques give small gains; each test of one alone is long.
- **Predecessor.** Freeze `main` at the start of the task
  (`python3 -m tools.strength.cli build-engine`). Later tasks use the last
  accepted engine.
- **Approximations.** Pruning and evaluation inside the tree may be approximate
  (design A-03). Do not add work at each node to make a pruned score exact; the
  native rules decide the real game.
- **Other games.** Tori Shogi uses the shared core. A core change must keep the
  Tori tests passing. Tori strength is not a requirement of a Chess task; switch a
  technique off for Tori if it is clearly worse there. The native rules stay the authority
for real games. Weight tuning of handwritten terms by regression on labeled positions
(Texel tuning) is authorized. Neural-network learned evaluation stays later work. MSH-160 (Chess Opening
Quality Assessment) stays held. MSH-153 (Classical Compensation Calibration) is archived
(2026-11-15); MSH-168 covers its subject. Shogi, Xiangqi, Chu and Tenjiku move to the shared
core in later tasks after MSH-165, in this order: Shogi and Xiangqi first, then Chu and
Tenjiku (project decision, 2026-11-15).
The tasks are MSH-180 (Shogi Search Core Adoption), MSH-181 (Xiangqi Search Core
Adoption), MSH-178 (Search Core History Scaling, needed before Chu), MSH-182 (Chu
Search Core Adoption) and MSH-183 (Tenjiku Search Core Adoption). MSH-179 (Clock
and Session for All Games) gives the games on the core a time budget and kept
engine state. MSH-50 to MSH-54 keep the strength criteria of each game.

# Earlier checkpoint — competitive diagnosis complete (superseded 2026-11-13)

MSH-161 (Competitive Gameplay Diagnosis) completed 200 primary cloud games and all
required reference analysis. See the [final diagnosis](competitive-gameplay-diagnosis.md),
[source-bound synthesis](evidence/msh161/cloud-primary/final-analysis/scientific-synthesis-astra.md)
and independent challenge. Results are 82 wins / 58 draws / 60 losses; the 180
prepared games score 52.5% with a conditional whole-pair family-stratified 95%
interval of 47.22–57.78%. Correlated unaided starts remain separate. This is not an
Elo result, proof of reference superiority or a guarantee of rating improvement.

**Stop for review. No engineering or new study is started by this roadmap.**
Original AutoClaim and unchanged native/opponent/image identities were used in all
primary batches. Claims censor conversion observations; draws alone are not failures.
The unsuccessful policy transition remains separate and abandoned.

Recommended practical investigation order, based on reviewed concrete choices:

1. Contrast consequential exchange/capture order, queen restriction, quiet defense
   and coordination failures with successful controls. Trace actual interior search
   admission, windows, TT/reductions/frontier and returned values to distinguish an
   omitted/cut-off continuation from one reached and misvalued. Separate child
   probes or nominal depth do not establish this distinction.
2. Contrast pawn/promotion timing with required king, rook, blocker and liquidation
   support. Preserve histories/claims and test intermediate consequences. The study
   does not select a blanket pawn bonus, tablebase project or search change.
3. Integrate early central/development choices into the same continuation diagnosis,
   coordinated with MSH-160 (Chess Opening Quality Assessment). Reuse this study's
   master-supported prefixes and unaided evidence; no competing corpus or book-first
   project follows from these findings.

These are investigation priorities, not measured category defect-rate rankings or
established mechanisms. Corrected overlapping category counts remain reviewer-
dependent, with 86 unknown supported roots. Concrete sequences, breadth across
families/colors/results, secondary/linked uncertainty and contrary successes govern
interpretation. Three bounded narratives include one independently certified finite
favorable segment; there is no exhaustive conversion-rate claim.

When the next bounded task is selected, define observable consequences, contrasted
failure/success controls and stopping criteria before changes. These exposed games
are development evidence, not fresh held-out material. Retain only general supported
corrections that improve consequence quality across more than one family and protect
success controls. A substantial later predecessor/current competitive comparison
needs separate approval after meaningful coherent progress. No per-change tournament,
learned evaluation or Shogi/Xiangqi work is authorized. MSH-153 (Classical Compensation
Calibration) stays held. Historical roadmap below is preserved, not an active queue.

# Conventional Chess: evidence-driven engineering roadmap

MSH-159 (Classical Evaluation Decisions) completed the focused classical-evaluation
investigation. See the [evaluation report](classical-evaluation-decisions.md).
The experimental general pawn-race and bishop-restriction corrections improved
both motivating five-second choices and established useful static weaknesses.
The wider frozen comparison also found two material strategic decision regressions,
so independent review rejected the entire candidate. Production remains the
verified MSH-158 (Chess Decision Corrections) predecessor; no alternative component
was selected after held-out exposure. All 80 held-out choices and exact objectives
were preserved, which establishes neither transfer nor strength improvement.

The next useful bounded question would explain those regressions and separate
feature consequences from time-limited coverage while preserving the established
target benefits. This is a recommendation, not a new engineering assignment.
MSH-153 (Classical Compensation Calibration) stays held: numerical disagreement
and these endgame findings do not select a general material-compensation change.
No further workstream or tournament is started by this completion.

The preceding MSH-158 checkpoint follows for historical context.

MSH-158 (Chess Decision Corrections) completes the requested focused round
following MSH-157 (Conventional Decision Improvement). See the
[correction report](chess-decision-corrections.md). The retained general fix preserves
a detected mating-threat extension across full-window verification: Qc8 replaces
Qf4 at five seconds with independent continuation support. The other 159 development,
all 80 held-out and all 40 fast-screen choices are unchanged; exact outcomes are
preserved. This supports one local decision improvement, not broad transfer or a
strength claim. The king-route/pawn-race and bishop-blockade failures remain open.

Four alternative trials were rejected. MSH-153 (Classical Compensation Calibration)
remains held; the evidence does not select a general compensation adjustment.
No further engineering queue or competitive study is started by this completion.
The sections below preserve the preceding roadmap and its reasoning. Major
architecture and substantial tournaments still require separate approval.

The objective is a demonstrably strong conventional engine at meaningful practical
thinking times. The one-second conditional local index is not a general rating or
fixed strength ceiling. Algorithm presence, nominal depth, speed and isolated score
agreement are not substitutes for good decisions or competitive results.

## Initial checkpoint — Time scaling and a stable benchmark

**Problem:** earlier one-second assessments could not show longer-time capability
or whether opponents benefit equally. **Evidence:** the frozen eight-root matrix
finds two historical mShogi weaknesses repaired at five seconds, still weak at the
sampled two/three seconds; fifteen-second alternatives remain defensible. Fresh
mate/tablebase controls are already solved at one second. References scale unevenly;
selected thirty-second differences persist. No relative match strength is measured.

**Completed outcome:** the [small decision benchmark](chess-decision-benchmark.md)
contains one fresh position per category, explicit proof/opinion tiers, complete
known histories and multiple acceptable choices. Preserved raw records distinguish
elapsed time, completed versus incomplete native iterations and outcome quality.
No supported regression appeared in this panel; it is small, selected and partly
ceiling-limited. These fresh positions now become regression controls, so later
transfer claims need additional independent holdouts.

**Implication:** do not commit to a major rewrite, compensate every error with
static bonuses, or infer a higher longer-time rating. Use the evidence to choose a
coherent round with a practical budget/outcome objective and observable alternatives.

## Completed coherent round — Cost gains without broad decision transfer

MSH-156 (Independent Chess Benchmark) froze 240 public-source positions and their
expectation tiers before production changes, with160 development / 80 held-out roots.
MSH-157 (Conventional Decision Improvement) completed the two complementary
workstreams and the full practical-time checkpoint. See the
[round report](chess-decision-round.md) and [protocol](independent-chess-benchmark.md).

Two exact native corrections remove obsolete-repetition claim work and reuse one
authoritative checked projection across intended claim reasons. Selected affected
histories show roughly 3.9–4.0 times as many visits at one second; fixed traversal
semantics remain identical. That benefit is worth retaining on its own evidence.
The ordinary-history control is only about 1.04 times, and throughput is not strength.

At five seconds,238 of 240decisions are unchanged, including all 80held-out choices.
The two development strategic changes require finite continuation judgments;
published preference membership alone does not establish better play. No broad
held-out transfer was demonstrated. The one-second screen is secondary and the
exact root objectives are ceiling-limited. The round does not satisfy its broad
decision-improvement gate and does not justify a competitive strength campaign.

## Revised diagnosis before further engineering

The development investigation supports weaknesses in long quiet king routes,
pawn-race timing, bishop blockade and quiet queen defense/coordination. It does not
establish population frequency or a single general defect. More time sometimes
finds a better move while the anticipated consequences remain inaccurately valued.
Pruning toggles confound coverage with completed depth; child-root probes also
change TT/window/order and extension context. Neither is enough to select a safe
general pruning or evaluation correction.

Reconsider where the search loses useful continuation information before another
series of local fixes. The discriminating question is the earliest divergence in
coverage or valuation along independently justified quiet continuations, with
comparable remaining search context and explicitly bounded expectations. Trace
intermediate state values and admitted/omitted defenses, rather than treating a
PV match or deeper nominal iteration as success. Use development material;
held-out families exposed for diagnosis must be promoted and replaced before
future independent validation. No production tuning occurred after this round's
candidate freeze.

Keep MSH-153 (Classical Compensation Calibration) on hold: the present evidence
does not causally select a general compensation term. Do not automatically open a
queue of bonuses, global pruning changes, or a wholesale representation rewrite.
The authorized coherent round is complete; this roadmap records the next research
direction, not an unbounded follow-on implementation campaign. A fresh bounded
assignment should state its causal question, stopping rule and desired decision
outcomes before changes. Retain the native authoritative rules/history boundary,
independent regressions and Level 1 risk-based verification for ordinary scalar
engine work.

## Deferred checkpoint — Unaided opening decision quality

MSH-160 (Chess Opening Quality Assessment) preserves a future M5 investigation
of the human-play observation that mShogi finds sound openings without a book.
It remains **To Do**, after the recorded MSH-158 (Chess Decision Corrections)
checkpoint. That task is already complete; before activation, confirm any
then-active classical evaluation/correction work has finished and record its
dependency. Opening research must not interrupt or block those corrections.
MSH-153 (Classical Compensation Calibration) remains held. This is distinct from
MSH-104 (per-game opening-knowledge research), with no requirement to wait for
learned evaluation or introduce opening assistance.

The proposed scope is approximately 100–150 balanced positions across opening
families, colors and development stages, preserving available full histories.
Compare one/five/selected fifteen seconds; distinguish master-game frequency
from independently supported soundness, including unusual alternatives and
middlegame consequences. Consider Lichess Opening Explorer/master games,
Chess Openings/ECO, Position Evaluations and selected independent Stockfish
continuations, subject to source terms, licenses, access limits and data-quality
verification before use. The task records source links and classification scope.
Optional short opponent book-guided sequences must continue legally and soundly
after any deviation; mShogi remains unaided.

**Execution gate:** establish a modest aggregate compute/artifact budget,
process bounds, stopping criteria and frozen protocol with independent review
before running engines. Reuse [existing experiment infrastructure](experiments.md).
This planning record authorizes no engines, large downloads, corpus construction,
new framework, production changes or opening book. It is not a competitive
qualification campaign and does not start automatically at dependency completion.

## Later checkpoint — Controlled competitive measurement

**Problem:** diagnostic roots cannot establish game-level relative strength. Earlier
small absolute-index estimates across different openings did not demonstrate a
reliable improvement. **Approach:** after meaningful engineering progress, propose a
matched predecessor/current comparison with shared fresh openings, paired colours,
comparable opponents, fixed source/configurations and a sample designed for a stated
effect/uncertainty. Include opposing mistakes, conversion and complete-game behavior.

For serious qualification, consider conventional clocks **with increments** and a
declared reference pool whose timing/resource conditions can be reasonably matched.
Record hardware/time-manager/hash differences. Do not combine distinct time controls
into one rating without a suitable model and evidence. Fixed-movetime diagnostics
remain useful for causal questions but cannot validate full-game time allocation.

**Gate:** Explicit approval must cover a substantial campaign's question, sample/stopping rule and
resource/artifact bounds before execution. An inconclusive comparison stays
inconclusive. No tournament is authorized by this roadmap. Routine development
matches retain the cumulative 15-minute allowance per task, including failures;
no per-optimization absolute-rating campaign follows automatically.

## Boundaries and preserved history

No learned evaluation or Shogi/Xiangqi development is authorized; all six games
remain requirements. MSH-140 (Player Thinking Time) remains separate product work;
current diagnostics change no desktop default. Native rules, histories, claims,
complete actions and outcomes remain authoritative. Use [existing experiment tools](experiments.md)
and [risk-based verification](../testing/verification-levels.md), not a second framework.

New and active tasks use concise Problem / Objective / Approach / Completion
sections, with conditions, failures and receipts in linked evidence. The
[MSH-154 roadmap](strength-roadmap-msh154.md) and [earlier history](strength-roadmap-history-2026-11-09.md)
preserve previous direction; completed evidence remains valid for its source.
The completed round froze public-source expectations and a predecessor before
production changes, used five seconds as primary, one second secondary and focused
fifteen-second probes. The original eight roots are regression-only. Held-out
families stay isolated until candidate freeze; exposed families are promoted and
replaced before later validation claims. MSH-157 (Conventional Decision Improvement)
records the failed broad-decision gate and the revised diagnosis above; the useful
exact cost repair is not presented as a strength improvement.
