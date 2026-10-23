# Rust development baseline

MSH-3 establishes two active packages: `mshogi-core` (reusable library) and `mshogi-console` (binary name `mshogi`). MSH-4 adds explicit chess position state and validated FEN parsing/serialization; MSH-5 adds checked ordinary moves/captures, promotions and exact undo; MSH-6 adds geometric attack queries; MSH-7 adds pseudo-legal ordinary candidates. The console provides help, version and partial-coverage metadata; it does not yet expose FEN commands. Special-move execution, the legal-move list, search, GUI and match protocols remain later work.

## Clean checkout

Prerequisites are Git, [rustup](https://rustup.rs/), and the host native linker/build tools. On macOS use the Xcode command-line tools; on Windows use the MSVC C++ build tools/Windows SDK with the rustup MSVC host; on Linux install a system C compiler/linker (for example the distribution's build-essential package). GitHub-hosted runners provide these prerequisites. This is a native Rust application; no Node, GUI framework, database or Python environment is needed for its Rust checks.

```sh
git clone --branch codex/rewrite git@github.com:jweathers777/mShogi.git
cd mShogi
rustup show
rustc --version
cargo --version
cargo build --workspace --locked
cargo run --locked --bin mshogi -- --help
cargo run --locked --bin mshogi -- --version
cargo run --locked --bin mshogi -- profile
```

`rust-toolchain.toml` selects **stable release 1.89.0**, minimal profile, rustfmt and Clippy. Rustup can install that exact toolchain/components on first use; initial installation requires network access. This deliberately reproduces the inspected host's stable baseline, not the newest moving `stable` release. It does not change the user's global default. `rust-version = "1.89"` and edition 2024 are shared package metadata. Toolchain updates should change the pin and recorded minimum together, rerun formatting/lint/tests and native CI, and record the actual evidence.

Rustup environment/directory overrides or `cargo +...` can take precedence over the repository pin. Check `rustup show` and `rustc --version` when reproducing evidence. The host installation and compiler version are not proof of successful tests on another platform.

## Repeatable checks

Run from the root; every command targets only the active workspace:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --locked
```

Use `cargo fmt --all` to apply formatting. The active crates inherit a forbid-unsafe-code lint and documentation warnings; Clippy treats all warnings as failures. Do not suppress all warnings to make this baseline green. The core crate may use Rust standard data structures, but rules/search must not depend on console or file I/O, UI or Python. Printing and argument/process handling belong to the console or future adapters.

The five subprocess tests on Unix (four on Windows, where the Unix byte-argument test is inapplicable) exercise documented CLI behavior from the system temporary directory with stdin closed. They verify help/default invocation, version, honest profile status, and usage failures (exit 2, empty stdout, diagnostic stderr). A Unix-only case rejects a non-Unicode argument without a panic. The core FEN integration suite now consumes all MSH-2 FEN contracts and canonical starting states, and tests error precedence, failed replacement, state identity, local king safety and bounded malformed inputs. To isolate it, run `cargo test -p mshogi-core --test fen_corpus --locked`. This does not exercise the corpus move/perft/classification expectations; those await the corresponding behavior tasks. MSH-5 adds `cargo test -p mshogi-core --test ordinary_actions --locked`: exact independently expected before/after sequences, complete LIFO restoration, atomic invalid-action/overflow rejection, explicit unsupported special moves and checked undo mismatch tests. MSH-6 adds `cargo test -p mshogi-core --test attacks --locked`: exact attacker sets and complete per-color attacked-square maps for the independent attack fixtures, the original corpus attack queries, pinned-attacker and occupancy-change cases. MSH-7 adds `cargo test -p mshogi-core --test ordinary_candidates --locked`: exact independent candidate sets, duplicate/own-capture/king-capture absence, distinct promotion transitions, a counter-bound case and an exhaustive comparison of every bounded action with `apply_ordinary` across the fixtures and the 35 corpus positions. All four suites run in the workspace CI without Python.

## Workspace and dependency policy

The root is a virtual Cargo workspace with resolver 3. Its committed `Cargo.lock` covers the two active packages and the core test dependency graph. The console depends only on the local core; neither package has a third-party production Rust dependency. Core integration tests use exact `serde_json = 1.0.145` to read the independently established JSON corpus, rather than duplicate its answers or implement a JSON parser. The official manifest/license were reviewed (MIT OR Apache-2.0); transitive versions and registry checksums are locked. This dev-only reader is not linked into the production library/console and does not change the release-licensing decision. Keep the lockfile committed for reproducible application builds. CI/build/test/lint use `--locked` so unexpected lock changes fail. New dependencies must address the selected task, preserve the library boundary, and receive a license/provenance review; record the reason and update/check the lockfile deliberately. Do not add a frontend, training framework or unused future subsystem here. Packages are `publish = false`; existing notices remain intact and release-licensing decisions remain deferred in the design.

`cargo tree --workspace --locked --edges normal` shows the dependency-free production boundary; without the edge filter it also shows the dev-only JSON reader. `cargo metadata --locked --offline --format-version 1 --no-deps` reports the active package/target inventory without scanning or building the preserved prototype. Production builds need no registry packages. For offline tests, fetch the locked dev dependencies first with `cargo fetch --locked`; once those and the pinned toolchain are cached, the workspace can test with Cargo's `--offline` flag.

The original prototype manifest, lockfile, source, variants and README are preserved byte-for-byte under [legacy/rust-prototype](../legacy/rust-prototype/PRESERVATION.md). `workspace.exclude` intentionally keeps it out of formatting/lint/build/test. The original `main_v2` WIP and C++ `master` remain available in Git. This separation preserves work while avoiding adoption of incomplete rules or console-bound board methods as the new library API. Its compiler failure is recorded, not repaired incidentally; port useful ideas in later bounded tasks with independent tests.

The [Python fixture checker](../tests/fixtures/chess/README.md) is optional research infrastructure outside the Rust dependency graph. The archived prototype's serde/toml dependencies likewise are not active product dependencies.

## CI and evidence

[The workflow](../.github/workflows/rust.yml) runs native `ubuntu-24.04`, `macos-15` and `windows-2022` jobs with the pinned toolchain. It checks formatting, Clippy, tests, build and all three documented console smoke commands. `fail-fast: false` preserves each platform's outcome. Checkout uses the reviewed official action v7.0.1 pinned to commit `3d3c42e5aac5ba805825da76410c181273ba90b1`, read-only repository permissions and no persisted credentials; hosted runners meet its Node 24 runtime requirement.

Pushes to `codex/**` and pull requests targeting `codex/rewrite` trigger the workflow. Ticket-only changes under `backlog/**` are skipped; the last tested code revision remains the relevant evidence. No default branch, repository policy or protection setting is changed to run it.

The MSH-3 task records host/clean-checkout commands, independent review and the actual workflow URL, revision, runner information and outcome per platform. A configured matrix is not a passed run. This task's native results establish scaffold build/test/smoke behavior only; complete M1 rules and packaged desktop evidence remain obligations of later tasks, especially MSH-14 and M3.

References: [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html), [rustup toolchain selection](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file), [Rust 1.89.0 stable release](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/).
