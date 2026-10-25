# M3 desktop session and interaction contract

Status: accepted implementation contract, 2026-10-25. M2 remains closed; its
[closure evidence](../review/m2-closure.md) and rules/search evidence are reused.
M3 remains open until all deliverables and the final integration review pass.
This is a chess-first slice of the six-game product in [the design](../design.md).

## Deliverables and batch boundary

| Required work area | Tickets | First batch |
| --- | --- | --- |
| 1. Session and interaction contract | MSH-27 | Yes |
| 2. Tauri application and packaging foundation | MSH-28 | Yes |
| 3. Authoritative native controller | MSH-29 | Yes |
| 4. Structured engine connection and supervision | MSH-30 | Yes |
| 5. Board, promotion, orientation, click and keyboard | MSH-31 | Yes |
| 6. Human versus engine; bounded analysis | MSH-32; MSH-33 | Play only |
| 7. Outcomes, recovery and cancellation | MSH-32 minimum safe play; MSH-34 broader recovery | Minimum only |
| 8. Save, reopen and inspect history | MSH-35 | No |
| 9. Synthetic 16 by 16 and compound route fixture | MSH-36 | No |
| 10. Integrated interaction/accessibility verification | MSH-37 | No |
| 11. Packaged verification on three systems; closure | MSH-38; MSH-39 | No |

The first batch is MSH-27–32. Foundation builds native packages on all three
systems and exercises the bundled engine there. That does not close the later
installed-application interaction, accessibility, persistence or M3 review work.
The first batch is complete (MSH-27–32 remain Done). The next authorized batch
is MSH-34, then MSH-33, then MSH-35 after both dependencies complete. MSH-36–39
remain open for subsequent work. Every Done ticket must satisfy its entire
acceptance checklist; M3 remains open.

## Authoritative session

The Tauri 2 Rust host owns a native session controller containing a `ChessMatch`
and uses the existing `AdjudicatedGame`, records and legal action generator.
Frontend state is presentation/selection only. React, strict TypeScript, Vite,
SVG and HTML controls render native snapshots. No JavaScript chess library or
frontend inference of legality, claims or outcomes is permitted.

Host commands, search acceptance and application run in one serialized native
controller. A frontend command includes the expected session version. Stale
commands fail without changing the game. New games and accepted game commands
advance a monotonic session version, independent of ply count or match version.
The host provides pieces, complete legal choices, claim choices, history, turn,
conclusion and engine state. Frontend poll responses also carry a monotonic
snapshot revision so an older async response cannot overwrite newer state.
Selection is cleared on a session change. A promotion is one complete native
choice; selecting its destination alone cannot commit it.

Initial play is untimed, from the standard chess position, with the human able
to select either color. The human may make a move only on their live turn.
Orientation is a view preference, independent of player color. New game and
cancellation remain available while an engine is working. No undo/branch or
historical-position navigation is implied by a displayed move list.

## Internal engine boundary

The bundled native `mshogi` executable retains `uci` compatibility and gains a
small versioned JSON-line internal search mode. The host alone locates and
launches the packaged sibling executable; the webview has no shell permission.
A fresh supervised process per search is sufficient for this slice. Requests
carry protocol version, unique request identity, monotonic session version,
engine-process generation, exact canonical `GameRecord` (including initial
state, known repetition history, claim policy, accepted claims, adjudication
mode and limits) and deterministic depth/node work limits. Responses echo the
entire envelope. A board/FEN comparison is insufficient.

The internal adapter reuses native search/coordination and emits one final
response. Board action, draw claim, already concluded game, no completed search
iteration and engine/search failure remain distinct typed results. Completed
iteration depth, total visited nodes, score perspective, continuation and stop
reason are actual native observations. There is no invented live telemetry.
The host validates all context fields immediately before applying a returned
choice through the authoritative match API. Duplicate, cancelled, replaced,
old-generation and context-mismatched responses cannot alter the live game.

Supervision uses one worker, bounded pending/result storage and bounded input,
stdout and stderr. Process launch, pipe I/O, waiting and cleanup are off the
webview command path. A watchdog terminates a missing/stalled response; cancel,
new game and shutdown terminate obsolete child work. Failures distinguish start,
I/O, unexpected exit, malformed/oversized response, missing response, timeout,
invalid returned choice and search failure. They pause engine play and remain
unscored application failures, never chess draws or wins. Retry starts a new
identity and generation. Cancellation suppresses move application even if a
completed iteration arrives concurrently. OS scheduling is not a hard realtime
guarantee.

## Rules, modes, outcomes and records

Interactive defaults: M2 adjudication, explicit optional claims, untimed match,
fixed bounded shallow search (depth and node cap recorded in request/results).
Checkmate and automatic draws come from the core. A valid current/intended-move
claim is an opportunity until actually exercised; intended moves are not played
by claiming. Engine-selected valid claims are applied as claims, not fallback
board moves. Evaluation scores do not resign or adjudicate draws. Unknown
mating feasibility is not proof that mate is possible. Any unresolved match
conclusion is explicitly unscored; process failures are separate again.

MSH-33 supplies explicit bounded analysis and an independently terminable path
for Full reachability work. Full mode must never run unbounded on a host/UI
event path. The initial internal/play boundary rejects non-M2 records clearly
before replay; it must not silently restore Full as M2. Later saved sessions
preserve mode and exact limits and either restore that supported execution path
or reject explicitly. Records preserve commands, policy, match/clock events,
engine settings and observations alongside moves. Deterministic record replay
does not promise identical reruns of time-dependent executions.

MSH-35 separates the immutable inspected historical position from the live
game. Viewing is read-only; returning live is explicit. Any later branch/change
requires an explicit command with new identity/version and cancellation of old
work. Closing the initial app loses the unsaved game; the UI says so until
persistence is delivered. No implicit persistence or crash recovery is claimed.

Timed play is outside this batch. Before it is enabled, its ticket must define
and test suspension/resumption semantics (including minimize, system sleep,
process pause, app quit/crash and restored clocks), monotonic event ordering,
clock ownership and flag-fall precedence. No timer is shown or silently paused
in this untimed slice. Physical tournament procedures remain outside software.

## Interaction and verification

Readable board-centered layout: side to move/check/outcome, human color,
orientation, move list and actual engine result details. SVG square controls
have accessible coordinate/piece labels and visible focus. Click source then
destination, or use arrows to move focus and Enter/Space to select; Escape
cancels selection/promotion without a game command. Promotion offers all native
choices by labeled buttons. Claims, failures and cancellation have distinct
plain-language status. Controls remain usable when search runs.

MSH-36 is visibly labeled a synthetic development fixture, not another game.
Its 16 by 16 board and multi-stage route choices exercise presentation and
selection; partial route selection never commits a game action.

Native tests cover stale command/result identities, exact history differences,
new-game ABA/version protection, generation mismatch, cancellation races,
malformed/failing/missing child responses and legal move/claim application.
Frontend tests exercise selection, promotion, orientation and keyboard behavior
against native-shaped choices. First-batch local evidence must include the
running packaged desktop app, real human/engine turns and screenshots. Native
CI builds packages and exercises the bundled engine on Windows, macOS and
Linux. Broader installed interaction checks follow in MSH-38 after all M3
features; instrumented test drivers must use a separate test build and are
excluded from normal distributables. An independent reviewer checks each
meaningful boundary and final M3 closure checks the actual integration, without
repeating the unchanged M1/M2 audits.

Implementation references: [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/),
[bundled sidecars](https://v2.tauri.app/develop/sidecar/).


## Second batch clarifications (owner direction, 2026-10-25)

The eleven-area mapping above is unchanged. Implement existing MSH-34, MSH-33
and MSH-35 in that order, preserving all prior acceptance criteria and evidence.

### Outcomes and recovery — MSH-34

Continue untimed play. Distinguish available current/intended claims, concluded
chess results, explicit resignation, unresolved unscored match conclusions,
cancelled/incomplete work and engine failures. Resignation uses the native M2
mating-feasibility contract; absence of a proof remains unresolved and unscored.
Recovery/retry preserves authoritative match/game history and creates fresh work
identity. No clocks or evaluation-based resignation/draw policy are added.

### Analysis and execution ownership — MSH-33

Ordinary move analysis is read-only, with an explicit request purpose and a
separate acceptance path that cannot apply an engine move or claim. Every result
binds to its exact target record/history, target selection, request identity,
session version, process generation, mode and limits. Technical details retain
native encodings/identities; the main view names the analyzed position, score
perspective and understandable pawn/mate distance. Only actual final native
observations are required; live progress is optional and needs bounded native
reporting.

Use one supervised execution resource with explicit ownership. Starting analysis
pauses/cancels any engine-turn work and invalidates its acceptance; it does not
resume play automatically. Starting/replacing analysis cancels the previous
analysis. Cancel leaves play paused. Explicit Resume engine turn retires analysis
and creates a fresh engine-turn request. A game change invalidates analysis and
old turn work. Purpose-specific acceptance checks prevent either kind from
acting as the other. Historical selection changes retire the old analysis.

Ordinary minimax move analysis remains distinct from advanced Full cooperative
mating reachability. The advanced operation is visibly labeled and executes in
a supervised child with deterministic position/transition limits and a process
watchdog/cancellation. Any potentially expensive reconstruction/replay/Full
adjudication occurs there, never on the host interaction path. Unknown,
cancellation or resource exhaustion is incomplete analysis, never a live chess
result. Record exact modes/limits; reject unsupported combinations explicitly
rather than downgrade. Full live-session restoration remains unsupported until
a supported controlled restoration path exists.

### Persistence and inspection — MSH-35

Save a versioned complete desktop document containing canonical MatchRecord and
GameRecord semantics, commands/events, policies, modes/limits, relevant desktop
settings, and exact-context engine/analysis observations. FEN/actions or the
existing GameRecord restoration helper alone are insufficient. Saved observations
are historical, never resumable pending work. Reopen with fresh runtime identities
and increasing frontend snapshot protection, engine work stopped until explicit
Resume. Preserve modes/limits; reject unsupported/timed/Full live restoration
explicitly before expensive replay.

Write a same-directory temporary file, flush it and atomically replace only on
success; failed/interrupted saves preserve the prior valid document and dirty
state. Validate/replay opens off the interaction path under byte/event/time
bounds, commit the candidate atomically only after full validation and exact
current-session precondition checks. Corrupt/unsupported/resource-limited opens
leave live state intact. Protect unsaved changes before New game, Open or Quit
with Save/Discard/Cancel choices. Forced OS termination/crash cannot promise an
interactive save prompt; ordinary native Quit and window-close are protected.

Historical viewing is visibly read-only, separate from the live match, with
explicit Return to live. Earlier-position analysis uses the exact known history
prefix, excluding future actions, claims and repetition information. Selection
identity prevents late results from describing a newly selected position.
Branching/variation editing is outside this batch.

### Evidence

Verify native/controller and frontend behavior separately, then exercise the
running packaged application through recovery/retry, analysis cancellation and
replacement, save/close/reopen, history inspection and return to live. Difficult
states may use a separately labeled verification build or copied sidecar; test
instrumentation must not enter normal distributables. Record exact revisions,
platform-specific evidence and independent review per ticket, reusing unchanged
M1/M2 and first-batch evidence. MSH-36–39 and M3 closure are not in this batch.
