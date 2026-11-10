# Independent Chess decision benchmark protocol

MSH-156 (Independent Chess Benchmark) precedes MSH-157 (Conventional Decision
Improvement). Owner request: [record](evidence/msh156/owner-request.txt).
The old [eight-position panel](chess-decision-benchmark.md) is regression-only.

## Selection and separation

Target 240 roots: 96 public puzzles across difficulty and motifs, 60 STS positions,
36 public multi-PV evaluations and 48 tablebase endgames. Freeze source-derived
expectations, deterministic selection and lineage groups before native baseline
search. A source-access shortfall may change these counts with a recorded reason,
never because the current engine performs too well or badly. Prefix downloads are
bounded and not a uniform sample of the full databases. Published tests may carry
unknown historical engine-tuning bias; this is independent of current development,
not a claim of globally unseen chess material.

Use approximately 160 development and 80 held-out roots. Keep source game,
transposition, mirrored/color-reversed equivalent, known solution-continuation and
composition families in one partition. Distinct source IDs alone are insufficient.
Retain residual unknown-lineage risk for FEN-only source rows. A data custodian
validates held-out facts; builders receive aggregate metadata only until the
combined candidate and comparison rubric are frozen. Do not use held-out engine
results for choice of algorithms, constants or stopping. If a case becomes a
motivating example, promote its whole related family to development and obtain
independent replacement material before another validation claim; retain original
first-pass results and identifiers.

## Expectation tiers and interpretation

Exact mate proofs and complete Syzygy move maps define their stated outcomes.
Published puzzle solutions and STS points are preferences unless separately proved.
Finite multi-PV scores/PVs are opinions under their recorded resources and state.
Never grade an unlisted move as wrong solely because the source did not analyze it.
Unknown alternatives remain unresolved until independent continuation evidence is
available. Exact WDL preservation is not proof of conversion quality or clocked-game
strength; DTZ is not mate distance. Preserve counters when supplied; omitted clocks
are explicit experimental assumptions, never recovered history. FEN imports do not
create preceding repetition history. Use available source games without inventing
missing moves; retain complete known origins/history in native admission.

Report exact outcome changes, published preference changes and finite-reference
judgments separately by category and difficulty. Record improvements, regressions,
unchanged decisions and unresolved alternatives. Do not combine these tiers into a
single accuracy/Elo. Inspect continuations for material judgments; a PV's presence
alone does not prove the relevant consequence was searched or correctly valued.

## Comparison and resource bounds

Freeze predecessor source/binary before production edits. Primary comparison uses
five seconds per root, cold native process/table, one thread, no visit ceiling,
depth safeguard 128 and 4096 TT entries. Use the existing frontier example and
MSH-155 supervisor adapted for configurable inputs/binaries; preserve native root
restoration and legal PV admission assertions. Record completed and incomplete
iterations, work, actual elapsed time, choices, root-side scores and PVs. Nominal
depth and throughput do not establish improved decisions.

One predecessor/candidate pass on 240 roots is 2400 nominal engine-seconds (40
minutes). This bounded checkpoint implements the owner's explicit few-hundred-root
comparison request. Development baseline may precede implementation; held-out
predecessor/candidate runs remain sealed until candidate freeze. Reserve up to
600 additional seconds: 80 for a fixed 40-root stratified one-second predecessor/candidate screen, 300 for adaptive single-factor five/fifteen-second development probes, 160 for changed-alternative reference adjudication and 60 for native cost profiles, including failures. Initial round ceiling
is 3300 aggregate active engine-seconds including an overhead/failure reserve;
stop before exceeding it and reconsider scope rather than silently expanding.
No opponent match or tournament is included. Larger commitments require approval.
Use sequential searches and a shared cross-folder budget registry/lock; ledger UTC start/end, wall/CPU, failures and hashes. Compilation, data retrieval, offline proof construction and correctness tests are separately reported. Final paired runs alternate predecessor/candidate order by fixed root index.

Select at most eight development failures and at least two success controls for
causal diagnosis using authority/severity and distinct mechanisms. Vary relevant
existing pruning controls one at a time at matched completed depth and equal time.
Do not call Exact mode a selective-search control: its frontier also changes.
Focused longer searches require a specific discriminating question. Freeze a
combined candidate once the chosen general corrections pass local checks and
independent review; do not repeatedly tune against held-out results.

## Engineering gates

Level 1 applies to these scalar native/diagnostic changes: independently expected
correctness/history/restoration tests, targeted search tests, source/binary/data
validation, bounded profiles and independent review. Broad Linux/Windows/GUI
campaigns are deferred to a checkpoint whose actual portability/integration risk
requires them; no unperformed platform evidence is claimed.

Execution and continuation-quality work are complementary. A cost repair can be
worth retaining without demonstrating strength, but the round must honestly report
whether broad decisions improved. If benefit is absent or limited to motivating
examples, revise the diagnosis rather than manufacture another sequence of fixes.
MSH-153 (Classical Compensation Calibration) remains held unless evidence selects
valuation. A substantial predecessor/opponent playing-strength campaign requires
separate approval and should consider conventional clocks with increments.

## Frozen finite-alternative adjudication rule

Changed decisions outside an exact rubric are unresolved by default. Choose at most
eight changed development/validation pairs for finite adjudication, prioritizing
exact/published authority and domain diversity then stable ID, never selecting by
which version appears stronger. Analyze both alternatives at the same parent with
identical cold Stockfish settings (one thread, Hash16MB, no tablebases), constrained
root moves, three seconds each; retain bound status and bestmove-aligned PVs. Up to
48 seconds for this first pass, with the remaining 112 seconds reserved for equal
longer paired confirmations or development causal questions. No verdict from
unequal resources or raw score difference alone: record the concrete forcing or
positional continuation mechanism, contrary evidence and finite-opinion status.
Budget exhaustion leaves cases unresolved. Identical moves are unchanged decisions
regardless of nominal depth or reported score. Listed lower STS points indicate
lower published preference only. This procedure cannot turn finite analysis into
a proof or an engine rating.

## Source rights

Lichess database exports are CC0. Syzygy generated-table redistribution is permitted by its author; API results retain attribution and exact context. The STS mirror has an MIT notice, but affirmative upstream fixture permission was not found. Therefore the repository retains STS retrieval identifiers, source hashes and extraction code, with source attribution and the limitation, rather than redistributing those upstream fixture rows. Local execution fixtures are reconstructed from the public pinned source. This does not assert that a mirror license settles upstream rights. See [source research](evidence/msh156/source-research.md).

## Round decision gate (before candidate results)

Call the round a demonstrated broad decision improvement only when supported
favorable changes span more than one decision domain, include genuinely held-out
material, and leave no unexplained exact-outcome regression. Published preference
counts alone do not satisfy that gate. Report adverse cases even when other cases
improve; no mixed-tier aggregate cancels an exact loss. A cost improvement without
this decision evidence is reported as an execution result and an unresolved
better-Chess objective. Stop to reconsider the causal diagnosis if the gate is not
met, rather than tuning against the revealed holdout or launching a tournament.

## Completed checkpoint and exposure overlay

The original frozen 240-root corpus and expectations remain unchanged. The
[completed round](chess-decision-round.md) records full development/held-out results,
quiet timing confirmations and the failed broad-decision-improvement gate.
A subsequent historical source audit conservatively identified the two-root
`near-root-cluster-001` development family as previously exposed (a mirrored
board match with different side/counters). Treat `syzygy-draw-d92cad1caf419688`
and `syzygy-draw-7eb891d11b770c34` as development/regression material in future use.
Do not describe all 240 roots as historically fresh. The whole-family-excluded
238-root sensitivity is reported separately; original measurements are retained.
No held-out matches were found within the explicitly bounded source/M5 evidence
scope. See [the exposure overlay](evidence/msh157/final-review/freshness-review.json)
for identities, scope and limits. Unknown lineage remains an explicit limitation.
