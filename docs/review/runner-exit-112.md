# MSH-112: bounded early-exit classification correction

Starting integration: `92a751d3e1df0ef4c4b9610cc55ec42e5e673770`,
`codex/rewrite`, 6 November 2026. Local correction and independent source review
pass; exact corrected-source Linux/macOS15/Windows native CI is pending.
MSH-112 remains In Review until its required platform evidence is reconciled.
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

Failure classification is not deferred to cleanup: a quit response or a
supervisor kill must not retroactively manufacture evidence of the original
nonzero failure. Actual successful protocol actions, native game records, bounded
stdout/stderr/identity retention, schema1 reports and unscored failures remain.

The finite10ms policy cannot promise that every arbitrarily delayed future OS
status will become observable. Its bounds were not increased. If status is still
unavailable, diagnostics and MissingResponse remain honest; later cleanup evidence
is separate. This is bounded uncertainty, not acceptance of either label when
the tested ordering makes a nonzero status available within that policy.

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
| Portable single-sample negative control | Same actual send/failure pipeline with a test-only zero polling budget reproduces MissingResponse/exitNone while cleanup reveals actual exit7. Native CI runs this on Linux and the other hosts; this is a negative control, not a passing product classification. |
| Corrected fast/slower visibility | Real status hidden for1/2/9/10 virtual polling intervals yields exact UnexpectedExit/code7 within the existing bound. This measures no wall-clock latency. |
| Clean exit / unavailable status | Clean0 stays MissingResponse with success diagnostics. Status withheld beyond the bound is not guessed; later cleanup evidence cannot rewrite the frozen failure. |
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

- Full `cargo test --locked -p mshogi-runner`: **55 passed,0 failed**, one
  subprocess-only fixture ignored in parent discovery and invoked by regressions.
  Includes real Chess/Shogi/Xiangqi process, recorded-report/replay and lifecycle tests.
- `cargo clippy --locked -p mshogi-runner --all-targets -- -D warnings`: pass.
- `cargo test --locked -p mshogi-engine --lib bridge::tests`: **2 passed**.
- Original-source and deliberate mutation negative logs remain distinct from
  corrected-source successes. An initial new-test borrow error and an incorrect
  `--test bridge` selector were corrected; neither is a product test pass.

Fresh read-only `msh112_fix_review` approved the initial correction and re-reviewed
the owner-requested Linux negative-control/partial-stream/startup/stress additions,
with no blocking finding. Final six-file source diff SHA256 against92a751d:
`9196a9e7b0bbe2238481ef8d327f6810cca0459bf0dcd483ae3901775cf520b9`.
The reviewer inspected both negative failures, native/typed failure and timestamp
semantics, bounded cleanup and test-only hooks; it did not duplicate the active
test campaign. Platform receipt reconciliation remains required before completion.

The single [verification record](evidence/msh112/verification.json) contains
source input hashes, actual commands/logs/outcomes, historical recurrences,
review and platform receipts. Required CI: existing Rust ubuntu24.04/macOS15/
windows2022 jobs at one corrected source. No runner API, game rule, action/history,
search/evaluation, desktop runtime, canonical schema, resource budget, protocol
identity or accepted packaged-product evidence is changed. No new GUI campaign
is required for this runner-only correction; any automatic unrelated Desktop
workflow is explicitly identified/cancelled rather than reported passed.

## Completion boundary

Original112 AC1–3 and DoD are evaluated only after controlled Linux execution,
all required native-platform jobs and final evidence reconciliation. CI is
currently pending. MSH-47 stays In Review with its acceptance items unchanged;
only its owner-authorized blocker handoff may be appended after112 acceptance.
M4 remains open. No MSH-48/109, M5, M6, variant development or refactor starts.
