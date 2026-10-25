# macOS native Quit interception

MSH-35 requires ordinary Quit to resolve Save/Discard/Cancel while the document
window still exists. Tauri 2.12.5 uses Tao 0.37.2. That Tao release registers
`applicationWillTerminate:` but does not register `applicationShouldTerminate:`.
Cocoa Dock Quit therefore reaches final teardown without Tauri's preventable
`ExitRequested` event. A custom application-menu Quit item fixes Cmd-Q, but does
not fix the Dock route.

The standalone desktop host patches only Tao's macOS application delegate. Its
`applicationShouldTerminate:` callback enumerates a retained Cocoa window array,
selects Tao windows, queues their existing `WindowEvent::CloseRequested` events,
and returns `NSTerminateCancel`. The callback neither destroys windows nor calls
application/frontend handlers synchronously. The integer window IDs are checked
against the runtime's window map when dispatched. Native dialogs and other
non-Tao windows are excluded. If no Tao document window exists, termination may
proceed normally. Repeated native requests are safe: the host/frontend retain a
single pending close decision.

The host prevents window destruction, pauses pending work, and emits the paused
snapshot with `desktop-close-requested`. Its exact document-revision check and
command-blocking close approval occur under the same controller lock. After
Save or Discard, `AppHandle::exit` follows Tauri's `Message::RequestExit` path,
which does not call Cocoa's `terminate:` and does not reenter this delegate.
The existing bounded child-shutdown observation still runs after approval.
Cancel keeps the window and the paused game. Save failure leaves close pending.
Forced process termination/crashes cannot promise an interactive dialog.

## Source provenance and review boundary

`desktop/src-tauri/vendor/tao` is the complete cached crates.io Tao 0.37.2
package, including its license and package metadata. Original crate checksum:
`d5777b06fe6435126ae8d816eba269e050e1e10d302cf5461e9c49eccabc3548`.
Its packaged VCS metadata names commit
`2d9b2ba15247879787878ae565e350bcc1b9f622` (the upstream package itself records
`dirty: true`). The standalone desktop manifest selects it with
`[patch.crates-io]`; the root workspace lock/dependencies are independent.

Byte comparison against all 119 files in the cached upstream package found one
changed file: `src/platform_impl/macos/app_delegate.rs`. All other files are
unchanged. The change adds imports, selector registration and one callback; it
adds no unsafe block and does not weaken the host's `unsafe_code = "forbid"`.
Review the callback delta against the original package rather than treating the
unchanged upstream source as new application code.

The callback compiles on macOS with Rust 1.90. Actual packaged Dock Quit,
Cmd-Q and window-close Save/Discard/Cancel evidence is required separately;
compilation alone does not establish the native UI behavior. Linux/Windows
compilation is also checked independently because the package remains shared.

The separate macOS `desktop-verification` build includes a predefined menu item
labeled `Native Cocoa Quit (verification)`. Unlike the application's custom
Quit item, this predefined Cocoa item invokes `NSApplication::terminate:` and
therefore exercises `applicationShouldTerminate:` through the same native
termination entry point as Dock Quit. It is compiled out of normal packages.
This probe permits precise native-route evidence when automation cannot bind
the Dock; it does not establish that the Dock UI itself was clicked.

References: [Tao 0.37.2 source](https://docs.rs/crate/tao/0.37.2/source/src/platform_impl/macos/app_delegate.rs),
[Apple termination delegate](https://developer.apple.com/documentation/appkit/nsapplicationdelegate/applicationshouldterminate(_:)),
[Tauri runtime source](https://docs.rs/crate/tauri-runtime-wry/2.12.3/source/src/lib.rs).
