---
id: MSH-13
title: Review real action boundaries with a bounded compound-action fixture
status: Done
assignee:
  - '@claude'
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-24T19:46:00Z'
labels:
  - architecture
  - variants
  - verification
milestone: m-1
dependencies:
  - MSH-5
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 13000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Catch public-interface assumptions that would prevent the promised larger games by reviewing the position and transition boundaries the chess implementation actually establishes.

## Design references

P-01; A-02; A-03; M1 compound-action exercise; MSH-4 and MSH-5 implementation boundaries

## Scope

Identify the concrete position, action, transition, and undo types/functions implemented by MSH-4 and MSH-5. Use a small explicitly synthetic fixture to explore a board index beyond chess size, distinct complete actions with equal endpoints but different routes, multiple rule-derived effects, atomic rejection, and complete undo. Exercise applicable production interfaces directly. Where a chess-specific type intentionally remains chess-specific, provide an explicit reviewed mapping to a future adapter and identify the changes that mapping requires. A disconnected toy model alone does not satisfy this task. Review how future interface serialization can preserve action identity without freezing its final schema.

## Verification expectations

Record the concrete production boundaries, which the fixture exercises, which it does not, and how each synthetic case maps to them. Use executable checks for the synthetic transition/effect cases, with independently inspected expected outcomes. Distinguish fixture evidence from claims about existing production behavior. Resolve any conflict with A-02/A-03 or create an explicit blocking follow-up and make MSH-14 depend on it before milestone closure. Keep this separate from supported-game claims.

## Out of scope

Implementing Chu or Tenjiku, inventing their rules, building a general rules DSL, requiring a universal action type, forcing chess to use a 256-square representation, or freezing the final public wire format.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The review names the actual position/action/transition/undo boundaries from MSH-4 and MSH-5, states which are intentionally chess-specific, and maps every synthetic case to those boundaries or a concrete future adapter.
- [x] #2 A square beyond the chess range and two complete actions with equal endpoints but distinct routes remain distinguishable in the reviewed mapping; applicable shared production interfaces are exercised directly.
- [x] #3 Executable synthetic checks demonstrate multiple rule-derived effects, complete reversal, and atomic rejection; the evidence explicitly distinguishes fixture-only behavior from production guarantees.
- [x] #4 The mapping distinguishes partial UI selection from a committed complete action, preserves action identity for future serialization, and does not make client-provided effects authoritative.
- [x] #5 Required changes and limits are recorded; any actual conflict with A-02/A-03 is resolved or tracked by a follow-up that blocks MSH-14, and no additional real game is claimed as implemented or validated.
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
1. Claim MSH-13 on claude/next-task at c55db47 (equal to origin/codex/rewrite), worktree .claude/worktrees/next-task. Owner-requested queue: MSH-13 then MSH-14. Inventory the real production boundaries from MSH-4..MSH-11 (Square, Piece, Position, Action, Promotion, apply/apply_ordinary, resolve/apply_resolved, ActionError, Undo/undo, candidates/legal_actions, the console text adapter).
2. Executable production checks (tests/action_boundary.rs):
   - Square cannot represent a beyond-chess index (the documented chess-specific limit);
   - (from, to, promotion) is a unique action identity in chess, asserted over every corpus legal set, so chess needs no route field;
   - the real multi-effect actions castling (king + rook + both rights + counters), en passant (off-destination victim) and capturing promotion (capture + replacement + captured home-rook right) resolve their effects from the board, not the client, and are atomic on rejection with exact undo;
   - console coordinate text round-trips every legal action identity.
3. Explicitly synthetic, test-only fixture in the same file: a 16x16 board (indices up to 255) with a compound two-step action carrying an ordered route. Two complete actions with equal endpoints but distinct routes (a capture-and-return versus a step-out-and-return) produce different rule-derived effects. It follows the production checked pattern: resolve from the board, apply to a candidate copy, validate, publish, snapshot undo. It covers atomic rejection of an invalid second step, complete reversal, partial selection being non-committal, and a serialization of action identity (route) that carries no effects and ignores none.
   Hand-inspected expected outcomes. No real Chu/Tenjiku rules are claimed or invented.
4. Review document docs/review/m1-action-boundary.md:
   - the boundary inventory, with what is intentionally chess-specific;
   - a mapping table from each synthetic case to the production boundary or a concrete future adapter (per-game Square/Position/Action types, route-bearing actions, a game-tagged action record), with the changes each requires;
   - exercised versus not exercised; fixture versus production evidence;
   - the A-02/A-03 conformance assessment.
   If a real conflict is found, create a blocking follow-up and make MSH-14 depend on it.
5. Independent review, integration, native CI, close.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-24): owner @claude, branch claude/next-task in worktree .claude/worktrees/next-task at c55db47, equal to origin/codex/rewrite; no other task In Progress. The user selected MSH-12 and then asked to carry on until no work remains, so this is the owner-requested queue MSH-13 then MSH-14, with separate records. Pinned backlog.md 1.55.1, Rust 1.89.0.

Implementation candidate on parent c55db47:
- New docs/review/m1-action-boundary.md:
  - an inventory of the real production boundaries (Square, Piece, Position, Action, Promotion, apply/apply_ordinary over private resolve/apply_resolved/validate, ActionError, Undo/undo, candidate/legal sets, diagnostics, the console/fixture text adapters), all intentionally chess-specific with no shared multi-game trait;
  - a mapping table from each synthetic case to its production status and a concrete future adapter change;
  - exercised versus synthetic-only versus not exercised;
  - the A-02/A-03 assessment (no conflict, so no blocking follow-up);
  - recorded limits.
- New crates/mshogi-core/tests/action_boundary.rs:
  - Part 1, production: Square rejects beyond-8x8 indices; (from, to, promotion) is unique over every legal set of the 35 corpus positions, so chess needs no route field; castling, en passant and capturing promotion on a home rook derive every effect from the board, with exact hand-inspected after-FENs and exact undo; castling through attack, a missing promotion choice and EP exposure reject atomically. The hand-written production expectations were independently confirmed with pinned python-chess 1.11.2 (all 5 checks true).
  - Part 2, explicitly synthetic and test-only (invented strider rules on a 16x16 board, explicitly not Chu/Tenjiku): an index-255 square; equal-endpoint capture-and-return versus step-out-and-return as distinct actions with distinct encodings and effects; a double capture with ordered effects and complete reversal; repeated undo rejected; atomic rejection of a bad later step, an own-piece step, an empty route and an over-long route; partial selection not committing; a route identity encoding that round-trips, carries no effects and rejects effect annotations.
- The coverage ledger row and development guide were updated.
Local: cargo test -p mshogi-core --test action_boundary --locked 10/10; fmt; clippy; cargo test --workspace --locked all suites; whitespace diff check. Native CI pending.

Independent review: a fresh delegated reviewer (Claude subagent, no repository edits) verified the inventory against the code, ran the action_boundary suite (10/10), checked every expected FEN and rejection by hand, and confirmed the serialization and equal-endpoint cases.
- It agrees there is no A-02/A-03 conflict: all public chess types are under mshogi_core::chess, no crate-wide type assumes 8x8, Action has no effect field, rejection uses a candidate copy plus validation before publishing, Undo's layout is private, and the regression fixtures carry rules_profile and complete actions without effects.
- Nothing blocking; no follow-up blocks MSH-14; no real Chu/Tenjiku rule is claimed.

Should-fix, resolved:
(1) ActionError is 14 variants, not 12; UndoError was added to the inventory.
(2) The double-capture test now asserts the full ordered effect vector (capture, step, capture, step).
(3) The chess route-uniqueness claim now rests on a stated rules argument; the corpus test is labeled a sanity check, not a proof.
(4) Limit 2 reworded: each real profile defines its own step and effect semantics from its references; the strider convention prescribes nothing.
(5) The Not-exercised list now includes effects on squares neither visited nor the destination.
(6) Added required change 6, a rules-owned history layer with undo that also restores history, repetition tables and caches. The console history Vec<Undo> is now named as adapter state, and the equality-based undo limitation is stated.

Nits, resolved:
(7) Partial selection is described as guaranteed by type design.
(8) The synthetic rejections happen before effects, while rejection after effects is shown by the production EP case.
(9) Undo is public and opaque with a private layout.
(10) The console has no game/profile selector (additive later).
Not done: (11) the stale task file name is left to the Backlog CLI.
Reruns PASS: action_boundary 10/10, fmt, clippy, whitespace diff check.

Integration and native verification (2026-10-24): implementation 4f4025ca211eb7edd7b6d00b23a9dcddfb21198e committed on claude/next-task and fast-forward pushed to origin codex/rewrite (c55db47..4f4025c). No force push, PR, default-branch or settings change.

Actual native CI PASS for exact head 4f4025c: https://github.com/jweathers777/mShogi/actions/runs/40126682337. All three jobs succeeded on rustc 1.89.0:
- Rust (ubuntu-24.04), job 120969524763
- Rust (macos-15), job 120969524745
- Rust (windows-2022), job 120969524568
Each job passed action_boundary 10/10 plus every other suite, and the console smoke printed perft 3 8902.

Next: MSH-14.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Reviewed the real action boundaries against A-02/A-03, recorded in docs/review/m1-action-boundary.md.

Inventory: the actual chess Square, Piece, Position, Action, Promotion, apply/apply_ordinary (private resolve/apply_resolved/validate, publish), ActionError and UndoError, Undo/undo, the candidate and legal sets, the diagnostics, and the console/fixture text adapters plus console adapter history. All are intentionally chess-specific, with no shared multi-game trait, per AGENTS.md.

crates/mshogi-core/tests/action_boundary.rs:
- Production checks, directly: Square bounds; chess action identity (a rules argument plus a corpus sanity check); the real multi-effect actions (castling, en passant, capturing promotion on a home rook) with exact oracle-confirmed after-FENs and undo; atomic rejection, including rejection after effects for EP exposure.
- An explicitly synthetic, test-only 16x16 fixture with invented strider rules: a square beyond 8x8; equal-endpoint actions with distinct routes, encodings and effects; exact ordered multiple effects with complete reversal; atomic route rejection; non-committal partial selection by type design; and an effect-free identity serialization.

Each case is mapped to production or a concrete future adapter: per-game square/position/action types, route-bearing actions, game-tagged canonical records without effects, and a rules-owned history layer.

Assessment: no A-02/A-03 conflict, so there is no blocking follow-up. Limits and required future changes are recorded, and no other game is implemented or validated. Independent review agreed with no blocking findings; its six should-fix accuracy items and nits were resolved. Native CI run 40126682337 passed on all three OSes for 4f4025c on codex/rewrite.
<!-- SECTION:FINAL_SUMMARY:END -->
