# Owner pause and cloud migration: stop-safety review

Independent source-only review, 2026-11-11. Scope: `run_batch.py`, `main_batch.py`, `notify_exit.py`; no process signals, engines, live polling, or frozen-helper edits. The supplied snapshot was batch 02 with seven completed pairs (14 games), pair 07 active and the native command in flight. This is a historical snapshot, **not evidence that execution has stopped**.

## Available stopping boundary

Neither runner has a pause request or stop-after-current-pair hook. `run_batch.py` immediately starts the next scheduled pair after saving a completed pair; its existing resume support requires matching identities, completed prior pairs, and no unresolved in-flight record. `main_batch.py` waits for the entire ten-pair native batch, then automatically performs all-move analysis and selected 5M confirmations. It processes only its explicitly supplied batch index and does not launch batch 03.

Allowing the already launched chain to reach its ordinary completion is the safest supported boundary that preserves active searches, watchdogs, cleanup and durable receipts without modifying frozen execution. The owner's explicit fallback permits finishing the current 20-game batch. Be transparent that the existing command also includes analysis: finishing that whole command is operationally convenient, but the review does not independently broaden a games-only authorization. Record the owner's pause as a durable prohibition on batch 03 and any new computation/adjudication launch; a queued completion message must lead to preservation/review, not automatic continuation.

There is no safer **supported durable pair-boundary stop** in these files. Sending TERM/INT to the wrapper forwards TERM to its child process group and may interrupt supervisors; native runners start their own sessions, so this is not a whole-descendant graceful shutdown. Killing a supervisor risks orphaning an active detached runner and losing receipt finalization. Suspending the pair supervisor also suspends its timeout handling. Suspending only an independently verified coordinator PID could technically leave its child native batch running while preventing later analysis, but that is an external OS intervention, not an implemented checkpoint: it leaves the coordinator/wrapper and in-flight record unresolved and requires explicit recovery. No such intervention was performed or validated here.

`notify_exit.py` makes one bounded `codex queue --thread … --message …` attempt after child exit. It does not start another batch. Its cleanup receipt concerns its own child process group, not all detached descendants. Ambiguous queue failure must not be blindly retried.

## Preservation and resumption

Preserve complete and partial reports, pair and query receipts, schedule/build/freeze hashes, chain in-flight state, raw protocol logs and exit-notification receipts. Do not relabel partial work completed or delete evidence to make a retry succeed. On the actual exit notification, inspect completion/cleanup once and record the final game count, failed or unresolved work and resource totals. Do not claim paused/stopped until the relevant exit and helper receipts support it.

`main_batch.py` creates its output directory exclusively and has no chain-resume option; rerunning index 02 over that directory is not a supported resume. Underlying native/query resume checks are narrower. Any interrupted-chain recovery needs an explicit plan reconciling immutable completed artifacts and unresolved in-flight attempts, preserving their identities. A future authorized batch 03 is a new invocation, not a side effect of this notification.

## Cloud validation before resumed computation

- Preserve the local hardware cohort and exact completed schedules/outcomes. Cloud hardware changes five-second search work and potentially decisions; do not silently pool cohorts as identical conditions. Record platform, CPU allocation/flags, contention, architecture and actual timing. Use an explicit bridge/calibration design if cross-cohort conclusions require comparability.
- Pin source, compiler/toolchain, dependencies, opponent version, analyst version/networks, options and fresh platform binary hashes. Existing exact binary/path freezes will not transfer unchanged; use a versioned migration manifest, retaining the original freeze, rather than weakening identity checks.
- Validate native full-history reconstruction, rights/repetition, native outcome and auto-claim semantics, legal moves and report/replay compatibility on the target. Preserve native rules/outcome authority; Stockfish self-play estimates do not replace the unconditional native claim policy.
- Validate equal primary five-second limits, worker counts, memory policies, disabled book/ponder/handicap/tablebases as applicable, watchdogs, process cleanup, bounded output, exclusive execution and exit notification using the existing runner. Select targeted platform checks; do not build a competing runner or change game/search policy to migrate.
- Transfer artifacts with checksums and preserve incomplete-attempt/resource accounting. Establish the concrete instance/cost/time bounds and secure artifact destination before substantive cloud work. Existing local success does not establish cloud process or time parity.

Conclusion: no source-supported immediate pair-boundary pause exists. The existing chain can finish without launching another batch; durable owner-stop instructions are essential for its completion wakeup. This review neither stops execution nor authorizes new cloud runs.
