# Xiangqi native consumer contract (MSH-84)

This is the consumer-readiness boundary of the existing
`mshogi_core::xiangqi` public module plus the concrete `native` record/command
adapter. It composes completed MSH-91–95/105 contracts; it introduces no new
rules profile, universal interface, engine, console or desktop implementation.
The [selected profile](profile-proposal.md), [owner decisions](owner-decisions.md)
and independently reviewed fixtures remain authoritative. MSH-45/M4 stay open.

## Entry points and responsibility

Consumers use the public module, not its private conduct, structural-transition
or proof-key helpers. Related public types have deliberately different roles:

| Consumer operation | Native entry point / returned authority | Boundary |
| --- | --- | --- |
| Identify the game and rules | `PROFILE_ID`, `PROFILE_EDITION`; `GameKind::Xiangqi`, `RulesProfile::XiangqiWxf2018V1` | Profile identity/edition is immutable; native schema, envelope1 and shared transport3 are separate identities. |
| Describe a board | `Position`, `Intersection`, `Piece`, `PieceKind`, `Side`, `FILES`, `RANKS`, `RIVER_BETWEEN_RANKS` |90 intersections, not squares. Red/Black, river and palace geometry remain native. No promotions, hands, drops, declarations or compound routes apply to this selected game. |
| Admit a root | `PositionRoot::{standard,from_fen,analysis}`, `Position::validate_local`, `Game::from_root` | Structural inventory/General-count admission is weaker than local geography/turn safety; neither proves prior reachability. Imported roots begin known history; supplied counters do not invent it. |
| Obtain complete local choices | `game.position().legal_actions()`, `local_status()` | No General capture action; includes check evasions. `Ongoing` means local actions exist, not that a full WXF game may continue or no-win proof completed. |
| Resolve/check/apply a local action | `resolve_action` → `ResolvedAction::effects`, `apply_resolved` / `apply` → `Undo`; `Position::undo` | Complete origin/destination action and derived mover/capture; exact full before-state guard. A local clone has no actual-game command authority. |
| Submit an actual command | `GameRequest { expected: game.context(), actor, command }`, `Game::submit`; or typed `dispatch_command` | Native admission owns side/turn/terminal/claim/offer rules. The shared adapter returns `NativeReply::Xiangqi(Result<Submission,GameError>)`, preserving both rejection-audit result and undo. |
| Inspect history/adjudication | `events`, `warnings`, `natural_count_eligibility`, `conclusion`, `pending_adjudication`, `event_prefix` | Events include same-ply warnings/claims/conclusions; count query is advisory. Snapshots are read-only and do not create a resumable branch. No consumer reimplements WXF classification or claimant arithmetic. |
| Undo actual history | `Game::undo(Submission::undo)` | Restores exact semantic ledger/state, never old runtime revisions or execution. Fresh lineage prevents old handles applying to an equal branch. |
| Save/replay native semantics | `GameRecord::{from_game,from_json,to_game}`; `CanonicalRecord::xiangqi` / strict envelope codec | Complete policy/root/commands/audits/events/warnings/modes/limits/assessment/conclusions, not FEN-only. Fresh runtime on reopen. |
| Read sound fast assessment | `Game::no_win_assessment` or `assess_natural_win` for a local position | Scoped bare-General negative proof, checked zero/one-ply positive witness, fast miss or failure. Ordinary queries never invoke the exact graph. |
| Explicit exact assessment | `ExactAnalysis::{new,select,begin}`, `ExactRequest::run`, `ExactAnalysis::accept` | Native read-only purpose/target/history-bound work. Retain `ExactObservation` and `ExactReport`, not only the projected `assessment()`. No engine process is launched. |
| Shared context/capabilities | `CanonicalRecord`, `NativeCommand`, `SharedRequest` / `SharedResponse`, `execution_capabilities` | Existing MSH-95 routing consumes actual native payloads. Xiangqi shared execution is unsupported/unscored with empty capabilities until MSH-97. Native exact analysis is not a Chess Full or shared-engine operation. |

[`position-api.md`](position-api.md), [`local-legality.md`](local-legality.md),
[`history-contract.md`](history-contract.md), [`shared-routing.md`](shared-routing.md)
and [`exact-verification.md`](exact-verification.md) provide detailed contracts.
The handoff requires no consumer to rebuild private classifiers or duplicate
movement rules in TypeScript or another engine.

## Native command and outcome authority

Only board `Play` requires the actor to move. Count claims are claimant-specific;
resignation need not consume a ply or belong to the moving side. Agreement offer,
acceptance and decline have their native admission windows. An invalid claim
can be an admitted audited `RejectedNaturalCount`; it is not an illegal action,
win/loss or invented foul. Pre-admission failures do not mutate even the audit.

Automatic ordering remains local mate/stalemate loss, then repetition, then
sound fast neither-can-win, then named automatic valid claims if selected.
Third-occurrence unilateral prohibited conduct records an obligation/warning;
fourth continued fault loses, otherwise the selected applicable repetition draw.
Persistent victim identity and the complete contiguous history determine
conduct; geometric attacks or board repetition equality are insufficient.
`Explicit` keeps claims optional; `AutoValidClaims` records actual automatic
submissions and results. Native events and conclusions already encode those
meanings. Consumers must use them rather than inferring a winner from turn,
clock/process failures or repeated board keys.

## Fast versus exact: result origin and uncertainty

Keep the original carrier and exact target in an observation:

| Origin / carrier | Actual result | Consumer meaning |
| --- | --- | --- |
| Fast: `Game::no_win_assessment()` | `ProvenNeitherCanWin` | Sound scoped fast proof; Game's existing fast-mode event ordering owns any actual conclusion. |
| Fast: same query | `ProvenNaturalWin { winner,witness }` | Genuine zero/one-ply natural board possibility, not forced play, strength or a win already awarded. |
| Fast: same query | `UnknownExactUnavailable` | Stable historical wire spelling for a fast miss; the explicit exact backend now exists. No assertion of winnability or draw. |
| Exact: accepted observation/report | `ExactVerdict::NeitherCanWin` | Complete reachable graph exhausted, not a material heuristic. Exact work statistics/limits remain attached. |
| Exact: same carrier | `NaturalWin { winner,reason,witness }` | Replayable finite checkmate or stalemate path. Distinct from fast evidence and from a forced-win result. |
| Exact: same carrier | `Unknown { reason }` | Incomplete/cancelled work; neither positive capability nor negative proof. |
| Fast or exact | `Failure` / `ExactVerdict::Failure { reason }` | Input/limit/invariant failure; not Unknown proof evidence and not a game outcome. |

`ExactReport::assessment()` is a convenience projection into the native
assessment vocabulary. It intentionally drops fast/exact origin and the exact
terminal reason; retain the request/observation/report for provenance. It maps
incomplete exact work to `UnknownExactIncomplete`, not a fast miss or positive
win claim. No exact verdict is an evaluator score.

The proof graph is the owner-selected **intrinsic** existential natural-board
question: ordinary local actions to checkmate or no-legal-action/stalemate loss.
Penalty/claim/voluntary/administrative conclusions are not proof goals or graph
barriers. This selected domain is why placement+turn is sufficient for the exact
key; actual WXF live-game identity still requires full history. In F002 Black
can receive a repetition-penalty win without that award proving Black has a
natural board-win continuation. This is not minimax against optimal resistance.

### Operational policy and actual native limits

The currently supported canonical mode is explicitly `sound_fast_only`.
`Game::conclusion()==None` means **no committed conclusion**; it is not an
`Ongoing` game conclusion or a completed no-win determination. The existing
native command API admits ordinary work under that recorded fast policy; it
does not silently run an exact graph or claim universal WXF completeness.
`LocalStatus::Ongoing` answers a separate board-only query.

When a consumer genuinely needs unresolved no-win work to determine an outcome,
the approved policy is to hold the experiment/host in **unresolved, unscored**
state and invoke the bounded explicit path if appropriate. Fast Unknown, exact
Unknown, cancellation or failure must not become an ordinary ongoing/winnable
verdict, a draw or a score. Preserve the authoritative game and complete save
path. An explicit retry may record new limits/traversal; an unchanged request
is not promised to succeed. This is the existing MSH-105/profile policy, not a
new rules adaptation selected by MSH-84.

`ExactAnalysis::accept` consumes only the precisely current request and returns
read-only proof data. It does not install a conclusion, resolve native repetition
classification pending state or overwrite an earlier actual conclusion. Future
MSH-97/100/102 consumers must integrate that approved host policy and retain
observations at their own execution/archive boundaries. They cannot treat this
handoff as complete product adjudication or assume an undocumented command for
committing exact proofs. No such command or new canonical mode is added here.
Actual `Game::pending_adjudication` specifically represents unresolved automatic
conduct classification and blocks `submit`; exact-analysis Unknown is separate.

## Identity audit

| Identity | Contents / owner | Never substitute |
| --- | --- | --- |
| Board/local legality | Native owner/kind per intersection and turn determine local geometry/safety. `Position` value equality **also includes declared counters**; `ResolvedAction`/`Undo` guard that full value. | Neither Position equality nor a board hash is live-game/request equality. Counter overflow is a representation error, not mate. |
| Repetition/conduct | Private Game board/turn anchor equality plus full contiguous known actions, root-assigned persistent piece IDs/victims and warning obligations for classification. | Identical types swapping locations can return the board while exchanging IDs; a repeated key alone is not responsibility. |
| Complete game/history | Opaque `GameContext`: root/provenance, claim policy, full command/result/event-end ledger and runtime token. Derived events remain native. | FEN, declared counters, move count or current board cannot replace it. |
| Canonical record | Validated game/profile/edition/schema and complete semantic ledger; strict replay checks redundant events/current state and field presence. Runtime tokens are absent. | Equal canonical bytes after undo/reopen do not mean equal runtime context. Envelope version is not a rules-profile edition. |
| Exact proof state | Private46-byte full placement+turn key for the independently reviewed intrinsic goal, normalized private counters; per-run cache only. | This is not a general game search cache, repetition state or history-sensitive score key. |
| Native request/session | `GameContext` fresh instance + monotonic revision; `ExactAnalysis` fresh session + monotonic generation, purpose/target/limits/full context equality. | Undo, selection/return, cancellation, replacement and reopen cannot revive retired work. Preserve the original carriers. |
| Shared execution | Whole `SharedRequest`: typed canonical context, purpose/target, request ID, session version, real process generation and limits. | A transport echo or native tag does not advertise real Xiangqi engine execution. Native in-process exact requests have no engine process ID to invent. |

History selections use event counts, not board plies; multiple events can share
a ply. The exact request retains the full source record and the selected event
boundary; the proof uses only that boundary's board, never a future board or
future repetition data. Every historical selection must go through
`ExactAnalysis::select`, including returning to an equal live board. Its host
also signals cancellation; retirement independently rejects any late result.

## Error and resource contracts

| Observation / refusal | Native type | Required interpretation |
| --- | --- | --- |
| Malformed/structural or inadmissible root | `PositionError`, `LocalPositionError`, `GameError::Root` | Diagnostic diagram/admission problem, not a no-action loss. |
| Illegal action / wrong board actor | `GameError::Action(ActionError)`, `WrongActor` | Atomic refusal; preserve detailed cause. `CounterOverflow` is representation exhaustion, distinct from movement/safety illegality. |
| Inadmissible agreement command | `GameError::DrawOffer` | No valid command in that window; not a chess-family result. |
| Concluded / native conduct pending | `Concluded`, `AdjudicationPending` | Actual terminal/no-overwrite or explicit unscored pause; local choices cannot bypass it. |
| Rejected claim | `Submission.result == RejectedNaturalCount` plus audit events | Admitted audit with no semantic penalty/board change. Different from pre-admission refusal. |
| WXF warning | `Warning` / `EventKind::Warning` | Active obligation, not a conclusion or process error. |
| Stale / wrong game | `StaleContext`, typed `DispatchMismatch` | No fallback to another game or old execution. |
| Native capacity / record capacity | `GameError::Capacity`, `RecordError::Capacity`, `EnvelopeError::Capacity` | Refuse before acceptance at that boundary; retain current complete saveable session. |
| Exact resource / cancel | `ExactVerdict::Unknown`, `ExactStop::{States,Transitions,Memory,Deadline,Witness,Cancelled}` | Explicit incomplete work reason, no score/result. |
| Execution/invariant failure | `GameError::Invariant`, `ExactFailure::{InvalidPosition,TransitionInvariant,UnsupportedLimits}`, fast `Failure` | Separate failure; no manufactured win/loss/draw or positive proof. |
| Strict record rejection | `RecordError` / `EnvelopeError` | Duplicate/unknown/missing/conflicting/unsupported/capacity or replay inconsistency; never normalize through a map before native strict parsing. |

Position/FEN limits are4096/256bytes. Native games bound512commands (including
claim audits),2048events and1MiB encoded JSON. Admission reserves the documented
native close path. The1MiB shared envelope includes its own overhead: a native
record fitting its own limit does not automatically fit the wrapper/transport.
The MSH-95 composition/boundary tests remain applicable; future adapters must
measure their actual complete encoding and preflight before committing. No
accepted history or observation is discarded and Save/Open does not reset limits.

Current measured exact defaults:20,000states,200,000edges,250ms,4MiB owned
capacity,4096witness plies, DFS. Configurable ceilings:1Mstates,20Medges,60s,
128MiB and4096plies. These are current explicit settings, not permanent universal
numbers. Preserve them with each observation and keep all work explicitly
invoked/cancellable. CPU time/RSS is not the owned-capacity statistic. Exact
reconstruction is not launched implicitly by ordinary record restore; unsupported
`bounded_exact` restoration is explicitly rejected. See MSH-105's retained
measurements and limits; no general feasibility conclusion is added here.

## Verification and remaining owners

The [MSH-84 evidence](native-handoff-evidence.md) maps focused integrated journeys
and reused independently reviewed fixtures/references to this contract. No
production refactor or shared framework was necessary. MSH-96 owns the real
console; MSH-97 owns functional evaluator/search/child execution and its exact
adjudication consumption; MSH-98/99 own session/presentation and archives;
MSH-100–103 own protocol/matches and product/complete-game journeys. MSH-106
starts only after complete MSH-45 delivery, not this native handoff. MSH-45/M4
remain open and all completed Chess/Shogi evidence remains intact.
