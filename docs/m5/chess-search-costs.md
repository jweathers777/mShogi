# Bounded Chess search cost checkpoint (MSH-136)

John requested this engineering checkpoint after MSH-49.2 and before conventional
Shogi/Xiangqi transfer. It runs no additional Chess match experiment or strength study.
Four fixed positions use depth8/20,000 visits without a clock; three before/after
rounds compare the same work. A short CPU stack sample identifies major costs.
All profiling producers together executed72 native search queries in27.43 seconds
of recorded query time, including one producer whose sampler was denied by the
tool sandbox. Sampling the same user's own process outside that sandbox worked;
no sudo, security setting change or extra engine was needed.

## Findings and correction

In the initial three-second mixed-position sample, exchange estimation accounted
for about40% of stack weight, native turn queries39%, and native push/undo16%.
Dynamic evaluation was about2%. Quiescence appeared on57% of stacks inclusively;
that overlaps the cost categories rather than adding another57% of work.

The main redundant operation was concrete: each recursive exchange step built
and checked the entire legal-action list, then discarded moves not ending on
the exchange square. The new native `Position::legal_actions_to` projects the
existing native candidates **before** their unchanged checked transitions. It
retains normalization of representation counters, king safety, EP, castling,
promotion choices and deterministic order. It is not an alternate move generator
or an unchecked make-move path. SEE retains its existing bounded legal-attacker
sequence, heuristic meaning and tie order.

The search also now borrows TT entries instead of cloning their PV before even
checking eligibility. Only an actual cutoff clones the stored PV. Static
evaluation and owned action copies follow reusable TT cutoffs; quiescence copies
its actions only after stand-pat needs expansion. Native outcomes and claims
still precede every reuse/pruning decision. Search policy, evaluation arithmetic,
repetition/history keys, node budgets and public execution schemas are unchanged.

The after sample put exchange work near7% and native turn queries near63%.
These are qualitative cost shifts: optimized inlining, different positions within
the short sampling window and concurrent workspace checks during the after
sample prevent precise per-operation timing or causal attribution. The separate
fixed-work timing comparison ran before those broad checks.

## Fixed-work result

| Position | Before median | After median | Interpretation |
| --- | ---: | ---: | --- |
| Standard |444.8ms|379.8ms|About17% faster (1.171×) |
| Developed tactical |844.6ms|423.3ms|About2× faster (1.995×) |
| Simple recapture |164.8ms|164.8ms|Essentially unchanged |
| Sparse endgame |94.1ms|93.2ms|Essentially unchanged; tiny difference is noise-sized |

All12 paired rows are identical in every field except elapsed time: score,
selected decision, full PV, completed/selective depth, node count and all recorded
SEE/TT counters. Input Games restore exactly and all PV actions replay
natively. This supports removal of redundant work, not stronger play, optimality,
a population speed estimate or a guarantee of equal gains in other games.

## Effective TT reuse

| Position | Real main probes | Position matches | Exact history matches | Depth/extension eligible | Cutoffs |
| --- | ---: | ---: | ---: | ---: | ---: |
| Standard |7234|1375|1083|514|409|
| Developed |1367|460|441|287|260|
| Recapture |5900|4106|642|0|0|
| Endgame |4126|1984|462|62|32|

Quiescence has no TT probing and accounts for64–93% of visited nodes here, so
all-node hit percentages obscure the actual eligibility stages. The TT works
where an entry has usable context and depth; a matching position is not permission
to reuse a history-sensitive bound. The recapture example has many position
matches but no depth-compatible exact-context cutoff. Full action-prefix identity
is deliberately conservative and loses ordinary cross-path transpositions. It
remains unchanged. Broader native future-state equivalence, replacement tuning
and quiescence TT reuse need separate evidence; none is required to eliminate
the measured redundant work in this bounded checkpoint.

## Correctness, evidence and transfer

New destination-query expectations follow the selected
[Chess position profile](../rules/chess-position-v1.md), with independently
reviewed fixtures for pins, unsafe king
captures, legal and horizontally pinned EP, promotion captures and castling
through attack. Exact counter-overflow refusal is distinct from position legality.
Every-square projections along up to40 native positions match the complete native list
without mutation. SEE additionally matches the former full-list implementation
on native tactical/EP/promotion/pin actions; existing independently solved SEE,
exhaustive exact-search, history collision, claim, terminal, undo and cancellation
tests remain required. Independent review approved the narrow source and all12
before/after comparisons. Final native regression/platform receipts reside in
the [single verification record](evidence/msh136/verification.json).

Baseline production is `b005f80` with a preserved observational-counter patch and
new diagnostic example. The original unformatted baseline example bytes match
its recorded SHA; rustfmt yields the current example byte-for-byte. Both versions
and the proof are preserved, so formatting is not misreported as the tested
source. Corrected tested/integration revisions are recorded with actual CI in
the verification record. Normal product code introduces no profiling timers.

For **Shogi (MSH-50) and Xiangqi (MSH-54)**:

At this checkpoint, Shogi's `NativeTree` calls `Game::submit/undo`, whose undo
token retains before/after Games and whose submission clones the Game for
atomic admission. Measure that native transaction/history cost early before
scaling search work; it is source-inspection evidence, not a measured Shogi
bottleneck yet. Xiangqi already uses a native checkpoint transaction and has
pending-adjudication, no-win and natural-count obligations before ordinary
search choices. Preserve those obligations when adapting cached turn queries.

- Make search queries express the native subset actually needed before expensive
  legality validation; retain complete native action/effect and adjudication
  ownership. Captures, drops, promotions and game-specific tactical dependencies
  need their own native projections and independently expected fixtures.
- Cache exact turn-boundary native queries. Avoid speculative copies or evaluation
  before a safe cutoff; preserve terminal/claim precedence and cancellation.
- Measure quiescence/exchange work separately from counted search visits. Measure
  TT position, history, depth eligibility and cutoffs separately. A board hash
  alone never replaces repetition, counter or whole-profile context.
- Remaining native turn queries and exact transitions dominate. Use proportionate
  game-specific profiles to decide the next improvement; do not transfer Chess
  constants, prune away rule checks, or prebuild a universal engine framework.

No major owner tradeoff is needed for these corrections. MSH-49.2 remains complete
and unchanged. Shogi/Xiangqi follow this checkpoint; deeper Chess strength
analysis stays in the later learned-evaluation phase.

Required [native CI](https://github.com/jweathers777/mShogi/actions/runs/42519269011)
passed all13 steps on Ubuntu24.04, macOS15 and Windows2022 at exact corrected
source `d9147713aa5ad5044a4e25a41d5142f6834a2685`, attempt1. Unchanged frontend
and packaged GUI evidence is reused. Final documentation/evidence integration
is identified by Git history and the completion report.

Independent final acceptance approved all four MSH-136 criteria and completion
items after reconciling the exact source, all three platform receipts, artifact
hashes and transfer facts. No technical or owner-decision blocker remains.
