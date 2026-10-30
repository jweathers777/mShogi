# Native snapshot fixtures

Generated from `DesktopSession::from_record` and `snapshot`, using the real
Rust core under explicit-claim M2 adjudication. They exercise frontend selection
and rendering against actual serialization; they are not an independent rules
oracle. M1/M2 rules fixtures remain authoritative correctness evidence.

Regenerate from the repository root:

```sh
cargo run --locked -p mshogi-desktop-session --example write_frontend_fixtures -- desktop/src/__fixtures__
```

Verification-only capacity and terminal-state fixtures also use the actual generator:

```sh
cargo run --locked -p mshogi-desktop-session --features desktop-verification --example write_frontend_fixtures -- desktop/src/__fixtures__
```

MSH-116 regenerates durable observations under schema1 without a transport wrapper.
The verification generator also refreshes native Take back capabilities; those
are existing native behavior, not a new frontend rule or feature.

Only frontend tests import these JSON files. Vite production imports do not
reference them, and the native fixture generator is never a bundled sidecar.

The Xiangqi standard snapshot, native capture/recapture played ledger and read-only Black-cannon inspection are generated
by the same authoritative shared controller (`xiangqi::Game::standard`), including
the native flat Xiangqi settings encoding,90 intersections and complete choices.
They are interaction/serialization inputs, not a second rules oracle. The frozen
XQ-B01 opening expectations and independently reviewed native fixtures own rules
correctness. No fixture, verification controller or fake result enters a normal
product package.
