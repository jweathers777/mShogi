---
id: MSH-225
title: Chess Document and Session Fit
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-22T09:57:00Z'
updated_date: '2026-11-23T09:54:00Z'
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
- [ ] #1 Measured comparison of the Chess session and document against the generic ones, with each difference classed as rules, product or history; result recorded in the task
- [ ] #2 Chess uses the generic document and session, or the parts that are the same are shared code with no copies; the reason for any remaining separation is recorded
- [ ] #3 Desktop-session and workspace tests pass; one fresh-agent review
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
<!-- SECTION:NOTES:END -->
