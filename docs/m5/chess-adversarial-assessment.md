# Chess engine adversarial assessment

MSH-154 (Chess Adversarial Assessment), 2026-11-10. Diagnostic source `d53f34413d808dc2b727a06a770b92f24191eabf`. **Proposal for owner review; no substantial engine implementation is approved or delivered by this assessment.**

The evidence supports improving the relationship between native execution cost, useful search coverage and classical judgment as a coherent system. It does **not** establish a single cause of a 1900 ceiling. Earlier small, different-opening studies did not establish a gain; they also did not establish that improvement was absent. The latest retained games precede the current source. Avoid treating that conditional local index as a precise present rating.

## What was independent

Architecture/performance used a fresh Astra High context; gameplay used a fresh Sol High context; methodology used a fresh Astra Medium context. Architecture and gameplay reports were frozen before exchanging findings. Gameplay initially saw complete raw match histories, moves, configuration and outcomes, not earlier diagnoses or source explanations. Repository instructions necessarily expose historical owner context; this was a diagnosis firewall, not perfect experimental blinding. The coordinator challenged the reports and the methodology reviewer challenged both experiment design and final synthesis.

All twelve retained games from MSH-151 (Chess Coherent-Round Assessment) were read: 605 candidate decisions, 5 wins/2 draws/5 losses, six games per colour and opponent. This is a complete **selected historical cohort**, not a population sample. Twelve human-selected candidate/control roots were frozen before reference probing. Current-source replays preserve full histories and use release, one thread, fresh 4096-entry TT, time-primary 1s; three discriminating roots also receive 5s. Independent synthetic history controls test a mechanism, not playing strength. No new matches or tournament were run.

## Findings and evidential limits

### Search performs much more work than its node counter shows

The implementation already uses bitboards, native cursor caches and ordinary conventional techniques. Its problem cannot be reduced to an absent algorithm or a naive board array. Source inspection identifies repeated checked projections, candidate resolution, full-board delta scans, feature construction, exact-history key operations and PV allocations. Several of those operations occur outside counted main/quiescence calls.

On the selected current-source panel, reported visits span approximately 93k–466k per second, with quiescence 76.6–94.8% of visits. Quiet projections alone reach 174k–792k per second; the first suspect root also performs 781,746 defensive-reply probes. These are operation counts, not exclusive CPU fractions or cross-engine comparable nodes. A large q fraction alone is not pathological: other engines also spend most nodes at the frontier. The important question is which operations produce useful discrimination.

Counterevidence matters: first moves account for 89.8–95.7% of main beta cutoffs in this panel. This conditional statistic does not establish globally optimal ordering, but it supplies no positive evidence of universal ordering failure. It does not assess costly root/PV/no-cutoff branches. Main TT cutoffs occur on 4.0–8.2% of probes; exact context matches vary considerably. A 4096-entry fresh TT, restrictive history/credit matching and mostly unkeyed recursive q work are real constraints. They do not demonstrate that increasing capacity or loosening bounds would improve play. Board-only bound reuse is not an acceptable shortcut.

Three fresh CPU profiles distinguish the costs. Exclusive top-of-stack samples in the tactical root put native attack geometry at 14.2%, move resolution at 9.7% and the dynamic evaluation body at 9.5%; in a live-repetition endgame, repetition-identity packing accounts for 23.8%, structural validation 17.2% and the claim-catalog body 9.1%. Hash/evaluation delta scans and clock reads also have measurable self time. These are sampled functions, not exact end-to-end category totals: inlining and one profile per root limit attribution. They justify targeted experiments in native reuse and claim admission, not a promised speedup from a wholesale representation rewrite.

Own visit throughput is measured; a fair normalized cross-engine work ratio is not. Goldfish’s local source counts different main/q entry paths and also excludes native operations. Its changed-cell updates and compact state are useful architectural comparisons, but its history/TT semantics cannot become mShogi’s authority. No opponent NPS or depth ratio is converted into a strength claim.

### A real history-dependent cost defect has limited observed prevalence

An occurrence repeated before an irreversible move remains in the complete public history map. The broad repeated-identity predicate then forces expensive claim catalogs and disables previews even though those old positions cannot recur. The native TT already uses a reversible suffix, demonstrating that full archival history and relevant search context need not be identical representations.

Controlled full-history/suffix roots preserve exact board and counters. Two synthetic irreversible-prefix examples retain identical fixed-depth decisions, scores and visits but take roughly 3.6–3.9 times as long at depth2; at 1s they achieve about one-fifth the visits. Nonrepeated controls are near equal. A live-repetition positive control correctly changes a claim to a move if history is discarded: **general history truncation is invalid**.

Offline legal-history replay finds the obsolete-only predicate at 44/605 decisions in 2/12 games, including a win, and at none of the twelve selected roots. A real affected historical root also reaches 98,376 visits/depth 6 with full history versus 423,907/depth 8 with an equivalent preserved reversible suffix at one second; both choose Rxc5. This single exploratory pair supports practical cost relevance, not better play. This is a credible narrow repair within native history semantics. It cannot explain the panel’s decision failures or justify making all subsequent development a repetition optimization campaign.

### Gameplay is mixed, and several attractive explanations do not survive challenge

The independent reviewer found purposeful passer conversion, attacks and tactical play alongside plausible failures in quiet coordination and defensive resource recognition. Reference probes narrowed those impressions rather than confirming all of them:

- `22.Nf5` in the first Oxidation draw and `23...Rf6` in the first Goldfish loss are the strongest candidate deteriorations; current source reproduces both at 1s.
- The apparently dubious `13.Bxg6` sacrifice is preferred by the reference; the proposed retreat fares worse. The opening `9...g5` pawn push is also not condemned by the reference. General anti-sacrifice or anti-pawn-storm bonuses would be poorly grounded.
- `52.Ne2` and the drawn rook ending’s `48...Rxh2` are not supported as errors. `66...Bxa4` and the motivating `27.e4` occur in positions the reference already rates badly; `27.e4` is itself its preferred move. Blaming the last visible exchange confuses consequences with causes.
- A reduced winning score after `34.Rc5` does not establish a lost win. Material advantage and a plausible attacking PV do not prove conversion was available.

These are finite-reference opinions unless a line is independently proved. No new tablebase oracle was queried. Initial reference scripts discarded bound flags, so their score differences are **not exact centipawn losses**; those outputs remain exploratory. A failed Simbelmyne screen was excluded before results were inspected and retained as failure evidence. Bound-aware targeted confirmation is recorded separately.

The bound-aware recheck supports improvement on **both** current-source cases when given five seconds:

| Full-history position | Current 1s choice | Current 5s choice | Independent finite-reference comparison |
| --- | --- | --- | --- |
| White move 22, first Oxidation draw | Nf5, depth 5 | Bf5, depth 7 | Last non-bound reference scores approximately −290 vs +18 cp |
| Black move 23, first Goldfish loss | Rf6, depth 7 | Rad8, depth 8 | Approximately −371 vs −37 cp |

Reference queries used the same 2-million-node ceiling per constrained choice, with full histories, fresh hash, preserved bound flags and legally replayed full PVs. Completed depths differ, so these are finite reference values, not precise centipawn loss or forced WDL. The compensation control keeps Bxg6 at both budgets. The Rf6 lines show why timing matters: immediate Rf6 meets Rh4 with inadequate coordination, while Rad8 first permits a different defensive continuation after Rh4. This is stronger than observing a changed move, but it still does not separate insufficient coverage, selective admission and endpoint valuation. No single-engine opinion is treated as proof.

## Competing explanations and what would distinguish them

| Hypothesis | Supporting evidence | Alternative/falsifier | Smallest next discriminating gate |
| --- | --- | --- | --- |
| Native overhead limits useful search | Repeated native work; high uncounted operation counts; measured history-path cost; two selected decisions improve under finite-reference evaluation with more time | Additional work may still value consequences wrongly; cheap copying need not dominate CPU | Semantics-equivalent native transition/delta prototype on diverse full histories, fixed-work equality plus time-primary decision outcomes and fresh controls |
| Frontier/selectivity does not reliably compare quiet defensive consequences | Two current1s suspect choices; substantial quiet/defence q work; both 5s choices improve reference outcomes | Wrong endpoint evaluation or reference error rather than omitted branch | Trace both candidate moves through best defenses, match remaining horizon, then vary one relevant selectivity mechanism while retaining the frontier; do not use Exact mode as a single-factor control |
| Classical valuation misranks compensation/conversion | Hypothesis motivated by unresolved endpoint judgments; material advantage alone does not establish a sound continuation | Search has not reached a stable endpoint; several proposed bad moves are actually defensible | Independent near-neighbor quiet endpoints with score decomposition and outcome-based continuation checks; require improved root decisions, not cosmetic score alignment |
| TT/ordering is the principal bottleneck | Small fresh table, exact context/credit restrictions, q reuse limited | Strong first-move cutoff shares; capacity cannot cure context mismatch | Measure lost reusable opportunities and time spent in keys first; separate legal board-only hints from exact history-safe bounds; reject broad table sweeps without a causal signal |

The first three explanations interact; the evidence does not yet assign a defensible percentage of playing weakness to each. A numerical Elo benefit cannot be forecast from a microbenchmark, node ratio or a changed move.

## Recommended coherent strategy

See the [forward roadmap](strength-roadmap.md). Proposed priorities are:

1. **Establish decision outcomes and remove measured native execution waste.** Continue best-defense diagnosis immediately, alongside exact-cost work. Fix measured claim/history inefficiencies within native semantics; prototype shared checked effects and changed-cell deltas only where profiles justify them. Preserve full canonical history, special moves, undo and cancellation. Gate exact optimizations on semantic/fixed-traversal equivalence, repeatable total-cost improvement and equal-time decision non-regression. Broad representation changes remain conditional; no second rules engine or guaranteed Elo benefit.
2. **Correct continuation coverage and classical judgment together.** Use a balanced independent panel with held-out tactical, defensive, positional and endgame outcomes; this diagnosis need not wait for a broad native rewrite. Trace two reproduced problems and success controls before altering frontier admission, low-material guards or weights. Integrate related changes as a round and require time-primary decision improvement without regressions. MSH-153 (Classical Compensation Calibration) becomes a held component of this stage, not an automatically selected bonus-tuning task.
3. **Measure the coherent round against its frozen predecessor.** Recommend a controlled earlier/current comparison only after meaningful decision gains: common openings, matched opponents/settings/resources, paired colours and a sample designed for a declared effect and uncertainty. Request approval for substantial resources before running it. Stop repeating tiny absolute-rating estimates after each optimization.

No learned evaluation or Shogi/Xiangqi transfer. All six games remain product requirements. The decisions now needed from John are approval or revision of this staged direction and, later, explicit scope/resources for any substantial matched-version study. No such study is proposed for immediate execution.

## Evidence and limitations

[Architecture initial audit](evidence/msh154/architecture-initial.md), [gameplay initial audit](evidence/msh154/gameplay-initial.md), [gameplay findings](evidence/msh154/gameplay.md), [methodology challenges](evidence/msh154/methodology.md), [measurement record](evidence/msh154/architecture-measurements.md), and [verification](evidence/msh154/verification.json) retain source identities, conditions, failures, scripts and raw observations. The small candidate panel has no independent fresh gameplay holdout; transfer and broad error frequency remain untested. No present rating, strength gain or conventional maturity claim follows.
