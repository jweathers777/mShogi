# Packaged application GUI verification

## Bounded public diagnostics (MSH-43)

About → Diagnostics captures one complete compact JSON snapshot when expanded.
It renders at most 65,536 UTF-16 units at once, with Unicode-safe Previous/Next
pages. Closing and reopening refreshes the capture; polling cannot mix versions
or rebuild a multi-megabyte text layout. Pages expose the complete captured diagnostic text.

The external observer uses real public page buttons, checks contiguous page
identity and stable count, and reconstructs the complete JSON before validating
game context. It bounds each response and the total collection; it must not
parse just page one or substitute a historical archive for current runtime
identity. Keep the observer synchronized with the public support surface.

## macOS launch identities

Use at most three reusable application launch identities. Never add a fixture, task ID,
commit hash, timestamp or random suffix to a bundle ID or launch path.

| Role | Fixed bundle identifier | Predictable location from repository root |
| --- | --- | --- |
| Normal | `org.mshogi.desktop` | `desktop/src-tauri/target/release/bundle/macos/mShogi.app` |
| QA | `org.mshogi.desktop.verification.r6.shogipromotion` | `desktop/src-tauri/target/verification/macos/mShogi Verification.app` |
| Synthetic | `org.mshogi.desktop.verification.r6.synthetic` | `desktop/src-tauri/target/verification/macos/mShogi Synthetic.app` |

Stage and launch from the primary integration checkout, including when source
is built in a temporary worktree. Invoke the helper stored in that primary
checkout; do not run a copied helper from a task checkout or create another
launch directory there. The fixed paths are launch identities, not disposable
per-task build directories. Keep one active instance per slot.

The QA and synthetic identifiers are deliberately retained historical IDs which
already had saved user approvals during R6. The QA ID does not restrict the app
to Shogi or promotion. Do not rename these IDs for cosmetic consistency. The
normal distributable remains independent; the synthetic fixture remains clearly
labeled as invented, and is not a supported game in New Game.

The QA build output is a staging input; do not launch it as a fourth identity.
The external `desktop/scripts/macos-verification.py` helper stages a separately
built QA bundle, sets its fixed identity and signs it once per actual build.
It refuses to replace a running staged app. Scenario selection writes a bounded
token atomically to an external sibling file while the app is stopped. The
QA-only host reads that file at startup through `MSHOGI_DESKTOP_FIXTURE_FILE`.
Missing, oversized or malformed configured files fail explicitly. The normal
host contains neither this reader nor its environment marker. Changing a
scenario does not touch Info.plist, signed resources, executable or signature.
Do not put the selected fixture in `LSEnvironment` or create another app copy.

From the primary repository root:

```sh
npm --prefix desktop run desktop:build -- --bundles app --features desktop-verification --config src-tauri/tauri.verification.conf.json
python3 desktop/scripts/macos-verification.py prepare qa
python3 desktop/scripts/macos-verification.py select qa xiangqi-count
python3 desktop/scripts/macos-verification.py inspect qa
python3 desktop/scripts/macos-verification.py prepare synthetic
python3 desktop/scripts/macos-verification.py select synthetic synthetic
```

Launch and control the reported exact path through the supported Computer Use
app API. Reuse its app handle while running and use New Game for ordinary game
changes. Quit through the application's guard before selecting another
startup fixture. Preparation and selection do not launch apps, manipulate UI,
or grant access. Optional externally delayed-engine preparation stays QA-only;
record its real bundled-engine and gate identities. Never replace a running
bundle or change a running session by rewriting its selector.

Bind input/capture helpers to the explicit slot's app handle and verify the
observed app identity. After native Quit, establish process absence with a separate OS observation.
Refreshing a stopped app handle can relaunch it.

Ad-hoc signing has no TeamIdentifier in these local builds; observed designated
requirements include an exact code hash. Bundle ID alone therefore does not
establish that every policy layer will reuse an approval after rebuilding. The
helper records signing and resource integrity. Record the exact launch and
any approval prompt with the package identity.

## Execution and provenance

The external `desktop/scripts/gui-smoke.py` client sends W3C pointer and keyboard
commands to the actual webview through pinned `tauri-driver 2.0.6`.
JavaScript is used only to read the DOM, focus and dimensions. SVG squares use
real W3C pointer input at observed viewport coordinates, checked against painted
descendants; the large-board journey uses the shared keyboard controls. The verifier does
not call Tauri IPC, inject input events, replace rules or supply engine replies.
This follows Tauri's [WebDriver CI approach](https://v2.tauri.app/develop/tests/webdriver/ci/).
The driver can retry a transient proxy failure once for read-only DOM
observations, screenshots and element lookups. Input and general script
commands use one attempt.

The Linux adapter uses an X11 display, Openbox and focused XTEST keyboard input
for actual GTK file pickers, Alt-F4 and Ctrl-Q. The normal Debian package is
installed with apt; its installed `/usr/bin/mshogi-desktop` and sibling engine
hashes are compared with the package inventory. A separately extracted
verification package supplies difficult game states. It is labeled as packaged
verification evidence, with no claim that a second Linux package was installed.

The Windows adapter uses the actual NSIS-installed executables, native UIA file
dialog controls, a WM_CLOSE request and actual mouse selection of the
mShogi → Quit mShogi menu. An interactive Default
desktop is checked before GUI work. The external driver is matched to the
installed WebView2 Runtime, rather than the Edge browser, using Microsoft's
[documented runtime detection](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/distribution#detect-if-a-webview2-runtime-is-already-installed)
and [WebDriver compatibility guidance](https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/webdriver).
Python 3.12 and the external UI package versions are recorded; they are not
application dependencies. Driver version, registry observations, official
source URL, signature and hashes are retained in `webview-driver.json`.

macOS uses actual CUA interaction with the normal `.app` and separately named
verification `.app`. The narrow Tao patch is documented in [macos-quit.md](macos-quit.md). Cocoa `terminate:` probe
results are identified as that entry, not as an actual Dock click.

## Required journeys

The installed normal application exercises a human move and real engine reply,
picker cancellation, demonstrably active analysis cancellation,
replacement with a bounded read-only suggestion, Save/Open, stopped
reopening, exact historical-prefix analysis, Return to live, separate Resume,
and protected native window close and Quit. The saved document is read
externally to verify the actual persisted context, not used to alter the UI.

Separately labeled verification applications exercise genuine missing-sidecar
failure and retry without losing history, promotion cancellation and complete
underpromotion, an optional intended-move claim, the rolling
256-observation retention policy and minimal saved document, and the shared
synthetic board's cancelled prefix and distinct same-endpoint complete routes.
The writable QA sidecar is restored in `finally` and checked against its original
hash. The actually installed Linux application keeps its original permissions.

A request must reach its application guard. Cancel must preserve the live
session. Explicit completion must terminate the host and bundled engine before
WebDriver teardown. Killing a driver or failure cleanup never counts as passing
native Quit/window-close evidence.

## Reports and reproduction

The checked-in workflow `.github/workflows/desktop.yml` builds normal packages,
checks their instrumentation exclusion, preserves their original identities,
prepares external drivers, builds separate QA packages, then runs the journeys.
Adapter files, Python packages and drivers are outside both application
bundles. Verification entry points are absent from normal distributables.

Each phase has its own evidence directory so picker screenshots and
window bindings cannot overwrite those from another phase. `gui-report.json`
records full source revision, run URL, runner image, actual executable hashes,
driver capabilities, runtime/display/window/process observations,
ordered actions, passed/failed status, desktop/webview screenshots and captured
DOM/focus. Package manifests record installer/archive hashes and installation
or extraction provenance. Failed runs preserve diagnostics; later fixes do not
turn those runs into passes.

For a prepared Linux environment, the external invocation is:

```sh
GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1 dbus-run-session -- xvfb-run -a -s '-screen 0 1440x1100x24' \
  python3 desktop/scripts/gui-smoke.py \
  --normal-host /usr/bin/mshogi-desktop --normal-engine /usr/bin/mshogi \
  --verification-host "$PWD/gui-extracted/usr/bin/mshogi-desktop" \
  --verification-engine "$PWD/gui-extracted/usr/bin/mshogi" \
  --output "$PWD/gui-evidence"
```

The Windows workflow contains the corresponding actual NSIS installation,
matching-driver preparation and Python invocation. Ordinary application
launch/build instructions remain in [desktop/README.md](../../desktop/README.md).
CI artifacts retain full reports and packages temporarily. Task records point
to the relevant run and package identities.
