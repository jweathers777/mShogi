# mShogi

mShogi is a native multi-game engine, console and desktop application. It supports
Chess, standard Shogi, Xiangqi, Tori Shogi, Chu Shogi and Tenjiku Shogi. Tenjiku has
two separately identified rules profiles: Adams and Muller. The native rules own
legal complete actions, history and game results for every client.

The Tauri desktop provides untimed human-versus-engine and local two-human play,
read-only analysis, Save/Open, Take back and history inspection through one shared
interface. Large
boards, drops, promotions and compound routes use the selected game's rules.
The six-game integration milestone is complete; see the
accepted delivery record. This is an unreleased
product. Signing, distribution policy and release qualification remain separate
work. Functional game support does not establish competitive playing strength.

## Start here

- [Desktop build, launch and play](desktop/README.md).
- [Native development, CI and platform prerequisites](docs/development.md).
- [Accepted product and architecture](docs/design.md).
- [Current search core](docs/m5/shared-search-core.md), [Chess evaluation](docs/evaluation.md)
  and [strength roadmap](docs/m5/strength-roadmap.md).
- [Agent workflow](AGENTS.md) and the repository Backlog task records.
- [Rules conformance policy](docs/testing/conformance-policy.md).

## Build and check

The native workspace pins stable Rust **1.99.1**. Install Rust and the host linker
as described in the development guide. From a clean checkout of `main`:

```sh
cargo build --workspace --locked
cargo run --locked --bin mshogi -- --help
cargo run --locked --bin mshogi -- --version
python3 tools/checks.py install-hooks
python3 tools/checks.py run --suite rust-format --suite rust-lint --suite rust-test
```

For the desktop, install its platform prerequisites, then:

```sh
cd desktop
npm ci
npm run desktop:dev
```

The desktop builds and supervises the native engine. Python is not a product
runtime dependency. Required fonts and their notices are bundled.

## Console and engine interfaces

Select a game explicitly: `mshogi --game chess shell`, or use `shogi`, `xiangqi`,
`tori`, `chu` or `tenjiku` in place of `chess`. Type `help` for the selected game's
commands and supported bounds. Each console uses the same native rules as the
desktop. Canonical records preserve complete actions and known history; board
notation alone is not a complete saved game.

The [UCI adapter](docs/uci.md) serves Chess. The [USI adapter](docs/m4/usi.md)
serves standard Shogi. The structured internal-search interface supports native
engine execution across all six games. The [match runner](docs/match-runner.md)
and shared experiment runner retain replayable records, executable identities,
resource bounds and failures. Their available protocol adapters are explicit;
not every game supports every external protocol.

Chess and Tori use the game-independent classical search core. The other games
retain their functional native search paths while shared-core adoption continues.
The native rules remain authoritative for the real game even where private
search uses approximate history-dependent analysis. Speed, correctness and playing
strength are separate measurements.

## Repository history and current scope

`main` is the integration and default development branch. The original C++ tip
is preserved by the annotated `legacy-cpp` tag at `941cd4dc`. The `master` and
`main_v2` branches retain the original C++ and earlier Rust histories. The former
`codex/rewrite` line is part of `main` history. No orphan root or history rewrite
is used. [BSD-LICENSE](BSD-LICENSE) and third-party notices remain applicable.

Mini Shogi, the old custom `.pgn`-named file format and legacy search/Force
controls are not current requirements. Local two-human desktop play uses the same
native actions and saved history as human-versus-engine play. See
[two-human play](docs/desktop/two-human-play.md). The optional frozen legacy Chu
opponent and runner adapters are tracked separately by MSH-173 through MSH-176; they are not
product or conformance dependencies.

Use Backlog.md **1.55.3** for task operations. Run `backlog instructions overview`
and the relevant operation guide before changing task records. Historical reports,
source-specific measurements and the bootstrap handoff remain available in Git
and the documentation; their dates and source revisions limit their claims.

## Checked Chess console example

This example exercises atomic rejection and exact undo. The failed castling
command emits the recorded diagnostic and exits with status 1.

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
