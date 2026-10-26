# Native shogi evaluation, search and execution (MSH-70)

This is a bounded functional baseline for the approved
`mshogi.shogi.csa36.v1` native game. It is not a competitive-strength result,
learned model, tsume solver, USI adapter or desktop shogi delivery. Native rules,
history, conclusions and records remain authoritative. Chess search/v2 requests,
saved observations and Full reachability are unchanged.

## Evaluator and score conventions

`mshogi_search::shogi::evaluate` is material only, identity
`mshogi.shogi.material.v1`. Units are hundredths of an unpromoted pawn. Values are
fixed untrained baseline parameters, not measured exchange values:

| Kind | Board, unpromoted | Board, promoted | Hand |
| --- | ---: | ---: | ---: |
| King | 0 | not applicable | forbidden |
| Rook | 800 | 1,000 (dragon) | 800 |
| Bishop | 700 | 900 (horse) | 700 |
| Gold | 500 | not applicable | 500 |
| Silver | 400 | 500 | 400 |
| Knight | 300 | 500 | 300 |
| Lance | 300 | 500 | 300 |
| Pawn | 100 | 500 (tokin) | 100 |

Both hands and native promoted identities contribute. `Evaluation` retains
Sente/Gote material totals; `score_for(side)` is that side's total minus its
opponent's, independent of current turn. Capturing an enemy promoted pawn removes
500 enemy board units but adds only100 own hand units. Dropping preserves raw
material. Kings contribute zero because captures are illegal and native outcomes
precede heuristic leaves. No positional, tempo or training term is present.

These units have no relationship to entering-king qualification points28/27 or
CSA competition half-points. A zero evaluation does not conclude a draw.
Static evaluation does not adjudicate a diagram, infer earlier history or prove
historical reachability.

Search identity is `mshogi.shogi.full-width.v1`. Scores always use the **captured
native root side-to-move perspective**, also emitted in telemetry. Actual native
wins score `1,000,000 - distance`, losses `-1,000,000 + distance`, draws0.
Distance counts selected **command edges from this search root**, including a
formal declaration; it is not the imported SFEN counter, board ply or a promise
of checkmate. A terminal win may be no-action loss, entering king, resignation,
false declaration or continuous-check loss. A root already concluded has
distance0. Heuristic material values remain well below terminal scores.

## Search and native authority

The baseline is iterative-deepening, deterministic full-width minimax, with no
pruning, transposition table, board-only cache or opening book. It traverses the
native complete legal set, retaining every move/drop/promotion/nonpromotion.
Each board transition is `Game::submit` with the complete native context; exact
`Game::undo` unwinds it. The caller is immutable and each search has private
working state. After every completed, limited, stopped or failed iteration the
private complete root is checked for exact equality. Native invariant errors
remain diagnostic failures and never establish a game outcome.

Native conclusions are consulted before declarations or static leaves. Winner
and root side determine the score; the algorithm never assumes that the final
mover won. Full known occurrence/check history, imported-root provenance and
declared length semantics remain in every working `Game`. A fresh identical
SFEN can therefore choose a different move from the recorded history.

At every live node, including a depth-zero board leaf, search asks native
entering-king eligibility for the actual current side. An eligible formal
declaration is submitted privately, its actual native terminal conclusion is
scored, and it is undone. It is an explicit `Declare { actor }` choice, not a
board action or already concluded host game. Board turn and counter do not
advance; minimax uses stable root perspective rather than alternating score
negation. An immediate valid declaration is an optimal shortest terminal win
and takes priority over an equal board choice. Otherwise ties retain the native
USI lexical order. Search never proposes an ineligible declaration, resignation,
evaluation-based draw or evaluation-based resignation.

## Exact work and interruption policy

Native `Limits` are mandatory and deterministic: depth1–4, total nodes0–20,000.
The measured default is depth2 with5,000 nodes. There is no unbounded native
shogi search operation. These bounds also apply to v3 engine-turn and ordinary
move-analysis requests; chess limits are unchanged. `DEFAULT_LIMITS`, `MAX_DEPTH`
and `MAX_NODES` are exported from `mshogi_search::shogi`.

A node is an admitted recursive visit, including root, terminal, static and
declaration nodes. Revisited nodes in later iterations and visited nodes in a
discarded partial iteration count toward the same total. Checks occur before a
node visit and before native transitions. Stop/limit checks that admit no visit,
native move enumeration, record parsing/replay and serialization are not counted
as search nodes. One individual native query remains bounded by the native board
and history limits. A declaration node consumes one extra visit at a leaf; PV
length can be completed board depth+1.

Only a fully completed iteration supplies a decision, score and PV. A partial
later iteration updates total work/status while preserving the last completed
iteration. With no complete iteration, `decision`/`score` are absent, PV empty and
completed depth0. There is no fallback move or fabricated score. Zero nodes
visits nothing, including an already concluded root. A visited concluded root
finishes at depth1 with one node. An immediate declaration requires two nodes.
The initial standard position completes depth1 at31 visits, depth2 at962 total
iterative visits. A32-node depth2 request retains only the complete depth1 result.

`search_with_stop` polls a cheap nonblocking callback. Cancelled/limited traversal
unwinds private transitions. Native transition/invariant failures discard the
search result and return the original diagnostic; no partial score is published.
The Rust error retains native diagnostic data. The wire failure retains the
existing bounded diagnostic and full submitted root context, without invented
search telemetry. Public legal inputs cannot deliberately produce the accepted
mutual-check impossibility invariant; descendant-error tests use a clearly
test-only injection seam, not a newly claimed legal mutual-check position.

The same `EngineBridge` supervises v2/v3 and both games. Cancellation/replacement
retires the entire request, including any completed result not yet accepted.
The one-shot child reports one final response; a killed/timed-out child has no
received partial iteration to recover. Watchdog failure is an unscored process
failure, distinct from a native node-limited result. The existing default15s
watchdog/250ms cleanup remain unchanged and may be configured by the host.
These wall-time bounds do not promise identical timed reruns. Record settings
and actual observations; deterministic equality is for equal finite native
depth/node requests under unchanged identities/configuration.

## Structured v3 operation and acceptance

`execution_capabilities(Shogi)` now advertises exactly `EngineTurn` and
`MoveAnalysis`. `SharedResult::Shogi` contains `ShogiResult::Choice`, `Finished`
or `Incomplete`; `SharedResult::Failure` remains unscored. Complete native board
actions serialize through the existing strict USI primitive. Declarations carry
their native actor. Native conclusions serialize with the same strict forms
already used by canonical records, including check intervals and no-action
check status. The canonical record wire format is unchanged.

`ShogiTelemetry` includes evaluator/search IDs, native root perspective, score,
completed depth, all admitted visits, PV and actual stop reason. No invented live
progress stream is present. Result families, score bounds, settings, identities,
root side, terminal record conclusion and observation consistency are checked
before publication. Actual host choice admission still belongs to native
`Game::submit`; transport acceptance is not another legality engine.

Ordinary analysis is read-only: the submitted native record is a private target,
the response keeps `move_analysis` purpose, and no host command is applied. A
target includes the exact complete event prefix and known board-action count,
not just SFEN or declared counter, native side, monotonic selection ID and host
session version. Native replay verifies side/event/action provenance. The
complete canonical record and every purpose/request/session/process/limit field
remain part of equality acceptance. A historical selection or semantic undo
cannot reset the single supervisor's retired request/generation ledger.

Chess Full reachability is still chess-specific. A shogi Full request receives
explicit unscored `UnsupportedMode`; it is not ordinary search or tsume. V2
chess decoding, APIs and saved historical observations remain unchanged. V3's
explicit native result family adds real shogi support where MSH-69 deliberately
returned unsupported; its old completion/evidence records remain historical.
Unknown/profile/schema/transport mismatches still reject through `RawValue` and
native strict parsers, with no map that could collapse duplicates and no game
fallback. Request1MiB, response2MiB and diagnostic4KiB limits were not raised.

## Actual measurements and process evidence

[Final measurements](shogi-engine-measurements.json) contain exact settings,
all three elapsed observations, complete native results, source/binary SHA256,
environment and encoded frame sizes. These are release one-shot child processes
on macOS27.0.1 arm64/Rust1.89; elapsed includes startup, parse/replay and output,
not just recursive work. Equal deterministic results were observed in all three
runs; elapsed times differ. They are not strength or all-platform performance
guarantees.

| Root | Requested depth/nodes | Completed depth | Actual visits | Median process ms |
| --- | --- | ---: | ---: | ---: |
| Standard initial | 2/5,000 | 2 | 962 | 27.226 |
| Kings9i/1a, Sente hand `2RBGSNLP` | 2/5,000 | 2 | 2,494 | 220.944 |
| Same kings, both hands `RBGSNLPrbgsnlp` | 2/5,000 | 1 | 5,000 | 1,006.002 |
| Both hands | 2/20,000 | 1 | 20,000 | 4,042.591 |

The hand roots are explicitly reduced diagrams
`8k/9/9/9/9/9/9/9/K8 b <hands> 1`, with523 local root choices. The
[preserved exploration](shogi-engine-exploration.json) recorded a100,000-node
drop-heavy request taking20,212.856ms median. That configuration was rejected
as the final supported cap; the exploratory binary/source hashes identify its
earlier provisional bounds. Final caps20,000 and defaults5,000 come from this
evidence, not copied chess depths/node defaults.

[Actual child play](shogi-engine-process.json) retains both human-side assignments,
four complete request/result observations with recorded work limits and native
final games. These are actual `EngineBridge`/`internal-search` subprocesses;
returned choices were admitted through native `Game::submit`. They are short
functional journeys, not full matches or playing-strength evidence.

Independent expected fixtures include material10800 for the full-stock hand,
capture/demotion arithmetic, unique promotion, protected mating gold drop,
same-position different-history best moves, native continuous-check winner and
no-action/length precedence, root/leaf declarations and exact node counts.
The read-only fixture auditor established expectations before product search;
board-only corroboration corrected a blocked bishop ray before the final
declaration-leaf expectation (`1i1h`, not `1i2h`). The native rules/proof were not
changed to fit either engine. Separate private finite-tree tests establish
minimax backup and descendant stop/error unwind without relying on a rules
oracle. Task MSH-70 records final independent review, verification and revisions.

## Runnable example

The native human console examples are in [the console contract](shogi-console.md).
Build once, then this optional Python driver creates an actual canonical root
through that console and submits an explicit read-only analysis to the native
one-shot entrypoint. Python is not a product runtime dependency:

```bash
cargo build --locked --release --bin mshogi
python3 - <<'PY'
import json, subprocess
exe = 'target/release/mshogi'
output = subprocess.check_output([exe, '--game', 'shogi', 'shell'], input='record print\n', text=True)
game = json.loads(next(line[7:] for line in output.splitlines()
                       if line.startswith('record ')))
target = dict(selection_id=1, event_index=0, board_ply=0, session_version=1,
              label='Initial shogi position',
              turn=dict(game_kind='shogi', side='sente'))
request = dict(protocol=3, purpose=dict(kind='move_analysis', target=target),
               request_id=1, session_version=1, process_generation=1,
               game=game, limits=dict(depth=2, nodes=5000))
response = subprocess.check_output([exe, 'internal-search'],
             input=json.dumps(request)+'\n', text=True)
print(json.dumps(json.loads(response)['result'], indent=2))
PY
```

It reports a real shogi choice with depth2,962 visits, root Sente score0 and PV
`1g1f 1a1b`; it does not apply the suggestion. For a live turn use
`purpose=dict(kind='engine_turn')` and admit the returned native choice in the
authoritative host only after full-context acceptance. Runtime hosts must use
fresh monotonic request/generation/selection identities when replacing work.

MSH-71 USI interoperability and MSH-72 shared desktop sessions are next; neither
is delivered here. MSH-73 archives and all desktop shogi/presentation duties stay
with their existing owners. MSH-41–47 and M4 remain open.


MSH-71 now supplies the separate [USI adapter and untimed experiment host](usi.md).
Its native terminal scores are not mislabeled as mate, declarations are not PV
board moves, and incomplete work has no fallback. MSH-70 measurements and
structured contracts remain unchanged; MSH-72 is next in the desktop sequence.
