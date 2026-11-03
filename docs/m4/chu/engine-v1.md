# Chu functional engine and execution contract

MSH-43 consumes the accepted `mshogi.chu.jcsa2019.v1` native Game and MSH-78's
current shared document/runtime contracts. This is a functional baseline, not
MSH-52/M5 playing-strength evidence. The native profile, edition and record1,
canonical envelope1, execution protocol4/observation1 and desktop envelope1 stay
unchanged. Normal fifth-game activation remains gated pending owner visual review.

## Search and score

`mshogi_search::chu` uses iterative complete full-width minimax over native
`Game::legal_actions` and `Game::submit`, with exact native reversal on every
normal, resource and cancellation path. There is no board-only transposition
cache or alternate legality/adjudication. Complete Actions retain both ordered
landings, returns and promotion. Native history includes Lion protection,
pre-fourth repetition obligations, command events and all surviving royals.

The transparent untrained material evaluator uses hundredths of a Pawn and the
native movement face. Native terminal results use a separate win/loss score band
with distance in complete board actions; a win is not necessarily mate. Scores
are from the captured root side, named Sente/Gote. Static material never creates
a conclusion. No evaluation resignation or automatic draw-agreement policy is
added. Actual native draw offers, opposing consent, refusal and concluded draws
remain distinct commands/history; search does not manufacture consent.

Play defaults are **depth1 /512 total visits**. Analysis initially uses those
same defaults, with explicit maximum **depth2 /2,000 visits**. Visits include
iterative revisits and discarded work. Zero-work requests in the native protocol
produce no score, continuation or action; ordinary desktop controls require a
positive budget. Only a fully completed iteration can publish a choice. If a
later iteration is interrupted, the last completed choice/score/PV remains usable
with its actual completed depth and stop reason. If none completed, Incomplete
has no fallback. Native failure, resource refusal and unresolved no-board-action
situations remain unscored; they do not become a win, loss or draw.

Release measurements on this local macOS arm64 machine (not latency guarantees):

| Root | Native choices | depth1 visits / elapsed | depth2 visits / elapsed |
|---|---:|---:|---:|
| Standard start |36|37 /5ms|1,370 /217ms|
| Two-stage Lion tactical fixture |91|92 /11–15ms|1,994 /1,111ms|

Debug depth2 observations were2,493ms and9,630ms respectively. This supports a
conservative depth1 interactive default rather than inheriting Tori's settings.
Reproduce with `cargo run --release --locked -p mshogi-search --example chu_work -- 1 512`
and then `-- 2 2000`. Finite native queries/reconstruction are themselves bounded;
the supervising bridge can terminate a child independently of cooperative stop.

## Supervised result and admission

`ExecutionResult::Chu` carries actual `ChuResult::Choice`, `Finished` or
`Incomplete`, with native complete Actions and telemetry. Failure remains the
existing separate execution result. Capability dispatch advertises real Chu
engine-turn and ordinary move analysis only; Chess Full and Xiangqi exact
operations do not become Chu operations.

The captured context binds profile/edition/schema, full canonical root/history,
purpose/target, work limits and request/session/process identities. Host polling
performs bounded shape/context checks. Heavy native root/PV/leaf validation and
candidate reconstruction run on the existing off-path admission worker. The PV
is admitted against native history; root-side score and actual leaf/terminal
meaning are checked, without asserting the heuristic choice is optimal.
`validate_with_work` reports actual reconstruction plus native PV transition
work, checked between bounded calls for cancellation/deadline.

The immutable document journals each observation **before** its exact re-admitted
engine action. Both enter one complete candidate. Missing, reordered or changed
following actions reject. Analysis observations never append board commands.
Wrong, superseded or retired context cannot become current after selection,
Return to live, Take back, cancellation, replacement or reopen. Play and analysis
share one supervised execution slot with explicit purpose; replacing one retires
its authority before starting the other. Browse-only suspension can restore play
with fresh identities; explicit stops/Load/Take back/failure remain stopped.

MSH-78's16MiB/4,096 retained events/8,192 controls/256 observations/12M aggregate
replay units/15-second worker deadline and actual ending/Close reservations remain.
Native512KiB/512events/4M work limits are unchanged. Actual context and PV replay
are charged in addition to retained record parsing and runtime installation.
Failed admission preserves the accepted complete document and prior file; it
cannot leave ghost Thinking work. Historical unsupported observations from
MSH-78 remain valid historical unscored evidence, not a current capability claim.

## Functional evidence boundary

Actual search/server, console subprocess, supervised engine child and desktop
controller tests are separate evidence. A32-ply standard-start progression is a
bounded smoke, not a complete game or strength result. Independently expected
royal, finite mate, material, promotion and repetition fixtures retain their
original provenance. The1,076 MSH-86 checks and MSH-78 consumer evidence are reused
where unchanged, never recounted as newly authored engine tests.

Measured debug complete documents with real observations:

| Candidate | Encoded bytes | Aggregate work | Retained native events |
|---|---:|---:|---:|
|12 board actions /12 observations|187,703|27,246|78|
|32 actions /32 observations|651,611|534,616|528|
|Take back to24, retain32 observations and branch|675,048|541,771|552|
|256 actual zero-work observations|2,601,304|28,820|0|

Selected-history decode took773ms at12 actions and5,112ms at32 actions locally.
The257th observation refuses with typed observation capacity while the accepted
bytes and Stop/Resign/Close save paths survive. These measurements do not promise
that arbitrary longer documents fit: repeated complete historical targets consume
native-event, byte and replay capacity. Structural capacity does not reset on
Save/Load. Near-limit operations may refuse truthfully; no history/observation is
silently removed and no limit is raised.

The structured MSH-43 checkpoint evidence records exact source/package revisions,
checks, failed probes, independent reviews and GUI screenshots. Owner aesthetic
approval and subsequent normal activation/packaged three-platform Chu gameplay/
final eight-criterion reconciliation remain separate from this visual checkpoint.
