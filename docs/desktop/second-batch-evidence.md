# M3 outcomes, analysis and persistence batch

Authorized order: MSH-34, MSH-33, MSH-35. M2 and MSH-27–32 stay closed/Done.
M3 remains open; MSH-36–39 are subsequent work. The eleven-area mapping and
owner clarifications are in [M3 design](m3-design.md). Unchanged M1/M2 and
[first-batch evidence](first-batch-evidence.md) remain applicable.

## MSH-34 — outcomes and recovery

A native serialized resignation command uses the existing M2 mating-feasibility
contract without an evaluation-based result. Missing proof can conclude play as
unresolved/unscored; a sound impossibility proof can establish a draw. Human
resignation is supported during either player's turn and retires pending engine
work. Version-bound confirmation cannot resign a newer position. Cancel only
applies to active work; Retry only applies to a paused engine turn. Neither
rewrites the authoritative match or game history.

Focused local arm64 macOS checks: 16 native all-feature tests; four public
controller/actual-process integration tests; warnings-denied native/host Clippy,
formatting and host tests; verification-feature host compilation; strict
TypeScript and 22 frontend interaction tests; normal and verification `.app`
builds. Native-generated frontend fixtures include current/intended claims,
claimed outcomes and unresolved resignation.

Actual macOS packaged journeys (not inferred from controller tests):

| Package and journey | Observed result | Screenshot |
| --- | --- | --- |
| Normal app: `e2e4 a7a5`, confirm resignation | Unresolved/unscored; same board and two-action history | [Resignation](evidence/recovery-resignation-macos.png) |
| Labeled verification: intended `a1b1` fifty-move claim | Claimed draw; no intention played, zero-action history | [Claim](evidence/recovery-intended-claim-macos.png) |
| Labeled verification: actual zero-node child, then Retry | Incomplete depth 0/no score; real retry `a2a3`, depth 2/442 nodes | [Incomplete](evidence/recovery-incomplete-macos.png), [retry](evidence/recovery-retry-macos.png) |
| Labeled verification: real missing-response helper after `e2e4`, restore actual engine, Retry | Ongoing paused game with one move; one real reply `a7a5`, depth 2/642 nodes | [Failure](evidence/recovery-failure-macos.png), [recovery](evidence/recovery-failure-retry-macos.png) |
| Labeled verification: external slow child after `g1f3`, Cancel | Three actions preserved, no conclusion; owned child removed | [Cancellation](evidence/recovery-cancelled-macos.png) |

The [retained report](evidence/recovery-local-macos.json) identifies the normal
host (`f5b149…`), unchanged actual engine (`1461e6…`), separate verification host
and observations. The normal package contains no fixture initializer; package
verification also runs actual UCI and structured searches. The feature build has
a separate name/identifier/title; only its sidecar was temporarily replaced for
failure/cancellation, then restored. Normal distributables have neither the
initializer nor the helper. [Reproduction/launch](../../desktop/README.md).

The [MSH-34 record](<../../backlog/tasks/msh-34 - Complete-desktop-outcomes-and-recovery-interactions.md>)
retains independent review, integrated source and exact native/platform CI.
This task adds no clocks or evaluation-based resignation/draw policy.


## MSH-33 — read-only analysis and controlled Full execution

[Internal protocol v2](internal-engine-v2.md) requires a purpose and exact native
target/history. The host has a separate `accept_analysis` path with no move/claim
application. One supervised bridge pauses play for analysis; cancel/replace
retires old work, and only explicit Resume returns it to play. Advanced Full
cooperative reachability runs reconstruction and solving in the child under
recorded position/transition caps and the process watchdog. Unknown or killed
work cannot adjudicate a live game, and missing observations are not invented.
Full live-record restoration is still explicitly refused before solver replay.

Independent review passed 22 all-feature session tests, two engine unit/eight
protocol tests, two actual-process analysis tests, 35 frontend tests and strict
TypeScript. A focused resignation/analysis retirement refinement was separately
reviewed and its regression rerun. Primary focused Clippy and all eleven console
process/play/analysis integration tests passed. Host format/Clippy/tests and the
normal TypeScript/Vite/package builds passed. Fourteen frontend fixtures retain
native serialization/actual native solver output; they are not a second rules
oracle.

Actual final normal macOS `.app` journeys, using its bundled engine:

| Journey | Observations and game protection | Screenshot |
| --- | --- | --- |
| Move analysis at standard start, depth 2/20,000 nodes | Actual depth 2/442 nodes, 0.00 pawns from White; suggested `a2a3` not played; zero-action board unchanged | [Move analysis](evidence/analysis-moves-macos.png) |
| Start depth 8/5,000,000 nodes, Cancel | Cancelled with no final observations; board unchanged and play remains paused | [Cancelled](evidence/analysis-cancelled-macos.png) |
| Replace active depth 8 work with depth 1/1,000 nodes | Only new depth 1/21-node suggestion displayed; no board action | Retained observations below |
| Advanced Full, Either side, one position/transition | Unknown position limit; one actual position/transition; live mode remains M2 and no chess result | [Full unknown](evidence/analysis-full-unknown-macos.png) |
| Explicit Resume then human `e2e4` | Board enabled only after Resume; real `a7a5` reply, depth 2/642 nodes, Black perspective | [Resumed play](evidence/analysis-resumed-play-macos.png) |

The [retained report](evidence/analysis-local-macos.json) includes final host and
engine identities, real packaged UCI/engine-turn/move-analysis/Full queries and
GUI observations. Technical details exposed selection/request 5, session 7 and
the exact Full limits for the zero-ply target. Pawn/mate presentation and
cooperative possible/impossible proof branches have composed native/frontend
coverage, not a claim of reaching them in the standard-opening GUI journey.
No instrumentation is present in the normal host. Platform CI and source are
recorded in the [MSH-33 completion record](<../../backlog/tasks/msh-33 - Add-bounded-desktop-analysis-and-explicit-Full-mode-execution.md>).

## MSH-35 — canonical documents and historical inspection

The [archive contract](archive-v1.md) preserves complete canonical match/game
semantics, accepted game/execution controls, settings and exact-context engine
observations. One bounded native worker handles atomic file replacement,
preflighted validation/replay and history-prefix reconstruction. Open commits a
validated candidate only against the approved version and document revision;
reopened work stays stopped under fresh runtime identities. Historical viewing
and analysis never consume future events or alter the live match.

Primary checks passed full Rust workspace format, all-target Clippy and tests;
29 all-feature desktop-session tests; three actual-file/process persistence
journeys; host Rust 1.90 format/Clippy/two tests and verification-feature compile;
52 frontend tests across nine files, strict TypeScript and Vite build. Sixteen
native-generated frontend fixtures include actual reopened/history snapshots.
Independent review separately passed the session, file/process, frontend and
host checks and compared all 119 vendored Tao files with the pinned upstream
package. Only the documented application-delegate source differs.

Review findings were fixed before completion: analysis selection watermarks,
bounded legal verification of saved Full witnesses, exact canonical prefix
metadata, restored command timestamp floors and keyboard focus recovery. Tests
also cover claim/resignation round trips at the same ply, interrupted saves,
stale saves/opens and unsupported/corrupt/resource-limited restoration. Full
live modes, timed matches and externally supplied match-command mating witnesses
are explicitly refused before uncontrolled replay. No rules change reopens M2.

Actual local macOS packaged journeys:

| Journey | Observed behavior | Evidence |
| --- | --- | --- |
| Play and save | Human `e2e4`, real `a7a5`; orientation and complete context saved | Retained archive/report below |
| Real save failure | Read-only isolated save directory produced Permission denied; prior file SHA-256 unchanged, document dirty; Save & continue could not quit | [Failed save](evidence/persistence-save-failure-macos.png) |
| Cmd-Q, save, close, relaunch and Open | Cancel retained game and SVG focus; successful Save & continue closed; fresh process opened document paused | [Paused reopen](evidence/persistence-reopened-paused-macos.png) |
| Initial historical analysis | Read-only event 0, White perspective, actual depth 1/21 nodes, `a2a3` suggested and not played; live game retained | [History analysis](evidence/persistence-history-analysis-macos.png) |
| Cancel and replace historical work | Depth 8/5,000,000 request cancelled; another retired by selecting event 2; only new Black one-ply result appeared, depth 1/21 nodes | Retained report |
| Return live and Resume | Explicit Return restored two-ply board and focus with play paused; Resume allowed `g1f3`, real `a5a4` reply, depth 2/631 nodes | Retained archive/report |
| Corrupt Open | Parse error; four-ply live game and valid save retained | [Corrupt Open](evidence/persistence-corrupt-open-macos.png) |
| Window close | Cancel preserved dirty orientation/four plies; explicit Discard then closed | Retained report |
| Final normal build | Open restored four plies and historical observations; initial-prefix analysis rechecked; Save As and same-file Open retained all five observations, fresh session 46/snapshot 47, paused | [Final saved game](evidence/persistence-final-saved-macos.png), [historical observations](evidence/persistence-final-observations-macos.png) |
| Actual Cocoa termination entry | Separate verification app's native `terminate:` menu item reached Save/Discard/Cancel; Cancel preserved state, repeated request with explicit Discard closed | [Cocoa entry](evidence/persistence-cocoa-quit-verification-macos.png) |

The Cocoa entry probe executes the same native selector/delegate used by Dock
Quit; it does not mock the close handler. Direct Dock interaction was unavailable
through the automation surface and is not claimed. The probe is macOS-only and
feature-gated in the separately labeled verification application; the normal
package verifier checks that neither its label nor the fixture initializer
enters a normal host. See [native quit provenance](macos-quit.md).

Implementation source is `c942a6fd088a2976f410164c24939bbdc2cd6539`; final
verification-only probe/exclusion source is
`0f916a3536c28145b9311ca3fb50db1b471f75bd`. The final normal package was rebuilt
and rechecked after that delta, with Open/history/analysis/Save As/reopen exercised
again. Its host SHA-256 is
`49dca855d7b5cc98a901d663283c128066a15ed20e73cd7905414be8693214bb` and bundled
engine SHA-256 is
`45d264831e806ea81b426b4fbbb94f2ecbcba384d23696a5c87537acca555ba5`.
The [retained report](evidence/persistence-local-macos.json) distinguishes both
normal builds and the separate verification host. The actual
[saved document](evidence/persistence-game.mshogi) retains four moves, eleven
controls and five observations, with SHA-256
`8e119aa0a8768474c944fcb157e65da643d9d99e065d3803b1e61240bced3038`.

Platform CI and integration references belong to the
[MSH-35 completion record](<../../backlog/tasks/msh-35 - Save-reopen-and-inspect-desktop-game-history.md>).
Windows/Linux package and engine checks do not claim installed GUI interaction.
M1/M2 and unchanged MSH-27–34 evidence are reused. MSH-36–39 stay open, including
synthetic large-board selection, integrated accessibility, installed interactions
on all three systems and M3 closure.
