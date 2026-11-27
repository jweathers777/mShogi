# Desktop application contract

`DesktopController` owns one active, game-tagged document and one supervised
engine bridge. The six games share the desktop lifecycle, document envelope,
command dispatch and result admission. Each game adapter supplies its own
rules profile, commands, history, choices and outcomes. Tenjiku Shogi uses one
of two immutable profiles per game.

The active session owns the validated document, current game, history selection
and recent engine observations. `RuntimeMetadata` owns the document identity,
version, revision, selection and request counters, saved revision, path and
capacity state. See the [document format](document-v1.md) for the saved
fields and the [internal execution contract](internal-execution-v4.md) for
engine request identity.

## Host operations

| IPC | Contract |
| --- | --- |
| `game_snapshot` | Get the game-tagged session, including choice and history data, capabilities, execution and file state. |
| `game_command` | Submit an operation for the game under the exact rendered `SessionToken`. |
| `game_new` | Submit a game, matching rules profile and qualified side, participant mode and exact `SharedOpenToken`. |
| `game_inspection` | Query complete read-only movement choices under the exact selection token. |
| `game_file` | Request `save`, `save_as` or `open` under the exact `SharedOpenToken`. The product exposes Save and Load. |
| `game_request_close` | Start close handling and stop active work. |
| `game_cancel_close` | Clear the close guard. The stopped game stays stopped. |
| `game_finish_close` | Approve close under the exact current Open token after operation reservations release. |

Commands and inspection requests reach the strict Rust parsers as JSON text.
The session resolves the actor and applies a complete choice or a typed game
command. Preferences share one settings shape across all games. The frontend
uses one `useGameSession` owner and `ApplicationView` for all games. The
shared board choice controller handles selection, completion and cancellation
with game-specific geometry and choices supplied by the active game.

The controller captures operation authority before starting a private
admission job. The host holds one exclusive lifecycle reservation through its
command wait or native picker and file wait. A command receipt or file
operation ID identifies the completion being awaited. Failed, cancelled and
stale candidates preserve the current accepted document.

## New game, Save, Load and Quit

First Save selects a path. Later Saves use the accepted path. Load and New
game replace the document on successful admission. The current game remains
installed while the chooser is open or the candidate is checked. The host
checks the document identity, version and document revision again after
a picker returns. Load also checks that close handling is clear. Successful
Load installs the replayed game with fresh runtime identities and stops play
until **Resume play**. New game starts with its new play intent.

Window close and menu Quit first stop work and send `game-close-requested`.
The frontend shows Save / Discard / Cancel for a dirty or never-saved game.
Save is allowed while the guard is open. Close approval checks the current
Open token and the absence of active reservations. Cancel keeps the game open
and paused. See [macOS Quit interception](macos-quit.md) for the Dock route.

## History and execution

Moves shows complete board actions. Warning, claim and other non-move events
belong to their command. History selection stops at complete command
boundaries. It retires pending work and shows a read-only position. Analysis
uses that position's exact history prefix. Resign submits directly under the
rendered token and displays the rules outcome or unscored uncertainty.

**Return to live** clears the selection and retires historical analysis. A live
game suspended only for browsing runs a fresh Resume admission. Load,
[Take back](take-back.md), cancellation, failure and capacity stops require
explicit Resume or Retry. A concluded game remains concluded.
Engine results stay in memory. Each session keeps its newest 256 observations,
and a new observation evicts the oldest at capacity.

## Resource diagnostics

A capacity or file state can report one `ResourceFailure`, with a `fact` and
a technical `diagnostic` bounded to 4,096 UTF-8 bytes. The fact category is
`game_root_bytes`, `identity_headroom`, `validation_budget` or `unknown`.
Its measurement can describe the accepted state, the proposed candidate, or
an unknown state. The count, reserve, limit, scope and unit are optional.
The shared notice uses the category supplied by the check. An unknown cause
uses the generic notice. Diagnostics preserve the details supplied by the host.

Resource refusals are separate from illegal commands, engine failures and game
outcomes. Observation retention is a memory policy, so reaching 256 results
does not refuse work. The [record admission rules](../game-resource-admission.md)
describe the remaining fixed bounds and the validation deadline.
