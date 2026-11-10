# Independent review: MSH-156 benchmark protocol

Reviewed `/tmp/msh156-protocol.md` and `/tmp/msh156-sources.md` before corpus freeze or native evaluation. Reviewer authored neither corpus nor harness and has not inspected held-out roots/answers or run any engine. Earlier contribution was read-only native-cost planning; held-out isolation remains intact.

## Disposition

The protocol is sound in direction and proportionate to the owner's explicit few-hundred-position request. Approve selection and data validation. Before freezing executable grading and running the comparison, address the concrete conditions below. None requires owner approval or a new framework. No new resource authorization is inferred for tournaments.

### 1. Make finite-opinion grading operational (required)

The written distinction is correct, but the fixture/summary schema must enforce it. An unlisted native move is unresolved, not a failed puzzle/STS/eval case. A listed lower-scored STS move is a weaker published preference, not an exact decision error. Multi-PV scores from unequal depth/bounds cannot establish candidate regret. Same move with altered score/depth is unchanged choice, not improvement.

Freeze a candidate-agnostic continuation adjudication procedure and explicit budget before first comparisons: analyze both predecessor and candidate alternatives under equal reference settings; retain bound status, bestmove/PV alignment, legal continuation and outcome mechanism. Unresolved remains valid if budget is exhausted. Include API/proof/engine evidence separately and do not require every unknown move to be resolved. Count exact improvements/regressions separately from supported finite judgments and changed-but-unresolved decisions. This protects the central better-Chess claim even when most source data consist of preferences rather than truths.

### 2. Exact outcomes require context (required)

For each tablebase item record root and every child category, halfmove clock, DTZ/precision flags and the chosen interpretation. Verify child-to-parent inversion and legal-move-set equality. Exclude cursed/blessed/unknown/rounded ambiguity from binary exact scores unless handled explicitly. A win that merely survives this one move is not converted victory. A FEN-only imported state is an experiment with unknown preceding repetitions; do not imply exact original-game WDL. For puzzle mates, distinguish a published mate line from exhaustive mate proof over all defender replies; allow all independently proved winning alternatives. Preserve source first move versus actual puzzle root semantics.

### 3. Rights must remain qualified (required before redistribution)

Lichess CC0 is affirmative. STS repository MIT plus attribution does not resolve absent original-author permission. Record that exact uncertainty adjacent to distributed STS data rather than making an unqualified 'licenses verified' assertion. If the project cannot justify redistribution, keep stable source IDs/URLs/hashes and local fetch extraction instead of bundling disputed upstream rows. This is a data-rights implementation choice; source research already documents the limitation honestly.

### 4. Freeze actual grouping and exclusions (required)

Require a machine-checked no-overlap receipt for actual family IDs and canonical equivalents across split. Same-game/known solution-continuation families must join before selection. Broad motifs/material classes are strata, not necessarily one family, but near-identical generated endgames must not straddle splits. Include known old regression roots and their continuations in exclusion/development-only checks across public sources. Record how unknown-game eval rows are conservatively grouped and the residual limitation; do not claim provenance guarantees unavailable in the source.

Custodian may see heldout answers for validation. Builders should receive only aggregate counts, hash commitments and validation receipts until candidate freeze. Commit candidate source/binary/options and grading code before heldout disclosure. After a failed heldout check, no retuning followed by reuse of the same set as independent validation; promote implicated families and replace or call later results development-only. Preserve the first result.

### 5. Bound the complete round, not just the main matrix (required)

240 * 2 * 5 = 2400 nominal seconds is correct. The 3300 active-engine-second ceiling is proportionate to the explicit task and does not authorize matches. Charge all native/reference diagnostic searches, failed/retried processes and overlapping worker CPU/search time consistently; make ledger definitions explicit. Keep compilation, proof construction, data download and local correctness-test time separate but visible. Sampled profiler search consumes the same engine allowance; profiler overhead is not an excuse to omit it.

600 diagnostic seconds plus 300 reserve can support a bounded investigation but not an unrestricted matrix of eight failures times five controls times 5/15 seconds plus reference adjudication. Allocate per-purpose sub-budgets and stop rules before running: adaptive single-factor tests only where discriminating, reserve some budget for changed-move adjudication, reduce breadth honestly if necessary. The one-second 'screen' should be a fixed stratified subset, not another full pair that consumes nearly the entire diagnostic allowance. No silently uncharged baseline reruns.

## Additional useful refinements

- Record achieved category/difficulty counts and selection attrition without native answers. Prefix sampling is not population-random; explicitly retain that limitation in final conclusions.
- Require sufficient-quality puzzle rating/deviation metadata where available and reference depth/knodes/PV provenance, without treating more depth as correctness.
- Keep the predecessor byte-for-byte frozen and use identical cold-start/resource conditions. Alternate predecessor/candidate order for final comparison to reduce thermal/load drift; sequential execution and absolute UTC ledger timestamps permit independent audit.
- A finite corpus can show practical transfer on its heldout selection; it cannot alone demonstrate population-wide strength or justify a numerical rating. The protocol already says this well.
- Performance-only native corrections may be retained after exact correctness and meaningful cost gains, but final report must explicitly say if broad decision quality is unchanged or unresolved rather than declaring the objective met through speed.

No engine experiments are requested by this review. Re-review the final schema, grouping receipt, resource allocation and grading tests before execution; that can be metadata/code-only and preserve reviewer blindness to heldout moves.

## Runner review (initial)

Inspected `/tmp/msh156-runner.py`, `/tmp/msh156-runner-tests.py` and their recorded fixture results; compared options/telemetry with `chess_frontier_profile.rs`. The runner reuses native semantics, forces Conventional/128/no visit cap/effective table/diagnostic options, validates legal PV and restoration, starts a fresh process, captures UTC and monotonic timing, preserves failure artifacts, charges failure wall time, refuses orphan/changed inputs, and freezes binary/driver hashes. This is sound for sequential timed cells. Author fixture receipt is not itself independent review; this paragraph records code review by a different agent.

Required small correction before use: assert native output `id` equals input root `id`; current FEN/options checks alone permit a mislabeled receipt. Add fixture rejection for wrong ID and wrong time/diagnostic options with otherwise-valid native receipt. Existing fixtures cover success playback, immutable panel refusal, orphan refusal, launch/result failures and no implicit reruns. A mock timeout cleanup fixture would strengthen process evidence without launching an engine.

Global budget is not enforced by this script: it enforces a folder-specific budget only. Coordinator must maintain/check the aggregate ledger across every prepared folder and all reference/profiling searches; independent per-folder allowances cannot each be interpreted as the round allowance. Successful early mate/completion charges actual search time, failed attempt charges wall time; do not infer the remaining nominal cells necessarily fit until the reserve check passes.

No blocker in the existing option adaptation or basic process design identified. Conditional approval pending the output-ID fixture correction and final corpus-schema interface check. Do not infer tests ran on another platform or that author tests exercise every failure path.

## Re-review after corrections

Reviewed updated protocol and runner. Required ID check and wrong-ID/time/diagnostic receipt rejection are implemented. Shared registry hashes every registered manifest, aggregates folder ledgers and holds a process lock through each cell; any registered orphan blocks further execution. I independently ran `/tmp/msh139/venv/bin/python /tmp/msh156-runner-tests.py`: all 14 fixture checks passed, zero engine searches; receipt directory `/var/folders/0d/v4yn9m6n1yv6z8q3cj_fv2pc0000gn/T/msh156-runner-tests-t7xbokdd`. This is independent test execution, separate from the author's earlier pass.

The updated protocol resolves source-rights distribution uncertainty by retaining STS retrieval references/code instead of fixture rows. It specifies purpose sub-budgets and finite adjudication selection/settings, identical-move semantics, unknown alternatives and paired-order policy. These changes satisfy the methodology corrections for proceeding with baseline once corpus metadata/validators are frozen. There is no material runner blocker for native baseline execution.

Remaining operational limits: the registry currently aggregates native folders only. Before external reference/profiling commands outside this runner, the coordinator must either register their charge receipts in compatible accounting or reserve their allocation from the native ceiling and maintain a consolidated total; a native-only registry is not proof that all engine activity is below3300. The current fixture suite does not test watchdog process cleanup (code-reviewed only). These do not require another baseline delay, but must be explicit in the final verification record. Corpus family/expectation validators remain the separate freeze gate; no heldout details were inspected by this reviewer.
