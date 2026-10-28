# Actual Pikafish playing-opponent admission (MSH-100)

This supplements, and preserves, the planning-only
[MSH-80 source audit](../comparator-audit.md) and MSH-91 board-harness evidence.
A network-free move probe is not a playing engine. Here unmodified upstream
Pikafish is independently built and run with a verified compatible NNUE. Its
admission is a bounded UCI0-9 **opponent**, not a WXF2018 outcome oracle.

## Reproducible source and build

The admitted source is official immutable release `Pikafish-2026-09-06`, commit
`4c17cee11f888ae1d48a9494f2e2239f019f0a1f`, tree
`43eb3e0166f6e8bda566aa8477d034288bb37e89`. The isolated checkout
`/private/tmp/msh100-pikafish-stable` was clean; no patch, downloaded binary or
mShogi implementation was used to build this opponent. Actual command:

```sh
git clone --no-hardlinks --shared /private/tmp/msh80-research/xiangqi/pikafish /private/tmp/msh100-pikafish-stable
git -C /private/tmp/msh100-pikafish-stable fetch https://github.com/official-pikafish/Pikafish tag Pikafish-2026-09-06
git -C /private/tmp/msh100-pikafish-stable checkout --detach Pikafish-2026-09-06
# Extract only the matching network/license from the pinned official archive.
cp /private/tmp/msh100-release-network/pikafish.nnue /private/tmp/msh100-pikafish-stable/src/pikafish.nnue
cd /private/tmp/msh100-pikafish-stable/src
make -j4 build ARCH=apple-silicon COMP=clang
```

macOS27 arm64, Apple clang21 (`clang-2100.3.34.2`); actual build flags, warnings
and successful link are retained in [stable build log](pikafish-stable-build.txt).
[Source manifest](opponent-source.json) pins twelve audited file hashes,
source/tree/clean state, compiler, binary, release archive and NNUE. Actual
binary800,616 bytes, SHA-256
`b768f26e7b7006eb4f7b1f595a3c865195dd0488f2a5b65971e9a1130a6b68b3`.

The matching network is member `pikafish.nnue` in
[official release archive](https://github.com/official-pikafish/Pikafish/releases/download/Pikafish-2026-09-06/Pikafish.2026-09-06.7z),
asset547038364. Archive53,603,424 bytes, SHA-256
`41952bbfe2520faceb5902c69e6ab4845cc999841d2b49a95cc1be7867a25e5b`.
Network50,706,378 bytes, SHA-256
`7d13d73569a9b571ba0eb20cf1596247bc2a42738967e61afef6482b231e900e`.
Actual engine search reports the loaded64MiB NNUE architecture
`(62083,1024,32,32,1)`, then real search telemetry and legal bestmove.

GPLv3-or-later engine notices remain upstream. The archive's separate
`NNUE-License.md`, SHA-256
`7f3320aa61ceefecec1d5cd158d6d1d2a8adde93728e83a79448df50903c367b`,
restricts weights to legal use and requires permission for commercial use.
This is local functional noncommercial verification, with no opponent/weights
redistribution or product dependency. Upstream README separately attributes
Pika Xiangqi Zero training data under ODbL; that does not replace the weight
license. Large source/build/network caches remain outside Git. This task does
not establish a distribution license for a future product dependency.

## Rechecked source contract and limitations

The older audit's October pin remains the MSH-91 board-reference identity. It
is not relabeled as this September playing-engine pin. Relevant source:

- [`src/uci.cpp:100–146,495–525,600–634`](https://github.com/official-pikafish/Pikafish/blob/4c17cee11f888ae1d48a9494f2e2239f019f0a1f/src/uci.cpp):
  actual UCI handshake/options, position/go/stop/quit/readiness/newgame,
  files a–i/ranks0–9 and matching complete native coordinate moves. No UCCI or
  selectable WXF2018 `RepetitionRule` exists at this pin. Invalid positions
  terminate explicitly; null output is not a native draw/resignation.
- [`src/position.cpp:147–270`](https://github.com/official-pikafish/Pikafish/blob/4c17cee11f888ae1d48a9494f2e2239f019f0a1f/src/position.cpp):
  canonical n/b board alphabet, w=Red, parser geometry/material/royalty domain,
  no-capture0..119 and fullmove0..100000 (upstream normalizes zero; mShogi never
  does). Runner accepts only strict positive native fullmoves1..100000 and
  native validated roots. Unknown counters/analysis provenance are rejected
  for live external conversion, not silently guessed.
- [`src/movegen.cpp`](https://github.com/official-pikafish/Pikafish/blob/4c17cee11f888ae1d48a9494f2e2239f019f0a1f/src/movegen.cpp):
  this pin exposes complete `LEGAL` enumeration with evasion/king-safety
  filtering, whereas the older audit pin uses `ALL`. Actual playing-build
  perft1 matches both independently expected Red/Black initial complete44
  choices from the unchanged MSH-91 fixtures. This scoped recheck does not
  re-admit all historical board fixtures under a changed comparator identity.
- [`src/position.cpp:1270 onward`](https://github.com/official-pikafish/Pikafish/blob/4c17cee11f888ae1d48a9494f2e2239f019f0a1f/src/position.cpp):
 120-ply rule60/check10, search-root-sensitive repetition/chase and material
  draw helpers remain different from selected native WXF claimant-specific100
  and third-warning/fourth-continuation semantics. Engine scores/outcomes are
  never imported as mShogi conclusions. Full known board prefix is sent, but
  this difference remains an opponent limitation, not proof of WXF compliance.

Fairy generic UCI1–10/UCCI0–9 remain unimplemented alternatives, as documented
in the source audit. Supporting another engine/dialect requires separately
admitted concrete evidence. The actual [upstream UCI documentation](https://github.com/official-pikafish/Pikafish/wiki/UCI-%26-Commands)
is supporting context; pinned code and runtime advertisements govern this pin.

## Actual admission and matches

[Admission JSON](admission.json) retains five real, separately bounded probes:
complete Red44 and Black44 independent initial action sets, actual standard
and FEN search replies followed by native accepted-command/event checks,
and explicit stop followed by a bestmove and successful native quit. Finite
admission stop probe depth8/nodes1,000,000 is an external-opponent diagnostic,
independently limited by readiness/stop watchdogs; it does not change mShogi's
measured ordinary depth1/128 default or depth3/512 ceilings. An actual search
reply need not solve P1: observed Pikafish depth1 chose `e7e1`, accepted
natively, whereas mShogi's independently verified P1 choice is `e7e8`.
No expected fixture was changed to match the opponent.

[Execution manifest](execution.json), configs and full reports retain the
actual source/binary/network/options/work/platform and every command, reply,
accepted context, event and process exit. Source producer
`8d6a0dd1851966ef1df3515155730f1abf4c74dd`:

| Actual experiment | Games | Native result / experiment stop |
| --- | ---: | --- |
| [Own self-play](selfplay.report.json) |4 |2 paired standard4-action cutoffs, unscored;2 Red P1 stalemate wins |
| [Independent Pikafish](opponent.report.json) |4 |3 explicit unscored4-action cutoffs;1 mShogi-as-Red P1 stalemate win |
| [Own finite-work repeat](selfplay-repeat.report.json) |4 |Identical full native games/actions/context digests/conclusions to self-play; OS times/PIDs retained separately |

All12 games completed their configured experiment successfully with no process
failures and clean captured exits;33 proposals were accepted,5 native stalemate
wins and7 unscored action-limit cutoffs. No competitive strength conclusion is
claimed. Recorded-game/event replay passes independently of engine artifacts.
Rerun equality is asserted only for the measured fixed single-thread/fresh-child
finite-work settings, never for wall-clock observations. A reviewer independently
reran own and external4game batches and compared native games, proposals,
context digests, position/go and conclusions with the retained reports.

[Actual no-work execution](no-work-execution.json) at final production source
`4d6481b` records two real own-engine zero-visit requests. Both emit no move or
score, stop as explicit unscored `incomplete_work`, preserve the original native
root and replay. Required exact analysis is not exposed by UCI; ordinary work
stays sound-fast-only, with no heuristic exact adjudication or fallback action.

Reproduce with the two verification-only Python scripts here, supplying already
built artifact paths and a new output directory. Neither is a product Python
runtime. `reproduce.py` runs strict native runner/replay; `admit-opponent.py`
retains real protocol probes, independent expectations and native reply admission.
Both fail explicitly on a failed verification result and refuse existing files.
Normal packages select production binaries; the named hostile protocol fixture
is separate test instrumentation.

## Preserved unsuccessful admission

The October26 source `9d7833aa...` initially built successfully, but its
actual playing probe failed to load October28's mutable `master-net` network
(SHA-256 `cd396741...`, metadata in [release snapshot](pikafish-network-release.json)).
Its NNUE architecture hash differs; the September archive network differs too.
Neither a successful handshake nor that build was called opponent admission.
[The exact failed match](exploratory/incompatible-network.report.json) retains
network errors, exit1,3 unscored failures and the accepted native prefix.
The fourth game ended natively before the failing engine was needed. All events
still replay. [The original build log](pikafish-build.txt) and initial self-play
report remain historical evidence. The source was not patched to accept
incompatible weights; the selected immutable release/source/network pair
resolved the reproducibility issue within the same admitted UCI0-9 contract.
