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

Games are unsaved: New game or closing the app discards them. The move list is
the live action history; historical viewing, persistence and the synthetic large-board fixture are later
M3 tickets. Bounded analysis and outcome/recovery controls are available now. Full
live-adjudication restoration is explicitly unsupported; the advanced read-only
Full operation described below is a separate controlled execution path. M3 remains
open; see [scope and evidence](../docs/desktop/first-batch-evidence.md).

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

Normal Quit/Cmd-Q routes through a native shutdown gate: commands stop, obsolete
engine work is cancelled, and an off-path observer waits for the supervisor's
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
`concluded` and `incomplete`. The last uses an actual zero-node engine request;
Retry restores ordinary play limits. Close before launching a different fixture.
This build is QA infrastructure, not a normal distributable. The external package
verifier checks that a normal host contains no fixture initializer marker.


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
