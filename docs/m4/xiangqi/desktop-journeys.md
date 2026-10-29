# Xiangqi analysis, outcomes and document journeys (MSH-102)

The accepted MSH-101 board and shared dark application shell are unchanged.
The selected profile remains `mshogi.xiangqi.wxf2018.v1`; native rules, canonical
records and bounded execution remain authoritative. This work does not close
MSH-45, MSH-103, MSH-88 or M4.

## Interaction and execution

Play details and Analyze position occupy the existing shared Analysis card.
Ordinary move analysis is read-only, sound-fast-only, depth 1–3 and 1–512 visits.
The UI submits the exact rendered native token with purpose-specific Analyze,
Cancel and Replace commands. One supervised execution resource is owned by the
native controller; an analysis result cannot enter the live move/claim handler.
Only actual native observations supply evaluation, Red/Black perspective,
completed depth, node count, continuation, status and target event. No live
progress, mate proof or Chess Full capability is invented. Selecting another
event removes the previous result from the current-result area; recorded searches
retain their target and historical identity in a separate disclosure.

Every native event is selectable, including nonmoving warnings, claims and
terminal commands at the same board ply. Native immutable prefix snapshots own
geometry, legality, warnings, provenance and conclusions. Prefixes inside one
atomic command are inspectable but lack a committed canonical game and cannot
be analyzed. Return to live is explicit and leaves play paused until Resume.
Cancellation, replacement, selection, reopening and resume retain native retired
execution floors, full context and request/session/process freshness protections.

## Outcomes and recovery

Human-readable native conclusions distinguish board win, repetition conduct
loss, resignation, repetition/count/agreement draws and a sound no-win draw.
Active warnings name the native offender and perpetual-check/chase obligation.
A draw offer is not a conclusion. Native advice controls offer/accept/decline
availability independently of board-action turn ownership; no TypeScript rule
inference and no new engine agreement policy are added. The engine may continue
ordinary board play rather than accept an offer. A natural-count opportunity is
claimant-specific and requires a formal confirmed submission. Formal refusal,
including stale authority, retains accepted history and reports an action failure.

Resignation, claims and draw decisions use captured-token confirmation dialogs.
A changed native snapshot invalidates confirmation. Unresolved adjudication is
explicitly stopped and unscored. Unknown fast assessment is not a positive win
witness or a draw; positive witnesses are not forced wins. Exact reports, when
saved, remain historical evidence; exact execution is still the explicit native/
console facility, not ordinary desktop rendering or move analysis. Search
incomplete/cancelled and engine failure remain separate from chess-family
outcomes. Retry and Resume use native commands and return focus to the live
status heading. No fallback move or result is supplied.

## Documents and limits

Normal product Open now uses the already strict native MSH-99 archive admission.
No archive version or compatibility change is needed. Authoritative replay,
duplicate-preserving dispatch, complete native commands/events/warnings/claims,
policies/settings/observations, selected prefix and root provenance are retained.
Reopening assigns fresh runtime identities and stops engine work until explicit
Resume. Saved observations are never pending work. Save/Open/close and unsaved
replacement reuse ProductApp/useGameSession's existing document/close guards,
atomic native file worker and monotonic snapshot acceptance.

Capacity remains admission-before-commit, including actual encoded bytes,
replay/control/event/observation limits and stop/close reserve. At a structural
limit the complete current game can still be saved; saving/reopening does not
reset capacity and the UI does not recommend retrying an impossible Save. File
failure retains current state and unsaved changes, including asynchronously
reported native file-worker failure. Resource failures are not illegal moves or
outcomes. No history or observations are dropped.

## Scoped sharing and future diagnostic findings

`NativeSessionPanels` shares Shogi/Xiangqi native observation formatting,
purpose-specific bounded analysis controls, exact-event history, recovery/capacity/
file-error notices and work-command focus restoration. Game-specific settings,
terminal actions, limits, labels and rules stay in their native adapters.
ProductApp, useGameSession, Dialog and ApplicationShell continue to own common
document/quit guards, freshness, modal focus and page composition. Chess is not
refactored to match a native adapter merely for symmetry.

Concrete MSH-106 diagnostic inputs (future, not started): Chess retains its older
native session hook and analysis/document presentation while Shogi/Xiangqi share
the tagged session hook; evaluate whether their genuinely identical failure and
focus semantics warrant further consolidation. Shogi/Xiangqi still compose
similar document header/Flip/technical disclosure markup, with different native
settings and semantic actions. Inspect real maintenance cost before changing
ownership. A finding of no material changes remains valid; filename symmetry
is not a goal.

## Verification instrumentation

Only the `desktop-verification` feature and separately named/id'd mShogi
Verification package admit `xiangqi-*` hard-state fixtures. These use actual
native fixture records and replay. The capacity fixture creates 255 real native
one-node incomplete analysis observations, leaving one accepted result before
refusing the 257th. No fabricated engine telemetry is used.

Only that QA host reads `MSHOGI_VERIFICATION_ENGINE`, permitting an external
wrapper that delays then execs the actual bundled engine. This makes Cancel and
Replace observable despite small ordinary search limits. Delayed launch is test
instrumentation, not product execution performance. Normal package smoke checks
reject this marker and Xiangqi fixture markers. Native/controller, frontend,
packaged GUI and CI/platform evidence must be recorded separately below.

Execution evidence: see the MSH-102 completion evidence record (linked on task
finalization). Existing MSH-99/101/76 evidence is reused for unchanged archive
semantics, geometry, sizing, inspection and native termination protection.
