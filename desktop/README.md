# Desktop development and packages

The desktop uses Tauri 2.12.5, React 19.3.1, strict TypeScript 7.0.2 and
Vite 8.3.5. npm and Cargo lockfiles are committed. Node 24.19.0 is pinned in
`.node-version`. The desktop host has its own Rust 1.90.0 toolchain and lockfile;
the existing native engine workspace keeps Rust 1.89.0.

Install the [native prerequisites](https://v2.tauri.app/start/prerequisites/):
Xcode command line tools on macOS, Visual Studio C++ build tools and WebView2 on
Windows, or WebKitGTK 4.1 development dependencies on Linux. Tauri 2.12 requires
Rust 1.90 or newer. From the repository root:

```sh
cd desktop
npm ci
npm run desktop:dev
```

`desktop:dev` first builds the actual engine using the root workspace's pinned
toolchain. The host resolves only a sibling `mshogi` executable (`mshogi.exe` on
Windows). The frontend has no shell or filesystem plugin. Vite listens on
127.0.0.1:1420 during development; production web assets are embedded and do not
require a web server.

## Playing this slice

Choose White or Black and press **New game** for an untimed standard chess game.
Click a piece and a highlighted destination. Promotion opens explicit native
choices. On the focused board, arrows move focus, Enter/Space selects, and
Escape clears selection. **Flip board** changes only orientation.

mShogi replies through the actual bundled engine at depth 2 with a 20,000-node
cap. The engine panel shows only final native observations, labeled with their
searched version and score perspective. **Cancel search** applies no fallback
move; **Retry engine turn** explicitly restarts paused work. Native claim
buttons distinguish current claims from intended moves, which are not played.

**Resign game…** opens a version-bound confirmation. Resignation ends play;
the native M2 proof limits can leave the outcome **unresolved and unscored**.
An engine failure, incomplete search or cancellation instead pauses the ongoing
game without adding a move or result; explicit Retry preserves its history.

Use **Save**, **Save as…** and **Open…** for complete `.mshogi` documents.
New game, Open and ordinary Quit protect unsaved changes with Save/Discard/Cancel.
Reopened untimed games remain paused until **Resume play**. Saved observations
are labeled historical and never restart pending engine work. Earlier positions
are read-only; **Return to live** explicitly leaves history inspection.

Full live-adjudication restoration is explicitly unsupported; the advanced
read-only Full operation below is a separate controlled execution path. The
synthetic large-board fixture and remaining interaction/platform integration
work are later M3 tickets. M3 remains open; see the
[batch evidence](../docs/desktop/second-batch-evidence.md).

Build a normal distributable with the platform's supported bundle format:

```sh
# macOS
npm run desktop:build -- --bundles app
open src-tauri/target/release/bundle/macos/mShogi.app

# Linux (Debian package)
npm run desktop:build -- --bundles deb
sudo apt install ./src-tauri/target/release/bundle/deb/*.deb
mshogi-desktop

# Windows (PowerShell): run the resulting per-user installer, then use Start menu.
npm run desktop:build -- --bundles nsis
Get-ChildItem src-tauri/target/release/bundle/nsis/*-setup.exe
```

These are development packages, without a promised release signing or
notarization arrangement. macOS local builds use the normal Tauri application
bundle; production distribution signing and minimum OS release policy remain
part of the later packaged verification work.

`npm run check` checks strict TypeScript; `npm test` runs focused interaction
tests against native-generated snapshots. `npm run build` builds embedded web
assets. In `src-tauri`, `cargo fmt --all -- --check`,
`cargo clippy --locked --all-targets -- -D warnings` and `cargo test --locked`
verify the native host. Native CI builds the normal packages on Windows,
macOS and Linux, extracts/installs them and runs their actual bundled engine
through UCI and the structured internal interface with bounded initial-position
searches. Its artifact
includes the package and `package-smoke.json` with package inventory, executable
SHA-256 and actual search observations.

`desktop/scripts/package-smoke.py` is an external verifier. Tests and fixtures
are not resources or sidecars; only the `mshogi` engine is listed as
`externalBin`. No instrumentation or test driver is compiled into the normal
host. The later MSH-38 ticket verifies installed application interactions on all
three systems after the remaining M3 features exist; a package build and engine
smoke do not establish that broader result.

Normal Quit/Cmd-Q, window close and macOS Dock Quit first resolve unsaved changes.
The narrowly patched native Quit route is documented in
[macOS quit handling](../docs/desktop/macos-quit.md).
After approval, a native shutdown gate stops commands and cancels obsolete
engine work. An off-path observer waits for the supervisor's
cleanup acknowledgement. macOS Cocoa termination can bypass an exit request;
the final event-loop teardown therefore has a fallback that observes the same
acknowledgement. Both routes share one observation deadline, two seconds from
the first shutdown signal. The final teardown fallback may wait on that thread,
while all child-process I/O, killing and reaping remain on the supervisor worker.
The deadline bounds the observation policy, not OS scheduling. Missing cleanup
acknowledgement is logged as uncertainty; force-kill/crash cleanup is not promised.


## Isolated verification application

Difficult outcome/recovery states have an explicitly opt-in
`desktop-verification` Cargo feature, not enabled by normal build or CI packaging.
On macOS, after building the normal application above:

```sh
npm run desktop:build -- --bundles app --features desktop-verification --config src-tauri/tauri.verification.conf.json
open --env MSHOGI_DESKTOP_FIXTURE=claim "src-tauri/target/release/bundle/macos/mShogi Verification.app"
```

The separate application name, identifier and window title label test state.
Fixture names are `standard`, `claim` (intended fifty-move claim), `current-claim`,
`concluded`, `incomplete` and `capacity`. `incomplete` uses an actual zero-node
engine request; Retry uses ordinary play limits. `capacity` starts with 255
genuine native zero-node analysis observations so the next accepted analysis
reaches 256 and subsequent work demonstrates the application limit. These
observations remain in saved documents. Close before launching another fixture.
This build is QA infrastructure, not a normal distributable. The external package
verifier checks that a normal host contains no fixture initializer marker.
On macOS its application menu also has **Native Cocoa Quit (verification)**.
Clicking that predefined native item exercises Cocoa's `terminate:` entry,
including the same interception used for Dock Quit. It is absent from normal
packages. Evidence records this as a native Cocoa termination test rather than
claiming an actual Dock click.


## Read-only bounded analysis

Choose **Analyze position** in the inspector. Set depth (1–8) and node budget
(0–5,000,000), then **Start move analysis**. Analysis pauses play; suggestions,
claims and continuations are never committed. **Cancel analysis** leaves play
paused, replacement retires old work, and **Resume play** explicitly returns the
single supervised resource to the engine turn or human play. Changing inspector
tabs alone neither starts analysis nor resumes play.

Evaluation is shown in pawn units from the named analyzed side's perspective,
or as a named side's mate distance. Zero evaluation is not a draw. Position,
requested budgets and actual final depth/nodes/PV are labeled; native request,
process and session identities plus raw encodings remain in technical details.
There is no simulated live progress.

**Advanced: Full mating reachability** asks whether legal cooperative play can
reach mate for Either side, White or Black. It is not forced-win search and does
not adjudicate the live M2 game. Mandatory position/transition caps (at most
100,000/1,000,000) and a 15-second child watchdog keep reconstruction and solving
off the interaction path. A proof, cooperative witness or unknown work result
is displayed as analysis only. Cancellation has no fabricated solver counts;
resource exhaustion and process failure never invent a chess outcome.

The [internal v2 contract](../docs/desktop/internal-engine-v2.md) documents
purpose-specific acceptance and limits. The [batch evidence](../docs/desktop/second-batch-evidence.md)
distinguishes controller, frontend and actual packaged GUI verification.

## Saving and inspecting games

A document preserves the canonical match and game records, policy and known
history, claims and conclusions, commands/events, modes and limits, human color,
board orientation, analysis defaults and accepted engine observations with their
exact request contexts. It is not an interchange FEN or PGN. See the
[archive contract](../docs/desktop/archive-v1.md) for format and resource limits.

The native controller checks complete archive capacity before accepting changes.
The limits are 256 observations, 4,096 game/match events, 8,192 desktop controls
and 16 MiB, with reserved shutdown and identity headroom. If the next operation
cannot fit, **Application limit reached** pauses work while keeping the complete
accepted document saveable. Use Save or Save As, then explicitly start another
document to continue. Saving and reopening preserve counts; they do not reset
the limit. A resource pause is separate from chess outcomes and engine failures.

Save writes an exclusive same-directory temporary file and replaces the target
only after a complete flushed write. A failed or cancelled save keeps the prior
file and leaves unsaved changes unsaved. Open validates on a bounded native
worker; corrupt, unsupported or resource-limited documents leave the current
game intact. Full live modes and timed matches are rejected explicitly, without
changing their mode or clocks. No automatic crash recovery is promised.

The history inspector includes nonmove events such as a claim or resignation.
Selecting an earlier event pauses play and displays a read-only board. Analysis
uses only that event's known history prefix, with no future moves or claims.
Selecting another event retires prior analysis. **Return to live** restores the
live board; **Resume play** is a separate action. Branching and variation editing
are outside this batch.
