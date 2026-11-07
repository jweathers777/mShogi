# MSH-134: synchronize Chess history-view assertions

Starting integration: `816f670d1bacc95c73eae7c62dfa7b1e3b132edb`, primary
`codex/rewrite`. Source review, local focused verification and all required
Linux/macOS/Windows native jobs pass. MSH-134 is Done after approved evidence publication at
`89abb3fe36f57cbe2d48f60beb830f0cb149bf5d`; all three original criteria and four
completion items are checked.

Frozen source: `f426c67e73af53b40aee6b9491a2d8cbde55bfb5`.
Required native run42413689537 uses that identical source for all three jobs.

## Cause and narrow correction

Original Windows Rust run42190693190/job127801682025 at
`44c86b01428ca5f0de56fdb96cbe7e5b013677ea` failed
`shared_moves_chess_real_admission_reopen_and_takeback_do_not_fabricate_cues`.
The test asserted a selected historical root while reconstruction could still
be pending. Same-source retry127812835546 passed; that did not fix the defect.
Both completed logs remain in the immutable MSH-112 evidence directory and are
referenced by this task's [single verification record](evidence/msh134/verification.json).

Chess Inspect admits a request immediately, allocates an I/O identity and queues
Work::View. The real document worker reconstructs the native prefix separately;
`poll_files` admits its result only for the current I/O id/document token.
AdmissionReceipt describes command admission, not completed file/view work.
The old private Moves helper returned after admission, so it could assert a
previous live or historical snapshot. Production already polls and displays
Viewing correctly; no production semantic change is required.

The private helper now captures the exact I/O id returned by the Chess Inspect
command. After admission it waits for that id to become Complete at the requested
event index. Failed completion reports its real diagnostic; superseding I/O
refuses success. A prior historical flag, previous completed view or matching
event alone cannot authorize return. Other commands retain their admission
contract. The existing20-second helper deadline includes dispatch and both
completion stages; no timeout or sleep was increased to conceal the race.

Only `shared_moves_tests.rs` and cfg(test)-guarded code in `persistence.rs` change.
The latter can hold result observation at the existing FileWorker poll boundary.
The native worker still executes and retains its actual result normally. Normal
builds contain no hold field, method or branch. Production rules, search, engine
supervision, application runtime, schemas, budgets and interfaces are unchanged.

## Controlled evidence and independent review

The fixture imports an actual native two-move Chess record into the paused
controller, then exercises real Inspect and worker completion. No snapshot
payload, native replay result or successful completion is invented.

| Control | Required observation |
| --- | --- |
| Live and prior historical prefixes;1/4 held observations | Receipt is already successful while actual published I/O remains Viewing at the previous prefix/cue. The helper must continue, then return exact completed root with two retained move rows and no root cue. Return to live restores the unchanged full native context/cue. |
| Repeated selected root | Root/event/cue already look correct, but a new request must still await its new I/O completion. |
| Actual superseding View | A different current I/O id cannot satisfy the original request. |
| Real token invalidation | Actual Completed::Viewed is refused by existing token validation; helper surfaces Failed and preserves the live native record. |
| Original admission-only mutation | Disabling only the new View-completion wait makes the unchanged controlled regression fail deterministically: Viewing instead of Complete, cargo101. Exact fixed source is restored and passes. |

These controls distinguish event ordering without a scheduler-speed or failure-rate
claim. They retain all original historical/cue/known-row assertions. No test now
accepts either stale or completed state indiscriminately.

Local macOS27.0.1 arm64, Rust1.99.1, compatible Backlog1.55.1:

- Shared Moves **7 passed**, native projection **4 passed**, persistence
  **19 passed**, workflow **11 passed**: **41 passed, zero failed**.
- All-target/all-feature desktop-session Clippy with warnings denied: pass.
- Workspace formatting: pass.
- Deliberate admission-only mutation: expected regression failure,0 passed/1 failed.
- Initial new-test DTO equality compile error and an incorrect ReturnLive
  expectation were corrected. Their logs remain distinct from final successes;
  no product trait or native behavior changed to satisfy them.

Fresh read-only `msh134_review` accepts the narrow source and mutation proof with
no material finding. It verifies exact I/O/target semantics, failure handling,
unchanged strict original assertions and normal-build exclusion of the gate.
All three actual native jobs now pass at the identical frozen source. Fresh final independent evidence reconciliation approves acceptance; normal
evidence publication89abb3f is complete. No packaged GUI campaign is
required or claimed for this test-only correction. Historical M4 acceptance and
the earlier nonblocking134 disposition remain valid at their recorded checkpoint.

## Completion boundary

Native Rust run42413689537 completed successfully, attempt1 with no retry:

| Platform | Actual job | Result |
| --- | --- | --- |
| Ubuntu24.04 | [128527069550](https://github.com/jweathers777/mShogi/actions/runs/42413689537/job/128527069550) | All8 required steps passed |
| macOS15 | [128527069747](https://github.com/jweathers777/mShogi/actions/runs/42413689537/job/128527069747) | All8 required steps passed |
| Windows2022 | [128527069788](https://github.com/jweathers777/mShogi/actions/runs/42413689537/job/128527069788) | All8 required steps passed |

Every completed raw log contains the four controlled cases and original strict
regression passing twice: default workspace tests and the all-feature constructor
suite. The single completion-aware monitor ended. Automatic Desktop42413689587
was intentionally cancelled at the same source and its raw receipt retained;
no passing GUI/package campaign is claimed.

All three original criteria and four completion items are supported and checked.
Approved evidence89abb3f was committed/pushed before Done134. Tested sourcef426c67
is unchanged by final metadata. MSH-48 is the next planned task; no M5 work starts. Required existing Rust jobs: Ubuntu24.04, macOS15 and Windows2022.
One completion-aware monitor observes the frozen source; automatic unrelated
Desktop runs are cancelled with their provenance retained. No new M5 work or
other task implementation is part of this assignment.
