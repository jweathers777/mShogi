---
id: MSH-6
title: Implement chess attack detection with independent examples
status: In Progress
assignee:
  - '@claude'
created_date: '2026-10-22T13:26:00Z'
updated_date: '2026-10-23T17:07:00Z'
labels:
  - rules
  - attacks
milestone: m-1
dependencies:
  - MSH-4
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 6000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Provide a precise attack query that legal move validation can rely on.

## Design references

A-01; A-04; M1 king safety

## Scope

Implement pawn, knight, king, and sliding attacks with occupancy and board-edge handling. Document attack semantics separately from complete legal move generation.

## Verification expectations

Use independent fixtures for boundaries, blockers, pins, king adjacency, and relevant occupancy changes. Test each piece category and both sides.

## Out of scope

Search evaluation, generalized royal-piece logic for all variants, and using the same generator as both implementation and oracle.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Attack queries cover all chess piece categories and both colors under documented semantics.
- [ ] #2 Sliding blockers and board boundaries are correct in curated independent cases.
- [ ] #3 Attack semantics do not accidentally exclude an attack merely because moving the attacking piece would expose its own king.
- [ ] #4 Tests include king adjacency and occupancy-sensitive examples needed by subsequent special-move legality.
- [ ] #5 The implementation remains independent of search and GUI state.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Claim MSH-6 from clean codex/rewrite fa6bd3e in a new isolated worktree; review design A-01/A-04, profile attack semantics, MSH-4 private validation scan and MSH-5 action code.
2. Add a public geometric attack API on Position: attackers(square, by) iterator and is_square_attacked(square, by), covering pawn (color-directed captures, independent of target occupancy), knight, king, bishop/rook/queen rays with first-blocker semantics and board edges. No pins/legal-move filtering, no EP or castling special effects. Replace the private FEN validation scan with this one implementation.
3. Document semantics (geometric, occupancy-only, pinned and king attackers count, own-occupied targets count as defended, attacked-by does not mean capturable) in the profile/API docs and coverage ledger.
4. Add a separate curated fixture family mshogi.chess.attacks.v1: hand-reviewed diagrams for every piece category and both colors, edges/corners, blockers and x-rays, pins/pinned attackers, king adjacency, castling transit, EP occupancy change and before/after blocker pairs. Each case stores exact attacker sets for curated queries plus complete attacked-square maps for both colors. Expected answers established only by pinned chess==1.11.2 via a non-rewriting checker tool.
5. Rust integration test consumes the fixtures (exact sets + exhaustive maps), plus the unchanged legacy corpus attack queries; core keeps no production dependencies.
6. Run focused and workspace checks; obtain independent review; resolve findings; integrate to codex/rewrite with normal push; inspect native CI; record evidence and close.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-23): owner @claude (Claude Code), branch claude/next-task in worktree .claude/worktrees/next-task created from codex/rewrite fa6bd3e (equal to origin/codex/rewrite). Existing Codex worktree /Users/johnweathers/.codex/worktrees/rewrite-bootstrap/mShogi is clean on codex/rewrite and untouched; no other task In Progress/In Review. Ready+ready lists MSH-6 and MSH-13; MSH-6 selected by ordinal. Global backlog CLI is 1.55.0; all record mutations use pinned backlog.md 1.55.1 installed in a session scratch directory. Rust 1.89.0 pinned.

Implementation candidate on parent fa6bd3e: new crates/mshogi-core/src/chess/attack.rs adds public Position::attackers(square, by) (a1..h8 iterator) and Position::is_square_attacked(square, by). Geometric semantics: color-directed pawn diagonals independent of target occupancy, knight/king offsets, sliders up to and including first blocker of either color, no x-ray, no edge wrap, pinned pieces/kings still attack, own-occupied targets count as defended, no EP/castling attacks. The former private FEN validation scan was removed; structural king safety and ordinary-action path checks now share this one implementation (A-01). No caches, bitboards, unsafe or dependencies. Semantics documented in docs/rules/chess-position-v1.md (new Rust attack query API section) plus rustdoc with doctest; coverage ledger, development guide, fixture README and console profile coverage line updated.

Fixtures: new separate family tests/fixtures/chess/attacks.json (mshogi.chess.attacks.v1): 23 curated positions, 131 hand-written exact attacker sets and complete white/black attacked-square maps (2,944 map answers). Diagrams/queries/sets were written by hand from FIDE/profile rationale first; pinned chess==1.11.2 (module SHA verified) Board.attackers confirmed all 131 hand sets and is_attacked_by enumerated the maps. Includes castling transit both colors, both colors' EP before/after occupancy, king blocking own check ray, pinned attackers both colors, double check, edges/corners, friendly battery. Verify-only checker tools/check_chess_attacks.py.

Local verification PASS, macOS 27.0 arm64, rustc/cargo 1.89.0:
- cargo test -p mshogi-core --test attacks --locked (4/4: fixtures; original corpus 3 attack queries; pinned-attacker; occupancy changes via apply_ordinary)
- oracle venv python -I tools/check_chess_attacks.py: PASS 23 positions, 131 attacker sets, 2944 map queries
- Focused hand examples in attacks.rs separately confirmed with the same oracle.
- Mutation check (temporary, reverted): pawn direction, knight offset, king Chebyshev, diagonal zero-offset and ray-blocker mutations each fail the attacks suite.
- cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo test --workspace --locked (5 CLI + 4 attacks + 8 FEN + 6 ordinary + 2 doctests); cargo build --workspace --locked; mshogi profile smoke; cargo tree -p mshogi-core --edges normal (core only); whitespace diff check.
Native Linux/Windows CI pending integration.

Independent review: fresh delegated reviewer (Claude subagent, no repository edits) inspected AGENTS, task, design/profile and the complete bounded diff. No blocking findings; attack.rs correctness, unchanged FEN-validation behavior and independence confirmed. Should-fix: (1) pinned attackers were only black although docs said both colors; (2) one EP sentence overclaimed that the victim square is never reported attacked. Nits: no last-rank pawn case; missing EP diagonal-opening, pawn-covered castling square and White b1 cases; one rationale/tag mislabelled e2 as pinned. Resolution: added 6 hand-curated oracle-confirmed cases (white pinned knight covering a king flight square, both colors' last-rank pawn checks, EP diagonal before/after, White castling pawn/rook cover), corrected EP wording, rationale and tag, and updated counts to 29 positions / 155 exact attacker sets / 3,712 map answers. Not added: an EP-answerable pawn check, because that is legal-move behavior for MSH-8/MSH-9. Re-review: every finding resolved; the remaining f5 flight-square wording nit was then fixed. Reviewer bounded differential probe, run twice (original and final code), in scratch outside the repo: 1000 positions (500 random reachable playouts up to 160 plies plus 500 random valid placements, seed 7), full ordered attackers and is_square_attacked for every square and color against pinned chess 1.11.2: 128,000 queries, 0 mismatches. Bounded evidence, not a committed MSH-12 harness. Final local reruns PASS: oracle checker 29/155/3712; cargo test --workspace --locked (5 CLI + 4 attacks + 8 FEN + 6 ordinary + 2 doctests); fmt/clippy clean; mutation check still fails on all five injected faults.
<!-- SECTION:NOTES:END -->
