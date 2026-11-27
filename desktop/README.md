# Desktop development and packages

The desktop uses Tauri 2.12.5, React 19.3.1, strict TypeScript 7.0.2 and
Vite 8.3.5. npm and Cargo lockfiles are committed. Node 24.19.0 is pinned in
`.node-version`. The desktop host has its own lockfile and
rust-toolchain.toml; both it and the mShogi engine workspace pin Rust 1.99.1.

Install the [prerequisites](https://v2.tauri.app/start/prerequisites/):
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

## Playing all six games

Press **New game**, choose a game and set **Players** to **Human against mShogi**
or **Two humans**. In human-versus-engine play, choose your side or Random. Choose
Adams or Muller when starting Tenjiku Shogi. Both profiles use the same board
and retain their own rules. Tenjiku fits at the application's 760 × 650 minimum.
Use about 960 × 1000 for more comfortable reading if your display permits.

Select a piece, then one of the highlighted choices. Game-specific choices
include drops, promotion and compound routes. When offered, **Complete action
here** commits the selected route. Arrow keys move board focus, Enter or Space
selects, and Escape clears an uncommitted selection. **Flip board** changes
orientation. The Xiangqi board uses intersections. Shogi-family games use
[bundled kanji](../docs/m4/shogi-font.md).

Use **Save** and **Load** for .mshogi documents. First Save asks for a path.
Load and New game replace the current document after successful admission.
Load starts stopped. Select **Resume play** to continue. Quit asks
Save / Discard / Cancel for dirty or never-saved games.

For the shared workflow, see the [desktop application contract](../docs/desktop/native-application.md),
[Take back](../docs/desktop/take-back.md), and [two-human play](../docs/desktop/two-human-play.md).

## Building packages

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
MSH-58 work in M6.

`npm run check` checks strict TypeScript; `npm test` runs focused interaction
tests against generated snapshots. `npm run build` builds embedded web
assets. In `src-tauri`, `cargo fmt --all -- --check`,
`cargo clippy --locked --all-targets -- -D warnings` and `cargo test --locked`
verify the desktop host. CI builds the normal packages on Windows,
macOS and Linux, extracts/installs them and runs their actual bundled engine
through UCI and the structured internal interface with bounded initial-position
searches. Its artifact
includes the package and `package-smoke.json` with package inventory, executable
SHA-256 and actual search observations.

`desktop/scripts/package-smoke.py` is an external verifier. Tests and fixtures
are not resources or sidecars; only the `mshogi` engine is listed as
`externalBin`. No instrumentation or test driver is compiled into the normal
host. See the [packaged GUI guide](../docs/desktop/packaged-gui-verification.md)
for actual application checks.

Normal Quit/Cmd-Q, window close and macOS Dock Quit first resolve unsaved changes.
The narrowly patched native Quit route is documented in
[macOS quit handling](../docs/desktop/macos-quit.md).
After approval, a shutdown gate stops commands and cancels obsolete
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
`desktop-verification` Cargo feature, excluded from normal distributables. CI
constructs a separately identified verification package for difficult states.
On macOS, after building the normal application above:

```sh
npm run desktop:build -- --bundles app --features desktop-verification --config src-tauri/tauri.verification.conf.json
python3 scripts/macos-verification.py prepare qa
python3 scripts/macos-verification.py select qa claim
python3 scripts/macos-verification.py inspect qa
```

Launch the fixed path reported by the helper; automated interaction uses the
supported Computer Use app API. The release/bundle verification output is a
staging input, not another launch identity. Keep the primary-checkout helper
and fixed launch paths even when source is built in a temporary worktree.
The separate application name, identifier and window title label test state.
Fixture names are `standard`, `claim` (intended fifty-move claim), `current-claim`,
`concluded`, `incomplete`, `promotion` and `capacity`. `promotion` uses a real
chess position with four a7–a8 promotion choices. `incomplete` uses an actual zero-node
engine request; Retry uses ordinary play limits. The `capacity` fixture
starts with 255 session observations. It exercises
retention of the newest 256 observations across further searches. Close before
launching another fixture.
Use `python3 scripts/macos-verification.py prepare synthetic`, then
`python3 scripts/macos-verification.py select synthetic synthetic`, for the fixed
separately labeled invented 16-by-16 strider fixture. It uses the shared board and selector, with
complete choices and a separate temporary record. Select `i9 → j10 → i9`
to capture and return, or Reset and select `i9 → h9 → i9` to return without that
capture. A valid short prefix requires **Complete action here**; selecting it
alone submits nothing. Escape cancels from the board or route controls. Flip
changes orientation; arrow keys move board focus. One complete
action ends the demonstration; Reset advances the version and clears its
temporary record. This is development infrastructure, not another real game.

Normal frontend builds force verification off even if its environment flag was
inherited. Verification builds use `dist-verification`; normal assets remain in
`dist`. The normal build checks emitted modules and markers, and package smoke
checks confirm synthetic IPC, fixture and native quit-probe markers are absent.
This build is QA infrastructure, not a normal distributable. The external package
verifier checks that a normal host contains no fixture initializer marker.
The verification application menu has **Verification window size** presets for
760 × 650 and 1180 × 820 logical content pixels. Its title reports the measured
content size after resizing, separately from screenshot pixels.
On macOS its application menu also has **Native Cocoa Quit (verification)**.
Clicking that predefined item exercises Cocoa's `terminate:` entry,
including the same interception used for Dock Quit. It is absent from normal
packages. Evidence records this as a Cocoa termination test rather than
claiming an actual Dock click.


## Read-only bounded analysis

Choose **Analyze position** in the inspector. Select the depth and node budget
within the limits shown for the current game, then **Start move analysis**. Analysis pauses play; suggestions,
claims and continuations are never committed. **Cancel analysis** leaves play
paused, replacement retires old work, and **Resume play** explicitly returns the
single supervised resource to the engine turn or human play. Changing inspector
tabs alone neither starts analysis nor resumes play.

Analysis shows the selected position, score perspective and actual final
depth, nodes and principal variation when available. A score uses the active
game's units. The [execution contract](../docs/desktop/internal-execution-v4.md)
defines the search and result admission.

## Saving and inspecting games

The [document format](../docs/desktop/document-v1.md) records the game,
human side and settings. It also describes atomic Save and guarded Load.
History inspection and engine results belong to the open session. Browsing
selects complete game commands and shows a read-only board. **Return to live**
restores the live board, with fresh admission if browsing alone suspended play.

## macOS verification identities

Use the fixed reusable QA and synthetic slots through
`python3 scripts/macos-verification.py`; see the current
[packaged verification instructions](../docs/desktop/packaged-gui-verification.md).
Do not copy, rename or re-sign an application for each fixture. Fixture selection
is an external startup file and must be changed while the app is stopped.
Normal app launches continue to use the primary checkout's normal package.
Computer Use approvals are owned by the user/platform, not this helper.
