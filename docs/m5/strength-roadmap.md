# Conventional Chess: evidence-driven engineering roadmap

John broadly approved MSH-154 (Chess Adversarial Assessment), then required
MSH-155 (Chess Time Scaling) before substantial implementation. The [time-scaling
report](chess-time-scaling.md) is complete and returns for owner review. **The next
engineering round below remains proposed; no substantial restructuring is authorized.**

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

## Proposed round — Useful search and efficient native execution together

These are complementary workstreams, not a sequence in which a large kernel rewrite
must finish before decision diagnosis can begin.

| Workstream | Problem and supporting evidence | Approach, benefit and competing explanation | Completion evidence |
| --- | --- | --- | --- |
| Continuation coverage and classical judgment | Nf5/Rf6 choices remain weak at the sampled two/three seconds; some supported decisions change with budget, but seeing replies in a PV never proves their consequences were properly valued. | Trace candidate and best-defense continuations with matched remaining horizons and bound-aware comparisons. Vary one relevant search mechanism or endpoint term only when it distinguishes coverage from valuation. Expected benefit is earlier sound choices; the dominant cause is still unresolved. | Better supported decisions or earlier attainment across a balanced set, plus fresh holdouts; preserve exact WDL/mate controls and credible alternatives. Retain adverse cases and equal-time costs. No fixture-specific bonuses or success measured only by score agreement. |
| Native search execution | MSH-154 profiles show substantial geometry/transition/evaluation/history work outside visits, and high claim-processing costs in relevant histories. The obsolete-repetition defect is real but narrow and absent at the two weakness roots. | Correct exact costs where measured. Prototype rules-owned checked transition reuse, changed-cell deltas and reversible search summaries alongside intact canonical history. Specialized internal state is permitted if profiling supports it. Expected speed scale and strength benefit are unknown; insufficient coverage/evaluation may still dominate. | Exact legal/state/history/outcome/restoration equivalence and cancellation bounds; fixed-traversal scores/work agree where appropriate. Repeatable total-cost gains on affected histories with no material regression on controls and equal-time decision non-regression. Reject a broad representation rewrite unless the bounded prototype justifies its complexity. |

A small execution gain must not be sold as sufficient to recover five-second
choices at one second. Conversely an exact cost repair need not change already-good
moves to be worthwhile. Preserve the distinction between execution improvement,
useful search efficiency and playing strength. A high unfinished-iteration fraction
is not automatically waste or permission to use partial-root scores.

TT/ordering/frontier changes follow actual opportunity evidence: distinguish safe
board-only move hints from exact history-dependent bounds; don't assume larger TT,
more pruning or broader quiescence is inherently stronger. Specialized native
state retains one authoritative legal transition/history/outcome implementation.
Compact changes have significant special-move/history regression risk; safe Rust,
existing conformance controls and independent review remain required.

**MSH-153 (Classical Compensation Calibration) stays To Do/on hold.** Its historical
criteria remain intact. Activate/refine it only if attribution justifies valuation
work; otherwise adjust the work sequence before tuning. No new queue of isolated
mistake tickets is created by this diagnosis.

**Round gate:** define practical thinking-time objectives and independent decision
outcomes before implementation, then use a coherent source checkpoint for combined
verification. If the round does not produce useful cost/decision benefits, revisit
the explanation and strategy instead of automatically creating more local fixes.
Ordinary scalar native work uses Level 1 verification; broader platforms follow
actual implementation risk, not an automatic unrelated GUI campaign.

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

**Gate:** John approves a substantial campaign's question, sample/stopping rule and
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
The next action is owner review of MSH-155 (Chess Time Scaling), not automatic
selection of an implementation ticket.
