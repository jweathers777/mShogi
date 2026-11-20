# Development

How to build, test and check mShogi. The checks each change needs are in
[AGENTS.md](../AGENTS.md#checks). Desktop development is in
[desktop/README.md](../desktop/README.md).

## Setup

You need Git, [rustup](https://rustup.rs/) and the host's native build tools:
the Xcode command-line tools on macOS, the MSVC C++ build tools on Windows, a C
compiler and linker on Linux.

```sh
git clone --branch main https://github.com/jweathers777/mShogi.git
cd mShogi
cargo build --workspace --locked
cargo run --locked --bin mshogi -- --help
```

`rust-toolchain.toml` pins stable Rust 1.99.1 with rustfmt and Clippy; the
workspace uses edition 2024. A `cargo +toolchain` or rustup override takes
precedence over the pin, so check `rustc --version` when results differ.

## Workspace

The root is a virtual Cargo workspace. `desktop/src-tauri` is a separate
package outside it.

| Crate | Role |
| --- | --- |
| `mshogi-core` | Native rules, positions, histories and records of all six games |
| `mshogi-conformance` | Development-only conformance machinery for the fixtures; no rules |
| `mshogi-search` | Shared alpha-beta core and per-game search positions and evaluation |
| `mshogi-engine` | Internal engine protocol and process |
| `mshogi-console` | The `mshogi` binary: shell, UCI and other adapters |
| `mshogi-runner` | `mshogi-match`: recorded and clocked engine matches and experiments |
| `mshogi-desktop-session` | Desktop session state between the native rules and the UI |

Rules and search must not depend on UI, console I/O or Python. Product code
forbids `unsafe`; Clippy treats warnings as errors.

## Dependencies

`Cargo.lock` is committed, and builds, tests and lints use `--locked`. A new
dependency must serve the task, keep the rules and search boundary, and have a
reviewed license (the workspace uses MIT or Apache-2.0 dependencies). Say why in
the task. Use `cargo tree --workspace --locked --edges normal` to inspect the
graph. After `cargo fetch --locked`, builds and tests can run with `--offline`.

## CI

Pushes to any branch, and pull requests to `main`, run the
lightweight `Routine checks`; expensive steps run only for the changed areas.
The Rust and Desktop workflows also run on demand:

```sh
# Rust: suite=configuration|search|process|persistence|fixtures|full
#       platform=ubuntu-24.04|macos-15|windows-2022|all
gh workflow run 393975947 --ref main -f suite=process -f platform=all

# Desktop: operation=configuration|host|package, same platforms
gh workflow run 395991468 --ref main -f operation=host -f platform=ubuntu-24.04
```

Use them for platform-sensitive changes on the affected systems. The full
three-platform `suite=full` and `operation=package` runs are for releases.
A skipped or configured-only job is not a pass.

## Rules verification tools

**Perft baseline.** `cargo run --release --locked -p mshogi-core --example
perft_baseline` times Chess perft to depth 4 on three reference positions and
asserts their independent totals. It measures a correctness tool, not search
speed.

**Sequences.** `cargo test -p mshogi-core --test sequences --locked` plays
fixed-seed legal sequences from every fixture start position with exact
reversal, FEN round trips and replay of saved regressions (in every test run).
Add `--release ... -- --ignored` for the deeper run.

**Chess differential.** `python tools/differential_chess.py --mshogi
target/release/mshogi` compares the real `mshogi shell` binary with pinned
python-chess: perft 2 at every start position, then legal sets, classification
and FEN at every ply of seeded playouts. Build the binary first with
`cargo build --release --locked --bin mshogi`. Options: `--seed`, `--playouts`,
`--plies`, `--perft-depth`; `--save FILE` writes disagreements as replayable
cases and `--fill FILE` adds oracle expectations. Add a failing case to
[sequence-regressions.json](../tests/fixtures/chess/sequence-regressions.json);
the Rust test replays it. python-chess (GPL-3.0-or-later) runs only as a
separate optional process and is never linked or shipped. See the
[fixture README](../tests/fixtures/chess/README.md).

Agreement in these runs is evidence about the sampled positions only. The
conformance rules are in [conformance-policy.md](testing/conformance-policy.md).
