# MSH-112: bounded early-exit classification correction

Starting integration: `92a751d3e1df0ef4c4b9610cc55ec42e5e673770`,
`codex/rewrite`, 6 November 2026. Corrected tested source:
`44c86b01428ca5f0de56fdb96cbe7e5b013677ea`. Local correction, independent source
review and all three required native CI jobs pass. Independent final evidence
reconciliation approves technical acceptance; normal publication precedes task
completion.
MSH-47 and M4 are not accepted or closed by this assignment.

## Original failure and established ordering defect

The original portable regression is
`process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored`.
An awaited fixture exiting7 during handshake sometimes yielded MissingResponse,
Broken pipe32 and no exit status, rather than required UnexpectedExit with coded
non-success exit diagnostics. The original112 record preserves Linux40874033735
at6dec5ab,119 recurrence41131548408/job124323454077 at eb027be7…,86 recurrence
41536948188/job125591097628 at136f8926…, and87 recurrence41816309261/
job126527815548 at9749eec0…. Its successful unchanged-source retry126537522942
did not fix the race. [R4 recurrence](evidence/msh119/runner-112-recurrence.json),
[R6 failure](evidence/msh121/checks/native176-linux-failure.log.gz), and the native
[Chu](../m4/chu/native-verification.json)/[Tenjiku](../m4/tenjiku/native-verification.json)
records retain their original provenance.

The [MSH-25 correction](m2-closure.md) addressed a different fixture ambiguity:
two faulty children could produce either awaited EOF or the other child's idle
exit. That fixture already faults only the awaited child and separately checks
idle exit. Its strict expectations and historical acceptance are preserved.

The112 product defect is the **asymmetric observation policy**:

1. Spawn creates child/stdin/stdout/stderr workers. An early child exit closes
   pipes; a failed stdin write can be acknowledged before an OS exit status is
   visible to `try_wait`. These observations need not arrive atomically.
2. Old `GameRun::send_at` sampled `observe_exit` once after write failure and
   froze MissingResponse if no non-success status was visible. Its string-based
   timeout distinction also conflated acknowledgement disconnection with timeout.
3. `failed()` could attach a later status without changing the frozen category.
   Later cleanup could collect/reap exit7 and stderr while the recorded primary
   failure still lacked the required interpretation.
4. Awaited stdout EOF already allowed a bounded10ms status-settlement loop and
   preserved the earlier EOF receipt. Hard stdin failure did not use that policy.

Controlled before/after evidence establishes the causal single-observation
defect; it does not claim every historical host's exact kernel scheduling trace
has been recovered. No rule outcome, fabricated score or cleanup hang was needed
to reproduce the category mismatch.

## Narrow production correction

Sources are limited to `crates/mshogi-runner`:

- `process.rs::WriteFailure` keeps acknowledgement timeout distinct from hard
  transport failure, without relying on platform error wording. It is private;
  existing serialized failure types/protocol identities are unchanged.
- `process.rs::settle_exit` names the **existing10ms** bounded EOF/status policy.
  Hard non-cleanup write failures now use the same polling loop. Nonzero status
  yields UnexpectedExit and actual code/signal; successful exit yields
  MissingResponse; unavailable status remains honestly unavailable. Broken pipe
  alone never establishes a nonzero exit.
- An unavailable hard-write/awaited-EOF status remains **provisional**, rather
  than freezing the primary report. `failed()` attaches/classifies definitive
  status if already observable. Otherwise `GameRun::cleanup` moves the existing
  cleanup grace (one quarter of cleanup_ms, capped at100ms and remaining budget)
  before any stop/quit/kill intervention for that pending failure. Natural exit7
  finalizes UnexpectedExit/code7; natural clean0 retains MissingResponse/code0.
  The same original failure/diagnostic object is updated without changing its
  receipt timestamp, stop reason, native match or score.
- The pending path calls `Engine::cleanup_after_exit_grace` so it does **not**
  receive another grace period. Grace, stop and quit all consume the existing
  remaining cleanup budget; kill/reap/worker cleanup use only what remains.
  Other Engine.cleanup consumers retain their existing policy.
- Acknowledgement timeouts retain ResponseTimeout/deadline handling and do not
  add settlement waits. Cleanup stop/quit writes also skip settlement, preserving
  their existing total budget. Cleanup retains its original grace/kill/reap and
  bounded worker-join behavior; no `wait()` or timeout increase was added.
- `send_at` captures the failed-write receipt **before** transcript formatting
  and status metadata acquisition. `write_failure` preserves that timestamp and
  the established dispatch-derived watchdog deadline. Native chess-clock
  precedence therefore cannot change merely because status collection took time.
- Shogi/Xiangqi retain their existing write-category mappings. Their shared EOF
  helper calls are mechanically renamed; no new game/protocol semantics result.
  Idle-other-engine exit behavior and MSH-25's isolation remain unchanged.

Only **natural status observed before cleanup intervention** can finalize the
pending classification. A quit response or supervisor kill cannot manufacture
evidence of the original nonzero failure. If status stays unavailable, the
primary failure records that fact; post-intervention exit diagnostics remain
separately available in EngineObservation. Successful protocol actions, game records, bounded
stdout/stderr/identity retention, schema1 reports and unscored failures remain.

Neither the immediate10ms phase nor the existing pre-intervention cleanup grace
promises arbitrary future OS status visibility. This is not an uninterrupted
natural wait for all cleanup_ms: termination/reaping still require budget.
No timeout was increased. A natural status observed during the existing grace
is used, not prematurely discarded; genuinely unavailable or intervention-tainted
status is not guessed. The tests distinguish these cases explicitly.

The existing failure distinctions remain:

| Observation | Report meaning |
| --- | --- |
| Child cannot spawn | Process-start failure for that engine. |
| Definitive natural nonzero exit | UnexpectedExit with available native status and diagnostics. |
| Clean exit without required reply | MissingResponse with successful exit status. |
| Hard transport failure, natural status unavailable through bounded observation | MissingResponse with an explicit unknown-status diagnostic; later intervention status is separate. |
| Live child misses its reply/write watchdog | ResponseTimeout and the established deadline. |
| Malformed protocol / native-illegal move | InvalidResponse / InvalidMove. |
| Resource refusal or cleanup failure | Existing distinct resource/cleanup evidence; no invented game result. |

Explicit cancellation keeps the existing cancellation/retirement contracts on
surfaces that support it. Process failure supplies no move, resignation, score or
timeout victory. Independently established native clock expiry retains its
existing precedence and original receipt time; exit-metadata collection cannot
create or move that deadline.

## Controlled reproduction, countercases and supplemental stress

The controlled regression spawns this unit-test executable as an actual child
that exits7 and emits early stderr. It first reaps the real status under bounded
test setup; a cfg(test)-only gate withholds its visibility for two observations.
The actual stdin write fails against the exited child. Tests call production
`GameRun::send/failed` and the same settlement polling loop. Only observation
visibility and elapsed polling time are controlled; no exit code, write failure,
RunnerFailure or game outcome is fabricated.

The subprocess fixture test is ignored during ordinary parent discovery because
its body terminates its process. Regressions explicitly execute it as a real
child. It is not an ignored production correctness obligation or a packaged engine.
No test hooks exist in normal product builds.

| Case | Independently required observation / result |
| --- | --- |
| Original production + test-only gate | Actual failure before the fix: MissingResponse instead of UnexpectedExit. Expected regression failure retained. |
| Deliberate single-observation mutation | Replacing only the new hard-write settlement with the original single sample makes the unchanged regression fail, cargo exit101. Restoring the fix passes. |
| Portable single-sample negative control | Same actual send/failure pipeline with a test-only zero polling budget reproduces preliminary MissingResponse/exitNone. The new finalizer then uses natural exit7 before intervention and repairs the primary failure. Native CI runs both assertions on Linux and other hosts; the preliminary mismatch is a negative control, not accepted final behavior. |
| Corrected fast/slower visibility | Real status hidden for1/2/9/10 virtual polling intervals yields exact UnexpectedExit/code7 within the existing bound. This measures no wall-clock latency. |
| Late natural0/7 and EOF | Actual status hidden beyond immediate settlement becomes definitive during the existing pre-intervention grace; primary failure gets exact0/7 classification/diagnostics with unchanged receipt. Suppressing only this finalization makes the unchanged late-exit regression fail, cargo exit101. |
| Live closed stdin / unavailable status / forced termination | A real live child with parent-side stdin closed remains MissingResponse/unknown through natural observation. Supervisor kill/reap is recorded separately, not relabelled as the original failure. No unsafe descriptor closure or invented exit status is used. |
| Timeout, cleanup and misleading error text | Typed timeout keeps ResponseTimeout/deadline. Hard error text mentioning timeout cannot invent that category. Cleanup takes no extra settlement polls. |
| Clock precedence / native state | Fixed49ms receipt precedes50ms native deadline;60ms receipt observes established flag precedence. Metadata never changes receipt. Native history/replay remain exact; failures remain unscored. |
| Partial/closed output / early stderr | Actual unterminated id line plus early stderr then0/7 exit retains precise MissingResponse/UnexpectedExit, diagnostics and successful reaping. Non-ASCII protocol output remains InvalidResponse; illegal move remains InvalidMove. |
| Startup-abort retirement | Second-child spawn failure aborts before handshake, retires/reaps the already spawned live first child and retains ProcessStartFailure for the correct child, with no game conclusion or leaked child. |
| Live nonresponse / blocked stdin / limits | Existing real-process watchdog, blocked writer, oversized output, stderr/identity/log bounds and cleanup tests retain their distinct assertions. |
|20 supplemental trials | Alternating actual immediate exit and yielded partial-output/early-stderr exit; all require exactUnexpectedExit/code7, unscored history/replay and reaped children. Not a substitute for the controlled proof or universal scheduler coverage. |

The synchronous UCI `run_experiment` has no public caller-cancellation channel;
this task does not invent one. Its existing startup-abort retirement path is
tested explicitly. Existing Shogi/Xiangqi retirement/generation tests and separate
EngineBridge cancel/replace/bounded-pipe-cleanup tests preserve the actual
cancellation surfaces; they are not relabelled as a new UCI cancellation API.

## Verification and independent assessment

Local macOS27.0.1 arm64, Rust1.99.1, compatible Backlog1.55.1:

- Full `cargo test --locked -p mshogi-runner`: **57 passed,0 failed**, two
  subprocess-only fixtures ignored in parent discovery and invoked by regressions.
  Includes real Chess/Shogi/Xiangqi process, recorded-report/replay and lifecycle tests.
- `cargo clippy --locked -p mshogi-runner --all-targets -- -D warnings`: pass.
- `cargo test --locked -p mshogi-engine --lib bridge::tests`: **2 passed**.
- Original-source and deliberate mutation negative logs remain distinct from
  corrected-source successes. An initial new-test borrow error and an incorrect
  `--test bridge` selector were corrected; neither is a product test pass.

Fresh read-only `msh112_fix_review` initially approved immediate settlement, then
**superseded that approval** after the full owner cleanup wording arrived. The
late-status counterexample showed the primary report still froze too early.
Checkpoint `d835c7ce36d2afd718147ef953b4f81879bc39f6` and Rust42187775585 were
deliberately superseded/cancelled for this correctness gap, not runner capacity.
All three native jobs acquired runners. The automatic Desktop42187775810 was
cancelled as unrelated runner-only verification. Actual receipts are retained.

The reviewer subsequently approved the revised pending/finalization policy and
distinguishing natural0/7/unknown/forced-intervention tests, with no blocker.
Incremental source diff SHA256 againstd835c7c:
`358bce6c855d4cd3ebb7228792b08171de2749e343673ea1f3a3640122f4f789`.
It inspected the additional finalization mutation failure, exact grace/remaining
budget and receipt/clock invariants without duplicating tests.

At corrected source `44c86b01428ca5f0de56fdb96cbe7e5b013677ea`, native
run42190693190 Windows job127801682025 failed before reaching the runner suite:
unchanged desktop-session test
`shared_moves_chess_real_admission_reopen_and_takeback_do_not_fabricate_cues`
asserted the historical root before asynchronous view reconstruction completed.
The failed log is retained. Independent read-only review traced Chess Inspect
to `queue_view`: it submits Work::View without an admission receipt, while the
test helper treats the immediate admission receipt as completed view work.
The helper can return before `poll_files` installs Completed::Viewed. A fresh
snapshot after receipt completion alone would not establish view completion.
Desktop-session, engine and workflows are unchanged from the starting source;
desktop-session has no dependency on the runner. Separate unassigned MSH-134
records this synchronization defect; its implementation is outside112.

One supported Windows-only retry at the identical source passed as
job127812835546, attempt2. No healthy platform was rerun. This establishes
platform execution evidence and does not correct MSH-134. Both attempts and
their source identities are preserved.

Required Rust run42190693190 finished successfully at the exact source above:

| Actual execution | Attempt | Job | Result |
| --- | --- | --- | --- |
| Ubuntu24.04.5 | 1 | [127801682198](https://github.com/jweathers777/mShogi/actions/runs/42190693190/job/127801682198) | All required steps passed |
| macOS15.8.2 | 1 | [127801682340](https://github.com/jweathers777/mShogi/actions/runs/42190693190/job/127801682340) | All required steps passed |
| Windows2022 | 2 | [127812835546](https://github.com/jweathers777/mShogi/actions/runs/42190693190/job/127812835546) | All required steps passed |

Each completed raw log independently contains all11 critical named controlled,
original-regression, stream, startup and20-trial stress cases passing. Linux
executes the portable single-sample negative ordering and corrected finalization;
success is not inferred from local macOS27. All eight required verification
steps pass on each platform. Raw packets retain native runner identities,
timestamps, source and every failed/superseded attempt. GitHub attempt2 issued
carry-forward Ubuntu127812836773 and macOS127812882169 receipt aliases with the
same original timestamps/runners; these are not new executions. The sole retry
was Windows. Sequential completion-aware monitors ended at each terminal run.

The single [verification record](evidence/msh112/verification.json) contains
source input hashes, actual commands/logs/outcomes, historical recurrences,
review and platform receipts. Required CI: existing Rust ubuntu24.04/macOS15/
windows2022 jobs at one corrected source. No runner API, game rule, action/history,
search/evaluation, desktop runtime, canonical schema, resource budget, protocol
identity or accepted packaged-product evidence is changed. No new GUI campaign
is required for this runner-only correction; any automatic unrelated Desktop
workflow is explicitly identified/cancelled rather than reported passed.

## Completion boundary

Independent final read-only reconciliation approves all three original112
acceptance criteria and DoD1–3 with no remaining source or evidence defect.
It independently verified all six source hashes,19 indexed evidence hashes,
three protected47 hashes, all required native steps and all11 critical cases in
each platform log. AC1–3 and DoD1–3 are checked; DoD4 remains only normal durable
publication and push. The production correction is already integrated/pushed
at44c86b0. MSH-47 stays In Review with its acceptance items unchanged; only its
owner-authorized blocker handoff may be appended after112 acceptance.
M4 remains open. No MSH-48/109, M5, M6, variant development or refactor starts.
