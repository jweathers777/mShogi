# Independent qualification execution review

Reviewer: separate execution_review agent,2026-11-10. Read-only; no engines.
Reviewed run_batch.py/protocol/build/schedule before qualification launch.

Required fixes: compare reported executable identity to frozen build hashes,
recheck each participant/runner before each pair, require two engine records and
nonempty searches. Primary builder applied all three before launch.

Approved four-game qualification execution with equal5s commands, paired colors,
native outcomes/replay,6000s pair watchdog, retained failures and inflight refusal.
Four start games cannot prove current relative strength; combine gameplay evidence
with historical results and expand limited diverse qualification if necessary.

Caveats: SIGTERM bypasses Python cleanup, so interrupted inflight needs explicit
process reconciliation. Wall/CPU records include failed work; response durations
include transport overhead and are not pure engineCPU. No new platform claims.
