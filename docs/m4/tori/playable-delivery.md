# Direct MSH-42 playable Tori delivery handoff

Source inspected for MSH-77 acceptance: production
`52b0390cac09faf727a0c5ed74c1fd673f468a11`, integration `febf4d2`.
The [shared acceptance](shared-acceptance.md) completes the77prerequisite.
This is an executable handoff within **existing MSH-42**, not another research
campaign, new task hierarchy or authorization to start implementation now.
All eight original MSH-42 criteria remain intact and unchecked.

## Delivered foundation and remaining criteria

| MSH-42 | Present evidence / remaining delivery |
| --- | --- |
| AC1 complete named profile | Selected `mshogi.tori.modern.v1`, edition `MSH81-MODERN-2026-11-01`, T01–T08, owner decisions and independent fixtures are accepted under81/85. No new profile ratification or historical acquisition gate. |
| AC2 native rules/history/actions | Accepted `tori::Game`, complete Action/Effects, atomic admission, context/reversal, repetition obligations/conclusions and canonical replay. Consumers must use these, preserving known history versus declared counters/reachability. |
| AC3 independent correctness | Preserve103expected fixtures/713native assertions and native sequence/property/perft/record evidence. Add actual search/console/result/desktop consumer tests; external engines remain optional, not prerequisites. |
| AC4 independently usable console | Missing: main selector/shell/help currently lists onlyChess/Shogi/Xiangqi. Add Tori native command/notation/diagnostic adapter and readable output without pretending SFEN/USI is Tori notation. |
| AC5 functional engine | Missing: no Tori search module, engine result family or execution capability. Add actual native history-aware bounded baseline, honest observations, supervised execution/cancel and reproducible bounded self-play/match smoke; MSH-51 owns measured competitive strength. |
| AC6 complete records | Native/current desktop record, strict replay, settings/history/branches/unsupported observations are delivered. Extend game-specific validation/admission for real new engine observations; retain current saved failures, exact history/provenance and all saveability/atomicity guarantees. |
| AC7 playable desktop | Shared semantic/runtime contract exists, but product admission is gated. Deliver deliberate Tori board/artwork/hands/notation and real human-engine play/analysis/recovery/SaveLoad/history through the same owner/shell. Finished visuals need owner approval. |
| AC8 full delivery review | Remains open. Independently reconcile all eight criteria and game coverage using actual complete-game evidence; keep46/47all-game/platform gates visible. |

## Recommended next Codex assignment: native playable consumer

Begin MSH-42 implementation with one coherent native engine/execution/console
checkpoint under one integrator. These share complete-action/notation/result
semantics and can be developed together; verify each boundary separately.

- **Rules-owned search:** add concrete Tori evaluator/search in
  `crates/mshogi-search/src/tori.rs` and export through `lib.rs`. Existing
  `shogi.rs` and `xiangqi.rs` demonstrate bounded result/iteration/stop patterns,
  not Tori rule authorities. Use a private native `tori::Game`, `legal_actions`,
  `submit`/`GameUndo` and `conclusion`; full history and T06 departure obligations
  matter at every node. `Game` currently has no public Clone: reconstruct the
  private owner from its admitted native record rather than assuming the Shogi
  clone API exists or adding a second rules state. Undo restores semantics while
  runtime authority advances; do not require retired tokens to become equal. `LocalPosition` is appropriate for movement inspection
  and local diagnostics, not history-aware search permission. Keep native
  Sente/Gote, nine faces/handed quails/hands and compulsory resulting faces.
  Establish a transparent baseline and measured finite interactive depth/visit
  limits. Do not inherit Shogi declarations, Chess Full, Xiangqi exact operations,
  board-only caches, heuristic adjudication or evaluation-based resignation.
- **Real supervised results:** `mshogi-engine/src/shared.rs` currently exposes
  no Tori operations and `server.rs` returns unscored UnsupportedMode. Add the
  smallest native Tori result/telemetry dispatch (following existing concrete
  result modules, not a universal framework). Preserve action/finished/incomplete/
  failure distinctions, root-side score meaning, actual work observations and
  exact purpose/history/request/session/process/limits/selected-target identity.
  Use the existing bridge/process owner and bounded wire/diagnostics; no second
  runtime or standalone Tori application protocol is required. Specify schema/
  transport compatibility from actual consumers, preserving historical failures.
- **Admit the real observations end to end:**
  `mshogi-desktop-session/src/shared_tori.rs::accept_tori_observation` currently
  accepts only Failure, its runtime admission has no ordinary engine scheduling,
  and `tori_document.rs` validates only the current unscored Failure family.
  These must evolve together with actual results before claiming engine support:
  native re-admission of proposed complete actions/conclusions, separate read-only
  analysis acceptance, exact PV/target validation, retained observations, stopped
  restore, Resume/retry/cancel semantics and stale suppression. Re-measure actual
  encoded/replay growth and256→257admission/reserved endings for real results;
  the new payload must not create unsaveable accepted work. Preserve old archives
  and meaningful diagnostics; do not silently widen limits.
- **Console and notation:** extend `mshogi-console/src/main.rs` explicit game
  dispatch with a concrete Tori session adapter; reuse bounded line I/O and
  `record_io.rs` atomic canonical operations. Supply standard/custom canonical
  roots, show/hands/status/legal complete choices/move/event undo/history,
  resignation/unscored stops and bounded native perft/divide. Choose documented
  readable game-appropriate notation from actual complete actions, including
  forced result identity and distinct LQ/RQ. A native formatter/parser can be
  shared with Rust Moves projection; no TypeScript movement/notation authority.
  Exercise TORI-C-NOTATION's exact native/display boundary; its example label
  `Fa4c-4b+` is not permission to bypass native admission. Follow the accepted
  multi-character console convention (lowercase native owner prefixes, no colon,
  stable cell widths). Human output and structured internal-search stdout stay
  separate. Do not advertise Standard Shogi USI/SFEN for Tori without a real
  lossless adapter; none is required merely to deliver this native consumer.
- **Functional smoke:** run bounded deterministic self-play using actual native
  records and real internal subprocess requests, recording commands, limits,
  accepted actions, conclusions and unscored interruption/failure. The existing
  experiment crate is `mshogi-runner` (currently Shogi/Xiangqi protocol adapters),
  not a nonexistent `mshogi-match` crate. Add only a concrete needed consumer or
  focused smoke harness; avoid a new generic runner/protocol framework or a
  mandatory independent opponent.

Acceptance for this checkpoint is actual legal native search and subprocess/
console results, correct same-board/different-history and T06 outcomes, exact
reversal, finite work/cancellation, no fallback before completed work, honest
scores/terminal distance and record/admission compatibility. A complete iteration
may remain usable when later work stops only under the documented result contract;
never fabricate a choice, evaluation or game result from incomplete work.
Normal product Tori New Game/Load remains gated until the playable desktop exists.

## Next coherent boundary: the real Tori desktop

Once native execution/observations are real, implement the product rendering and
player journeys together in the existing `ProductApp`/`ApplicationView`/
`GameBoard`/`GameFrame` and native session. The current Tori branch of `GameBoard`
is an explicitly technical verification renderer. Replace its presentation with
appropriate Tori pieces, promotion/ownership orientation and integrated hands;
keep native complete choices and read-only opponent inspection. Reuse the one
`useGameSession`, `useNativeChoiceController` and common shell/theme/document/
analysis/history/actions slots. Do not introduce a ToriApp or copy the lifecycle.

Render all7×7 squares without board scrolling/zoom at supported sizes; make all
nine faces, handed quails, promotion, drops and Sente/Gote clear with pointer,
keyboard, focus, announcements and exact current/historical move cues. Use native
notation/Move projection, excluding non-board events from move rows. Keep developer
information separate from the normal player view. Obtain **owner visual approval
of the actual finished board/artwork** before final product acceptance; this is
not another architecture/research checkpoint.

Enable ordinary Tori product exposure as one intentional checkpoint: current
`PlayableGameKind`/chooser, host `game_command`/`game_new` and
`queue_product_open`/archive guards, native scheduling/capabilities, analysis/
recovery panels and document defaults must agree. Real observations precede any
thinking/score/progress display. Reopen stays stopped until explicit Resume;
Save/Load/Take back/history/Return/cancel/dirty Quit retain exact identity and
complete data. Preserve normal/QA/synthetic package isolation and stable macOS IDs.

## Verification and final acceptance boundaries

For the native checkpoint, inspect exhaustive result/game dispatch in **both**
Cargo workspaces plus relevant frontend types/old unsupported assertions early.
Use focused native search/engine/codec/controller/console tests, then one coherent
cross-game regression checkpoint. Required native three-platform CI covers the
changed source; include standalone host/frontend checks for changed result/DTO
consumers. An automatically triggered full desktop GUI campaign is not by itself
an acceptance gate for unrelated native-only work. Declare actual required jobs
from the changed boundary and preserved criteria before dispatch.

At the desktop integration boundary, require real packaged Tori play/analysis/
cancel/recovery/inspection/history/SaveLoad/Take back/Resume/quit evidence and
appropriate native/desktop CI. Record actual platforms/packages; broad shared
six-game/all-platform acceptance remains46/47/88/89ownership. Do not claim another
platform's GUI from a build or omit any platform evidence required by the active
criteria. Retain independently expected rule fixtures, native history, capacity
boundaries and representative mandatory success versus near-limit typed refusal.

Finish MSH-42 with an independent complete-game eight-criterion review and current
coverage ledger. Reuse unchanged native/shared evidence; add actual game-specific
engine/console/desktop/record journeys and the owner visual verdict. Do not close
MSH-42 on a native-engine checkpoint alone. No new implementation tickets or
planning-only assignments are created here; the next authorized worker can begin
the native playable-consumer checkpoint directly under MSH-42.
