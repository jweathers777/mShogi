# MSH-191 (Shogi Strength Anchor): method before play

Date: 2026-11-18. Owner: Claude Code, branch `codex/msh-191-shogi-anchor`.
Approval: John, 2026-11-17, recorded in the task (cloud, 8 workers, one game per
VM, one machine type; 6 h and US$15 caps for all match work; about 300 scored
games; about three families; no public servers).

## Candidate

mShogi at the branch commit that adds USI clock controls. Its search and
evaluation are identical to main `de5ae4ae` (MSH-188 integrated); only USI `go`
clock parsing and the runner's clocked game mode were added. Linux x86-64
release build, Rust 1.99.1, locked dependencies, `-C target-cpu=x86-64`.

## Anchors

Floodgate rating list `players-floodgate-20261118.html`, archived as
`floodgate-players-20261118.html.gz`. Ratings are fixed at that snapshot.

| Engine id | Floodgate player | Rating | Games (W/L) | Family | Build and settings |
| --- | --- | ---: | --- | --- | --- |
| gikou-d3 | gikou_depth3 | 2637 | 112/138 | gikou | Gikou v2.0.2 (`f5773abb`), DepthLimit 3 |
| gikou-d5 | gikou_depth5 | 2790 | 103/144 | gikou | Gikou v2.0.2, DepthLimit 5 |
| gikou-d7 | gikou_depth7 | 3130 | 107/135 | gikou | Gikou v2.0.2, DepthLimit 7 |
| elmo-1000k | elmo_WCSC27_479_1000k | 3131 | 908/1406 | yaneuraou-elmo | YaneuraOu v4.79 KPPT (`074f3eb2`, tag v4.79engine2017) + elmo WCSC27 evaluation, NodesLimit 1,000,000 |

Gikou settings: OwnBook false, Threads 1, FischerMargin 1000,
MinThinkingTime 10, other options default; data files from the official
v2.0.2 release package. YaneuraOu settings: Threads 1, Hash 256, NodesLimit
1000000, BookFile no_book, EnteringKingRule CSARule27, MaxMovesToDraw 320,
EvalDir to the elmo tables from the author's official `elmo_wcsc27` release.
mShogi has a fixed 16 MiB table; Gikou uses its default table. Hash is therefore
not equal across engines; this is recorded, not corrected.

Mapping assumptions: floodgate does not publish versions or options. The
`gikou_depthN` entries are assumed to be Gikou 2 with `DepthLimit N`; the elmo
entry is assumed to be YaneuraOu v4.79 with elmo WCSC27 at 1,000k nodes. The
floodgate entries may have used opening books; our games use none. No
reproducible open-source rated engine was found near 2950 from a third author:
`komadokun_depth*` has no known source, Bonanza N100 builds and YSS are closed,
and NNUE 1000k entries are above 3400. The study therefore has two families.

## Conditions

60 s + 1 s Fischer for both sides through the runner's clocked mode
(`mshogi-match shogi-match`): persistent processes, `go btime wtime binc winc`,
elapsed time charged by the controller from the written `go` to `bestmove`,
grace 500 ms. The rules profile `mshogi.shogi.csa36.v1` decides every move and result;
`bestmove win` is a game declaration; the 320-move rule and repetition are. Clock exhaustion, process failures and protocol failures are unscored
and reported; no result is invented. One game per c3-standard-4 VM in
us-central1, eight workers.

Openings: MSH-190 catalog `shogi-openings-v2.json` (315 openings), consumed in
file order, one fresh opening per pair with colours reversed.

## Model

`tools/strength/anchor_stats.py` unchanged (MSH-185; removed after the study,
see Git history): one logistic Elo
parameter, opponent ratings fixed, draws as half points, all scored games
including bracketing, pair-cluster sandwich interval with the conservative
routing envelope below 30 pairs. Prior Normal(2900, 500²), chosen from the
scout before play; sensitivity at SD 250 and 1000.

## Sequential rules

Bracketing: rounds of four pairs (eight games) against one opponent. Start with
gikou-d5, the admitted anchor nearest 2900. After each round refit and choose
the admitted anchor nearest the estimate. Stop bracketing when the interval
half-width is at most about 100 or after one hour of campaign time.

Main phase: rotate both families within about ±150 of the estimate, choosing
in each family the setting nearest the estimate; batches of 24 pairs split
evenly across families. Replace a setting whose running score leaves 25–75%
with the family's nearer setting and record the reason. Stop at the first of:
about 300 scored games; half-width at most 40 with at least 30 pairs and at
least 8 pairs per family; the 6 h or US$15 cap. If the campaign heads beyond
4.5 h or US$10, find and record the cause.

Report the joint estimate, its approximate 95% interval, each family's estimate
and any spread over 150 points, prior sensitivity, every round, opponent change
and failure. The number is a conditional floodgate-referenced index, not an
official floodgate rating.

## Addendum before the main phase (2026-11-18, after bracketing, before any main game)

Main-phase rule as applied: each batch alternates pairs between gikou-d5 and
elmo-1000k (the second family), so both families get equal pair counts. The
±150 window cannot be met for the second family; its games are retained and
fitted like all others, and the report states the expected low score against
it and the reduced information per game. The 25–75% replacement rule applies
to Gikou settings only (elmo has no other admitted setting). Stopping rules are
unchanged: about 300 scored games, or half-width ≤ 40 with at least 30 pairs
and at least 8 pairs per family, or the caps.
