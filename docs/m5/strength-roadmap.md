# Conventional Chess: proposed next engineering round

**Awaiting John’s review and approval.** MSH-154 (Chess Adversarial Assessment),
2026-11-10, supersedes automatic selection of MSH-153 (Classical Compensation
Calibration). Read the [assessment](chess-adversarial-assessment.md) for evidence,
competing explanations and limits. This is a forward plan, not authorization to
begin substantial implementation.

The objective is better practical decisions at comparable thinking time. Faster
execution, deeper nominal search, algorithm presence and changed static scores
are intermediate measurements, not substitutes for that objective. Existing
conditional rating studies do not establish a fixed 1900 ceiling or a reliable
change between versions.

## Stage 1 — Establish decision outcomes and remove measured execution waste

Freeze a balanced diagnostic set with complete histories and explicitly justified
outcomes: forcing tactics, quiet defense, compensation, pawn races and conversion,
plus successes and genuinely fresh holdouts. Separate proof/tablebase outcomes
from finite reference opinions. Preserve the current source and binaries as the
comparison point. Continue locating best-defense consequences of the two current
reproduced suspect choices; do not wait for a kernel rewrite to do this.

In parallel, correct demonstrated exact-cost problems inside the native rules
boundary. The repeated-history predicate is one narrow candidate, not the overall
explanation. Optimize semantically necessary claim handling as well as obsolete
history checks where profiles support it. Profile-guided prototypes may reuse
checked transition effects, changed-cell deltas, attack information and reversible
search summaries; complete canonical history remains intact. A specialized internal
native search state is permissible. A second approximate legality/history engine
is not.

**Gate:** exact legal actions, state/history/outcomes, restoration and cancellation
remain equivalent; fixed-traversal score/work results agree where applicable.
Demonstrate repeatable total-cost improvement on targeted affected histories and
no material regression on unaffected controls, with decision non-regression at equal time. Include captures, promotions,
castling, en passant, counter boundaries, intended claims and live repetitions.
Reject a broad representation rewrite unless measured costs justify its complexity.
Local Level 1 correctness and independent review normally suffice.

**Benefit/confidence/risk:** measured local overhead offers high-confidence cost
opportunities; broad throughput scale and playing-strength benefit are unknown.
Compact native deltas have medium-to-high implementation complexity and high
history/special-move regression risk. Begin with a bounded interchangeable prototype,
not a precommitted wholesale rewrite.

## Stage 2 — Improve continuation coverage and classical valuation as one round

Use Stage 1’s outcome definitions and source baseline. Investigate why the candidate
and best-defense continuations receive their backed-up values; a move appearing in
a PV is insufficient. Separate missing coverage, selective bounds, unresolved
frontier and wrong quiet-endpoint ordering through matched-horizon continuation
comparisons and one-factor controls.

Select coherent changes from those findings: frontier work admission and reuse,
low-material search policy, ordering of expensive branches, and classical endpoint
valuation. TT changes follow measured reusable-opportunity evidence; distinguish
board-only legal move hints from exact history-safe score bounds. Do not assume a
larger table, more pruning, or a positional bonus is the answer.

MSH-153 (Classical Compensation Calibration) is **To Do and held for this review**.
Its existing criteria and historical evidence remain intact. If valuation is the
limiting mechanism, refine it as part of this stage; if coverage dominates, change
the work sequence before implementing compensation tuning. Reject special cases
for the motivating positions.

**Gate:** materially better decision outcomes or faster attainment of justified
outcomes across more than the motivating roots, retaining successes and fresh
holdouts at comparable time. Report contrary cases, iteration completion and CPU
cost; score alignment alone does not pass. Independent review must challenge the
causal claim as well as correctness. Stop the round when the predeclared outcomes
are met or evidence redirects the mechanism; avoid an endless chain of tiny fixes.

**Benefit/confidence/risk:** current time-sensitive choices give medium-confidence
reason to pursue useful coverage; classical calibration remains a live alternative.
Neither mechanism’s strength contribution is quantified. Search/evaluation changes
interact and may regress quiet defense or zugzwang; use outcome-based counterexamples,
not algorithm-presence checklists.

## Stage 3 — Test the combined round against its predecessor

Only after meaningful Stage 1/2 progress, propose a controlled earlier/current
comparison with common fresh openings, paired colours, matching time and resource
conditions and comparable opponents. Define the effect of interest, uncertainty,
opening dependence, stopping rule and resource/artifact bounds before games. Choose
the sample to answer that question; do not repeatedly estimate absolute strength
from a handful of different openings.

**Gate:** John approves substantial study scope/resources before execution. Report
both positive and negative findings; an inconclusive result remains inconclusive.
Routine local development matches retain the cumulative 15-minute allowance per
task, including failures/concurrent runs. This roadmap authorizes no new tournament.

**Benefit/confidence/risk:** this measures competitive transfer rather than assuming
it. Statistical sensitivity depends on effect and sample; no fixed Elo target or
unapproved campaign size is imposed.

## Boundaries and preserved work

Native Rust rules, transitions, complete histories, claims and outcomes remain
authoritative. Specialized internal state, caching and incremental operations must
be demonstrably consistent with that authority. Safe code is the default. No learned
evaluation or Shogi/Xiangqi transfer is authorized; all six games remain required.
Player thinking-time configuration remains separate MSH-140 (Player Thinking Time).
Broader platform compatibility is checked at implementation/integration risks,
not through an unrelated packaged-GUI campaign for this diagnostic work.

Reuse the [experiment tools](experiments.md) and [risk-based verification policy](../testing/verification-levels.md).
New and active tasks use concise **Problem / Objective / Approach / Completion**
descriptions, with detailed conditions and receipts in linked evidence. No new
parallel tracking system is introduced.

The [previous roadmap](strength-roadmap-history-2026-11-09.md) preserves the complete
historical sequence and superseded decisions. Completed engineering and assessment
evidence remain valid for their recorded sources. The current [assessment](chess-adversarial-assessment.md)
is the decision checkpoint; no next implementation task is automatically eligible
until John reviews this direction.
