---
id: MSH-68
title: Deliver shogi console notation and canonical record adapters
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T15:08:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-67
  - MSH-69
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/shogi-console.md
type: task
ordinal: 68000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Real console process tests, malformed/oversized input atomicity, notation/record fixtures and preserved chess command regression evidence.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The standalone console explicitly selects shogi and can inspect position,hands,turn,complete actions/history/outcomes,apply/undo and run legal/perft/divide diagnostics; existing chess shell/UCI behavior remains compatible.
- [x] #2 SFEN and USI move/drop/promotion text plus the selected human-readable notation preserve complete native choices. Any additional import/export adapter documents losses and rejects unsupported semantics instead of becoming the canonical record.
- [x] #3 Canonical game/match load/save/replay retains all applicable profile/policy/event semantics and bounded hostile-input behavior; diagnostic stdout never contaminates machine protocol output.
- [x] #4 Actual console subprocess journeys cover drops,optional/mandatory promotion,rejection,history,conclusions and record roundtrips with independently expected outputs.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add explicit mshogi shogi human-console entrypoint while preserving chess shell/UCI/internal-search contracts and bounded line reading.
2. Reuse native Game/Position/SFEN/USI and common CanonicalRecord envelope. Native admission owns actions, harmless declaration queries, formal declarations, resignation and conclusions; preserve same-ply terminal events and exact native undo, including restored history.
3. Present complete legal choices with custom explicit source-square long notation (not a named standard), native promoted identity, drop marker and promotion decision; inspect board/hands/side/root provenance/events/outcomes.
4. Add bounded file record load/save/replay, distinct from4096-byte commands: raw strict envelope parsing, complete native record validation before replacing the session, explicit wrong-game/identity/resource failures, atomic no-clobber save publication. No desktop archive expansion.
5. Measure initial/hands branching and select conservative local diagnostic depth; document position-only perft/divide semantics. Verify actual process journeys against independent existing fixture facts, illegal/malformed/oversized atomicity, complete record/undo roundtrips and chess regressions.
6. Obtain separate completed-source review, run proportionate native/subprocess/desktop checks and CI, integrate/push, close68 with separate evidence and archive its checkout before activating70.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Primary builder/integrator @codex on codex/shogi-console, managed /Users/johnweathers/.codex/worktrees/shogi-console/mShogi, based078d9c49287b490d094be6522d153acf9dd01187. MSH67/69 Done; approved native rules/history and shared v3 are established inputs. Pinned Backlog1.55.1 matches AGENTS. Independent read-only fixture audit delegated; no competing builder. Owner explicitly authorizes68 then70;70 remains inactive until68 completion.

Implemented explicit mshogi shogi mode, native Game admission/reversal and loaded-history undo, root/provenance/hand/event/conclusion inspections, complete native USI choices and custom source-explicit readable notation. Eligibility remains advisory; formal declarations/resignation use native semantics and same-ply events. Strict MSH69 canonical-envelope file load validates/replays before replacement;1MiBfile and4096-bytecommand caps are distinct. Save syncs a private sibling then publishes a new file without overwriting via hard link; unsupported filesystems/existing paths fail explicitly without losing session/prior file. No native rules/record/proof changes.

Independent read-only fixture audit supplied established SH-F003/005/010/011 and SH-F028/029/034/037 facts plus a new independently derived two-silver ambiguity root (13 legal choices, distinct4e/6e->5d labels). Product tests then verified these expected facts. Actual console suite cargo test -p mshogi-console --locked PASS57 tests, including8 new shogi process journeys plus preserved chess/UCI/internal and desktop play/analysis/persistence subprocess regressions. cargo fmt and console all-targets clippy -D warnings PASS. Initial large-record test assertion failed because compact12-event record was below4096; corrected test to a genuinely larger valid pretty encoding, without changing semantics or limits, rerun8/8 PASS.

Release diagnostic measurement on macOS27 arm64/Rust1.89 (3 actual process runs/root/depth): initial30/900 nodes depth1/2; Sentehand523/1446; bothhands523/251223. Depth2 medianprocess6.285/4.950/107.912ms; initial coldlaunch248.384ms recorded honestly. Chosen interactive shogi cap2 instead of chess4. See docs/m4/shogi-console.md. Separate completed-source review msh68_final_review is pending; no acceptance checked yet.

Independent completed review msh68_final_review: separate read-only agent traced complete diff/native root+submit+undo+strict raw parser and rebuilt console under separate /tmp target; all57 tests passed independently on Darwinarm64 Rust1.89. Additional actual-process assertions passed for formal-declaration restoration/undo/rejection after conclusion, Defaulted counter provenance through play/undo/re-export, duplicate schema/unknown raw native fields retaining ledger+undo, and symlink no-clobber save preserving victim+cleaning temp. No material findings, no file edits; not a human or cross-platform review. Reviewed shogi_session SHA25603d99a20312c19cbb45752f5d02a24928804ce7a17491fb5977d186b080d1b89, shogi_cli4a3686f9d02f414dda5e06cc99a42370b7d68bc9e7fff5c4a87ce074387ead8f.

Reviewed source8a60b28b5efdf37beeb6e8ae4d2179eb4ac44e16 integrated normally and pushed as ada65923d4ae84c0323ebc5aef1294ae7e968ddc. App-created checkout initially had detached HEAD; the first merge by an assumed branch name correctly refused without changing main. Attached the reviewed commit to codex/shogi-console, retried normal merge/push successfully. Ownership and final source unchanged. Normal CI now pending on exact ada6592: Rust40344086656 and Desktop40344086642. Task remains In Review until current checks and final completion record;70 remains unstarted.

MSH68 final gates PASS at integrated ada65923d4ae84c0323ebc5aef1294ae7e968ddc: native Rust workspace https://github.com/jweathers777/mShogi/actions/runs/40344086656 and Desktop packages https://github.com/jweathers777/mShogi/actions/runs/40344086642, Linux ubuntu24.04/macOS15/Windows2022 all successful, no job rerun. Native format/lint/tests/QA constructors/snapshots/build/console smoke PASS; normal desktop packages and bundled engine checks allthree plus installed GUI journeys Linux/Windows PASS. Existing M3 macOS GUI/native termination/Tao evidence reused unchanged, not a newmacGUI claim. Accepted fixtures, MSH67 profile/proof/history and MSH69 shared boundary unchanged. AC1 explicit actualconsole/nativeinspection/actions/undo/diagnostics + old57-test regression;AC2 complete native USI and source-explicit custom notation independent ambiguity/promotion/drop facts;AC3 strict complete envelope file handling and hostile atomicity;AC4 actual8journeys and independent reviewer rerun/ad-hoc boundary checks. All criteria supported. Source8a60b28 integrated/pushed viaada6592; final task-only completion record is separate. Completed checkout will be archived before70 starts.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed MSH68 only before continuing the authorized queue. Explicit mshogi shogi mode uses authoritative untimed native Game, SFEN/USI, complete source-explicit readable labels, hands/history/conclusions and exact native undo. Advisory queries, formal declarations and same-board-ply terminal commands remain distinct. Strict bounded canonical files retain full root/policy/event semantics; invalid replacements preserve the session; atomic new-file saves never overwrite an existing file. Perft/divide remain position-local with measured depth2 cap.

Reviewed source8a60b28b5efdf37beeb6e8ae4d2179eb4ac44e16 integrated/pushed as ada65923d4ae84c0323ebc5aef1294ae7e968ddc. Local and separate independent57-test console runs PASS, including8 new actualshogi process journeys; independent read-only final source review found no material issue. Native CI40344086656 and Desktop CI40344086642 PASS Linux/macOS/Windows. Reproducible examples, custom notation, measured branching and file semantics documented in docs/m4/shogi-console.md and README. No rule/proof/record-format/desktop archive change or engine capability claim. MSH70 is next explicitly authorized;71/72 and41–47/M4 remain open.
<!-- SECTION:FINAL_SUMMARY:END -->
