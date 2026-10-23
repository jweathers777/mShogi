# mShogi rewrite

This repository baseline describes the rewrite of John Weathers's mShogi. It contains a minimal Rust library/headless-console workspace, the design and Backlog.md queue, and the earlier Rust prototype preserved under `legacy/rust-prototype/`. The core implements explicit chess state/FEN plus checked ordinary moves, captures, promotions and undo. Special-move execution, move generation and full adjudication remain later work. The retained prototype has not been certified against the new design.

The product goal is a reusable library with complete implementations of chess, shogi, tori shogi, chu shogi, tenjiku shogi, and xiangqi; capable mShogi engines; a standalone and interoperable console application; and a polished desktop application for Windows, macOS, and Linux.

## Start here

- Product and architectural contract: [docs/design.md](docs/design.md).
- M1 chess position profile: [docs/rules/chess-position-v1.md](docs/rules/chess-position-v1.md), [coverage ledger](docs/rules/chess-coverage.md), and [independent fixture corpus](tests/fixtures/chess/README.md).
- Working instructions for every implementation session: [AGENTS.md](AGENTS.md).
- Importing this package into the real repository: [START-HERE.md](START-HERE.md).
- Build/check commands and dependency policy: [docs/development.md](docs/development.md).
- Current work and evidence: the native task records under `backlog/`.

Design and product decisions are developed with John in ChatGPT. Codex or Claude Code implements bounded work from this repository. A fresh session must be able to continue without access to an earlier conversation.

## Current objective

Establish a chess rules kernel and a headless verification console on the new build/workflow baseline. This first milestone establishes legal move generation, reversible state changes, useful diagnostics, and independent correctness evidence. It is not a claim of complete chess adjudication or competitive playing strength.

Search, protocol interoperability, the desktop application, complete variant implementations, and learned evaluation follow in the roadmap in `docs/design.md`.

## Repository and existing work

The rewrite integration branch is `codex/rewrite`, based on `main_v2` at `fd280dff02fa7ef73809e20b28bdec83fa9c955f`, including its local WIP commit. The legacy `master` branch remains at `941cd4dc37e6e6210d4f993c96a553753e228b19`; neither existing branch was changed during bootstrap. The original `README` is retained under `legacy/rust-prototype/`; `BSD-LICENSE` remains at the root.

MSH-3 moved the pre-existing manifest/lockfile, `src/`, `variants/`, and original README byte-for-byte into [legacy/rust-prototype](legacy/rust-prototype/PRESERVATION.md). That incomplete experiment is excluded from active builds and remains available for deliberate reuse. Its presence does not satisfy the new rules profile or conformance criteria. MSH-1 records the import, MSH-2 the profile/fixtures, and MSH-3 the workspace and build evidence.

## Build and check

Install [Rust through rustup](https://rustup.rs/) and a native C/C++ linker toolchain (see [platform prerequisites](docs/development.md)). The repository pins stable Rust **1.89.0** with rustfmt and Clippy. From a clean checkout of `codex/rewrite`:

```sh
cargo build --workspace --locked
cargo run --locked --bin mshogi -- --help
cargo run --locked --bin mshogi -- --version
cargo run --locked --bin mshogi -- profile

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

The current console reports its version, target profile and partial coverage; game inspection, moves, perft, and engine protocols arrive in later tasks. The core library exposes [position/FEN and ordinary transition APIs](docs/rules/chess-position-v1.md#rust-position-api). Tests exercise real headless subprocess startup, output streams and usage errors. The production core library has no external dependencies or console I/O; a pinned test-only JSON reader consumes the independent corpus. Native Windows/macOS/Linux checks are configured in [the Rust workflow](.github/workflows/rust.yml); MSH-3 records actual per-platform outcomes and revisions separately from configuration.

## Backlog commands

The starter was generated and validated with Backlog.md 1.55.1. Use that version for this project. If a different global CLI is installed, invoke the pinned version with `npm exec --yes --package backlog.md@1.55.1 -- backlog <command>`; a global upgrade is optional.

```bash
npm install -g backlog.md@1.55.1
backlog --version
backlog instructions overview
backlog task list --status Ready --ready --plain
backlog task view MSH-1 --plain
backlog browser
```

`--ready` checks dependencies. Use it together with `--status Ready` when choosing new implementation work. The tool's dependency check does not establish user authorization, correctness, review completion, or compliance with acceptance criteria.

The optional Python fixture checker remains separate research infrastructure; it is not needed to build, test or run the Rust workspace. See [corpus reproduction](tests/fixtures/chess/README.md) for its own evidence and limits.

## Source of truth

The design states requirements and decisions. Backlog.md records work, status, verification, and handoff. Git records changes. There is no parallel GitHub Issues queue or mandatory chronological handoff diary.
