# Conventional Chess engine (MSH-49.1, refined by MSH-136/137)

The active native engine is `mshogi.chess.conventional.v1`, with evaluator
`mshogi.chess.conventional-tapered.v1`. UCI, supervised play and read-only analysis
use the same search coordinator. This implements the conventional architecture
stage of the [strength roadmap](strength-roadmap.md). MSH-49.2 supplies the
owner-authorized small practical diagnostic; it establishes no rating or
statistically validated competitive-strength claim.

## Components and contracts

| Component | Implementation |
| --- | --- |
| Native search state | `mshogi_core::chess::SearchCursor` owns an exact private Game. Lazy native turn metadata uses one legal witness where safe; claim-sensitive turns retain the complete native catalog. Derived occupancy/rays, target-only capture queries and checked prepared transitions avoid unrelated work. Every board transition uses the same checked native resolution. Cache, history, Full-mode proof, hash and evaluation state restore on pop/error/stop. |
| Exact diagnostic | `SearchMode::Exact` uses PVS, deterministic ordering and collision-checked TT bounds without quiescence, check extensions or selective pruning. Independent exhaustive native fixtures compare exact scores with table reuse enabled/disabled. Original full-width/material and intermediate candidate APIs remain available. |
| Main search | Iterative deepening, aspiration windows with widening, PVS re-search and TT ordering/bounds. Scores retain root-side perspective and mate-distance normalization. Only completed nominal iterations are published. |
| Ordering | Legal TT move, good exchanges/promotions, quiet killers, quiet/continuation history, then losing captures; capture history and lexical ties are deterministic. Signed history gravity and quiet maluses supplement ordering. An admitted TT move is tried before sorting/SEE for unrelated moves. A bounded least-valuable **legal** recapture sequence supplies an exchange estimate, including pins, EP and promotions. It is a heuristic, not exchange minimax or a legality oracle. |
| Quiescence | Native captures/promotions, bounded quiet checks and all legal check evasions, with separate guarded delta/SEE omissions. Confirmed supported Queen-contact threats suppress stand-pat and protect real defences; this is bounded coverage, not complete quiet-threat search. The ordinary tactical budget is16 additional plies; checked nodes continue evasions to the absolute128-board-ply ceiling. At that global ceiling a nonterminal state receives an approximate static horizon score, including in check; it never becomes a native outcome or fabricated evasion. |
| Selectivity | Verified null probes, late quiet reductions with unreduced/PVS re-search, shallow futility, history/killers-guarded late quiet pruning, SEE omissions and up to4 check extensions per path. Check, native terminal/claim children, low material and mate-band guards protect sensitive cases. |
| Classical evaluation | Inspectable tapered material, piece placement, pawn structure/passed pawns, activity/mobility, king shelter/attack pressure, threats and endgame conversion. Base material/placement/raw phase update incrementally; dynamic features use native geometric attack maps. Geometric mobility is not legal-move admission. No trained parameters, copied tables or model runtime. |

Native outcomes and claims precede static evaluation at every visited turn,
including quiescence. They are also checked before omitting a pushed child.
AutoClaim remains mandatory; optional claims compete at zero and win ties.
An intended claim action is never played. A static zero is not a concluded draw.
Only a native result at the actual request root can produce `Finished`.

The default TT retains4096 entries in buckets of at most four, with depth/age
replacement and protection of current main entries against q-entry flooding.
Explicit diagnostic options admit at most16384 entries; the short4K/16K test did
not justify a larger default. Main and q bounds have separate depth namespaces;
q bounds require the same remaining tactical budget. Every bound requires exact
native context equality after fingerprint filtering. M2 retains current Position
(including raw EP and both counters), immutable root policy/origin and occurrence
multiplicities since the last pawn move, capture or castling-rights loss. Those
boundaries make earlier Chess identities unreachable; complete Game history is
still preserved. Repetition identities use lossless34-byte storage, not hash-only
equality. Full adjudication retains complete prefix/proof provenance; synthetic
probes have no bound key. A table starts afresh per request. Board-only matches
supply ordering hints, never incompatible-history scores.

Prepared children reuse only native checked effects/turn metadata under exact
context equality and a narrowly proved claim-insensitive guard. Foreign/stale
tokens re-admit the action normally. See the [refinement argument and evidence](chess-classical-refinement.md).

Null probes use a detached native Position with the side changed and EP cleared.
The actual Game/history is untouched; synthetic nodes have no canonical action,
native Game conclusion, claim or TT key. Probes are disabled at root/PV/check,
claim opportunities, near the fifty-move threshold, Full adjudication and low
non-pawn material. Every proposed non-mating null cutoff is verified on the real
history. A synthetic counter-overflow abandons that hypothesis; real transition
errors and cancellation remain explicit. Null verification is selective search,
not proof of a native result.

ProbCut and singular extensions are excluded from this checkpoint. Short profiles
show most work in quiescence/exchanges and relatively few reusable deep bounds;
exact history-sensitive excluded-move verification remains expensive. Adding
calibrated probabilistic margins or excluded-move verification now would increase
cost and history-sensitive complexity without established benefit. Their omission
is explicit; the existing pruning package is not described as exact minimax.

## Time, depth and compatibility

Depth/nodes remain hard admitted work ceilings; node visits include discarded
iterations, quiescence, probes and verification. The bounded exchange estimator
polls stop/deadline separately and is not counted as a searched node. Native work
is cooperative, so one native operation and OS scheduling can extend wall time.
Timed Full-mode library search is explicitly refused: its native solver cannot
poll the engine deadline inside a transition. Untimed opt-in Full analysis still
has its separate configured native reachability limits.

Time allocation has a soft iteration boundary and hard per-node/exchange check.
Per-move allowances use70% soft time; UCI clocks allocate remaining-time/moves and
increment with a clock reserve and bounded extra iteration allowance. No first
completed iteration means honest native Incomplete; UCI alone retains its existing
explicitly labelled fallback requirement. A later incomplete iteration retains
only the permitted prior completed observation.

Live Chess uses depth ceiling128,20,000 visits and1,000ms hard time. User analysis
defaults/admission bounds remain unchanged. Optional wire `time_ms` is Chess/M2
only, bound into exact request identity. Optional `selective_depth` states the
deepest actually visited board ply, including incomplete later work; it can be0
for a root conclusion or claim. Nominal completed depth does not promise every
branch was searched to that depth. Every reported PV is natively re-admitted
within its explicit selective bound, with at most a final unplayed claim.

Missing optional fields serialize exactly as before and preserve historical
nominal PV bounds. The prior fixed play settings remain loadable without changing
the stopped document or old observations; explicit Resume adopts the new defaults
for future work. Current host/worker artifacts support the additive fields; older
strict binaries refuse unsupported fields rather than silently reinterpret them.
See [internal execution](../desktop/internal-execution-v4.md) and [UCI](../uci.md).

## Verification and handoff

Independent arithmetic, exact search oracles, legal PV replay, terminal/claim
precedence, hash collision/history separation, special-move/evaluation/cache undo,
null overflow/cancellation, low-material guards, re-search and native process/
archive tests cover the implementation. Short profile examples are diagnostics,
not strength measurements. Routine match work for the whole task is capped at
900 aggregate seconds; no formal study is authorized or run here.

The current [MSH-137 verification record](evidence/msh137/verification.json) pins
production source `f1484b15f44817e5469a849af28796d844393223`, local checks, profiles
and review. Required actual Ubuntu24.04/macOS15/Windows2022 native CI passed
at fixture-corrected `2a2ff7c00897fdcd10527c8db8cc77f5e1220e65`; production code
is unchanged from `f1484b15`. The refined native engine does not require a new packaged GUI
campaign; accepted unchanged GUI evidence remains applicable.

The historical [MSH-49.1 verification record](evidence/msh49-1/verification.json) holds that stage's exact source,
commands, failures, profiles, any development matches, independent review and
actual required Rust Ubuntu24/macOS15/Windows2022 receipts. Affected frontend and
standalone host checks accompany native verification. Existing packaged evidence
is retained; a new broad GUI campaign is not required for this engine boundary.

The original MSH-49.1 implementation was integrated at `eb4601d`; the fixture-generator completion
correction was integrated at `b8cc851`, with product code unchanged. Required
[native CI](https://github.com/jweathers777/mShogi/actions/runs/42492362033)
passed on Ubuntu 24.04, macOS 15 and Windows 2022 at that exact corrected source.
The earlier macOS snapshot failure remains in the verification record: command
admission preceded background history reconstruction, so the generator now waits
for the exact IO operation to finish before publishing its historical prefix.

In MSH-49.1, one timed self-play pair consumed49.6924 of that task's900-second allowance.
One game reached a native mate; the other reached the 192-ply cap and remains
unscored. Both histories replayed, without process or cleanup failures. These
are development regressions, not a playing-strength estimate. MSH-49.2 subsequently completed the owner-authorized8–12-game unrestricted
Stockfish diagnostic at one second per move within30 minutes. The diagnostic is now complete. Owner direction subsequently required MSH-136
and MSH-137 before cross-game transfer; the baseline alone did not establish
classical maturity. The current refinement record supersedes the original
immediate-transfer handoff while preserving all historical evidence.

Algorithm research used the primary [Stockfish classical search](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/search.cpp),
[ordering](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/movepick.cpp)
and [evaluation](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/evaluate.cpp)
as references. Implementations and arithmetic are original; no upstream source,
tables, fitted weights or engine binary becomes a product dependency.
