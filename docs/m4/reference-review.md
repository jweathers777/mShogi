# MSH-64 independent fixture and reference review

Date: 2026-10-26. Reviewer: fresh Codex review agent, separate from the fixture,
harness and product authors. This is an independent agent review, not a human
review. Worktree: `next-task`, branch `codex/shogi-board`; final review base HEAD
`826cd2f14b9740985e3d263042e7299c20def041`. The reviewed reference changes were
uncommitted working files. Concurrent native state/legality work was outside
this review.

**Verdict: PASS for MSH-64's bounded independently expected board corpus and
pinned reference admission/harness. No material findings remain.** Actual
reference execution was required and completed before this verdict. Native
mShogi differential comparison is not run here and belongs to MSH-66. This
does not close MSH-41, certify historical reachability, complete the SH-F001–055
inventory, or establish match/history outcomes.

## Independent method and scope

Read `AGENTS.md`, the Backlog overview, MSH-64 and its acceptance criteria,
relevant design requirements, the approved `mshogi.shogi.csa36.v1` profile,
clause coverage ledger, planning fixture inventory and verification proposal.
Read every concrete record in `tests/fixtures/shogi/board-rules.json`, the
complete `tools/shogi/reference.py`, both new tool/evidence guides and all
retained admission/run/source manifests. Rechecked the primary JSA
[movement/capture/drop/promotion explanation](https://www.shogi.or.jp/knowledge/shogi/02.php),
[piece movements](https://www.shogi.or.jp/knowledge/shogi/03.php),
[check and mate](https://www.shogi.or.jp/knowledge/shogi/04.php), and
[prohibited play](https://www.shogi.or.jp/knowledge/shogi/05.php).

Independently reasoned the assertions from source rules, coordinates, occupancy,
ownership, promotion camps, hands and king safety before consulting their
reference observations. The six final rook-blocker/knight-jump/request-exclusion
records were reviewed before their first reference execution. The reference's
answers did not supply replacement expected answers. The reviewer changed only
this report; the author applied the discussed corrections and review metadata.

The final corpus has **101 concrete cases**. Its tags touch SH-F001–024, but
those tags do not certify every obligation within those planning rows. In
particular the full all-piece central/edge movement matrix, horse/dragon
movement matrix and bishop/lance blocker matrix remain broader MSH-66 coverage
work. Most records assert selected legal/forbidden choices, not a complete
independently enumerated move set. SH-F001 is the exact named 30-choice start
set. Sparse diagrams are structurally admitted analysis positions with one
king per owner and bounded material, not proven full-game reachable positions.

## Independently checked board facts

- The standard start's 30 choices follow its blocked placement: nine pawn,
  two lance, four silver, six gold, three king and six rook moves. Knights and
  bishop have no available destination.
- Side-relative movement and promotion assertions use sente's a–c and gote's
  g–i camps. Entry, movement within and exit permit both flags where the piece
  remains live; pawn/lance on the last rank and knight on the last two require
  promotion. The lance rank-three/rank-two nonpromotion and P/L/N/B/R check
  evasions remain complete choices. Moves wholly outside camp cannot promote;
  promoted P/L/N/S use gold movement and cannot promote again.
- Rook rays stop before the friendly pawn and on the capturable opposing
  silver. Knight jumps survive intervening own pieces. Mirrored cases reverse
  forward direction, ownership and camp without changing these facts.
- Capturing each of the six promoted kinds with a promoting rook leaves a
  dragon on the destination and exactly that captured base kind in the correct
  hand. Every listed drop creates an unpromoted owned piece and spends one
  hand piece; two identical rooks do not create two distinct USI choices.
  Empty final-rank pawn/lance and final-two-rank knight exclusions cover both
  sides. Occupied targets, absent resources, wrong-owner moves and same-square
  moves are absent. Those membership checks do not establish native atomic
  rejection or state preservation for invalid product requests.
- Nifu uses an own unpromoted pawn on that file. An own tokin and an opposing
  pawn do not trigger it. Both orientations' asserted alternatives are safe
  from unrelated king checks.
- In SH-F015, dropping P5b checks K5a; its four other adjacent destinations are
  occupied, G5c protects 5b, and no defending piece can capture that pawn.
  Removing L6a permits king escape; removing G5c permits king capture; replacing
  P4b by G4b permits a defending capture. Adding B3c pins that G4b to K5a, so
  its proposed capture exposes the king and no longer saves the pawn drop.
  Existing-pawn advance and gold-drop mates remain legal board actions. The
  rotated gote constructions preserve each escape/capture proof.
- SH-F018 permits a pinned gold to stay on the checking rook's file and
  excludes leaving it. Single-check cases separately admit king escape,
  checker capture with either promotion flag, interposition and a blocking
  drop; irrelevant actions remain absent. Double-check interpositions still
  leave the other line open. Capturing a bishop-defended rook with the king is
  forbidden.
- BOARD-PIN's origin x-ray is explicit: with K5h/R5a, moving K5h to 5i empties
  the origin and leaves the complete rook ray open, while 6i is off it. In the
  pinned-attacker contrast, S4c geometrically attacks 5d although moving that
  silver would expose its K4a to R4i; K5e may therefore not enter 5d under the
  selected geometric-influence contract. K5e–6e is safe. This is the profile's
  stated operational interpretation supported by independently checked
  geometry, not a claim that JSA explicitly names this rare configuration.
- The nested sequence captures a tokin into a base-pawn hand, shuffles the
  opposite king, spends the pawn on a drop, then moves the already promoted
  rook without demoting it. Both final SFEN strings and next-ply 6 were checked
  independently. No earlier history or outcome follows from the supplied
  counter.

## Reference provenance and harness assessment

The upstream checkout's HEAD is exactly
`6985fe5b516c34072e27c63cf429847f03193522` and its tracked index/worktree are
clean. All nine admission source/license hashes matched the actual checkout;
all ten rule-source archive hashes matched their source manifest. The root
license and inherited source-header distinction is accurately recorded. The
reference remains a separate optional developer process, with no cshogi
dependency added to Rust/product build manifests.

Read the pinned Cython/C++ chain independently: `Board.legal_moves` constructs
`LegalMoveList`, whose C++ wrapper constructs `MoveList<LegalAll>`. LegalAll
uses `Evasion<..., true>` in check or `NonEvasion` otherwise and then removes
king exposure and illegal pinned moves. Its complete-promotion distinction
from search-pruned `Legal` is visible in source. The harness derives all board
membership from this path, grammar-checks USI and pushes only enumerated integer
moves. Permissive parser/`is_legal` and `is_draw` calls occur solely in named
API-gap diagnostics and never determine expected legality or outcomes.

The successful isolated build uses Python 3.11.0, cshogi 1.1.0, Cython 3.2.4,
NumPy 2.3.5, setuptools 80.9.0, wheel 0.45.1 and Homebrew clang 19.1.7 on macOS
arm64. The retained verbose log actually compiles/links the sources and ends
with successful build/install. Its SHA-256 is
`55a82fa1fb8414e2e0ebb6abc5e455154ed668c4cc7774728e2c26f553c9799a`;
the loaded extension matches
`903a5af71398c687fda7ed3bdddcdc2595e2c28461dac43f67c7b896b82c37ad`.
The recorded commands contain no history-ban definitions and the source
`src/ifdef.hpp` definitions are under disabled branches. Earlier compiler/header/
linker failures are correctly distinguished from this successful admission.
The reviewer reran admission/runtime, rather than performing another complete
compiler bootstrap or claiming execution on another platform.

SFEN comparison retains owners, promotion, both hands, turn and next-ply. Exact
canonical text differences also report whether parsed semantics differ. USI
comparison preserves promotion/drop identity and compares unordered complete
sets. Reversal snapshots cover board, hands, turn, counter, check, history,
LegalAll set and the reference's own hash; no cross-engine hash equality is
assumed. Input checks reject duplicate case IDs, contradictory membership,
unknown source labels, malformed SFEN and material above physical bounds.

## Executed evidence and resolved findings

Independently executed from this worktree:

```sh
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --diagnostics-demo --report /private/tmp/msh64-independent-reference-review.json
```

Final result: **101 fixtures passed, 0 failed; 4 API-gap sentinels; 4 diagnostic
demonstrations; 0 native comparison sequences**. The review output is identical
to the retained `docs/m4/reference/reference-run.json`.

Also independently reran `--admit-source /private/tmp/msh62-cshogi-audit
--build-log /private/tmp/msh64-reference-build.log` with the recorded LLVM CXX
and temporary output manifest; it reproduced the admission manifest exactly.
A deliberately false extension hash was rejected with exit 1 and an explicit
binary-differs diagnostic. Independently invoking admission against the author's
temporary staged-dirty source checkout also exited 1 with the clean-pinned-source
diagnostic. Separate temporary subprocess checks exercised
nonzero exit, unexpected stderr, truncated JSON, duplicate/malformed choices,
timeout and a valid response; all produced their intended classifications.
`git diff --check` passed.

The runtime gap sentinels show why parser/`is_legal` acceptance cannot be a
negative-rule oracle: both accept a dead pawn drop and compulsory-promotion
omission that LegalAll excludes. The king cycle has two known root occurrences
after one cycle and four after three, but `is_draw()` already returns its
repetition category after the first. LegalAll continues to enumerate board
choices. No match result is inferred.

All four diagnostic subprocesses fail first at ply 2 of seed 20261026's retained
five-move path `2h1h 8b7b 2g2f 7b5b 1h5h`. They respectively remove `1g1f`, add
`P*5e`, exit 17, or emit truncated JSON. Each is detected and reduced to prefix
`2h1h 8b7b`, with root/history, original path, failing position and distinct
missing/extra/transport evidence retained. This is shortest failing **prefix**,
not globally minimum material or permission to erase history.

Review findings resolved by the author before final verification: a mirrored
silver-drop assertion appeared in both positive/negative lists; the initial
source guard missed staged changes; early-failure prefix counts overstated
executed comparisons; king-origin x-ray/pinned-attacker cases and independent
final-sequence SFEN checking were missing. The corrected source guard covers
both index/worktree changes, the fixture contradictions are rejected, counts
are accurate, and the new geometry/transition cases are reviewed and executed.
Empty-square dead-pawn drops were also added to isolate rank prohibition from
occupied-square prohibition. No expected rule answer was changed to fit the
reference.

## Final reviewed bytes and remaining boundary

| File | SHA-256 |
| --- | --- |
| `tests/fixtures/shogi/board-rules.json` | `579842bc0be0ab74567e1501100ef589be160c140c9371373768dc9fc43b6738` |
| `tools/shogi/reference.py` | `d48449d57cfaff3163f8fa7e493773cb9142b4c8237c82f49c175fbbe924bca2` |
| `docs/m4/reference/cshogi-admission.json` | `e38d6b6ae57e1a87256fb04638e97c691da932bbb670e5ed082beda79ed45427` |
| `docs/m4/reference/reference-run.json` | `4b2bb754889cd12dee6ec32ef9ddd4c1d367ec515b7d717cbd2c9aaa4e1bb69c` |
| `docs/m4/reference/rule-source-manifest.json` | `5b846cc9778eb3e014e64f11db817840b9077bc6fbef3673fe62979070b3a3a7` |

The accepted result is a usable complete-board-choice reference harness and
independently reviewed concrete subset, not native product conformance.
MSH-66 must supply actual native probe/differential evidence and remaining
movement coverage. MSH-67 still owns history/adjudication, declarations and
outcome-dependent fixtures; adapters, search, runner and application integration
retain their later owners. Task completion/integration remains the integrator's
responsibility.
