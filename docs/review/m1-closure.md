# M1 closure review (MSH-14)

Milestone: **M1 - chess rules kernel and verification console** ([design §4](../design.md)).

Reviewed revision: `codex/rewrite` at `35a4d28`. Its product code is identical to `4f4025c`, the last behavior change (MSH-13), whose native run [40126682337](https://github.com/jweathers777/mShogi/actions/runs/40126682337) passed on all three systems. The closure commit changes only documents and records; its own native CI run is recorded in the MSH-14 task. Every task record under `backlog/tasks/` holds commands, environments, revisions and outcomes.

M1 delivers a **position-level chess rules kernel**. It is not complete chess adjudication, not a playing engine, not a GUI, and it implements no other game.

## Exit criteria and evidence

| # | Exit criterion (abridged) | Evidence | Status |
| --- | --- | --- | --- |
| 1 | Promised move categories and classifications have independent expected fixtures | The corpus (MSH-2: 35 positions, 33 FEN contracts), plus these families: ordinary transitions (MSH-5: 49 sequences / 74 after-states), attacks (MSH-6: 29 / 155), ordinary candidates (MSH-7: 17 / 274), special actions (MSH-8: 21 positions, 6 sequences, 18 rejections), classification (MSH-9: 8), divide (MSH-10: 14). Both colors of castling, en passant and every promotion choice are covered, along with check, checkmate and stalemate. Independence differs by family:<br>• Corpus legal sets and perft, transition after-FENs and divides: diagram and rule rationale curated by hand, exact answers enumerated by pinned python-chess 1.11.2.<br>• Attacks, candidates, special actions and classification: hand expectations written first, then confirmed by the oracle.<br>Every family has a verify-only checker, and no answer comes from mShogi | Met |
| 2 | Legal move sets and perft match the corpus at documented depths | `legal_actions` suite (MSH-9): exact legal sets and classification for all 35 corpus positions. `perft` suite (MSH-10): 38 corpus perft totals (depths 0–3), 11 ordered divides, and 3 depth-4 divides under `--ignored`. Three Stockfish sf_17.1 depth-5 totals reproduced (local macOS) | Met |
| 3 | Apply/undo restores the full position, counters, rights and derived caches | Full-snapshot `Undo`; reversal at every step in the ordinary, special and sequence suites; counter-overflow atomicity matrix (MSH-9). No derived caches exist yet, and that is documented | Met |
| 4 | Fixed-seed legal sequences exercise restoration and round trips; failures preserved as compact regressions | `sequences` suite (MSH-12): 6,720 plies in CI and 316,880 plies deep, from 92 start positions; the `sequence-regressions.v1` replay family. The differential tool saves and replays failures, shown by fault injection | Met |
| 5 | Console inspects, lists legal moves, applies/undoes sequences, runs perft/divide with documented output and errors | `mshogi shell` (MSH-11): 12 process tests (11 on Windows, where one Unix byte-argument case does not apply), a README example reproduced byte for byte, and a CI shell smoke on all three platforms since `8670c73` | Met |
| 6 | Action-boundary review traces a synthetic compound-action exercise to the real interfaces | [Action-boundary review](m1-action-boundary.md) and the `action_boundary` suite (MSH-13). No A-02/A-03 conflict | Met |
| 7 | Actual Windows, macOS and Linux build/test/smoke evidence | **Primary:** run [40126682337](https://github.com/jweathers777/mShogi/actions/runs/40126682337) on `4f4025c`, identical product code to the reviewed revision. It passed on `ubuntu-24.04`, `macos-15` and `windows-2022` (rustc 1.89.0): fmt, clippy, all tests, build, and console smoke of help, version, profile and a piped shell session (perft 3 8902). The closure commit's run is recorded in MSH-14.<br>**History:** each behavior task's own run passed on all three systems: MSH-4 39969119678, MSH-5 39974586573, MSH-6 39992460675, MSH-7 39994336350, MSH-8 39996442602, MSH-9 40003189442, MSH-10 40009700442, MSH-11 40012369679, MSH-12 40126126848. Runs before MSH-11 smoke-tested help, version and profile only | Met for build/test/smoke. The deep, differential, baseline and fresh-clone runs are macOS-only and local; they are not claimed for other systems |
| 8 | Independent review: no unresolved correctness issue; evidence reproducible | A fresh delegated reviewer for each behavior task (MSH-4 to MSH-13), with findings resolved and recorded. A milestone reviewer for MSH-14 (see the task record). Fresh-clone reproduction of the documented workflow, recorded in MSH-14 | Met (see MSH-14 record) |
| 9 | Coverage ledger separates implemented, deferred and unresolved requirements; M1 is not labeled complete chess or strong play | [Coverage ledger](../rules/chess-coverage.md). README, console profile line and API docs state position-level scope and no adjudication | Met |

## Owner decisions during M1

- **2026-10-23 (MSH-9):** chess legality and the M1 classifier are independent of counter representability. `CounterOverflow` is a representation failure, not illegality. Perft and divide never skip legal branches or treat overflow as a leaf (MSH-10 AC #6).

## Deferred obligations (not M1 failures)

- Dead position (FIDE 5.2.2).
- Repetition and other history-dependent rules, claims, the automatic fivefold/75-move rules, resignation, agreed draws, time and clocks.
- A rules-owned history layer (action-boundary review, required change 6).
- Search, evaluation and strength.
- Learned evaluation and neural infrastructure (M5).
- Protocol adapters.
- The desktop application.
- The other five promised games.

All are recorded in the ledger and roadmap and remain requirements.

## Known limits carried forward

- Diagnostic performance baseline only (about 3.2M perft nodes/s, Apple M1 Pro, release build). Full-snapshot undo and checked filtering; no optimisation claim.
- Structural rejection reasons are matched by message text in one test, because `FenError` has no structured reason enum.
- Console nits recorded in MSH-11: raw input echo, Unicode whitespace and `+` accepted, and unbounded line length and depth.
- The python-chess oracle (GPL) is optional, run out of process, and never linked or shipped. Agreement with one oracle is bounded evidence.
