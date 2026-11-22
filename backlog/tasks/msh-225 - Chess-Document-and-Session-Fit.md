---
id: MSH-225
title: Chess Document and Session Fit
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-22T09:57:00Z'
updated_date: '2026-11-22T16:03:00Z'
labels:
  - desktop
  - refactor
milestone: m-6
dependencies:
  - MSH-226
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

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Comparison (2026-11-22, main 14f7184). Chess code: lib.rs 2,486 (DesktopSession, ChessMutationCommand journal, own admission/command/poll/accept paths; tests from line 1810), persistence.rs 3,815 (ChessDocument archive, own file/admission worker, Full analysis certificates; tests from line 1892), analysis.rs 612, takeback.rs 107, capacity.rs 335 (shared), chess_snapshot in shared.rs (~250).
RULES (real, D-11): ChessMatch core records timestamped requests (Start, Move, Claim{reason, intended}, Resign{witness}, OfferDraw/AcceptDraw/DeclineDraw, Observe); optional draw claims with a recorded auto-claim policy; mating-feasibility adjudication. All are commands of one core API (ChessMatch::submit -> MatchReply, events, prefix, replay) and fit NativeGame. The desktop is always untimed, so clocks add only timestamps.
PRODUCT (real, generalize): Chess Full reachability analysis (bounded proof with certificates, cached verification) is a second analysis kind; play limits are time-based (1 s); analysis defaults per kind.
HISTORY (drift): own session struct, own saved format (match + game JSON, ControlEntry journal, TakeBackEvidence instead of retired branches, ArchivedObservation), own admission worker, own capacity checks, own snapshot builder, own browse state. Same jobs as the generic document and session.
Fit: Chess can use the generic document and session if (1) the document record is the match record while the engine context stays CanonicalRecord::Chess(GameRecord) (new NativeGame::context hook used for requests and observation matching); (2) MatchRequest version/at_ns are filled by the Chess submit wrapper; (3) SessionGame gets a hook for extra analysis operations and their result validation (Full); (4) take-back evidence becomes generic retired branches.
Plan: same steps as MSH-224/226; regenerate Chess fixtures (already stale per MSH-221); measure work limit; review.
<!-- SECTION:NOTES:END -->
