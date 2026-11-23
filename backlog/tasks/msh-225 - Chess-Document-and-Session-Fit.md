---
id: MSH-225
title: Chess Document and Session Fit
status: Done
assignee:
  - '@codex'
created_date: '2026-11-22T09:57:00Z'
updated_date: '2026-11-23T10:51:00Z'
labels:
  - desktop
  - refactor
milestone: m-6
dependencies:
  - MSH-228
priority: medium
ordinal: 232000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Chess has its own DesktopSession, ChessDocument and mutation journal (lib.rs, persistence.rs), from the first desktop slice. John (2026-11-22) asked for a serious check whether Chess belongs in the single generic document and session, or at least reuses its parts without copies that can drift. Chess match rules (D-11: claims, auto-claim policy, clocks, ordered match events) are real differences; the rest may be history.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Measured comparison of the Chess session and document against the generic ones, with each difference classed as rules, product or history; result recorded in the task
- [x] #2 Chess uses the generic document and session, or the parts that are the same are shared code with no copies; the reason for any remaining separation is recorded
- [x] #3 Desktop-session and workspace tests pass; one fresh-agent review
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Separate the generic saved native record from the engine context, keeping existing five-game encodings unchanged. Add a compact accepted-command Chess record reconstructed through ChessMatch.
2. Implement Chess native/session hooks, including claims, Full analysis, presentation and take-back. Route Chess through generic admission, save/open, polling and bounded in-memory observations; remove the old journal and copied lifecycle paths.
3. Migrate behavior tests and fixtures, run required checks and targeted persistence CI, obtain a fresh-agent review, then integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Comparison (2026-11-22, main 14f7184). Chess code: lib.rs 2,486 (DesktopSession, ChessMutationCommand journal, own admission/command/poll/accept paths; tests from line 1810), persistence.rs 3,815 (ChessDocument archive, own file/admission worker, Full analysis certificates; tests from line 1892), analysis.rs 612, takeback.rs 107, capacity.rs 335 (shared), chess_snapshot in shared.rs (~250).
RULES (real, D-11): ChessMatch core records timestamped requests (Start, Move, Claim{reason, intended}, Resign{witness}, OfferDraw/AcceptDraw/DeclineDraw, Observe); optional draw claims with a recorded auto-claim policy; mating-feasibility adjudication. All are commands of one core API (ChessMatch::submit -> MatchReply, events, prefix, replay) and fit NativeGame. The desktop is always untimed, so clocks add only timestamps.
PRODUCT (real, generalize): Chess Full reachability analysis (bounded proof with certificates, cached verification) is a second analysis kind; play limits are time-based (1 s); analysis defaults per kind.
HISTORY (drift): own session struct, own saved format (match + game JSON, ControlEntry journal, TakeBackEvidence instead of retired branches, ArchivedObservation), own admission worker, own capacity checks, own snapshot builder, own browse state. Same jobs as the generic document and session.
Fit: Chess can use the generic document and session if (1) the document record is the match record while the engine context stays CanonicalRecord::Chess(GameRecord) (new NativeGame::context hook used for requests and observation matching); (2) MatchRequest version/at_ns are filled by the Chess submit wrapper; (3) SessionGame gets a hook for extra analysis operations and their result validation (Full); (4) take-back evidence becomes generic retired branches.
Plan: same steps as MSH-224/226; regenerate Chess fixtures (already stale per MSH-221); measure work limit; review.

Cost finding (2026-11-22): the Chess saved record is a MatchRecord (timestamped request log with replies; rejected requests are recorded as audit events), not a CanonicalRecord, while engine contexts use CanonicalRecord::Chess(GameRecord). Moving Chess onto the generic document needs: (1) a per-game record type in the generic payload (today CanonicalRecord everywhere: parse, profile, to_json, observation prefix checks); (2) a record-to-engine-context hook; (3) a decision on rejected-request audit events (the generic journal keeps accepted commands only); (4) Chess Full analysis (certificates, cached verification) as a generic extra analysis kind. Estimate: larger than MSH-224 and MSH-226 together; touches lib.rs, persistence.rs (about 1,900 lines of tests) and full_archive_tests. Asked John whether to do it now or after Chu work.

Paused 2026-11-22 (John): MSH-228 (Minimal Saved Game Document) first; with the minimal document Chess fits much more cheaply (its match log can become moves, claims, offers and result, without timestamps or rejected-request audit).

Codex started after MSH-228 integration (main 8f5e55a). Reassess Chess against the minimal document and current generic session; preserve Chess claims, offers, mating-feasibility and Full analysis. Branch codex/msh-225-chess-document. Next: measure current differences and record implementation plan before code changes.

Updated comparison after MSH-228: lib.rs 2490 lines; persistence.rs 3815; analysis.rs 612; takeback.rs 107; generic native_session.rs 1429 and game_document.rs 356. Rules differences remain ordered match commands, optional claims, draw offers and mating-feasibility outcomes. Product differences remain Full analysis, Chess presentation and 1-second play limits. Separate runtime versions, saved observations, controls, take-back lineage, capacity reservations and admission/poll/save/browse implementations are history, not required differences. Implement full generic session integration. The compact Chess record will retain initial canonical history and accepted player commands, with deterministic untimed replay through the unchanged ChessMatch core; runtime timestamps, rejected requests and Start/Observe bookkeeping will not be saved. This follows John’s recorded MSH-228-first direction.

Checkpoint: generic document payload now owns G::Record, with shared canonical encoding for the existing five games and separate engine-context hooks. Added compact ChessRecord replay through unchanged ChessMatch; tests cover moves/offers/agreement, intended claim without a move, unresolved resignation, imported history/policy, rejected commands, malformed input and cancellation. Desktop-session tests passed 240/240 before the final canonical-wiring macro cleanup; cargo check passed after it. This is foundation work only: the active Chess session still uses the old archive/runtime. Next: implement NativeGame/SessionGame for Chess, Full analysis hooks, route controller and persistence through generic session, then remove old journal paths and migrate tests/fixtures. Branch codex/msh-225-chess-document.

Checkpoint: Chess NativeGame and SessionGame adapters now work on a transitional GenericChess active-document variant. Focused tests pass for generic Full analysis (read-only, remembered kind), wrapped Chess failure stopping without automatic retry, and play/browse/take-back across imported history. Existing 241 desktop-session tests passed before adapter routing was added. Review found wrapped Chess failures and duplicate uncancellable witnessed-resignation verification; fixed with native failure extraction and the existing desktop refusal of witnessed match commands. Full read-only validation remains cooperative. Normal Chess New/Open/default construction still uses the legacy path; next switch those entry points, remove legacy session/archive/journal code and migrate its relevant behavior tests/fixtures. Imported initial history remains browseable but Take back cannot cross its root.

Checkpoint: all Chess entry points now use NativeSession and the minimal shared document. Removed legacy DesktopSession/archive/control-journal paths. Migrated native tests, frontend snapshots, GUI assertions and document contract. All-feature desktop tests passed 203/203 before the final review fix; Python GUI tests passed 176/176. Fresh review found completed analysis incorrectly advancing selection identity; fixed read-only response admission and added a target-identity regression. Focused Chess tests now pass 37/37, including the 4096-command boundary and mid-replay cancellation. Next: complete workspace/frontend checks, targeted persistence CI on affected operating systems, then integrate. Branch codex/msh-225-chess-document.

Final review confirmed the read-only analysis identity fix; no remaining findings. Frontend: 609/609 tests and TypeScript passed. fmt and workspace clippy passed. Debug workspace stopped on the same four pre-existing Chu watchdog failures as MSH-228 (MSH-199); no rules or test limits changed. Release workspace and targeted Linux/macOS/Windows persistence run 45155718339 are running. Main was merged (already up to date). Final measured common files: lib 822, persistence 517, analysis 80, takeback 43 lines; new Chess adapter/record/validation files total 1583 lines. Remaining Chess code is native match rules, Full result validation, presentation and ownership/take-back mapping; lifecycle, admission and file code are shared.

Real-process Chess persistence test exposed a shared phase mismatch after engine moves: paused=false but phase=Paused. Fixed successful live responses to report Idle, preserving Paused for read-only analysis. All three real-process persistence tests now pass, including minimal save/reopen, failure preservation and same-ply resignation history.

Latest checkpoint (8bfc824): all-feature desktop-session tests 204/204, real-process persistence 3/3, frontend 609/609, Python 176/176, TypeScript, formatting and workspace clippy passed. Corrected persistence CI run is 45156397616 (the superseded run was cancelled). Release workspace is compiling with two jobs. No code changes pending; next finish workspace/platform checks, integrate main, set Done and remove task build/run data.

Windows CI exposed a test scheduling race in concurrent_save_as_keeps_new_admission_dirty_and_open_retires_it: Open could finish inside shared_command before the supposed pending command was queued. Added existing test-only file/admission result gates to establish the required order (queue both, publish Open first). Kept all cancellation and restored-state assertions. Focused test passes locally. Production code is unchanged by this fix.

Platform results: Linux and macOS persistence passed on production commit 8bfc824 (run 45156397616). Windows hit the test-only Open scheduling race; corrected test is ccb61c6 and Windows run 45157620360 is in progress. The first local release run compiled before the engine-phase fix and hit its expected Idle wait failure; final release workspace run now includes both corrections. No product or rules changes remain pending.

Windows persistence rerun 45157620360 passed. Final real-process tests found New Game installation overwrote Idle with Paused while leaving paused=false; fixed common prepare_install to preserve running intent after retiring old execution. Open remains stopped. The process-failure test now starts request 1 with a human move instead of assuming New Game resets request numbering; both MissingResponse and UnexpectedExit and no-extra-move assertions remain. All 9 real-process play/analysis/persistence tests pass. Fresh reviewer confirmed the final fix has no findings. Final release workspace check next.

Integrated into main at e1af145. Final checks: cargo fmt and workspace clippy passed; release workspace passed 1184 tests with 11 ignored; all-feature desktop-session passed 204; real-process Chess play/analysis/persistence passed 9; frontend passed 609; Python GUI scripts passed 176; TypeScript passed. Debug workspace retains the four known Chu watchdog failures tracked by MSH-199. Fresh review and final fix review have no remaining findings. Persistence CI: Linux/macOS passed in https://github.com/jweathers777/mShogi/actions/runs/45156397616 ; Windows passed after deterministic test gating in https://github.com/jweathers777/mShogi/actions/runs/45157620360 . Final production commit routine CI passed in https://github.com/jweathers777/mShogi/actions/runs/45158629737 . Packaged GUI/release campaign not run (not required for this task). Logs uploaded to gs://mshogi-experiments-216675633801/runs/msh-225/ ; no GCP compute or VMs used. Existing MSH-199 is the test follow-up; next ready medium-priority task is MSH-218 (Shared Protocol Runner).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Chess now uses the shared minimal document and NativeSession lifecycle. Accepted match commands preserve claims, offers and native conclusions; Full analysis remains read-only and results stay in session memory. Removed about 5,000 net lines of separate archive/session code. Release workspace, frontend, process and affected-platform persistence checks passed; known debug Chu watchdog failures remain in MSH-199. Fresh review findings fixed. Integrated into main.
<!-- SECTION:FINAL_SUMMARY:END -->
