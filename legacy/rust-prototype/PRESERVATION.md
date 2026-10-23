# Preserved Rust prototype

MSH-3 moved the original root `Cargo.toml`, `Cargo.lock`, `src/`, `variants/`, and
`README` here without changing their bytes. Source reference:
`main_v2` at `fd280dff02fa7ef73809e20b28bdec83fa9c955f`, including John's local WIP
commit; the same files were retained through rewrite commit
`99adfa8fc580a46e0fd1f5b927ea336390ea1568`. Existing BSD notice remains at the
repository root. The original branches and history are intact.

This directory is excluded from the active workspace, formatting, lint and test
commands. It preserves experiments and regression ideas, not an implementation
of the accepted rules profile or a rules oracle. Do not format or repair these
snapshots as incidental workspace cleanup. Port useful behavior deliberately in
the scoped implementation task with independent expectations.

Before preservation, `cargo test --locked` on the available macOS host reached
compiler error E0063 at `src/game/variant.rs:118`: the `Variant` initializer is
missing its WIP `rules` field. Board code also mixes rules/state and console
printing, and the executable suppresses warnings. Repairing that experiment does
not by itself establish the new profile, library boundary or rules conformance.
No successful prototype build or test result is claimed.
