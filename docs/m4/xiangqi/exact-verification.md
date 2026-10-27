# Natural board-win exact verification (MSH-105)

Status: active research/implementation, not completed game delivery. Governing
profile and goal remain `mshogi.xiangqi.wxf2018.v1` and
`mshogi.xiangqi.natural-board-win.exists.v1`. Read the controlling
[owner decision](owner-decisions.md#g-nowin-resolution-during-msh-94-2026-10-27)
and [WXF source reconciliation](sources.md). No comparator outcome helpers are
proof or expected-data sources.

## Independently expected fixtures, frozen before implementation

A separate Codex reviewer (`/root/nowin_goal_review`) derived these facts from
native geometry before execution of the proposed exact verifier:

- **N0, bare Generals:** all108 admitted different-file palace placements/turns
  have a legal vertical exit, no check/capture and successors in the same class.
  This reproduces the existing94 proof without expanding its scope.
- **N1, one Advisor:** Red K + one Red Advisor versus bare Black K (and color
  reversal) cannot produce a board terminal. Neither piece crosses its palace
  to check the opponent. A bare General retains a vertical exit; if on the same
  file the Advisor screens it and remains between the Generals. For the owning
  side, an unpinned corner Advisor can move to e1 unless occupied by its General,
  which then has a vertical exit. A pinned upper-corner Advisor leaves a vertical
  General exit behind the screen. A pinned center Advisor leaves lateral General
  exits. Exhaustive static placement enumeration and successor closure must
  independently cross-check this argument before any fast-path expansion.
- **P1, three-piece stalemate:** Red K d0, R e7; Black K f9; Red to move.
  `e7e8` leaves Black unchecked but controls both e9 and f8, its only palace
  exits. Stalemate is a natural loss, unlike Chess.
- **P2, beyond the existing fast path:** FEN
  `5k3/R8/9/9/9/9/9/9/9/3K5 b - - 0 1`.
  `f9e9 a8f8` ends in Black stalemate: d9 faces Red K d0, e8 is controlled
  along rank8, f9 along file f. Initially f9e9 is Black's only action and its
  successor is ongoing, so no zero/one-ply witness exists.
- **P3, existing checkmate/stalemate roots:** retain the distinct independent
  terminal reasons in `tests/xiangqi_game_boundary.rs`; zero-length witnesses
  are valid only for an already locally terminal board.
- **U1:** the standard starting position with zero or deliberately tiny exact
  work is Unknown, not proof of either possible win or draw.
- **L1:** exhausting any configured work/storage/deadline/witness limit returns
  Unknown. An invalid local root or transition invariant failure is Failure.
- **C1:** cancellation before or during graph work is Unknown; live native
  position, commands/events, runtime context and save record are unchanged.
- **H1:** F002's actual same-position histories have different warning/penalty
  outcomes; none is a Black natural-win witness. Historical inspection and undo
  retire runtime work even when board equality returns. Retain actual history
  in request provenance, separately from the proof's state identity.

These are reasoning expectations, not implementation observations or a claim
that exhaustive graph closure has already run. Exact measurements, domain review,
implementation contract and final verification will be recorded below.

## Independently reviewed exact domain

The reviewer accepted this domain before implementation. A state is the complete
90-intersection native `(owner, kind)` placement and side to move. Edges are all
ordinary locally legal actions, with captures and General safety from the native
MSH-93 API. A vertex with no legal actions is a goal: opponent wins, with checked
mate and unchecked stalemate kept distinct. Stop exploring a goal. Both players
cooperate existentially; one natural terminal suffices to disprove neither-can-win.
A result names that side only; it does not prove anything about the other side.

This is the owner's intrinsic natural-course question, not the graph of match
commands. History penalties, optional/automatic claims, agreement, resignation,
capacity refusals and other non-board conclusions are neither edges nor terminal
barriers in this intrinsic graph. Otherwise an opponent's penalty conduct or
chosen match policy could change intrinsic mating/stalemating capability. The
controlling owner interpretation and `Position::legal_actions` make this quotient
sound: no board movement or board-terminal condition reads prior occurrences,
persistent IDs, warning state, claim policy or declared counters. The working
position uses unknown counters so representational overflow cannot delete an
edge. Every transition still uses native legal resolution, not an unchecked
parallel rules engine.

This does **not** discard actual history. Concluded/pending `Game` command
admission remains authoritative; historical views retain their exact event
boundary and complete source record. The F002 game can award Black a penalty win
while the intrinsic computation has no Black natural witness. The new test
executes this difference and rejects a result bound to the earlier equal board.
A history-sensitive live-game transposition cache is not this proof cache.

## Proof and algorithm

`Position::verify_natural_win` performs forward reachability with a bounded
open-addressed full-key set. Keys use46 bytes (90 native nibbles plus turn),
not FEN/declared counters, a lossy Zobrist key or a universal board encoding.
Hash collisions compare complete keys. Parent indices retain complete witness
paths. Breadth-first and depth-first orders use identical native edges and proof
obligations. Only complete exhaustion without a goal returns `NeitherCanWin`.
Unknown frontiers are never negative proofs. Every observed positive in the
measurement probe is replayed from the target through native checked transitions
and its terminal side/reason checked before output.

For a single-root question, reachable closure establishes the negative directly;
SCC construction adds no stronger negative fact and is unnecessary to terminate
cycles once full states are memoized. Retrograde/material-universe tables can
amortize many roots, but require a complete independent universe, capture-class
closure and new indexing/provenance obligations. They are not needed for these
measured single-root results. Compact native keys and two exploration orders
were investigated rather than inferring infeasibility from one naive search.
A sparse perfect ranking/tablebase or analytical closed-class proof may improve
larger negative cases; this batch does not establish their general cost or
reject their feasibility. No product/profile weakening is recommended.

The independent test model uses only palace coordinates, adjacency, Advisor
diagonals, occupancy and facing-file screening; it calls no product legal,
attack or transition helper to construct its expectations. It enumerates all108
bare-General states and510 states for each Advisor color, checks nonempty
expected choices and closed successors, and compares every action and resulting
board against native rules. Native BFS/DFS graph closure from representative
roots agrees with the independently traversed reachable components, including
exact state/edge/cache-hit counts. The reachable components (27 and255 states)
are smaller than the full static universes; neither is mislabeled exhaustive
coverage of all Xiangqi material. This is separate evidence for narrow negative
proofs, not a proof that every larger position is feasible.

## Native assessment and execution ownership

`ExactAnalysis` is the native read-only assessment adapter. `begin` binds the
full `GameContext`, canonical source record, exact live/historical target,
`NaturalBoardWin` purpose, configured limits, fresh analysis session and
monotonic request/execution generation. `ExactRequest::run` is separately bounded
cooperative native work. `accept` checks complete request equality and current
GameContext and consumes the result once. Selection, replacement and cancellation
retire work. Semantic undo increments Game's runtime revision; reopen and a new
analysis resource allocate fresh identities. An equal board cannot resurrect a
result. There is no subprocess here and therefore no invented engine-process
identifier; later process consumers must retain their own process generation.

`ExactReport::assessment` maps actual proofs, incomplete exact work and failure
to the existing native assessment vocabulary; a fast miss retains its existing
wire spelling `UnknownExactUnavailable`, while a requested incomplete exact run
is `UnknownExactIncomplete` with its stop reason. Failure remains separate.

Acceptance **does not** mutate Game, replace its recorded fast assessment,
resolve repetition-classification pending state or emit a conclusion/event.
This explicit advanced operation supplies the exact native assessment boundary;
MSH-97 and later native consumers own its real execution/adjudication use. They
must not map Unknown to ongoing/winnable or draw: if an experiment requires this
result for adjudication, it remains unresolved and unscored. A read-only intrinsic
proof cannot overwrite a real repetition/claim conclusion. This is not a claim
of complete WXF live-product adjudication or Xiangqi engine/GUI delivery.

Existing native records remain strict `sound_fast_only`, preserving all prior
mode/limit/history and chess/shogi compatibility. Exact execution is not a
restorable game mode or pending saved work; unsupported `bounded_exact` record
restoration still rejects explicitly before any exact replay. No canonical
history or observation is silently discarded. The complete current session
remains saveable because analysis never commits changes. Future persistence of
these historical observations belongs to the existing consumer/archive owners.
The source record is bounded to1MiB/512commands/2048events; historical targets
come from bounded native snapshots, not an expensive replay on this path. Exact
proof is never implicitly run by canonical decoding or ordinary live commands.

## Bounds, cancellation and practical policy

Defaults:20,000 states,200,000 edges,250ms,4MiB verifier capacity,4096 returned
plies, depth-first traversal. Configurable maxima:1,000,000 states,20,000,000
edges,60seconds,128MiB capacity and4096 witness plies. Unsupported ceilings are
explicit Failure, not a chess result. Configured zero/tiny limits produce
Unknown. Memory accounts for actual vector capacities of nodes, index table,
frontier and witness plus32KiB conservative bounded scratch (board/action
vectors/locals); allocator metadata and total process RSS are reported separately.
The full requested capacity is reserved before traversal. A memory budget too
small for that request returns Unknown even if the position would be easy.

Cancellation/deadline checks surround native board operations and occur at every
edge, hash probe and witness step. Each uninterruptible native operation is
bounded by90 intersections and32 pieces. Cancellation cannot change the source
position, game/history, assessment or save record. No worker/queue/child process
is introduced; no process cleanup is inferred. Fixed maximum arrays, bounded
native context and capped witnesses keep communication/diagnostics finite.

Keep the existing immediate bare-General and zero/one-ply fast recognizer; no
new material shortcut is inferred from these observations. Measured small palace
and one-Cannon roots are practical bounded interactive *native* requests on this
machine. Larger requests remain explicit advanced work with cancellation, not
blocking automatic reconstruction. Depth-first is the measured default because
it finds a standard-position witness within20k; breadth-first is selectable for
shorter witnesses and did much better on the two-Cannon sample. Neither order
uniformly dominates. Later hosts may try the alternate order under a separately
recorded total work budget; they must not conceal extra work or promise success.

Unattended adjudication needing a proof must pause/record unresolved unscored
work on Unknown, preserve the complete session and allow an explicit bounded
retry/alternate strategy. It must not award a result or retry unchanged limits
as if success were guaranteed. This safe policy preserves the selected semantic
obligation; it does not establish universal ordinary-play WXF completeness.
Remaining Unknown workloads include the four-defenders-per-side sample under
both20k/100k requests, BFS standard position under those budgets, DFS two-Cannon
at20k, and any target exhausting a limit/cancellation/witness bound. They are
not declared live, dead or intrinsically infeasible. Universal scaling, tablebase
coverage and playing strength are unmeasured.

## Actual feasibility observations

Release builds, stable Rust1.89.0 aarch64-apple-darwin, Darwin27.0.0 arm64,
Apple M1 Pro,32GiB RAM. Retained raw observations, exact FENs/configurations,
work, elapsed times, cache probes/hits and all positive witness actions:
[20k runs](exact-evidence/measurements-20k.jsonl) and
[100k runs](exact-evidence/measurements-100k.jsonl). There are31 observations
per run:15 roots ×2 orders plus10ms cancellation. These are actual timed
observations, not promises of identical timed reruns. Commands:

```
cargo run --release --locked -p mshogi-core --example xiangqi_exact_probe -- 20000 1000
/usr/bin/time -l target/release/examples/xiangqi_exact_probe 100000 1000
```

Selected20k results (complete raw records retain every measured field):

| Root / order | Verdict | States / expanded | Edges / cache hits | Max choices | Elapsed ms |
| --- | --- | --- | --- | --- | --- |
| Bare Generals / BFS | negative closure |27/27 |54/28 |3 |0.10 |
| One Advisor / BFS | negative closure |255/255 |674/420 |6 |0.65 |
| Two Advisors / BFS | negative closure |474/474 |1372/899 |5 |1.35 |
| One Elephant / BFS | negative closure |378/378 |1167/790 |6 |1.02 |
| Two Elephants / BFS | negative closure |1134/1134 |4338/3205 |8 |3.87 |
| Advisor+Elephant / BFS | negative closure |1779/1779 |6610/4832 |10 |6.27 |
| One Cannon / BFS | negative closure |9924/9924 |95090/85167 |20 |60.04 |
| Two Cannons / BFS |5-ply stalemate witness |11062/2762 |48541/37480 |36 |28.84 |
| Two Cannons / DFS |Unknown(states) |20000/10935 |111302/91302 |37 |71.85 |
| Four defenders each / BFS |Unknown(states) |20000/7061 |54726/34726 |10 |49.56 |
| Standard / BFS |Unknown(states) |20000/532 |22471/2471 |53 |20.67 |
| Standard / DFS |513-ply stalemate witness |14068/519 |15111/1044 |59 |11.56 |

Each20k request reserved1,423,104 capacity bytes. At100k this becomes6,689,536;
BFS standard remains Unknown after182,099 edges/100,000 states in275.86ms,
whereas DFS finds the same513-ply witness in17.17ms. Four-defenders each remains
Unknown at100k in385.73ms(BFS)/196.05ms(DFS). Two-Cannon DFS now finds a
2871-ply checkmate witness after37,463 states/142,347 edges in118.13ms.
The extra-piece sensitivity and order differences are concrete; toy closure does
not establish general feasibility. One-Cannon cache hits are about90% of edges,
showing memoization avoids repeated cyclic work.

The100k probe process measured1.25s real,1.06s user and18,972,672 bytes maximum
RSS (18,268,568 peak memory footprint). Its final cancellation experiment requests
1M capacity (64,429,568 accounted bytes, lazily committed) and returns Unknown
at10.00ms after6,329 states; process RSS is distinct from reserved capacity.
An initial sandboxed hardware/RSS query was denied by the OS; the authorized
read-only measurement was rerun successfully. No other operating system's
performance is inferred. Final source revision, focused/workspace/CI outcomes
and independent implementation review are recorded in the task and evidence
review below.
