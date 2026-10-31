# MSH-124 independent native CI failure disposition

Reviewer `/root/shell124_review`, 2026-10-31. Read actual final-source attempt1 logs `/private/tmp/msh124-native-mac-failed.log`, `/private/tmp/msh124-native-linux-failed.log`, local focused-test logs, unchanged runner test/production/fixture sources, MSH112 and historical119/122/72/76 evidence. No product/task changes made; this is bounded diagnosis, not implementation of another ticket.

## Production causality boundary

`git diff 97b9470943107ae842c8184081c5e8e4cd26ef32..4a624d6c4f129ce774d385192fbbe6a9bdcf547a -- crates desktop/src-tauri Cargo.toml Cargo.lock` is empty. MSH124 changes shell/frontend presentation and external GUI checks, not runner/process/native logic or Rust dependencies. Initialde5209d native workflow41206493928 is reported by root as all3PASS; reviewer has not independently fetched that workflow in this diagnosis. Final4a624d workflow41207699395 attempt1 contains the failures below and must not be reported green.

## Actual failures and existing evidence

1. **Linux** `process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored`, runner.rs172: expected UnexpectedExit, actual MissingResponse for engine0 handshake, Broken pipe(os error32), exitNone, at_ns1455086. This exactly matches current To Do/unstarted **MSH112** description and its preserved119 recurrence; docs/review/frontend-application-119.md196–207 and linux-gui-triage-122.md192–195 explicitly preserve it. Early write-error/EOF can be observed before process exit status; failure-kind/diagnostic distinction remains unresolved. The task explicitly prohibits accepting either category or raising timeouts to mask the issue.
2. **macOS** `missing_retirement_eof_cannot_hide_a_delayed_duplicate_bestmove`, shogi_runner.rs355: expected ResourceLimit, actual LateReply. Historical MSH72 and MSH76 records preserve the identical assertion/failure. Native fixture emits second bestmove390ms after quit; cleanup500ms reserves1/4, giving375ms drain cutoff, with5ms bounded receives/scheduler wakeup near cutoff. Existing code logs a retired bestmove as LateReply; absent EOF at deadline adds ResourceLimit only if no earlier cleanup error. This explains a scheduling-dependent first-error category without MSH124 changes. Both source branches reject the candidate; failed CI stops before subsequent safety assertions, so that failed run alone does not prove all later invariants. Current MSH112 specifically owns early-exit classification, not automatically this separate retirement test; do not silently broaden its ownership. Preserve the72/76 evidence and current recurrence accurately.

## Verification and disposition

Read root-run exact local macOS focused tests: runner1PASS/0FAIL (0.62s), shogi_runner1PASS/0FAIL (1.05s), logs `/private/tmp/msh124-native-focused-runner.log`, `/private/tmp/msh124-native-focused-shogi.log`. These test all assertions when passed locally; they do **not** establish Linux/macOS CI success, eliminate nondeterminism or fix either issue. No assertions, timeouts, production runner, rules, outcomes, task gates or ownership changed in MSH124.

Independent disposition: these are documented unchanged native race recurrences outside MSH124, not evidence of a new shell semantic regression. Preserve both failed attempt logs and existing unresolved native ownership. A normal failed-job retry on identical final source after matrix completion is proportionate; outcome is pending and must be recorded separately. Do not describe retry as fixing or waiving semantics. GUI124 acceptance and platform evidence retain their own requirements; this diagnosis does not substitute for them.

## Recorded identical-source retry disposition

Root's completed workflow audit reports `41207699395` attempt2 at unchanged4a624d6 **all3platforms PASS**, after the normal failed-job retry; attempt1 Linux/Mac failures above remain preserved. No native source, assertion or timeout changed between failed/successful attempts. This is successful retry evidence of unchanged tests, **not a fix** of MSH112 early-exit or the separately recorded72/76 retirement timing race. The reviewer retains the concrete source/log diagnosis above; root's workflow-status audit supplies this actual retry outcome.

Final91eb13e native workflow41208986510 currently has Linux/macOS PASS and Windows still running according to root's audit. No final overall green is claimed here until Windows completes. The91 production delta is decorative supportSVG only; native source remains identical. Root separately reports doctorPASS and retainedMac capture-hash verification; those are task-finalization/evidence checks, not native race remedies.

## Final current-source native status

Independently read retained `docs/review/evidence/msh124/native-final-ci.json`: workflow41208986510, exact91eb13e2451af77150b82b62a448bf97e9d5ac58, attempt1 completed success; Rust ubuntu24.04/macos15/windows2022 all completed success, including their recorded workspace and isolated QA-constructor steps. `desktop-final-ci.json` similarly identifies41208986515/all3 success; actual installed/packaged Linux/Windows GUI reports remain separate evidence. This supersedes the earlier pending finalWindows status, not the historical failures or unresolved runner-race ownership. No native code/assertion/timeout fix is claimed.
