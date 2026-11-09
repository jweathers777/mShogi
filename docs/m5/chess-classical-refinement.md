# Continued conventional Chess engineering (MSH-137)

John's correction after MSH-136 keeps conventional Chess development active.
MSH-49/49.1/49.2/136 remain completed historical deliveries. Their completion
never establishes that the practical classical engine is finished. MSH-137 gates
Shogi/Xiangqi transfer and requires an independent assessment of remaining work.
No additional opponent match, statistical study or Elo estimate is part of this
assignment. Routine algorithm and representation choices remain autonomous.

**Verified checkpoint:** production refinement integrated at `f1484b15`, with the
regenerated fixture at `2a2ff7c00897fdcd10527c8db8cc77f5e1220e65`. All required
[actual native CI jobs](https://github.com/jweathers777/mShogi/actions/runs/42538719298)
passed at the latter exact source. Final independent evidence reconciliation approved all parent/subtask acceptance
criteria; MSH-137 and its three increments are complete. MSH-138 remains the next
product checkpoint before Shogi/Xiangqi transfer.

## Substantive corrections

- Native target-centred attacks and derived occupancy/ray maps replace repeated
  origin scans. The canonical array, checked native resolver/effects and precise
  errors remain authoritative. The original board-only attacker query supplies
  an independent algorithmic cross-check; FEN rebuilds check incremental caches.
- Legal queries retain checked resolution but avoid repeated structural scans
  and discarded Undo snapshots whose invariants resolution already preserves.
  Public Position apply retains the full structural validator. Game transitions
  preserve exact errors, history and atomic rejection; exhaustive bounded action
  comparisons cover 163,840 ordinary/promotion/error attempts.
- Lazy native turn summaries establish terminal/claim semantics before cutoffs
  with one legal witness. Claim-sensitive turns still enumerate the complete
  native intended-action catalog. Quiescence can request only native captures
  and promotions where quiet checks/evasions are not needed.
- Exact M2 future-context keys admit compatible transposed histories after
  irreversible boundaries. Main/q table entries have distinct depth namespaces,
  exact q budgets and bounded four-slot buckets with depth/age replacement.
  Current main entries are protected against q-entry flooding. Full adjudication
  retains conservative prefix/proof identity; synthetic probes have no bound key.
- A natively admitted TT move is tried before unrelated SEE/sorting work. Signed
  history rewards/maluses, bounded quiet q checks, independent guarded SEE/delta
  omissions and mate-distance windows preserve native claims/terminal precedence.
  An existing null probe's mating threat can spend one shared extension credit
  on real defences; its hypothetical score/outcome is never published.
- Classical evaluation corrects backward-pawn support direction, excludes rear
  doubled passers from independent runway bonuses, respects hostile promotion
  control, and adds original inspectable pawn-storm/safe-check pressure terms.
  These are untrained heuristics, not native rules or proofs of playing strength.
- Lossless34-byte repetition identities, direct capture-to-target generation and
  cached king lookup remove remaining hash/SEE scans. Basic bare-king KQ/KR
  conversion now rewards king approach rather than contradictory centrality.
- Opaque native prepared transitions avoid Game/evaluation push/pop for discarded
  children. Only checked native effects and exact context equality permit reuse;
  sensitive histories, Full mode and synthetic branches use ordinary admission.

## History and prepared-transition argument

A pawn move, capture or castling-rights loss makes earlier repetition identities
unreachable. M2 keys retain exact current Position, including raw EP and both
operational counters, plus all still-relevant occurrence multiplicities. Complete
canonical Game history stays intact. Fast fingerprints only reject candidates;
exact identity/count comparison guards collisions. Immutable root equality keeps
policy, imported origin and mode distinct. Within one real root, exact side and
fullmove number also determine played ply, preserving absolute-horizon meaning.

Preparation is deliberately narrower. With no identity repeated and a parent
clock below 98, the child can create only one second-occurrence identity. Its
intended next action has the opposite side to move from that identity, so it
cannot create a threefold claim. Child/intended clocks are at most 98/99, excluding
fifty/seventy-five-move and fivefold conclusions. Mate, stalemate and material
death still use the shared native outcome function. Opaque tokens carry checked
Position/turn data; stale/foreign tokens discard that data and re-admit the action.
Compatible histories may share effects while keeping their own canonical steps.

## Verification and bounded diagnostics

Independent expected attack/special-action/perft/history fixtures remain required.
New checks add all-square/color native walks and reflection, cache reconstruction,
full-public-transition comparisons, exact-context and forced digest collisions,
second-occurrence/intended-claim boundaries, corrupted foreign prepared payloads,
Full/null/undo/cancellation paths, table capacities 1/3/4/5 and q/main budget
isolation. Tactical coverage includes seven distinct immediate mating mechanisms,
three forced three-ply sacrifices with every defence explained, a forcing knight
fork, quiet-check horizons, native stalemate avoidance and precise feature terms.
These establish specific behavior; they are not a tactical-rating or strength study.

The historical queen grab is now a controlled tactical regression. After
...Qxa1 Qg5, a fixed White policy (Qg7/Qh6/Nxh2) traverses all native Black
defences: 255 positions establish mate within six further plies. The earlier
suspected ...gxf6 defence is illegal because it exposes the king to Qg5;
negative admission checks retain that fact. The engine now finds mate within
seven plies after ...Qxa1 at depth 4/200,000 visits and its PV replays to native
checkmate. In the isolated one-second original-root probe it instead chooses
...gxf6 (before Qg5, when legal), at -35cp with completed depth 7. This addresses
that concrete horizon failure without changing the accepted historical 49.2
report or claiming broad strength from a single position.

The final 20,000-visit profile uses three rounds of four positions on the local
M1 Pro. Search policy/evaluation changed, so these are descriptive throughput
observations, not equal-work ablations or strength measurements.

| Position | Starting median (ms) | Refined median (ms) |
| --- | ---: | ---: |
| Standard | 379.060 | 93.055 |
| Developed | 412.814 | 132.602 |
| Recapture | 162.056 | 43.436 |
| Endgame | 92.089 | 32.559 |

Unchanged-policy intermediate pairs retain identical recorded outputs/work and
show local timing differences. Exploratory packets have documented source-completeness
limits, so they do not all support complete causal reproduction. Final fixed-visit RSS is 4,194,304 bytes. A separate fixed-depth 4K/16K table
comparison gave similar work/time with RSS 3,981,312 versus 6,127,616 bytes; retain
the 4K default. One earlier final timing batch overlapped tests and is preserved
as contended, excluded from performance comparisons.

The single [verification record](evidence/msh137/verification.json) records exact
source/artifact hashes, commands, all profile producers including failures, actual
native checks and final review/platform receipts. Short mixed-position stack
samples are qualitative; inlining and partial coverage prevent exact attribution.
Fixed-node results after search-policy/evaluation changes are descriptive, while
paired unchanged-policy stages additionally compare recorded decisions/PVs/work.
No opponent matches ran. Previously accepted GUI/game-specific evidence is reused;
no packaged GUI campaign is required by these native-only changes.

## Research and remaining assessment

Techniques were researched and independently implemented from primary classical
references: [Stockfish 11 native geometry](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/position.cpp),
[search](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/search.cpp),
[move ordering](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/movepick.cpp),
[pawn evaluation](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/pawns.cpp),
[classical evaluation](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/evaluate.cpp)
and [TT policy](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/tt.cpp).
[Crafty's original command documentation](https://craftychess.com/hyatt/craftydoc.html)
describes extending real search after a null-move mating threat. No upstream code,
tables, trained weights or external engine binary became product dependencies.

Fresh independent whole-engine assessment supports practical conventional-engine
readiness, reconciled with the final platform receipts and integrated source. Remaining work is
broader quiet-threat patterns, further move-order/selectivity tuning, specialized
endgames and coefficient calibration; benefits now need marginal evidence or
larger engineering effort. This is not a claim of Elo, Stockfish parity or optimal
classical play. Queen-contact detection is deliberately incomplete; selective
search and bounded horizons remain approximate. No further strength study ran.

Ordinary desktop play still requests 20,000 visits and a 1,000ms hard search limit
(the process-supervision watchdog is separate). It receives lower latency and
algorithmic improvements but can exhaust visits before one second. The larger
visit allowance used in timed engine/UCI checks establishes a separate capability,
not maximum GUI strength. MSH-138 is the next bounded Chess product checkpoint before cross-game transfer.
It tracks proportionate desktop-default calibration
within existing admission ceilings, without another opponent study.

Transfer the measured query/state/cache discipline to Shogi then Xiangqi, not
Chess equivalence formulas: Shogi captures/drops can restore material, while
Xiangqi lateral pawn movement and chase/pending/no-win state require their own
native history proof. MSH-50/54 retain those gates and game-specific correctness.

The first native CI campaign at `f1484b15` exposed an expected-output omission:
all three workspace test suites passed, but macOS snapshot regeneration changed
only the Chess mate-analysis telemetry (29 to 65 visits, selective depth 2 to 7).
Mate-in-one, score and PV stayed unchanged. The complete generator confirmed
only that file differed; the updated fixture passed 34 relevant frontend tests
under Node 24.22.1. The superseded run, macOS failure and cancelled matrix remainder
are preserved. The corrected fixture passed the new exact-source CI campaign recorded below;
no required checks or production search behavior were changed to obtain a pass.

Final corrected-source native receipts (attempt 1, no retries):

| Platform | Job | Outcome |
| --- | --- | --- |
| Ubuntu24.04 | 128908684741 | Passed |
| macOS15 | 128908685074 | Passed |
| Windows2022 | 128908685190 | Passed |

The workspace suites passed 1,169 tests on Linux/macOS and 1,166 on Windows,
with 11 intentionally ignored on each platform. All later required subprocess,
desktop-session, fixture, build and smoke steps passed.
Earlier failed/cancelled campaigns remain historical evidence, not successful
required jobs. No production code changed after the frozen profile source.

Owner consolidation, 2026-11-09: MSH-138 (Desktop Search Limits) replaces the
ordinary20,000-visit default with time-primary play. MSH-139 (Chess Strength
Qualification) adds the independently grounded playing-strength gate before
MSH-50 (Conventional Shogi) and MSH-54 (Conventional Xiangqi). The completed
MSH-137 (Chess Search Refinement) evidence above remains an engineering checkpoint;
it does not itself qualify that new gate. See the current [roadmap](strength-roadmap.md).
