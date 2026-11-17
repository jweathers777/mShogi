# Shogi opponent proposal

MSH-186 (Shogi Strength Harness), 2026-11-17. This is a proposal for John's
future approval. No external engine was installed, built or run for this work.
No opponent games, rating games or cloud jobs were started. The evidence below
is a review of official source and documentation. It does not establish that an
engine works with our runner.

## Recommended pool

Use **YaneuraOu v9.40 with its material-only evaluation** and **Gikou v2.0.2** as
the first two candidate families. Admit each only after the checks below. This
pool can measure scores against fixed configurations. It cannot supply an
absolute Shogi rating. An older release is selected deliberately for a stable
source identity; the table does not claim that each release is current master.

| Candidate | Exact source | License and evaluation | Proposed role |
| --- | --- | --- | --- |
| YaneuraOu v9.40 | [`717da871e7a620702b8b9433bd8f9f181710435a`](https://github.com/yaneurao/YaneuraOu/tree/717da871e7a620702b8b9433bd8f9f181710435a) | [GPL v3](https://github.com/yaneurao/YaneuraOu/blob/717da871e7a620702b8b9433bd8f9f181710435a/LICENSE). Build `YANEURAOU_EDITION=YANEURAOU_ENGINE_MATERIAL`, `MATERIAL_LEVEL=1`; no external neural evaluation file. | First candidate. Strong search with a simple material evaluation; its strength at this setting is unknown. |
| Gikou v2.0.2 | [`f5773abb6b93c53508bca4d6d5705463e56ccb0f`](https://github.com/gikou-official/Gikou/tree/f5773abb6b93c53508bca4d6d5705463e56ccb0f) | [GPL v3 source](https://github.com/gikou-official/Gikou/blob/f5773abb6b93c53508bca4d6d5705463e56ccb0f/Copying.txt); source headers permit v3 or later. Classical learned evaluation tables, not NNUE. The release's required data files and their notices must be inventoried before use. | Second candidate; use a fixed reduced depth for the first baseline. |
| Fairy-Stockfish 14 | [`f3e6969d11d1bec17eba26e7ae0e629ad4af71dd`](https://github.com/fairy-stockfish/Fairy-Stockfish/tree/f3e6969d11d1bec17eba26e7ae0e629ad4af71dd) | [GPL v3](https://github.com/fairy-stockfish/Fairy-Stockfish/blob/f3e6969d11d1bec17eba26e7ae0e629ad4af71dd/Copying.txt). A large-board build is needed for 9×9 Shogi; use classical evaluation if investigated. | Reserve for move/protocol checks. Not proposed for the full-profile baseline until entering-king support is established. |

The YaneuraOu [build file](https://github.com/yaneurao/YaneuraOu/blob/717da871e7a620702b8b9433bd8f9f181710435a/source/Makefile)
explicitly supplies the material-only edition and level 1. This is a distinct
configuration from the NNUE engines used in tournament results. Those results
cannot be assigned to this build. A later NNUE candidate would need a separate
named network, source, license and hash; it is not part of this proposal.

Gikou's [README](https://github.com/gikou-official/Gikou/blob/f5773abb6b93c53508bca4d6d5705463e56ccb0f/README.md)
reports second place at WCSC 2016 and third place in 2017. It also gives rough
human-site levels for depth settings. These are historical claims, with other
conditions, not calibrated anchors for our machine, clocks or openings. Gikou's
learned classical tables are an external opponent dependency only. They do not
change mShogi's evaluation design. Its [release](https://github.com/gikou-official/Gikou/releases/tag/v2.0.2)
provides a Windows package; Linux operation needs a reproducible source build
and the required data, not an assumption that this package runs on Linux.

Fairy-Stockfish's [release](https://github.com/fairy-stockfish/Fairy-Stockfish/releases/tag/fairy_sf_14)
adds NNUE support for Shogi, but gives no usable Shogi rating for the proposed
classical build. Its UCI Elo/skill settings must not be treated as Shogi ratings.
Multiple settings of one engine are one family, not independent anchors.

## Rules and protocol fit

The sole game authority is [`mshogi.shogi.csa36.v1`](../m4/shogi-profile.md).
The important boundaries are:

- Four occurrences of board, hands and side to move cause repetition. The
  initial position counts. A continuous checker loses over the required
  first-to-fourth interval. The runner must send the complete known prefix.
- Entering king is an explicit declaration on the player's turn. The king must
  be in the enemy camp and not in check; at least ten other friendly pieces must
  be in that camp. Camp and hand points must reach 28 for sente or 27 for gote.
  King points are zero; rook/bishop points are five; other pieces count one,
  using their unpromoted types. Time must remain. An invalid declaration loses.
- No legal action loses, even without check. At 320 completed plies the native
  draw rule applies, after higher-priority native terminal conditions. Engine
  evaluation, predicted mate and search repetition do not settle a game.

| Candidate | Source evidence | Remaining admission question |
| --- | --- | --- |
| YaneuraOu | [`position.cpp`](https://github.com/yaneurao/YaneuraOu/blob/717da871e7a620702b8b9433bd8f9f181710435a/source/position.cpp) has repetition/continuous-check logic and the 27-point declaration path. [`types.cpp`](https://github.com/yaneurao/YaneuraOu/blob/717da871e7a620702b8b9433bd8f9f181710435a/source/types.cpp) names `CSARule27`; [`search options`](https://github.com/yaneurao/YaneuraOu/blob/717da871e7a620702b8b9433bd8f9f181710435a/source/engine/yaneuraou-engine/yaneuraou-search.cpp) exposes `EnteringKingRule` and `MaxMovesToDraw`. | Check the chosen material build's advertised options, `bestmove win`, full-history handling, finite work and shutdown. Search shortcuts are not proof of native fourfold semantics. |
| Gikou | [`position.cc`](https://github.com/gikou-official/Gikou/blob/f5773abb6b93c53508bca4d6d5705463e56ccb0f/src/position.cc) checks CSA 28/27 eligibility; [`thinking.cc`](https://github.com/gikou-official/Gikou/blob/f5773abb6b93c53508bca4d6d5705463e56ccb0f/src/thinking.cc) calls that check and emits `bestmove win`. [`node.cc`](https://github.com/gikou-official/Gikou/blob/f5773abb6b93c53508bca4d6d5705463e56ccb0f/src/node.cc) has repetition and continuous-check search logic. | Its search detects short cycles within a bounded history window. This is not a native fourfold adjudicator. Verify legal replies and explicit declaration at the protocol boundary. |
| Fairy-Stockfish | [`variant.cpp`](https://github.com/fairy-stockfish/Fairy-Stockfish/blob/f3e6969d11d1bec17eba26e7ae0e629ad4af71dd/src/variant.cpp) defines fourfold repetition, illegal perpetual check, pawn-drop mate restrictions and losing stalemate for the Shogi base. | No CSA 28/27 declaration or `bestmove win` path was found in the reviewed position/variant/USI/search files. This is a source-review limitation, not a proof about all builds. Do not silently add declarations for the opponent or change the profile to admit it. |

The Fairy-Stockfish check also covered those files at current master
[`014b0775c0e282ce104c76cf271abe31873eb9d7`](https://github.com/fairy-stockfish/Fairy-Stockfish/tree/014b0775c0e282ce104c76cf271abe31873eb9d7)
(commit dated 2026-11-15). That inspection did not resolve the declaration gap.
Do not substitute a moving master revision for the pinned proposal.

The existing [`experiment-external usi`](../../crates/mshogi-runner/src/shogi.rs)
path maps `bestmove win` and `bestmove resign` to native actions. It validates
advertised options and records/replays the native game. However, its current
configuration is **untimed, fresh process per turn**, requires finite depth
**and** nodes, allows at most four openings per config and stops after at most
32 actions. It is suitable for short compatibility checks. It cannot yet run the
full-game clocked baseline below. A 32-action stop is not a draw or a scored game.
The native predecessor clock mode added by MSH-186 (Shogi Strength Harness) does
not, by itself, remove these external-USI limits.

## Admission checks before a study

Use the existing adapter for a bounded set of at most 12 scripted requests per
candidate, with depth 4, 10,000 nodes and a 5-second response watchdog. Include
ordinary moves, drops, promotion, both colors, nonempty hands, a full repetition
prefix and eligible/ineligible declaration positions. Check the protocol with
fixtures that have independent native expected results. Include valid `win`,
invalid `win`, resignation and no-legal-action handling in adapter tests; an
engine is not required to make an invalid claim. Test stop/quit and process
cleanup. These checks do not score engine strength.

Record source/build revision, compiler/flags, executable and evaluation-file
SHA-256 hashes, licenses, all advertised/effective options, full requests and
replies, native replay, watchdog failures and cleanup. Required data files must
have explicit distribution/use provenance; a source GPL notice alone does not
establish the provenance of an arbitrary downloaded evaluation file. Do not
admit a candidate with missing assets or silently use a zero evaluation.

Before full games, extend and independently verify the existing USI route for
persistent per-game processes, full prefixes, clock/increment commands, native
declarations, 320-ply completion and bounded cleanup. Reuse the native runner's
clock and result machinery. Do not create a second game authority. USI transport work does not authorize a
new native clock-loss rule; such a change needs a separate explicit rules
decision. This is a
prerequisite for a later measurement task, not a claim that it was implemented
or tested by this document.

## Proposed first external baseline — not approved

The purpose is to find whether the frozen mShogi version is competitive with two
fixed opponent configurations and to detect a large mismatch. It is not an Elo
anchor study. Proposed conditions are:

| Item | Proposed fixed condition |
| --- | --- |
| Candidates | YaneuraOu v9.40 material level 1, unrestricted depth; Gikou v2.0.2 with `DepthLimit=4`. Keep the source revisions above. |
| Resources | One thread per player; 256 MiB hash per player; no pondering, own opening books or tablebases. Retain each player's process/hash for a game; reset between games. |
| Rules | Native `mshogi.shogi.csa36.v1`; YaneuraOu `EnteringKingRule=CSARule27`, `MaxMovesToDraw=320`. Gikou uses its CSA declaration path. |
| Clock | 60 seconds plus 0.6 seconds per move for both players. Gikou's depth cap is an explicit handicap, not equal search work. Clock exhaustion remains an unscored stop under the accepted native Shogi profile, even if protocol/process health is established. Stop the sample and report it; do not count it as a loss or draw. |
| Options | Freeze the full advertised/effective option manifest before play. Gikou `OwnBook=false`, `USI_Ponder=false`, `Threads=1`, `USI_Hash=256`, `FischerMargin=100`, `MinThinkingTime=10`, `DepthLimit=4`; record its unchanged resignation threshold. For YaneuraOu use `Threads=1`, `USI_Hash=256`, no book/ponder, and the stated rules options, subject to actual option validation. |
| Openings | Freeze a 16-pair schedule per opponent from the six legal prefixes in `shogi-openings-v1`: cycle through the file order twice, then use its first four entries. Alternate sente/gote within each pair. This is six distinct prefixes, not 16 independent openings. No opening selection from results. |
| Maximum sample | 16 pairs per candidate, 32 pairs / 64 games total. First four pairs per candidate are the scout and remain in the sample. No replacements beyond the game-attempt cap. |
| Decision | After the scout, stop a candidate if its mShogi score is at most 10% or at least 90%, or if a process/protocol defect appears. Otherwise complete its 16 pairs. Do not change depth, openings or opponent after seeing results within this sample. Report mismatch and propose a separate fixed setting if needed. |
| Compute | Existing Google Cloud Batch path only. One `c3-highcpu-4` machine type, at most eight simultaneous VMs, one game per VM; record actual CPU/region. No local long study. |
| Hard caps | At most 64 game attempts, 8 VM-hours, 90 minutes wall time including preparation/cleanup, and US$3 estimated compute. Stop at the first cap. Refresh the official regional price before approval; reduce the VM-hour cap if needed to stay under US$3. Reserve cleanup time and count failed allocations/startup in the resource evidence. |

Gikou's [USI option defaults](https://github.com/gikou-official/Gikou/blob/f5773abb6b93c53508bca4d6d5705463e56ccb0f/src/usi.cc)
include a 12-second Fischer margin and a 1-second minimum thinking time. Those
are unsuitable defaults for this fast proposal; the explicit values above are
part of the proposed condition and need clock-fixture verification. Reduced
strength is identified by exact settings, not a claimed human rank. Material
level 1 and depth 4 can still be badly mismatched with mShogi; the scout makes
that an explicit possible result.

Report W/D/L and score for each configuration and color, pair counts, descriptive
uncertainty grouped by opening prefix (including both colors and repeated uses),
all failed/unstarted attempts, wall time, actual VM-hours,
estimated cost and verified deletion of owned jobs/VMs/disks. Keep healthy native
results; do not turn a process failure into a loss or discard an unfavorable
result. Preserve failures separately. Repeated use of one prefix is not extra
independent opening coverage. With only six opening groups, treat uncertainty
as coarse; do not claim a narrow precision target from 32 pair labels. At this small sample, intervals can be wide and
an all-win/all-loss sample is not evidence of exact strength. Any uncertainty
statement must disclose the scout stopping rule; ordinary fixed-sample intervals
are descriptive after that stop. Do not reuse Chess CCRL anchors or the Chess
Elo estimator as an absolute Shogi scale.

John's later decision must approve the exact admitted configurations, protocol,
limits and spending. Approval of this harness task, or the standing native
predecessor-game allowance, does not approve this external baseline. If an asset,
compatibility check or resource limit fails, stop and report the specific gap;
do not substitute another engine or setting under the same result label.
