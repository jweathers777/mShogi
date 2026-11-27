# Xiangqi consumer contract (MSH-84)

This is the consumer-readiness boundary of the existing
`mshogi_core::xiangqi` public module plus the concrete `game` record/command
adapter. It composes completed MSH-91–95 contracts; it introduces no new
rules profile, universal interface, engine, console or desktop implementation.
The [selected profile](profile-proposal.md), recorded decisions
and independently reviewed fixtures remain authoritative. MSH-45/M4 stay open.

## Entry points and responsibility

Consumers use the public module, not its private conduct, structural-transition
or proof-key helpers. Related public types have deliberately different roles:

| Consumer operation | Entry point / returned authority | Boundary |
| --- | --- | --- |
| Identify the game and rules | `PROFILE_ID`, `PROFILE_EDITION`; `GameKind::Xiangqi`, `RulesProfile::XiangqiWxf2018V1` | Profile identity/edition is immutable; record schema, envelope1 and shared transport 3 are separate identities. |
| Describe a board | `Position`, `Intersection`, `Piece`, `PieceKind`, `Side`, `FILES`, `RANKS`, `RIVER_BETWEEN_RANKS` |90 intersections, not squares. Red/Black, river and palace geometry remain. No promotions, hands, drops, declarations or compound routes apply to this selected game. |
| Admit a root | `PositionRoot::{standard,from_fen,analysis}`, `Position::validate_local`, `Game::from_root` | Structural inventory/General-count admission is weaker than local geography/turn safety; neither proves prior reachability. Imported roots begin known history; supplied counters do not invent it. |
| Obtain complete local choices | `game.position().legal_actions()`, `local_status()` | No General capture action; includes check evasions. `Ongoing` means local actions exist, not that a full WXF game may continue or no-win proof completed. |
| Resolve/check/apply a local action | `resolve_action` → `ResolvedAction::effects`, `apply_resolved` / `apply` → `Undo`; `Position::undo` | Complete origin/destination action and derived mover/capture; exact full before-state guard. A local clone has no actual-game command authority. |
| Submit an actual command | `GameRequest { expected: game.context(), actor, command }`, `Game::submit`; or typed `dispatch_command` | Game admission owns side/turn/terminal/claim/offer rules. The shared adapter returns `GameReply::Xiangqi(Result<Submission,GameError>)`, preserving both rejection-audit result and undo. |
| Inspect history/adjudication | `events`, `warnings`, `natural_count_eligibility`, `conclusion`, `pending_adjudication`, `event_prefix` | Events include same-ply warnings/claims/conclusions; count query is advisory. Snapshots are read-only and do not create a resumable branch. No consumer reimplements WXF classification or claimant arithmetic. |
| Undo actual history | `Game::undo(Submission::undo)` | Restores exact semantic ledger/state, never old runtime revisions or execution. Fresh lineage prevents old handles applying to an equal branch. |
| Save/replay game semantics | `GameRecord::{from_game,from_json,to_game}`; `CanonicalRecord::xiangqi` / strict envelope codec | Complete policy/root/commands/audits/events/warnings/modes/limits/assessment/conclusions, not FEN-only. Fresh runtime on reopen. |
| Read sound fast assessment | `Game::no_win_assessment` or `assess_natural_win` for a local position | Scoped bare-General negative proof, checked zero/one-ply positive witness, fast miss (`Unknown`) or failure. This is the only no-win assessment; there is no exact graph analysis. |
| Shared context/capabilities | `CanonicalRecord`, `GameCommand`, `SharedRequest` / `SharedResponse`, `execution_capabilities` | Existing MSH-95 routing consumes actual game payloads. Xiangqi shared execution is unsupported/unscored with empty capabilities until MSH-97. |

[`position-api.md`](position-api.md), `local-legality.md`,
[`history-contract.md`](history-contract.md) and `shared-routing.md`
provide detailed contracts.
The handoff requires no consumer to rebuild private classifiers or duplicate
movement rules in TypeScript or another engine.

## Game command and outcome authority

Only board `Play` requires the actor to move. Count claims are claimant-specific;
resignation need not consume a ply or belong to the moving side. Agreement offer,
acceptance and decline have their game admission windows. An invalid claim
can be an admitted audited `RejectedNaturalCount`; it is not an illegal action,
win/loss or invented foul. Pre-admission failures do not mutate even the audit.

Automatic ordering remains local mate/stalemate loss, then repetition, then
sound fast neither-can-win, then named automatic valid claims if selected.
Third-occurrence unilateral prohibited conduct records an obligation/warning;
fourth continued fault loses, otherwise the selected applicable repetition draw.
Persistent victim identity and the complete contiguous history determine
conduct; geometric attacks or board repetition equality are insufficient.
`Explicit` keeps claims optional; `AutoValidClaims` records actual automatic
submissions and results. Game events and conclusions already encode those
meanings. Consumers must use them rather than inferring a winner from turn,
clock/process failures or repeated board keys.

## Fast no-win assessment: results and uncertainty

| Result of `Game::no_win_assessment()` | Consumer meaning |
| --- | --- |
| `ProvenNeitherCanWin` | Sound scoped fast proof; Game's fast event ordering owns any actual conclusion. |
| `ProvenNaturalWin { winner, witness }` | Genuine zero/one-ply natural board possibility, not forced play, strength or a win already awarded. |
| `Unknown` | Fast miss. No assertion of winnability or draw. |
| `Failure` | Input/invariant failure; not Unknown and not a game outcome. |

The question is the selected **intrinsic** existential natural-board question:
ordinary local actions to checkmate or no-legal-action/stalemate loss.
Penalty/claim/voluntary/administrative conclusions are not proof goals. In F002
Black can receive a repetition-penalty win without that award proving Black has
a natural board-win continuation. This is not minimax against optimal
resistance. Only this fast recognition applies; there is no exact analysis.

### Operational policy and actual limits

The only canonical mode is `sound_fast_only`.
`Game::conclusion()==None` means **no committed conclusion**; it is not an
`Ongoing` game conclusion or a completed no-win determination. The
command API admits ordinary work under that fast policy and does not claim
universal WXF completeness. `LocalStatus::Ongoing` answers a separate
board-only query.

Fast Unknown or failure must not become an ordinary ongoing/winnable verdict,
a draw or a score. Preserve the authoritative game and complete save path.
Actual `Game::pending_adjudication` specifically represents unresolved automatic
conduct classification and blocks `submit`; fast Unknown is separate.

## Identity audit

| Identity | Contents / owner | Never substitute |
| --- | --- | --- |
| Board/local legality | Rules owner/kind per intersection and turn determine local geometry/safety. `Position` value equality **also includes declared counters**; `ResolvedAction`/`Undo` guard that full value. | Neither Position equality nor a board hash is live-game/request equality. Counter overflow is a representation error, not mate. |
| Repetition/conduct | Private Game board/turn anchor equality plus full contiguous known actions, root-assigned persistent piece IDs/victims and warning obligations for classification. | Identical types swapping locations can return the board while exchanging IDs; a repeated key alone is not responsibility. |
| Complete game/history | The `Game`: root/provenance, claim policy, full command/result/event-end ledger. Derived events remain. | FEN, declared counters, move count or current board cannot replace it. |
| Canonical record | Validated game/profile/edition/schema and complete semantic ledger; strict replay checks redundant events/current state and field presence. Runtime tokens are absent. | Equal canonical bytes after undo/reopen do not mean equal runtime context. Envelope version is not a rules-profile edition. |
| Request/session | Opaque `GameContext`: fresh instance + monotonic revision, without a copy of the history. A fork, replay or reopen is a new instance. | Undo, selection/return, cancellation, replacement and reopen cannot revive retired work. |
| Shared execution | Whole `SharedRequest`: typed canonical context, purpose/target, request ID, session version and limits. | A transport echo or tag does not advertise real Xiangqi engine execution. |

History selections use event counts, not board plies; multiple events can share
a ply.

## Error and resource contracts

| Observation / refusal | Game type | Required interpretation |
| --- | --- | --- |
| Malformed/structural or inadmissible root | `PositionError`, `LocalPositionError`, `GameError::Root` | Diagnostic diagram/admission problem, not a no-action loss. |
| Illegal action / wrong board actor | `GameError::Action(ActionError)`, `WrongActor` | Atomic refusal; preserve detailed cause. `CounterOverflow` is representation exhaustion, distinct from movement/safety illegality. |
| Inadmissible agreement command | `GameError::DrawOffer` | No valid command in that window; not a chess-family result. |
| Concluded / conduct pending | `Concluded`, `AdjudicationPending` | Actual terminal/no-overwrite or explicit unscored pause; local choices cannot bypass it. |
| Rejected claim | `Submission.result == RejectedNaturalCount` plus audit events | Admitted audit with no semantic penalty/board change. Different from pre-admission refusal. |
| WXF warning | `Warning` / `EventKind::Warning` | Active obligation, not a conclusion or process error. |
| Stale / wrong game | `StaleContext`, typed `DispatchMismatch` | No fallback to another game or old execution. |
| Execution/invariant failure | `GameError::Invariant`, fast `Failure` | Separate failure; no manufactured win/loss/draw or positive proof. |
| Strict record rejection | `RecordError` / `EnvelopeError` | Duplicate/unknown/missing/conflicting/unsupported/capacity or replay inconsistency; never normalize through a map before strict parsing. |

Position/FEN limits are 4096/256 bytes. Games, records, the shared
envelope and engine transport have no command, event or byte limit (MSH-268).
No accepted history or observation is discarded.
