# MSH-161 (Competitive Gameplay Diagnosis): cloud migration handoff

Owner direction of 11 November 2026 supersedes automatic local continuation.
Read `owner-request.txt` and `execution-policy.json`. Coordination is **repository
only**; no inter-chat messaging or dependency on a chat ID. The coordinator owns
MSH-161, scientific interpretation and `codex/rewrite`. The separate cloud-runner
agent owns its execution implementation in an isolated worktree and dedicated
`codex/` branch, with its own bounded task/evidence. Do not run competing builders
over the frozen study helpers. Do not start production search/evaluation/rules work.

## Local state and safe stop

At the migration-request snapshot, batch02 had **seven completed pairs /14games**,
with `main-02-07` in flight. Batch01 has20complete, fully screened games with reviewed
5M/selected20M evidence. There are also20separate qualification games. Thus the
snapshot contains34complete main games, of which20have full study analysis;
qualification games must not be added to the main denominator. Batch02 has no
completed reference analysis at this snapshot. Its game outcomes are evidence,
but cannot support unperformed move-error judgments.

The frozen local runner has no cooperative pair-boundary pause hook. Killing its
parent does not safely cancel/reconcile the independently grouped native process.
We chose the owner's fallback: allow the **already running batch02 chain** to finish,
including its existing all-move and5M stages. No source mutations, signals, new
engine jobs or polling watcher were used to attempt a synthetic checkpoint.
This is a draining state, **not a claim that local execution has stopped**.

The only active chain is `../main/batch-02/chain.json`; wrapper receipt is
`../notifications/main-batch-02.json` (execution session1756). The notifier queues
a chat message; it does not run batch03. Its old message instructing continuation
to batch03 is explicitly superseded. On its exit, the coordinator must preserve
all final/failed outputs, verify cleanup, record actual completed/analyzed counts,
and mark the local state stopped. **No local batch03, new20M probes, or retry may
be launched automatically.** Preserve failures and investigate without restarting.
Do not modify any file in the original freeze while the chain depends on it.

`request-snapshot-bundle.json` verifies the captured state and completed pair
artifacts in `batch02-completed-pairs-at-request.tar.gz`. Active pair files remain
untouched in the live directory; they are not falsely archived as complete.
This snapshot is not the final stopping receipt. Existing batch01 pre20M and20M
bundles preserve all completed raw evidence. The five exact Mac executables have
verified local copies in ignored `local-executables/`, indexed by
`local-executables-manifest.json`; binary build output is not committed to Git.

## Scientific ownership and primary cloud sample

The Mac games become an explicitly labeled **exploratory hardware stratum**.
The preferred primary comparison reruns **all200 originally planned games** on
standardized cloud workers:90source-cleared histories played with both colors,
plus10paired unaided starts. Preserve every opening, original ordering/identity,
both colors and all unfavorable results. Do not count the Mac games as a uniform
part of the cloud200, or remove a cloud pair because its Mac result was inconvenient.
Repeated unaided starts and paired openings remain correlated; do not treat each
move or each repeat as an independent statistical sample.

Keep Oxidation0.6.0 fixed. Both players receive `go movetime 5000`, one worker each,
with recorded default memory, fresh processes/game, no book beyond the supplied
history, no pondering/strength handicap/tablebases. The native controller retains
rules/history/outcome authority, automatic claims and the existing512-ply unscored
cap. Python chess libraries remain diagnostic tools, never alternate adjudicators.
Match time is primary; do not substitute equal node counts for the games.

Stockfish19/Threads1/Hash64/UCI_ShowWDL remains the analysis reference. Preserve
full-history500k unrestricted/played screening for both players, v1 uncertainty
handling, matched5M followups and fixed-hash nonflagged audit,20M consequential
adjudication, phase conventions, actual results and independent completed-batch
reviews. Do not silently change gates, clear hash behavior, score perspective,
thresholds, source balance or claim policy. See `../main-method.md` for the full
protocol. The interim assessment identifies limitations, not permission to tune it.

The automatic-claim limitation in batch01 pair06/game1 must carry forward: a native
draw may coexist with favorable Stockfish continuations because the controller
automatically claims where a player could decline. Do not infer a rules defect or
actual-result conversion failure from that discrepancy alone.

## Inputs and source records

`cloud-inputs.json` records verified repository-relative paths and hashes for the
original freeze and all ten schedules. Original absolute Mac paths are provenance,
not cloud deployment paths. Create a separate cloud execution manifest with explicit
path mappings and new platform binary identities; never rewrite the original
freeze to pretend Linux executables have the Mac hashes.

| Input | Repository location / identity |
| --- | --- |
| Original protocol/freeze | `docs/m5/evidence/msh161/main-method.md`, `main-freeze.json` and frozen `docs/m5/competitive-gameplay-diagnosis.md` |
| Candidate and original native runner | Source `77101fe3d38712361af8dd99b3ca2bcc6b5c17d3`; build command and original hashes in `build.json` |
| Oxidation source | `https://github.com/Mathmagician8191/Liberty-Chess.git`, commit `46a705b80e7191e7c302838eb8b85053b5994d65` |
| Stockfish provenance | `build.json`, `docs/m5/evidence/msh49/verification.json`, `docs/m5/chess-strength.md`; official19release, verify cloud source/network/build identities |
| Frozen pairs | `main/batch-01-schedule.json` through `main/batch-10-schedule.json` |
| Source selection/history | `sources/main-roots.json`, `sources/main-selection.json`, source manifests and source/admission reviews; full source histories retained |
| Native Rust commands | `crates/mshogi-runner/src/main.rs`: `experiment-external uci CONFIG REPORT`, `experiment-external-replay REPORT` |
| Native execution/evidence | `crates/mshogi-runner/src/experiments/{mod,run,evidence,native}.rs`, experiment tests and CLI tests |
| Existing study orchestration | `docs/m5/evidence/msh161/{run_batch,main_batch,analysis,screen_v1,phase_tags,review_checks}.py` |
| Original confirmation/analysis dependencies | Existing manifests record Python/python-chess versions, engine options, node budgets, report hashes and process limits |
| Completed interpretation | `docs/m5/msh161-interim-gameplay-assessment.md`, batch01 reviews/overlays, `main/cumulative-categories.json` |

Rebuild platform executables from pinned sources, with licenses and embedded
network identities preserved. Binary identity will change across platforms; record
that honestly. Keep candidate engine semantics/source unchanged. If the new system
needs native harness changes, isolate them and establish evidence-format/rules/
history/command equivalence; do not bundle engine improvements with migration.

## Validation required before primary use

The cloud-runner agent should publish commands, tested source revisions, artifacts,
failures, costs and limitations in its task and this repository. The study
coordinator reviews that evidence before authorizing primary study execution.

- Establish Linux native outcome/history/replay compatibility on representative
  complete histories, both colors and automatic/intent-to-move claims. Preserve
  typed failures, cap/unscored behavior and the distinction between played moves
  and unplayed intended claim actions. Select relevant Level2process/platform
  checks; do not infer Linux behavior from Mac evidence.
- Verify exact UCI options/commands and full histories, time enforcement, fresh
  processes/clear hash, stdout/stderr bounds, watchdogs and complete cleanup.
  Reference test positions at fixed nodes can compare adapters; timed matches need
  not reproduce identical Mac moves. Do not assume cross-platform score identity.
- Record worker image/digest, machine class, CPU architecture/features, available
  and allocated cores, compiler/flags, memory, engine/network hashes and actual
  timing/load. Demonstrate that parallelism distributes independent work without
  oversubscribing a worker or favoring one competitor. Freeze one primary hardware
  class; exceptional/retried hardware strata remain explicit.
- Prove durable exclusive job claims, immutable attempt IDs, full artifact upload,
  hash verification and resumability after worker/preemption/upload failures.
  Preserve every attempt; retries cannot overwrite failures or be counted twice.
  A pair is complete only after both native games, accepted records and required
  replay evidence exist. An incomplete pair must remain identifiable/replayable.
- Validate missing/duplicate work detection against all200planned games and every
  required analysis query, including deduplicated same-choice controls. Preserve
  screening/audit selection exactly; deterministic job ordering/IDs must not depend
  on observed wins or scores. Test non-game and game failures, not just happy paths.
- Keep cost/resource accounting separate for attempts, parent elapsed time and
  analysis; prevent double charging in scientific totals. Use operational per-job
  bounds, failure handling and cost monitoring. There is no arbitrary scientific
  compute ceiling that permits truncating the approved survey.
- Use completion-driven coordination, with no periodic model watcher. Retain the
 20-game review cadence: parallel execution must not bypass independent gameplay
  review/confirmation or silently launch the whole campaign without checkpoints.

No cloud jobs have been launched by this migration handoff. The cloud agent should
record its isolated branch/worktree, task ID, implementation/validation commits and
proposed cloud execution manifest in the repository. Integrate only after review;
do not alter frozen/live inputs in the coordinator's checkout. After validation,
the coordinator freezes the cloud execution provenance and continues the full
scientific study, retaining Mac exploration and interim hypotheses separately.
