# Native shutdown verification (macOS)

The first actual controlled-child check failed: macOS's predefined Quit invokes
Cocoa termination directly, bypassing Tauri's ExitRequested event. The child
survived until its 30-second natural exit. This was a real integration finding,
not a passing shutdown result. The retained
[proof](evidence/shutdown-macos.json) includes that failed observation and the
subsequent corrected observations with executable identities.

The fix routes the ordinary Quit menu/shortcut through a custom menu event and
the serialized shutdown gate. Final Exit also signals/observes cleanup for
direct native termination routes. Both paths share the first shutdown signal's
two-second observation deadline. Actual process cleanup stays on the supervisor;
the last-resort final-teardown wait only polls metadata. Timeout/cleanup
uncertainty is logged, including after cancelled publication.

Verification uses a separately named copy of the normal application. Its host
executable is byte-identical to the normal host. Only the copied sidecar is
replaced by an external fixture; the normal application and its real engine
remain unchanged. There are no production test commands, hidden restore
endpoints or fixture startup flags.

Reproduce on macOS after building a normal package:

1. Copy the `.app` to `/tmp/mShogi Shutdown Verification.app`. In its copied
   Info.plist set CFBundleIdentifier to
   `org.mshogi.desktop.shutdown-verification`, and CFBundleName/DisplayName to
   `mShogi Shutdown Verification`. Preserve CFBundleExecutable. Record SHA-256
   of both host executables and assert equality.
2. Compile the external fixture and copy it **only** over the verification
   copy's `Contents/MacOS/mshogi`:

   ```sh
   rustc --edition 2024 --crate-name mshogi_shutdown_fixture \
     desktop/scripts/verification/shutdown-child.rs -o /tmp/mshogi-shutdown-fixture
   ```

3. Launch the verification copy, then start the external POSIX observer:

   ```sh
   python3 desktop/scripts/verification/shutdown-observer.py
   ```

   It clears only its named `/tmp/mshogi-*` observation files, waits for the
   fixture's PID, and samples that process until it disappears. It does not
   terminate the child. This PID observation procedure was run on macOS; it is
   not Windows application evidence.
4. Choose Black and start a new game. Confirm Searching with zero plies. Record
   Unix milliseconds in `/tmp/mshogi-quit-request-ms` immediately before using
   the actual Quit shortcut/menu or window close control. In the recorded run,
   native UI automation wrote this marker and delivered the UI action together.
5. The observer must have seen the child alive after the recorded request, then
   gone before the 15-second watchdog and 30-second natural completion. Preserve
   its JSON. Repeat separately for window close. Recheck that the normal
   engine's SHA-256 is unchanged.

Corrected actual observations: Quit removed the child 127.477 ms after the
request; window close on the final host removed it after 42.663 ms. These are
observations, not promised realtime limits. Force-kill and machine failure
cannot guarantee graceful cleanup. Full three-platform installed interaction
verification remains MSH-38; this focused native check does not close M3.
