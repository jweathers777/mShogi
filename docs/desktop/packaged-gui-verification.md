# Packaged application GUI verification

MSH-38 verifies actual installed or packaged applications. Normal package
construction, inventory and bundled-engine protocol reports are separate
supporting evidence. M3 stays open until Windows, macOS and Linux interaction
requirements and the independent MSH-39 review pass.

## Execution and provenance

The external `desktop/scripts/gui-smoke.py` client sends W3C pointer and keyboard
commands to the actual native webview through pinned `tauri-driver 2.0.6`.
JavaScript is used only to read the DOM, focus and dimensions. SVG squares use
real W3C pointer input at observed viewport coordinates, checked against painted
descendants; the large-board journey uses the shared keyboard controls. The verifier does
not call Tauri IPC, inject input events, replace rules or supply engine replies.
This follows Tauri's [native WebDriver CI approach](https://v2.tauri.app/develop/tests/webdriver/ci/).

The Linux adapter uses an X11 display, Openbox and focused XTEST keyboard input
for actual GTK file pickers, Alt-F4 and Ctrl-Q. The normal Debian package is
installed with apt; its installed `/usr/bin/mshogi-desktop` and sibling engine
hashes are compared with the package inventory. A separately extracted
verification package supplies difficult native states. It is labeled as packaged
verification evidence, with no claim that a second Linux package was installed.

The Windows adapter uses the actual NSIS-installed executables, native UIA file
dialog controls, a native WM_CLOSE request and Ctrl-Q. An interactive Default
desktop is checked before GUI work. The external driver is matched to the
installed WebView2 Runtime, rather than the Edge browser, using Microsoft's
[documented runtime detection](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/distribution#detect-if-a-webview2-runtime-is-already-installed)
and [WebDriver compatibility guidance](https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/webdriver).
Python 3.12 and the external UI package versions are recorded; they are not
application dependencies. Driver version, registry observations, official
source URL, signature and hashes are retained in `webview-driver.json`.

macOS uses actual CUA interaction with the normal `.app` and separately named
verification `.app`. MSH-37's exact-source evidence is reused where MSH-38 changes
only external verification tools. The narrow Tao patch and its original
provenance remain in [macos-quit.md](macos-quit.md). Cocoa `terminate:` probe
results are identified as that native entry, not as an actual Dock click.

## Required journeys

The installed normal application exercises a human move and real engine reply,
unsaved replacement cancellation, demonstrably active analysis cancellation,
replacement with a bounded read-only suggestion, native Save/Open, stopped
reopening, exact historical-prefix analysis, Return to live, separate Resume,
and protected native window close and Quit. The canonical saved archive is read
externally to verify the actual persisted context, not used to alter the UI.

Separately labeled verification applications exercise genuine missing-sidecar
failure and retry without losing history, promotion cancellation and complete
underpromotion, an optional intended-move claim, observation 256 accepted and
257 rejected while the complete archive remains saveable, and the shared
synthetic board's cancelled prefix and distinct same-endpoint complete routes.
The writable QA sidecar is restored in `finally` and checked against its original
hash. The actually installed Linux application keeps its original permissions.

A native request must reach its application guard. Cancel must preserve the live
session. Explicit completion must terminate the host and bundled engine before
WebDriver teardown. Killing a driver or failure cleanup never counts as passing
native Quit/window-close evidence.

## Reports and reproduction

The checked-in workflow `.github/workflows/desktop.yml` builds normal packages,
checks their instrumentation exclusion, preserves their original identities,
prepares external drivers, builds separate QA packages, then runs the journeys.
Native adapter files, Python packages and drivers are outside both application
bundles. Verification entry points are absent from normal distributables.

Each phase has its own native evidence directory so picker screenshots and
window bindings cannot overwrite those from another phase. `gui-report.json`
records full source revision, run URL, runner image, actual executable hashes,
driver capabilities, native runtime/display/window/process observations,
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
matching-driver preparation and native Python invocation. Ordinary application
launch/build instructions remain in [desktop/README.md](../../desktop/README.md).
CI artifacts retain full reports and packages temporarily; selected reports,
screenshots and exact CI references are committed with final task evidence.

## Current result

First external Windows/Linux run: source
`9b7a2a6b29c30b8d417d75cb4393dd6ba43f6ad0`,
[desktop CI 40228382037](https://github.com/jweathers777/mShogi/actions/runs/40228382037).
The first Linux attempt failed on SVG group interactability and produced a blank
native window under Xvfb with DRI3 warnings. Its [failed evidence](evidence/m3-platforms/linux-attempt-1/attempt.json)
is retained, separately from any corrected rerun. Real W3C pointer input and a
recorded CI-only `WEBKIT_DISABLE_COMPOSITING_MODE=1` override address these
observed issues; [WebKit documents the override](https://trac.webkit.org/wiki/EnvironmentVariables).
Actual corrected GUI results remain pending. Static checks are not GUI passes. macOS actual interaction
is recorded in [the completion-batch evidence](completion-batch-evidence.md).


The first Windows attempt established actual NSIS-installed play on an
interactive desktop. It then exposed a verifier input problem: Element Clear
left React's previous node-budget state, so typing appended to it and the
frontend correctly rejected the invalid budget. The [failed attempt](evidence/m3-platforms/windows-attempt-1/attempt.json)
is preserved. The correction uses real Ctrl-A/Backspace/typing and checks the
observed field value; it does not set DOM values or bypass validation.

The second Linux attempt at `41866a3c5c30efd88d72584140510c8d1806e902`
([run 40229968153](https://github.com/jweathers777/mShogi/actions/runs/40229968153))
confirmed a readable native desktop, real bundled-engine play, active analysis
cancellation and bounded replacement. Its native Save wrote the complete
archive, but the verifier left GTK's default extension after the supplied name,
producing `normal-complete.mshogi.mshogi`. The exact-path check correctly failed;
the [partial attempt](evidence/m3-platforms/linux-attempt-2/attempt.json) remains
failed. The adapter now selects the entire native path field, captures it before
acceptance and requires the exact requested file. This correction still requires
an actual rerun; it does not accept the doubled-extension file as the target.

The second Windows attempt in that run also passed real play, active analysis
cancellation and bounded replacement, then saved the picker's default
`game.mshogi` instead of the requested path. Its [failed evidence](evidence/m3-platforms/windows-attempt-2/attempt.json)
also exposed a genuine product focus defect: a newly spawned bundled engine
created a visible Windows console during analysis. The supervised internal
engine launch now uses Windows `CREATE_NO_WINDOW` with its existing pipes;
standalone console/UCI invocation is unchanged. This uses Rust's
[Windows process flags](https://doc.rust-lang.org/std/os/windows/process/trait.CommandExt.html#tymethod.creation_flags)
and Microsoft's [documented console-free launch flag](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags).
Windows native verification must reject a visible console belonging to the
exact bundled engine and validate foreground focus at capture time. The picker
adapter also needs exact-PID ownerless-modal handling and observed real filename
input. These findings require a corrected packaged application rerun.
