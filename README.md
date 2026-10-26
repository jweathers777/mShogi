# mShogi rewrite

This repository baseline describes the rewrite of John Weathers's mShogi. It contains a minimal Rust library/headless-console workspace, the design and Backlog.md queue. The core implements explicit chess state/FEN, geometric attack queries, pseudo-legal candidate generation, and checked moves, captures, promotions, castling and en passant with exact undo. It also exposes the legal-action list and position-level check/checkmate/stalemate classification. Perft and divide diagnostics are available in the library and, with every other M1 diagnostic, through the headless `mshogi shell` console. The core also owns [game history and canonical records](docs/rules/chess-history-record-v1.md). A separate native search crate provides the [transparent material evaluator](docs/evaluation.md); [full-width search](docs/search.md) and [bounded live match adjudication/clocks](docs/rules/chess-match-api.md) are available in native libraries. The separate [UCI engine mode](docs/uci.md) connects the native search to external controllers. An opt-in [exact mating-reachability procedure](docs/rules/chess-completeness.md) extends adjudication beyond the default M2 subset, with explicit resource uncertainty.

The product goal is a reusable library with complete implementations of chess, shogi, tori shogi, chu shogi, tenjiku shogi, and xiangqi; capable mShogi engines; a standalone and interoperable console application; and a polished desktop application for Windows, macOS, and Linux.

## Start here

- Product and architectural contract: [docs/design.md](docs/design.md).
- M1 chess position profile: [docs/rules/chess-position-v1.md](docs/rules/chess-position-v1.md), [coverage ledger](docs/rules/chess-coverage.md), and [independent fixture corpus](tests/fixtures/chess/README.md).
- Working instructions for every implementation session: [AGENTS.md](AGENTS.md).
- M1 closure evidence: [docs/review/m1-closure.md](docs/review/m1-closure.md).
- M2 integration evidence and limits: [docs/review/m2-closure.md](docs/review/m2-closure.md).
- Desktop launch/build instructions: [desktop/README.md](desktop/README.md); M3 integration evidence: [docs/review/m3-closure.md](docs/review/m3-closure.md).
- Historical bootstrap record (M0, completed): [START-HERE.md](START-HERE.md).
- Build/check commands and dependency policy: [docs/development.md](docs/development.md).
- Current work and evidence: the native task records under `backlog/`.

Design and product decisions are developed with John in ChatGPT. Codex or Claude Code implements bounded work from this repository. A fresh session must be able to continue without access to an earlier conversation.

## Current objective

The native chess baseline provides rules/history, live match adjudication and clocks, transparent evaluation, full-width search, cancellation and UCI. The [match runner](docs/match-runner.md) connects real UCI engines to that rules boundary and preserves replayable experiment evidence. Default M2 dead recognition remains a sound subset; exact opt-in analysis can exceed practical resource limits. No competitive playing-strength claim follows.

The Tauri desktop application plays untimed chess against the bundled native engine, runs bounded read-only analysis, saves and reopens complete games, and inspects history. Its [three-platform packaged GUI evidence](docs/desktop/packaged-gui-verification.md) distinguishes actual application interaction from package construction. Remaining games, measured engine strength, learned evaluation and release distribution follow the preserved roadmap in `docs/design.md`.

## Repository and existing work

The rewrite integration branch is `codex/rewrite`. The original C++ branch `master` and earlier Rust branch `main_v2` retain their history; `BSD-LICENSE` remains at the root.

The aborted Rust prototype is no longer part of the active tree. Its manifest, lockfile, source, variants and original README remain available at [historical revision `fd280dff02fa7ef73809e20b28bdec83fa9c955f`](https://github.com/jweathers777/mShogi/tree/fd280dff02fa7ef73809e20b28bdec83fa9c955f).

## Build and check

Install [Rust through rustup](https://rustup.rs/) and a native C/C++ linker toolchain (see [platform prerequisites](docs/development.md)). The repository pins stable Rust **1.89.0** with rustfmt and Clippy. From a clean checkout of `codex/rewrite`:

```sh
cargo build --workspace --locked
cargo run --locked --bin mshogi -- --help
cargo run --locked --bin mshogi -- --version
cargo run --locked --bin mshogi -- profile
printf 'position startpos\nperft 3\n' | cargo run --locked --bin mshogi -- shell

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

The console reports its version, rules profile and coverage, and `mshogi shell` runs the chess diagnostics described below; `mshogi uci` provides the documented first engine protocol adapter. The core library exposes [position, action, legal-move, classification and perft APIs](docs/rules/chess-position-v1.md#rust-position-api). Tests exercise real headless subprocess startup, output streams and usage errors. The production core uses pinned Serde/JSON for canonical history records and has no console I/O; tests also use that JSON reader for the independent corpus. Native Windows/macOS/Linux checks are configured in [the Rust workflow](.github/workflows/rust.yml). Actual per-platform outcomes and revisions are recorded in each task and summarized in the [M1 closure review](docs/review/m1-closure.md), separately from configuration.

## UCI engine mode

`cargo run --release --locked --bin mshogi -- uci` starts the [bounded UCI adapter](docs/uci.md). It supports handshake/readiness, complete position history, depth/node search limits, basic clock/increment or movetime budgets, and responsive stop/quit. Protocol output uses stdout; diagnostics use stderr. The custom `ClaimPolicy` option defaults to explicit; unattended controllers can select auto so future search nodes follow their recorded claim policy. The controller handles actual draw claims because UCI has no claim command. See the adapter contract for supported controls, fallback behavior and limitations. The [native runner and recorded independent-engine experiments](docs/match-runner.md) exercise this path.

## Native match experiments

`cargo run --release --locked --bin mshogi-match -- run CONFIG.json REPORT.json` runs paired self-play or external UCI games through the core match API. `mshogi-match replay REPORT.json` validates their canonical games, commands, clocks and results without launching opponents. Configurations retain executable/model identities, options, deterministic work or clock limits and failure evidence. See [match-runner.md](docs/match-runner.md) for actual Stockfish interoperability, setup, bounds and the distinction between record replay and conditional deterministic reruns.

## Headless chess console

`mshogi shell` reads chess commands from standard input, one per line, interactively or from a pipe or file. It starts at the standard position. It is a diagnostic console for people and simple scripts, not a UCI engine or a machine protocol. Every rule comes from the core library.

| Command | Effect and output |
| --- | --- |
| `position startpos` / `position fen <FEN>` | Load a position and clear the undo history; prints `fen <canonical FEN>` |
| `fen` | Print `fen <canonical FEN>` |
| `show` | Board diagram (White upper case, rank 8 first), then `fen`, `side`, `castling`, `en-passant`, `halfmove`, `fullmove`, `status` and `history` lines |
| `legal` | `legal <count> <actions...>` in divide order |
| `move <action>...` | Apply the actions in order, all or none; prints the new `fen` |
| `history` | Action count, known-history origin, stored claim policy and current repetition count |
| `record` | `record <JSON>`: canonical chess action/history record |
| `record load <JSON>` | Atomically restore a move-only record, including undo/history; observation-bearing match records are explicitly refused |
| `undo [n]` | Undo the last `n` applied actions (default 1); prints the new `fen` |
| `status` | `status quiet`, `check`, `checkmate` or `stalemate`, from the limited M1 position classifier, not game adjudication |
| `perft <depth>` | `perft <depth> <nodes>` |
| `divide <depth>` | One `<action> <nodes>` line per legal root action in divide order, then `total <nodes>` |
| `help` / `quit` | Show the commands / end the session (end of input also ends it) |

- **Actions** use lowercase coordinates: `e2e4`, a promotion suffix `q`/`r`/`b`/`n` (`e7e8n`), castling as the king's move (`e1g1`), en passant as the capturing pawn's move.
- **Input:** ASCII only, with space, tab, CR, vertical tab and form feed as separators; LF ends a command. Blank lines and lines starting with `#` after trimming are ignored. Lines are limited to 4096 bytes before LF (including CR); an oversized line is discarded in bounded memory, and processing resumes at the next line. Numeric arguments use one or more ASCII digits, with leading zeros allowed and no sign. Undo counts must be positive and fit `usize`; shell perft/divide depth is capped at 4 (divide still needs at least 1). Deeper diagnostics use the uncapped library API or release example.
- **Errors:** results go to standard output. A failed command writes `error: line <n>: <reason>` to standard error and changes nothing in the session: malformed FEN, unknown commands or arguments, an illegal or unrepresentable action anywhere in a `move` sequence, undo beyond the history, invalid UTF-8 or non-ASCII input. Echoed tokens escape control characters. Parse and legality failures in move sequences both identify the action index and preserve the whole session.
- **Exit status:** 0 when every command succeeded, 1 when any failed, 2 for invalid program arguments.

Example (`mshogi shell < session.txt`), checked by the console's process tests:

<!-- console-example:input -->
```text
# Lines starting with # are comments.
position startpos
move e2e4 e7e5 g1f3 b8c6
move e1g1
status
undo 4
perft 3
position fen 7k/8/8/8/8/8/8/K7 w - - 0 1
legal
divide 2
```

Standard output:

<!-- console-example:stdout -->
```text
fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
fen r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3
status quiet
fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
perft 3 8902
fen 7k/8/8/8/8/8/8/K7 w - - 0 1
legal 3 a1b1 a1a2 a1b2
a1b1 3
a1a2 3
a1b2 3
total 9
```

Standard error (exit status 1, because line 4 failed and changed nothing):

<!-- console-example:stderr -->
```text
error: line 4: e1g1 (action 1 of 1): action rejected: castling right, rook or empty path is missing; no actions applied
```

## Standalone shogi console

`cargo run --locked --release --bin mshogi -- shogi` explicitly selects the
untimed native `mshogi.shogi.csa36.v1` game. Type `help` for commands. For example:

```bash
cargo run --locked --release --bin mshogi -- shogi <<'EOF'
position startpos
move 7g7f 3c3d
show
history
undo 2
perft 2
EOF
```

This prints the native board, hands, side, event history and conclusions;
initial local perft2 is900. SFEN imports start a new explicitly identified
known-history segment. For a sparse drop/promotion example:

```text
position sfen reduced 8k/9/9/4P4/9/9/9/9/K8 b P 17
legal
move 5d5c+
history
undo
move P*4e
```

`eligibility sente` is a harmless query. `declare sente` is a formal command:
an ineligible declaration loses under the selected profile. `resign gote`
submits native resignation, even outside Gote's turn. `undo` reverses an event,
including declarations/resignation that do not advance board ply.

`record save <new-path>` publishes a complete canonical record to a new file;
it never overwrites an existing file. `record load <path>` strictly validates and
replays the complete record before replacing the current game. Paths may contain
spaces and UTF-8. Record files have a separate1MiB limit; command lines remain
4096bytes. SFEN/USI text alone cannot preserve provenance or event history.
See [shogi console contract and evidence](docs/m4/shogi-console.md) for readable
notation, diagnostic limits, canonical semantics and exact record examples.

The existing `shell` chess commands, `uci` adapter and structured
`internal-search` wire contracts remain separate.

Native shogi v3 now supports engine-turn proposals and bounded read-only move
analysis. The transparent baseline uses board/promoted pieces and both hands,
with explicit native declarations/outcomes and root-side score observations.
Defaults are depth2/5,000 nodes; supported finite bounds are depth1–4 and
0–20,000 nodes. See the [native engine contract and runnable structured example](docs/m4/shogi-engine.md).
Shogi USI interoperability and desktop play remain later tickets.

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
