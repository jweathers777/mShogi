# Rust development baseline

MSH-3 establishes two active packages: `mshogi-core` (reusable library) and `mshogi-console` (binary name `mshogi`). MSH-4 adds explicit chess position state and validated FEN parsing/serialization; MSH-5 adds checked ordinary moves/captures, promotions and exact undo; MSH-6 adds geometric attack queries; MSH-7 adds pseudo-legal ordinary candidates; MSH-8 adds checked castling/en passant through `Position::apply` and special candidates; MSH-9 adds the legal-action list and position classification; MSH-10 adds perft and divide. MSH-11 adds the `mshogi shell` command session (see the README) over those APIs; the console provides help, version and coverage metadata as well. MSH-19 adds `mshogi-search`, a material-only static evaluator with no outcome semantics (see [evaluation](evaluation.md)). MSH-16 adds rules-owned Game/history and [versioned chess records](rules/chess-history-record-v1.md); the console delegates apply/undo and record reload to that layer. MSH-17/18 add live adjudication and semantic match/clock records; MSH-20 adds full-width native search. MSH-21 adds [bounded request coordination](search-coordination.md); MSH-22 adds the [first UCI adapter](uci.md) as `mshogi uci`. MSH-23 adds `mshogi-runner` / `mshogi-match` for [recorded UCI experiments](match-runner.md). The desktop remains later work.

## Clean checkout

Prerequisites are Git, [rustup](https://rustup.rs/), and the host native linker/build tools. On macOS use the Xcode command-line tools; on Windows use the MSVC C++ build tools/Windows SDK with the rustup MSVC host; on Linux install a system C compiler/linker (for example the distribution's build-essential package). GitHub-hosted runners provide these prerequisites. This is a native Rust application; no Node, GUI framework, database or Python environment is needed for its Rust checks.

```sh
git clone --branch codex/rewrite https://github.com/jweathers777/mShogi.git
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

The console subprocess tests run the real binary from the system temporary directory. With stdin closed they verify help/default invocation, version, honest profile status and usage failures (exit 2, empty stdout, diagnostic stderr); a Unix-only case rejects a non-Unicode argument without a panic. With piped stdin they run `mshogi shell` sessions: the full load/show/legal/move/undo/status/perft/divide workflow with exact stdout, a rejected action and malformed FEN with preserved state and exact stderr, every argument error, exit statuses, invalid UTF-8 input, `quit`, and the README example reproduced byte for byte. The core FEN integration suite now consumes all MSH-2 FEN contracts and canonical starting states, and tests error precedence, failed replacement, state identity, local king safety and bounded malformed inputs. To isolate it, run `cargo test -p mshogi-core --test fen_corpus --locked`. The FEN suite itself does not exercise the corpus move, perft or classification expectations; the later suites below do. MSH-5 adds `cargo test -p mshogi-core --test ordinary_actions --locked`: exact independently expected before/after sequences, complete LIFO restoration, atomic invalid-action/overflow rejection, explicit refusal of special actions at the ordinary-only boundary and checked undo mismatch tests. MSH-6 adds `cargo test -p mshogi-core --test attacks --locked`: exact attacker sets and complete per-color attacked-square maps for the independent attack fixtures, the original corpus attack queries, pinned-attacker and occupancy-change cases. MSH-7 adds `cargo test -p mshogi-core --test ordinary_candidates --locked`: exact independent candidate sets, duplicate/own-capture/king-capture absence, distinct promotion transitions, a counter-bound case and an exhaustive comparison of every bounded action with `apply_ordinary` across the fixtures and the 35 corpus positions. MSH-8 adds `cargo test -p mshogi-core --test special_actions --locked`: exact special candidates and complete legal sets for independent positions, exact castling/en passant sequences with reverse undo, expected atomic rejections, the exhaustive candidate contract for `apply`, and the full legal sets of all 35 corpus positions. MSH-9 adds `cargo test -p mshogi-core --test legal_actions --locked`: exact legal sets and classifications for all 35 corpus positions and the independent classification fixtures, legal sets for the special-action and candidate families, apply/undo of every legal action (except `CounterOverflow` at the u32 limit) with atomic rejection of every other bounded action, empty legal sets in and out of check, and the counter-domain decision. MSH-10 adds `cargo test -p mshogi-core --test perft --locked`: the corpus perft totals, the independent ordered divides with a first-disagreement locator, depth/error semantics, state preservation and counter-independent traversal. Its three depth-4 divides are `#[ignore]`d; run them with `cargo test --release --locked -p mshogi-core --test perft -- --ignored`. MSH-12 adds `cargo test -p mshogi-core --test sequences --locked`: fixed-seed legal sequences with exact reversal, FEN round trips, independently restated transition effects, the candidate rejection contract, and replay of saved regression sequences. MSH-13 adds `cargo test -p mshogi-core --test action_boundary --locked`: production chess boundary checks and the explicitly synthetic compound-action fixture of the [action-boundary review](review/m1-action-boundary.md). All nine suites run in the workspace CI without Python.

The UCI subset has two parser/stream unit tests and eleven real child-process tests in `cargo test -p mshogi-console --locked`; isolate the process boundary with `cargo test -p mshogi-console --test uci --locked`. These check handshake, readiness while thinking, rules-owned history and atomic position replacement, work/clock limits, optional/auto policy, legal bestmoves, stop/quit, stale suppression and protocol stdout purity. Build a release engine with `cargo build --release --locked --bin mshogi`, then launch `target/release/mshogi uci` (`mshogi.exe uci` on Windows). See [UCI compatibility and limitations](uci.md) before connecting a controller.

## Workspace and dependency policy

The root is a virtual Cargo workspace with resolver 3. Its committed `Cargo.lock` covers the four active packages and their test dependency graph. Search depends on the local core; the console depends on local core and search, including its native request coordinator. The UCI adapter adds no new registry dependencies. MSH-16 adds production `serde = 1.0.229` (derive) and promotes exact `serde_json = 1.0.145` from test-only to production for the real chess-record consumer; their existing transitive versions and registry checksums remain locked. Local manifests and license texts were reviewed (`MIT OR Apache-2.0`); see the [record contract](rules/chess-history-record-v1.md). Core tests also use that JSON reader for the independent corpus. These dependencies add no console, filesystem, network, UI or Python I/O to the rules boundary. Keep the lockfile committed for reproducible application builds. CI/build/test/lint use `--locked` so unexpected lock changes fail. New dependencies must address the selected task, preserve the library boundary, and receive a license/provenance review; record the reason and update/check the lockfile deliberately. Do not add a frontend, training framework or unused future subsystem here. Packages are `publish = false`; existing notices remain intact and release-licensing decisions remain deferred in the design.

MSH-23 adds exact `sha2 = 0.10.9` for streaming executable/model fingerprints in the runner, without changing the core/search boundary. Cached registry manifests, license texts and incremental API source were reviewed: RustCrypto sha2, digest, block-buffer, crypto-common and cpufeatures use MIT OR Apache-2.0; cfg-if, libc and typenum do likewise; generic-array 0.14.7 uses MIT and version_check uses MIT/Apache-2.0. The locked dependency graph compiles on the pinned Rust 1.89; declared transitive minimums are older. No cryptographic implementation was copied or written locally. The `mshogi-runner-fixture` target under runner tests is portable process-test support, not an independent chess engine or a shipped desktop component.

`cargo tree --workspace --locked --edges normal` shows the production Serde/JSON dependency boundary and its locked support packages. `cargo metadata --locked --offline --format-version 1 --no-deps` reports the active package/target inventory without scanning or building the preserved prototype. Fetch the locked registry dependencies with `cargo fetch --locked`; once those and the pinned toolchain are cached, both builds and tests can use Cargo's `--offline` flag.

The original prototype manifest, lockfile, source, variants and README are preserved byte-for-byte under [legacy/rust-prototype](../legacy/rust-prototype/PRESERVATION.md). `workspace.exclude` intentionally keeps it out of formatting/lint/build/test. The original `main_v2` WIP and C++ `master` remain available in Git. This separation preserves work while avoiding adoption of incomplete rules or console-bound board methods as the new library API. Its compiler failure is recorded, not repaired incidentally; port useful ideas in later bounded tasks with independent tests.

The [Python fixture checker](../tests/fixtures/chess/README.md) is optional research infrastructure outside the Rust dependency graph. The archived prototype's serde/toml dependencies likewise are not active product dependencies.

## CI and evidence

[The workflow](../.github/workflows/rust.yml) runs native `ubuntu-24.04`, `macos-15` and `windows-2022` jobs with the pinned toolchain. It checks formatting, Clippy, tests, build and the documented console smoke commands: help, version, profile, and a piped `mshogi shell` session that loads, moves, shows, undoes, classifies and runs perft/divide (a failing command fails the step through its exit status). `fail-fast: false` preserves each platform's outcome. Checkout uses the reviewed official action v7.0.1 pinned to commit `3d3c42e5aac5ba805825da76410c181273ba90b1`, read-only repository permissions and no persisted credentials; hosted runners meet its Node 24 runtime requirement.

Pushes to `codex/**` and pull requests targeting `codex/rewrite` trigger the workflow. Ticket-only changes under `backlog/**` are skipped; the last tested code revision remains the relevant evidence. No default branch, repository policy or protection setting is changed to run it.

The MSH-3 task records host/clean-checkout commands, independent review and the actual workflow URL, revision, runner information and outcome per platform. A configured matrix is not a passed run. Those first results established scaffold behavior only. Each later behavior task records its own native run, and the [M1 closure review](review/m1-closure.md) identifies the integrated-revision evidence. Packaged desktop evidence remains an M3 obligation.

References: [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html), [rustup toolchain selection](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file), [Rust 1.89.0 stable release](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/).

## Perft performance baseline

`cargo run --release --locked -p mshogi-core --example perft_baseline` times depth-4 perft of three reference positions and asserts their independent totals. Adding the `upstream` argument also runs three Stockfish sf_17.1 depth-5 totals.

These numbers record the speed of a correctness diagnostic. They are not a search-speed or playing-strength claim, and they are not a target. The implementation deliberately uses full-snapshot undo and candidate filtering by checked application, without bitboards, caches or unsafe code. Only depth-1 leaves are bulk-counted from the legal list.

Recorded on 2026-10-23 for MSH-10 (Apple M1 Pro, 10 cores, 32 GiB, macOS 27.0.1, rustc 1.89.0 / LLVM 20.1.7, `aarch64-apple-darwin`, default release profile, single thread):

| Case | Depth | Nodes | Seconds | Nodes/s |
| --- | --- | --- | --- | --- |
| initial | 4 | 197,281 | 0.064 | 3,066,081 |
| kiwipete | 4 | 4,085,603 | 1.284 | 3,181,688 |
| promotion-castling | 4 | 422,333 | 0.132 | 3,194,548 |
| kiwipete-sf (`upstream`) | 5 | 193,690,690 | 59.474 | 3,256,737 |
| tactical-sf (`upstream`) | 5 | 89,941,194 | 27.700 | 3,247,007 |
| middlegame-sf (`upstream`) | 5 | 164,075,551 | 50.024 | 3,279,967 |

Timings vary with host load; rerun the command to compare like with like.

## Sequence and differential verification

The purpose and limits of each command:

| Command | Purpose | Limits | Where it runs |
| --- | --- | --- | --- |
| `cargo test -p mshogi-core --test sequences --locked` | Fast default: SplitMix64 seed `0x6D53686F67690001`, 2 playouts of up to 40 plies from each of the 92 distinct start positions of the corpus, attack, special-action, classification and ordinary-candidate families; replays every saved regression | 6,720 plies (asserted), about 4 s in debug | Every workspace test run, including native CI |
| `cargo test --release --locked -p mshogi-core --test sequences -- --ignored` | Deeper fixed-seed run: seeds 1 to 8, 4 playouts of up to 120 plies per start | 316,880 plies (asserted), about 10 s in release | On request (not in CI) |
| `python tools/differential_chess.py --mshogi target/release/mshogi` | Independent differential: the real `mshogi shell` binary against pinned `chess==1.11.2`. Compares perft 2 at every start position, then the full legal set, classification and after-FEN at every ply of seeded oracle-chosen playouts | Default seed 20261024, 3 playouts × 60 plies per start (15,008 plies, a few seconds). Use `--seed`, `--playouts`, `--plies`, `--perft-depth` for larger bounded runs | On request, in a separate Python environment (see the fixture README); not in CI |

Build the binary first with `cargo build --release --locked --bin mshogi`. Agreement in any of these runs is bounded evidence about the sampled positions only. It is not a proof of complete chess rules or of behaviour on a platform where it was not run.

Every start position at the u32 counter limit is played with ordinary counters. At the limit, legal actions are rejected as `CounterOverflow`, as the owner-approved profile requires, and the legal-actions suite already covers that.

**Saving and replaying failures.**

- The pinned ply counts are a trajectory canary: a mismatch means the generated legal sets differ from the pinned run; run the differential to locate the difference.
- A failing Rust sequence prints a replay record: the start FEN and the action prefix up to the first failing ply. Add it to [sequence-regressions.json](../tests/fixtures/chess/sequence-regressions.json).
- `python tools/differential_chess.py --fill <file>` adds oracle expectations to cases that lack them.
- `--save <file>` writes every distinct differential disagreement there as a case with oracle expectations. The failing ply is located from the first missing or differing console line, so a rejected action, a crash or a timeout still yields a replayable prefix, with the full diagnostic text kept as the reason. A perft disagreement is first localized through `divide`, descending to the shallowest node whose legal set differs, so its case replays without a perft comparison.
- The Rust test replays every saved case, checking all invariants and expectations, without the original random run.

Licensing: python-chess is GPL-3.0-or-later and runs only as a separate optional research process that talks to mShogi through its console text. It is not linked into, vendored in, or shipped with mShogi, and no Rust dependency was added.
