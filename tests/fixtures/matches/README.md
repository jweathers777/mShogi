# Recorded M2 UCI experiments

These are actual local process experiments captured on 2026-10-25 Europe/Paris, using macOS 27.0.1 / Darwin 27 arm64, Apple M1 Pro (10 cores, 32 GiB), Rust/Cargo 1.89.0 and default release builds. They establish the exercised integration and conditional rerun observations, without a playing-strength claim. CI replays these observations on other platforms; it does not claim Stockfish was run there.

## Executables and provenance

| Artifact | Identity / source |
| --- | --- |
| mShogi engine | Source `e35ce585883313294f9d8d43b9ab7e58f9bbbb26`, `cargo build --release --bin mshogi --locked`; SHA-256 `5bd9b9c1f71828cbedf2f2811e17e1533193524e02e5f3b8d7213d7856be85d6`; copied to `/tmp/msh-m2-engine` |
| Runner | Source in this evidence's containing implementation commit; `cargo build --release -p mshogi-runner --bin mshogi-match --locked --offline`; executable SHA-256 `268f21fbc05cc4f431c6277af394fbade14cc44c7f777a1f6c0803c228db6719` |
| Independent opponent | Official [Stockfish 19 release](https://github.com/official-stockfish/Stockfish/releases/tag/sf_19), release source `edb0d9d`; [macOS universal archive](https://github.com/official-stockfish/Stockfish/releases/download/sf_19/stockfish-macos-universal.tar.gz) SHA-256 `a1f0e3bcc5a6927a11fe6fc8e54a779754645f3c2bae2cf13420fd1957adaa77`; executable SHA-256 `8eed61129d1493c5d1f2fd9323f0c54c47ac49319911fbde18c6b9c87e8b13c5` |

The independent binary identifies itself as Stockfish 19. Its default embedded network is `nn-1a298aa575a0.nnue`; the executable fingerprint includes the embedded data. It runs with Threads 1, Hash 16 MiB, MultiPV 1, Skill Level 20, UCI_LimitStrength false and Ponder false. Every game starts fresh processes. Both engines receive depth 2 and 10,000 nodes, with no clock or movetime for the work-limited reruns. mShogi selects its advertised ClaimPolicy auto; Stockfish has no such custom option, and the controller remains the authoritative claimant/adjudicator.

Stockfish is an optional separate GPL-3.0 engine, downloaded with its release notices into temporary storage. Its binary, model and source are not vendored or linked into mShogi. The report preserves its actual handshake, commands, search observations and identities.

## Commands and results

From the rewrite root, after building and placing the independent executable at the configured path:

```sh
cp target/release/mshogi /tmp/msh-m2-engine
target/release/mshogi-match run tests/fixtures/matches/selfplay.config.json selfplay.report.json
target/release/mshogi-match run tests/fixtures/matches/stockfish19.config.json stockfish19.report.json
target/release/mshogi-match run tests/fixtures/matches/timed-selfplay.config.json timed-selfplay.report.json
target/release/mshogi-match run tests/fixtures/matches/known-repetition.config.json known-repetition.report.json
target/release/mshogi-match run tests/fixtures/matches/search-failure.config.json search-failure.report.json
target/release/mshogi-match run tests/fixtures/matches/clock-cancellation.config.json clock-cancellation.report.json
target/release/mshogi-match run tests/fixtures/matches/selfplay.config.json selfplay.rerun.report.json
target/release/mshogi-match run tests/fixtures/matches/stockfish19.config.json stockfish19.rerun.report.json
target/release/mshogi-match replay stockfish19.report.json
```

The committed outputs live alongside these configurations. Use fresh output filenames when reproducing: the CLI refuses overwrites. Paths in the configurations reflect the actual macOS run; adjust them for new runs and retain the new executable identities. Only offline replay requires neither installed engines nor those paths.

| Configuration | Recorded outcome |
| --- | --- |
| selfplay | Forced ending: White mates after two plies in both color assignments. Standard start: both games conclude through a valid threefold claim after 13 played plies; the intended claim action is not played |
| stockfish19 | Forced ending: White mates after two plies in both assignments, with both engines searching. Standard start: Stockfish wins by rules-verified mate as Black after 48 plies and as White after 87 plies |
| timed-selfplay | Two complete forced-ending games with 1 second per side and 100 ms increment. Recorded Observe/Start/Move events replay the actual balances and ordering; no identical timed rerun promise |
| known-repetition | Eight known knight-cycle prefix actions establish three occurrences. Controller claims before any `go`; this is an exercised claim, not an automatic fivefold outcome |
| clock-cancellation | Real mShogi has an outstanding depth-128 / 10-second-movetime search under a 50 ms chess clock. Controller records actual delayed flag detection, no played move, explicit unresolved/unscored outcome, then successful quit of both processes without receiving a bestmove |
| search-failure | Maximal imported fullmove counter permits the root move but causes a descendant search representation failure. Real mShogi exits 1; runner records `unexpected_exit`, no chess conclusion, one unscored failure |

All thirteen first-run games replayed successfully. Eight untimed games were then rerun with the same configuration, executable/model identities and documented deterministic settings. Their complete core command sequences, color assignments and conclusions matched; wall times, PIDs and transport observations were excluded from that comparison. The two additional reports preserve the rerun evidence. No output/identity/stderr truncation or cleanup failure occurred in any of these twenty-one games.

The forced ending is independently hand-derived: `7k/8/6K1/5Q2/8/8/8/8 b - - 0 1` gives Black only `Kh8-g8`, because White's king covers g7/h7. White's `Qf5-c8#` covers rank eight while the king covers f7/g7/h7. This is a compact interoperability fixture. Standard-start games exercise longer histories; this tiny, shallow sample does not measure strength.

## Portable verification

`cargo test -p mshogi-runner --test recorded_evidence --locked` replays all eight reports / twenty-one games, checks the conditional rerun comparison and verifies the actual claim/failure distinction. It does not launch an external engine. Native process tests separately exercise start/exit/invalid/missing/timeout failures, stream bounds, cleanup and clock precedence using the portable Rust fixture. Unix-only checks cover FIFO and non-UTF-8 arguments. Source revisions, independent reviews and actual three-platform CI links remain in MSH-23 and MSH-25.
