# External controller-lock probe

This is a standalone evidence client, outside the product workspace members,
examples and CI. It measured production source
`3a290024df82d23f215165707c81de4928ad3a4a` with Rust 1.99.1, offline release mode,
on the macOS development machine. `stdout.jsonl` is the original observed output;
the shared verification record owns interpretation and acceptance.

The actual public `DesktopController` is inside `std::sync::Mutex`, with the
host's shutdown/lifecycle checks around inspection. For each profile, the client
uses the existing native verification inputs for standard, dense Fire Demon and
dense Area3 positions: 156 actual original identities in every case. It queries
each of the 156 occupied origins three times. `Instant` measures acquisition wait,
acquired-to-guard-drop time, and JSON serialization **after** guard release
separately. Three snapshot calls per case use the same actual guard boundary.

The contention sample starts a second thread holding the real controller mutex
for one actual inspection. A readiness channel coordinates the attempt; its
overhead is explicitly included in the worker's held time. No synthetic sleep,
timing threshold or guaranteed response time is used. Token, native canonical
context and dirty state are asserted unchanged after every case.

This is an actual standard-library controller-lock measurement. It does not
measure Tauri IPC, query JSON parsing, native WebView paint, scheduler guarantees,
production deadline enforcement or platform-wide worst cases. Tiny uncontended
wait values include timer/instrumentation effects; they establish no finer
lock-wait bound. The fixed run order and allocator/cache effects also limit direct
timing comparisons between profiles.

From the repository root, reproduce with the same production source and toolchain:

```sh
MSHOGI_ROOT="$PWD"
CARGO_TARGET_DIR="$MSHOGI_ROOT/target" cargo +1.99.1 run --locked --offline --release \
  --manifest-path "$MSHOGI_ROOT/docs/m4/tenjiku/evidence/controller-lock/Cargo.toml" \
  > /private/tmp/msh79-controller-lock-repeat.jsonl
```

The manifest uses only relative repository dependencies and pinned `serde_json`;
the lockfile contains no local machine paths. The existing target cache can be
reused. Running against later production changes is a new observation: the
source reference embedded in this frozen client identifies the original run and
must not be attributed to a later revision without recording that revision.
