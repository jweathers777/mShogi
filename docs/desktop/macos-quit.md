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
proceed normally. Repeated requests are safe: the host/frontend retain a
single pending close decision.

The host prevents window destruction, pauses pending work, and emits the paused
current session snapshot with `game-close-requested`. The single frontend owner
fetches that authoritative paused state and presents its shared guard. Its exact
Open-token check (document, version and document revision) and command-blocking
close approval occur under the same controller lock. After
Save or Discard, `AppHandle::exit` follows Tauri's `Message::RequestExit` path,
which does not call Cocoa's `terminate:` and does not reenter this delegate.
The existing bounded child-shutdown observation still runs after approval.
Cancel keeps the window and the paused game. Save failure leaves close pending.
Forced process termination/crashes cannot promise an interactive dialog.

## Source

The patch lives on John's fork of Tao, branch
[`mshogi/v0.37.2-should-terminate`](https://github.com/jweathers777/tao/tree/mshogi/v0.37.2-should-terminate):
one commit on the upstream `tao-v0.37.2` tag that changes only
`src/platform_impl/macos/app_delegate.rs`. `desktop/src-tauri/Cargo.toml`
selects it with `[patch.crates-io]` at a pinned revision. The change adds no
unsafe block. The same change is offered upstream; when a Tao release with it
reaches Tauri, remove the patch entry.

The separate macOS `desktop-verification` build includes a predefined menu item
labeled `Native Cocoa Quit (verification)`. Unlike the application's custom
Quit item, this predefined Cocoa item invokes `NSApplication::terminate:` and
therefore exercises `applicationShouldTerminate:` through the same
termination entry point as Dock Quit. It is compiled out of normal packages.
This probe permits precise route evidence when automation cannot bind
the Dock; it does not establish that the Dock UI itself was clicked.

References: [Tao 0.37.2 source](https://docs.rs/crate/tao/0.37.2/source/src/platform_impl/macos/app_delegate.rs),
[Apple termination delegate](https://developer.apple.com/documentation/appkit/nsapplicationdelegate/applicationshouldterminate(_:)),
[Tauri runtime source](https://docs.rs/crate/tauri-runtime-wry/2.12.3/source/src/lib.rs).
