> Current delivery review — MSH-103 (2026-10-29): see the
> [complete-delivery acceptance matrix](delivery-review.md) for current source,
> independent review, executed checks and platform-qualified product evidence.
> MSH-114 supplies the capacity-message correction; MSH-103 remains In Review
> pending its separate resumed acceptance/finalization. See the review addendum.
> MSH-113 adds native Take back and retained-history Save/Open. MSH-45/M4 remain
> open; MSH-106 is unstarted. Task-time gates below are historical evidence.

> Current desktop adaptation — MSH-102 (2026-10-29):
> [analysis, outcomes, recovery and document journeys](desktop-journeys.md) now
> use this native contract in the normal product, including strict Xiangqi Open,
> exact-event history, stopped reopen and explicit Resume. Final source is
> `a1f8234e6d82add1c485e7e2537e414cffd4d49d`; see
> [execution/review/CI evidence](../evidence/msh102/completion-2026-10-29/README.md).
> Earlier unstarted/product-gate statements below are preserved historical
> handoffs, superseded by these bounded deliveries. MSH-45/M4 remain open;
> MSH-103 and MSH-106 are not started.

# MSH-98: native Xiangqi session and semantic presentation

Current product adaptation (2026-10-29): [MSH-101](desktop-product.md) implements
untimed human/engine play through this native contract; its actual visual checkpoint
has explicit owner approval. Its New Game/play and
inspection adapter supersedes the initial product gate described below; full
Xiangqi product Open remains gated for MSH-102. The following MSH-98 delivery
description preserves the original native boundary and its evidence.

This delivers the native consumer boundary, not finished Xiangqi desktop UI. MSH-99 adds [lossless native desktop archives/admission](desktop-archive.md); MSH-101 owns the actual intersection-board product in the accepted shared shell. The normal Tauri command endpoint rejects Xiangqi product admission until that work exists. Normal Chess/Shogi new-game, archive and presentation contracts remain available. The Rust library's typed Replace operation admits actual canonical Xiangqi records for native consumers and the separately gated development probe.

## MSH-102 native terminal advice

`xiangqi.terminal_actions` supplies offer/accept/decline eligibility for the human
actor. Core `Game::can_offer_draw` and `can_respond_to_draw` are reused by formal
admission, preserving private offer history and terminal actor semantics apart
from board-action turn ownership. Advice itself never concludes a game. Native
labels distinguish actual conclusions from pending unscored adjudication.

## Authority and execution

`DesktopController` serializes explicit game dispatch. Native `xiangqi::Game` alone admits complete Play, resignation, agreement and claimant-specific count commands. Only board actions require the mover's turn; terminal commands retain their native actor semantics. Invalid structural/illegal/stale commands cannot retire current work or mutate history. Semantically rejected count claims retain the native audit without creating a result. Native semantic Undo consumes real reversal tokens and retires execution; it never rewinds runtime identity.

One existing supervised `EngineBridge` serves Chess, Shogi and Xiangqi, with one current purpose/request owner. Every change of selection, cancel, replacement, semantic undo, new analysis, retry or resume retires prior work. Monotonic document/version/selection/request/process identities never come from restored history. Acceptance compares the complete original request (native canonical root/history/policy, purpose, target, limits and every identity) against the current exact selection. Ordinary MoveAnalysis has a separate read-only acceptance path. An engine-turn proposal must pass native command admission; an analysis proposal is never applied to the live game. Failures, incomplete work and unresolved native adjudication remain distinct and unscored. Cancellation invents no final observation.

Ordinary play uses the actual MSH-97 default (depth1,128 visits); supported ordinary analysis remains depth1–3 and1–512 visits. The engine exports the default from its existing native search constant. Rendering, restoration and ordinary requests perform no exact graph analysis. They retain `sound_fast_only` provenance: Unknown is neither a chess outcome nor proof of positive win capability. Chess Full is explicitly rejected for Xiangqi. The existing MSH-105/MSH-97 separately bounded native exact API and console operation remain unchanged; this task does not advertise an unimplemented shared exact execution operation. Positive existential witnesses remain distinct from forced-win scores.

## Presentation and historical selection

The semantic snapshot supplies native Red/Black labels, owners, piece kinds, complete native actions, ninety intersections, river separation and the two owned palaces with their diagonals, plus both orientations. Intersections use an explicit `intersection` choice origin and native a0–i9 coordinates; the compatibility `PresentedPiece.square` string remains an opaque coordinate key, not a square-cell topology. No hands, promotion or declarations are advertised. No glyph, piece art, board size, scrolling, layout or universal board model is prescribed. Future UI must fit the complete board at supported desktop sizes and preserve the common shell.

Native all-event history viewing remains available, including a warning or claim event inside one atomic command. Each selection has a fresh token. Pieces, warnings, offers, pending state, conclusions, fast assessment and `selected_events` describe that exact immutable prefix, never future facts. `root` preserves original provenance; `selected_position` is explicitly a native diagram, not a fabricated game history.

A canonical game record can describe only a committed command boundary. At such a selection, `selected_canonical` and `SharedSnapshot.context` contain the exact native prefix, and `canonical_context_scope` is `selected_committed`. Inside an atomic command, `selected_canonical` is absent and scope is `live_with_immutable_selection`: `context` explicitly retains the authoritative live canonical record alongside the separately identified immutable selection. Ordinary analysis is disabled and rejected at those intermediate boundaries; view and native movement inspection still work. Count advice is unavailable there rather than borrowed from the future. Returning to live is explicit and leaves play paused until Resume.

Read-only opponent inspection takes the exact snapshot token and selected native position. Core `Position::inspection_actions(owner)` handles palace/river, blockers and General safety; the frontend computes no movement rules. Returned complete choices carry `read_only: true` and cannot authorize an enemy action. Inspection neither polls nor cancels execution.

## Bounds and persistence boundary

Native admission retains existing record byte/command/event bounds and exact parsing. MSH-99 extends the original observation retention guard with [complete archive admission](desktop-archive.md) before every accepted native change: actual encoded bytes, controls, native history, combined ordinary/exact observation count and measured replay work, including reserved stop/close headroom. No limit or deadline was raised and no accepted history/observation is dropped. Save/reopen cannot enlarge a structural limit. The native archive consumer is available; normal Tauri New Game, game commands and product Open remain explicitly gated until the existing UI owners complete them. MSH-61 Chess/Shogi saveability evidence remains applicable and is regression-tested.

## Verification

Development probe (excluded from normal builds by `required-features = ["desktop-verification"]`):

```sh
cargo build --locked -p mshogi-console --bin mshogi
cargo run --locked -p mshogi-desktop-session --features desktop-verification \
  --example xiangqi_session_probe -- target/debug/mshogi
```

This exercises an actual supervised native child: engine turn, read-only live/history analysis, cancellation, Return to live and acknowledged process cleanup. It is native/process evidence, not packaged GUI evidence or approved visual design. No new frontend harness or production test instrumentation was added.

Focused tests cover independent initial44 choices/32 pieces; existing frozen board fixtures; actual native engine-entry responses; wrong purpose/history/limits/IDs/selection; cancellation, cross-game replacement and Undo retirement; native terminal commands at unchanged board ply; complete event-prefix warnings with no future facts; strict duplicate/unknown fields; inspection immutability; and256-to257 observation admission plus encoded-byte reservation. Task evidence records actual revisions, independent review and CI separately.

The external packaged-GUI driver permits one logged retry after a proxy transport reset for explicitly read-only DOM observations, screenshots and element lookups. Generic script execution, input, session lifecycle and native mutations remain single-attempt. Semantic errors and repeated transport failures remain fatal; all journey assertions are unchanged. This verification-only correction follows two retained Linux failures on read-only observations, each followed by successful failure captures in the same session. Driver boundary tests establish retry/non-replay behavior; only actual package runs establish GUI results.
