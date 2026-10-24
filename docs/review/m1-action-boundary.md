# M1 action-boundary review (MSH-13)

Reviewed revision: `codex/rewrite` at `c55db47` plus the MSH-13 change. Scope: design requirements A-02 (explicit state and complete actions) and A-03 (position, history and match state), checked against the boundaries the chess kernel actually implements.

This review covers only interfaces. It implements no shogi variant and claims none. Chu, Tenjiku, tori shogi, shogi and xiangqi remain unimplemented.

## 1. Production boundaries as implemented

All of these live in `mshogi_core::chess` and are **intentionally chess-specific**. There is no shared multi-game trait or type yet. Per AGENTS.md, none should be prebuilt before a second game needs it.

| Boundary | Concrete item | Introduced | Chess-specific aspect |
| --- | --- | --- | --- |
| Square | `Square(u8)`, `Square::new(file, rank)` for 0..8, a1 = 0 | MSH-4 | 8×8 only; `new` rejects anything larger |
| Piece | `Color`, `PieceKind`, `Piece` | MSH-4 | Six orthodox kinds, two colors |
| Position | `Position`: board `[Option<Piece>; 64]`, side, `CastlingRights`, raw EP target, two u32 counters; `from_fen`/`set_fen`/`to_fen`, read-only accessors, full-tuple `Eq` | MSH-4 | FEN is the chess analysis boundary, not a universal save format |
| Complete action | `Action { from, to, promotion: Option<Promotion> }` | MSH-5 | No route, drop or multi-step field; castling is the king's move |
| Checked transition | `Position::apply` (all chess actions), `apply_ordinary` (ordinary subset). Both are private `resolve` → `apply_resolved` on a candidate copy → `validate_structure` → publish | MSH-5, MSH-8 | Effects (captures, rook relocation, EP victim, rights, counters) are derived from the board; the caller supplies only `Action` |
| Rejection | `ActionError` (14 variants, including `CastlingKingAttacked`, `CounterOverflow` and `IllegalPosition`); `UndoError::PositionMismatch` | MSH-5, MSH-8, MSH-9 | Atomic: rejection leaves the whole position unchanged |
| Undo | Opaque `Undo { before, after }`; `Position::undo(&Undo)` restores only when the complete current state equals `after` | MSH-5 | Private in-memory snapshot, never a saved-game format |
| Choice sets | `candidates`, `special_candidates`, `ordinary_candidates` (pseudo-legal), `legal_actions` (legal) | MSH-7 to MSH-9 | Chess action sets |
| Diagnostics | `classify`, `in_check`, `perft`, `divide`, attack queries | MSH-6, MSH-9, MSH-10 | Chess profile semantics |
| Text adapter | `mshogi shell` coordinate notation (`e2e4`, `e7e8q`); fixture coordinate strings | MSH-11, fixtures | Diagnostic and test adapters, not a canonical record or protocol |
| Adapter session state | The console session keeps `history: Vec<Undo>` for its `undo` command | MSH-11 | An adapter-level stack, **not** rules history: repetition and other history-dependent rules are not represented |

## 2. Synthetic cases and their mapping

Executable evidence is in `crates/mshogi-core/tests/action_boundary.rs`:

- **Part 1** exercises the production chess interfaces directly. Its expected results were hand-written and then confirmed with pinned python-chess 1.11.2.
- **Part 2** is an explicitly synthetic, test-only fixture. It uses invented "strider" rules on a 16×16 board: one or two king steps, a capture on each enemy-occupied step, and a second step that may return to the origin. It follows the production checked pattern: resolve from the board, apply to a candidate copy, publish, and keep a full-snapshot undo. Its expected outcomes were hand-inspected.

Part 2 is evidence about a contract shape, not about production behavior.

| Case | Synthetic evidence | Production status | Future adapter and required change |
| --- | --- | --- | --- |
| Square beyond chess range | Squares up to index 255 (`Sq(255)` = (15,15)) are addressable and used by a capture | `Square::new(8..)` returns `None`, which is tested. Chess intentionally cannot express it | Each large game gets its own square type sized for its board (16×16 for Tenjiku). Do not widen chess `Square` or force chess onto 256 squares |
| Equal endpoints, distinct routes | Capture-and-return and step-out-and-return both start and end at (8,8), yet are unequal actions with distinct encodings and different effects | Chess needs no route, by a rules argument: `from` fixes the piece, each orthodox piece reaches a destination in exactly one way, castling is the unique king two-file move, en passant is the unique diagonal onto the raw target, and the promotion choice is explicit. A corpus sanity check confirms no legal set of the 35 corpus positions repeats a triple; it is a check, not a proof | A game with multi-step moves uses an action type carrying an ordered route (and drop/choice fields as its rules require). The chess `Action` stays as is |
| Multiple rule-derived effects | A double capture produces the exact ordered effects capture, step, capture, step (asserted); the effect list is output only | Real multi-effect chess actions, exercised directly: castling (king, rook, both rights, counter), en passant (victim off the destination), capturing promotion on a home rook (capture, replacement, opponent right) | Resolution may return ordered internal effects for diagnostics, but never accepts them as input. Production already enforces this: `Action` has no effect field |
| Atomic rejection | A valid first step followed by an invalid second step leaves the board unchanged, as do an empty and an over-long route. These synthetic rejections all occur during resolution, before any effect is applied | Tested: castling through attack and a missing promotion choice (rejected during resolution), and en passant exposing the king, which is rejected **after** effects are applied to the candidate copy. Rejection after effects is therefore shown by production, not by the fixture | Same candidate-copy-then-publish pattern; no partial turn is ever published |
| Complete reversal | A double capture restores the exact start; repeated undo is rejected | Full-snapshot `Undo` plus the MSH-12 sequence suite (every ply, 92 start positions) | Each game's undo restores every field and any future cache (A-03). Its layout stays private |
| Partial selection | Shown by type design: a `Selection` holds no board reference, so selecting or abandoning steps cannot change state; only a committed complete action reaches the checked boundary. The test illustrates the shape and cannot fail by construction | Production has no partial-selection API, by design. The console accepts only complete actions | Selection state lives in the UI or adapter. It filters by `legal_actions` (or a game-specific query) and commits one complete action |
| Serialized identity | `239>255>239` round-trips; it carries origin and route only; an effects annotation or an out-of-range index is rejected | The console text `e2e4`/`e7e8q`/`e1g1` round-trips chess identity (MSH-11 process tests, MSH-12 differential) | A future canonical game record stores a rules-profile ID, the initial state, and each complete action in its game's own action encoding (route and choices included, effects excluded). The final schema is deliberately not frozen here |

## 3. Exercised and not exercised

**Exercised directly in production:**

- the `Square` bounds;
- the action-identity uniqueness of chess;
- the real multi-effect actions: castling, en passant and capturing promotion;
- atomic rejection by castling, a missing promotion choice and en passant exposure;
- exact undo.

These add to the existing MSH-5 to MSH-12 suites.

**Exercised only synthetically:**

- squares beyond 8×8;
- route-distinguished actions with equal endpoints;
- route-step rejection;
- UI selection modelling;
- route serialization.

**Not exercised:**

- effects on squares that are neither visited by a route nor its destination (chess en passant covers only a single off-destination victim). A future action type must not assume its effects lie along its route;
- a rules-owned history layer;
- drops and hands, promotion zones and optional promotion, repetition and other history-dependent outcomes, match commands (resignation, draw claims), clocks, and any shared trait across games. These belong to later milestones (M2 match semantics, M4 variants) and the coverage ledger.

## 4. A-02 / A-03 assessment

- **A-02 complete actions:** satisfied for the implemented game.
  - Chess actions are complete player choices.
  - Effects are rule-derived, and no client victim list exists.
  - Rejection is atomic.
  - Partial selection is not a turn.
  - Representing multi-step routes, drops and capture choices is a requirement for the future action types of the games that need them. A-02 explicitly leaves the types to implementation, and this review found nothing in the chess boundary that would force those games into the chess `Action`.
- **A-03 state and history:**
  - `Position` holds only local saved state.
  - History, repetition and match state are absent and recorded as deferred.
  - `Eq` is saved-state identity, documented as distinct from repetition and cache identity.
  - `Undo` restores every field. The type is public and opaque, and its layout is private, so public records cannot depend on it.
  - `Position::undo` decides by full saved-state equality, so an identical post-state reached in another context is indistinguishable. That is correct for a local position without history, but see required change 6.
- **Conflicts found: none.** No follow-up blocks MSH-14.

## 5. Recorded limits and required future changes

1. Every future game defines its own square, position, action and undo types inside its rules profile. A shared trait is introduced only when a second implemented game needs one (AGENTS.md: no speculative universal framework).
2. Route-bearing actions: each real profile defines its own step and effect semantics from its recorded rule references. (The fixture's strider convention, such as vacating the origin, is invented and prescribes nothing.) Whatever the semantics, the whole action must be validated, including checks after effects, before anything is published.
3. Text and record formats in this repository (console coordinates, fixture action strings, `sequence-regressions` JSON) are chess adapters. A multi-game record format must carry the rules-profile ID and per-game action encodings, without effects. The console has no game or profile selector yet; adding one is additive.
4. The current `Undo` snapshot costs a full position copy per action. That is acceptable for chess diagnostics. A larger game or search may replace it with incremental undo only with the same restoration guarantee and the MSH-12-style sequence checks.
5. The synthetic strider rules are invented and do not describe the Chu or Tenjiku lion, igui or any real rule.
6. History-bearing state: games and match rules whose outcomes depend on history (repetition, for example) need a rules-owned history layer (A-01, A-03), separate from adapter undo stacks such as the console session. Its undo must restore history, repetition tables and any caches as well as the position. This is deferred with M2 match semantics and is not a conflict for the local chess position.
