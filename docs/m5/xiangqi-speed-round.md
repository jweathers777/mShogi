# MSH-203 (Xiangqi Speed Round)

A follow-up to the Xiangqi program after MSH-198 (Xiangqi Blunder Screen), on John's request (2026-11-19). **Result: a doubling of time is worth about +209 Elo at 5 s + 0.1 s; the speed set (about 20% more nodes per second) plus a 64 MB default table passed its sequential test against the predecessor after 200 pairs, about +41 Elo (approx. 95% 19–64).** Evidence: [evidence/msh203](evidence/msh203/).

## Plan (recorded before the work)

Predecessor: main `4d8497fd` (the MSH-196 engine). First measure what speed and table size are worth; stop the speed work if a doubling were worth less than about 30 Elo. Then one coherent speed set, validated by perft, native differential tests and the sequential predecessor test (H1 = 15). All tests local, macOS arm64, eight games in parallel, 5 s + 0.1 s, xiangqi-openings-v1.

## Measurements of value

| Test | Result | Pentanomial |
| --- | --- | --- |
| Time odds: 10 s + 0.2 s against 5 s + 0.1 s, same engine | **H1** at 40 pairs, +209 Elo (164–261) | [0, 0, 5, 27, 8] |
| Table 64 MB against 16 MB | no decision at 600 pairs, +11.6 Elo (−1.2 to +24.4) | [20, 141, 251, 155, 33] |

Infrastructure for these tests: a UCI `Hash` option for Xiangqi (1–1,024 MB) and per-engine clocks (time odds) in the clocked match route, checked on replay.

## Speed set

| Change | Effect |
| --- | --- |
| Chariot and Cannon targets from precomputed rank (9-bit) and file (10-bit) occupancy tables instead of ray walks | Differential test against the ray walk for every square on random games |
| Attack tests from the target square with direct leg, eye, palace and river checks (no forward target computation), also in SEE's least-attacker search | Differential tests against full target scans (attacks and SEE) |
| Danger-set filter in `make`: the full legality test runs only for General moves, moves in check, moves leaving the General's rank or file or an enemy Horse's leg square, and moves landing on the General's rank or file; the check test only for moves that can give check | Test: legality and check flag equal the native rules for every pseudo-legal move on random games |
| Redundant pseudo-legality test removed from `make` (callers pass generated, killer-checked or decoded moves; kept as a debug assertion) | — |
| Default table 16 → 64 MB | as measured above |

Throughput on two middlegame positions: about 1.06 → 1.28 million nodes per second (+20%). Fixed-depth best moves are unchanged; node counts differ slightly in two of four positions because SEE can choose a different one of two equal attackers.

## Game test

Candidate `e491bcdb` against the predecessor: **H1 at 200 pairs**, pentanomial [4, 35, 90, 52, 19], LLR 3.93, +41.0 Elo (approx. 95% 18.8–63.6), no unscored game, 20 minutes locally. The gain is below the +55 Elo that the time-odds result predicts for 20% more speed; the speed measured on single positions may not hold with eight games in parallel and larger tables.

## Verification

Release suites for search, engine, runner, console and desktop session pass (909 tests, 16 ignored); Clippy clean. Perft and native differential tests pass unchanged.

## Limits

One control (5 s + 0.1 s) and one machine. The table-size effect alone was not decided. The SEE attacker order still follows the old piece values (Horse before Cannon) although the fitted material now values the Cannon slightly lower; this is unchanged from the predecessor.
