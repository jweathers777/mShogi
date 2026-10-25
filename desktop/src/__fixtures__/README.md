# Native snapshot fixtures

Generated from `DesktopSession::from_record` and `snapshot`, using the real
Rust core under explicit-claim M2 adjudication. They exercise frontend selection
and rendering against actual serialization; they are not an independent rules
oracle. M1/M2 rules fixtures remain authoritative correctness evidence.

Regenerate from the repository root:

```sh
cargo run --locked -p mshogi-desktop-session --example write_frontend_fixtures -- desktop/src/__fixtures__
```

Only frontend tests import these JSON files. Vite production imports do not
reference them, and the native fixture generator is never a bundled sidecar.
