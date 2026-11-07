> MSH-134 verification update — 2026-11-07: the test-only history-view
> synchronization correction at `f426c67e73af53b40aee6b9491a2d8cbde55bfb5`
> has passed all three required native CI jobs and focused independent source
> review and final evidence reconciliation. Normal publication is pending; see the
> [correction record](../review/history-view-134.md). Earlier MSH-47 references to
> open134 describe its acceptance checkpoint. M4 remains completed; no product
> behavior or original milestone verdict changed. No M5 work starts here.

# Final M4 delivery acceptance — MSH-47

Resumed review: **7 November 2026**, primary `codex/rewrite`, starting integration
**`68fef9f07760a7670f5d591705274ab4e078bd80`**. The original six-game/seven-profile
audit below is reused, with its exact source and environment qualifications.

## Current verdict: accepted — M4 complete

All four original MSH-47 obligations have supporting evidence. F47-01 is resolved
by the independently accepted MSH-112 correction. The separately identified
MSH-134 history-view test synchronization defect is real and remains unfixed,
but the source and independent retained behavioral evidence establish no missing
original M4 function or material coverage gap. This is not a conclusion drawn
from its passing retry. Fresh independent final review approves acceptance. Approved evidence was
normally committed and pushed at **`4731800ac3459b88de200f57a4728d06d9e54852`**
before Done47. MSH-47 is **Done**, all four original criteria and four completion
items are checked, and the supported milestone list reports **M4 Completed77/77**.
Neither M5 nor M6 starts here.

The **6 November closure-withheld finding and review remain historical evidence**
at `92a751d3e1df0ef4c4b9610cc55ec42e5e673770` and in the dated chapter below.
They were correct at that checkpoint. The compact existing
[acceptance index](evidence/msh47/verification.json) retains that verdict and
records the changed facts, rather than overwriting the failure history or
creating another platform ledger.

## F47-01 — resolved by the actual MSH-112 correction

Exact corrected and tested production:
**`44c86b01428ca5f0de56fdb96cbe7e5b013677ea`**. MSH-112's approved evidence was
published at `6aabc97e92676af32856f5166ecfb1ef52b5e2e7`; its final integration is
`68fef9f07760a7670f5d591705274ab4e078bd80`. See the
[bounded correction report](../review/runner-exit-112.md) and its
[single structured record](../review/evidence/msh112/verification.json).

Source inspection corroborates the accepted independent assessment:

- `run.rs::write_failure` distinguishes typed transport failures from timeouts,
  preserves the original receipt and uses the existing10ms exit settlement.
- An unknown hard-write/awaited-EOF status remains provisional. `cleanup` moves
  the existing quarter-budget/max100ms grace before stop/quit/kill, consuming
  the same cleanup budget. A definitive natural nonzero exit finalizes
  UnexpectedExit with diagnostics; clean exit without a required response
  remains MissingResponse. Unknown status stays explicit and honest.
- Supervisor-induced termination is separate evidence, never an invented
  natural nonzero exit. No timeout or cleanup allowance increased, no second
  grace was added, and the final failure is serialized after cleanup.
- Native history/clock precedence, failure receipt, strict unscored failed
  experiments and bounded reaping are preserved. Controlled original-source
  and mutation failures, late natural0/7/EOF, live-child/forced-exit separation,
  stream/startup cases and20 real supplemental trials per platform support this
  correction. MSH-47 revalidates their records; it does not rerun that campaign.

Live GitHub reconciliation confirms run **42190693190**, corrected source above,
completed successfully. These are the actual accepted executions:

| Platform | Attempt | Job | Required verification |
| --- | --- | --- | --- |
| Ubuntu24.04 | 1 | [127801682198](https://github.com/jweathers777/mShogi/actions/runs/42190693190/job/127801682198) | All8 steps passed |
| macOS15 | 1 | [127801682340](https://github.com/jweathers777/mShogi/actions/runs/42190693190/job/127801682340) | All8 steps passed |
| Windows2022 | 2 | [127812835546](https://github.com/jweathers777/mShogi/actions/runs/42190693190/job/127812835546) | All8 steps passed |

Each completed raw log contains all11 critical runner cases passing. Original
historical failures, the superseded insufficient correction, Windows attempt1
job127801682025 and both intentional automatic-Desktop cancellations remain
retained. Attempt2 Ubuntu/macOS carry-forward receipt aliases preserve the
original execution timestamps/runners; they are not reruns of healthy platforms.
No native or GUI evidence is inferred from a different operating system.

## MSH-134 — unfixed test synchronization defect; nonblocking disposition

The failed Windows assertion was
`shared_moves_chess_real_admission_reopen_and_takeback_do_not_fabricate_cues`,
`shared_moves_tests.rs:174`. Its required historical-root/cue/row semantics remain
valid. The helper at lines12–29 waits for **admission**, then returns a snapshot.
Chess Inspect instead queues asynchronous Work::View in
`persistence.rs::queue_view` (1559–1611). It has no admission transaction receipt,
so `shared_archive.rs::shared_command_with_receipt` (1828–1838) supplies immediate admission success.
That does not promise view completion. The helper can return the still-live
snapshot before `poll_files` applies Completed::Viewed.

The production path has a distinct completion contract: current I/O id and
document token guard installation in `poll_files` (1613–1664), then it publishes
Complete and advances revision. The frontend polls fresh snapshots
(`useGameSession.ts:19–25`); `ApplicationView.tsx:19,43` treats Viewing as busy and
shows the pending-position status. It does not promise an immediately historical
snapshot or use this test helper. No production historical-view defect is
established by the failed assertion.

The regression reliability concern was explicitly challenged against original
AC3, P-05 and required current-format/history behavior. Three independent lines
of evidence cover the exact semantics despite this faulty helper:

1. `moves/tests.rs:24–48` derives actual native Chess projections, retains known
   move rows and verifies that selecting root cannot invent a last-action cue.
2. `persistence.rs:1858–1868,2037–2077` waits for the exact view I/O id and checks
   historical prefix/conclusion/native-record preservation, Save/reopen and
   Return to live. It does not confuse admission with completed view work.
3. Original MSH-88 Windows **and** Linux `normal-cross-game` raw reports each
   contain two completed Chess root observations: I/O ids3 and14 are Complete,
   the native selected root has zero events, two known e4/a5 rows remain, and
   `last_action` is null. Reverse Chess Load/Return/Take back preserves native
   context and retained observations; Take back has zero rows and no cue.
   The original GUI driver also waits for settled Moves selection before root
   cue assertions (`gui-smoke.py:984–1042`). These are actual packaged consumers,
   not only build statuses, synthetic boards or the faulty helper.

Thus no material missing original behavior or coverage guarantee was found.
Treating every diagnosed false early assertion as a new universal no-flake M4
gate would add scope; treating its green retry as a fix would hide a real defect.
The accepted disposition does neither. Required native steps passed at the
corrected source, the strict assertion and failed receipt are preserved, and
the independent evidence above supports its intended completed-state meaning.
No failure-rate estimate or promise of universally deterministic CI is made.

**MSH-134 remains Ready, unassigned and unimplemented**, with its original three
criteria intact. It is the recommended next narrow quality correction before
starting new M5 work, not transferred unfinished M4 product functionality and
not marked Done or waived by this acceptance. This assignment does not fix it.

## Reused scope, sources, limitations and closure action

The original AC1/2 game/shared/architecture audit below remains supported. All
six games and both immutable Tenjiku profiles retain their sourced conformance,
authoritative effects/history/outcomes, functional supervised search, console/
notation, strict records, Save/Load, Take back/history, actual desktop behavior,
cross-game isolation and accepted architecture evidence. All13 referenced
acceptance-record hashes and all202 immutable MSH-88 evidence hashes revalidate.

Packaged/shared source remains **`bc92bfe5e3b20e9e999df12cbf54b91682c71b8d`**:
Rust42028972679 and Desktop42028972586, all six original jobs/required steps
reconfirmed live. Actual Windows/Linux49-phase reports and qualified local
macOS27 interaction remain at that source. Hosted macOS15 verification is
distinct. From that checkpoint, four neutral core rustdoc comments and the
bounded MSH-112 runner change are the only changed runtime-tree inputs; the
desktop/session/frontend/engine/search/rules/protocol/workflow inputs are
unchanged. Current integration has no input diff from corrected44c86b0. New
native correction evidence supplements the original package evidence; no new
package built or actual GUI executed at an evidence-only revision is claimed.

Dense minimum-window Tenjiku kanji/badges, incomplete comprehensive screen-reader
verification, release signing/notarization/distribution, bounded search depth,
unmeasured competitive strength and advisory architecture investigations retain
their original classifications below. Natural exit status beyond bounded
observation can remain unknown;134 retains bounded test nondeterminism. No rule,
schema, budget, accepted owner visual decision or original M4 criterion changed.

| Original MSH-47 criterion | Resumed verdict |
| --- | --- |
| AC1 — preserved26/39 and complete41–46 | Supported by unchanged accepted detailed evidence. |
| AC2 — independent complete six-game trace | Supported; previous audit reused, changed facts independently reviewed. |
| AC3 — reproducible integrated/platform evidence; no material required gap | Supported by112 correction/actual CI and explicit source-supported134 assessment above, independently approved and published. |
| AC4 — durable scope/verdict/limitations and honest handoff | Supported by this current decision plus preserved closure-withheld history; no unfinished original functionality moves to M5/M6. |

Backlog1.55.1 has no independent milestone-completion mutation: completion is
derived from assigned task statuses. Before finalization M4 was active76/77,
with47 the sole unfinished assigned task. After approved evidence publication
at4731800, supported `task edit MSH-47 --status Done` completed the final assigned
task. `milestone list --show-completed --plain` then reported **Completed77/77**.
This records the independently established outcome; a percentage alone remains
insufficient acceptance. Archival is retirement only; no archival, milestone
assignment, dependency or original milestone definition changed. MSH-133 and the
unrelated historical-access worktree remain untouched.

MSH-134 is recommended next; the first actual M5 assignment remains MSH-48,
followed by the existing109 gate before substantial optimization. Neither is
started or replanned here. This review adds no game implementation, test change,
broad native/frontend/GUI/CI campaign, refactor or new task hierarchy.

## Preserved closure-withheld review — 6 November 2026

Everything in this dated chapter describes the earlier review checkpoint. Its
unresolved112/status/count statements are historical, superseded by the explicit
resumed assessment above. The original report bytes and compact index remain in
Git at92a751d; failed CI provenance and independent blocked-review verdicts stay
retained. The unchanged six-game audit and environment qualifications are reused.

Review date: **6 November 2026**. Reviewed integration:
**`07f1bac4c366c0a327dd7f31e9528ee9aa444b5b`**, primary `codex/rewrite`.
Accepted runtime/platform source:
**`bc92bfe5e3b20e9e999df12cbf54b91682c71b8d`**.

This independently reviewed evidence was normally committed and pushed to
`codex/rewrite` at **`a666763ff65d8a30c0e6f9f10d8438d3cda4c643`**. Publication
integrates the review and handoff; it does not complete MSH-47 or close M4.

### Verdict: closure withheld

**The six complete games and shared desktop delivery have supporting evidence.
Formal M4 closure is not yet supported.** The existing, unfixed MSH-112 runner
failure-contract mismatch remains relevant to the original console/match scope
and MSH-47 AC3's failed-check gate. It is neither an architectural advisory nor
M5 tuning. MSH-47 must remain unfinished and M4 active until this issue is
resolved under its existing bounded contract and closure is reassessed.

This does not revoke completed native/game/shared acceptance, reopen M2/M3,
establish a new rules defect or require a redesign. It distinguishes their
bounded verdicts from final whole-milestone acceptance. No implementation or
acceptance contract was changed in this review. A fresh independent final
assessment **approves this closure-withheld report, not M4 acceptance**.

### Original scope and acceptance authority

The [original milestone definition](../../backlog/milestones/m-4%20-%20m4---variant-expansion-and-conformance.md)
requires complete Shogi, Tori, Chu, Tenjiku and Xiangqi against explicit sourced
profiles, preservation of Chess completeness and honest conformance before deep
shared optimization. The [design baseline](../design.md), not a task percentage,
defines the promised outcome:

| Original obligation | Required M4 interpretation and evidence traced |
| --- | --- |
| P-01 complete reusable six-game implementations | Native complete states/actions/effects/history/adjudication and records; MSH-26 and41–45, plus original game criteria and independently expected fixtures below. No promised game is replaced by a synthetic board. |
| P-03 independent console, other interfaces, self-play and matches | Current game-selected console and notation, native structured execution, actual UCI/USI/required protocol consumers and recorded complete/limited match journeys. Production runner failure diagnostics are part of this consumer surface. |
| P-04 desktop for all games on Windows/macOS/Linux | Actual native human/engine and file/history/lifecycle evidence, final packaged verification88, integration46 and independent contract review89. Release-distribution obligations are separately scoped in58/M6. |
| A-01/A-02 one native authority and complete atomic actions | Native action/effect distinction, exact re-admission and reversal; complete routes, promotion/drop/igui alternatives and native derived captures/burns. Partial UI entry is not a turn. |
| A-03/A-04 history/match/profile and honest coverage | Explicit immutable profiles, root provenance, exact histories/commands/outcomes; saved/repetition/search/runtime identities have different purposes. Claims, declarations and multiple-royal conclusions remain game-owned. |
| A-05 isolated search and interfaces | Native private search state, bounded work, cancellation/stale rejection, native supervised engine and independent console; no frontend/Python/node legality or node round trips. |
| A-06 records/interoperability | Strict current canonical replay, explicit unsupported conversion and real required external adapters. Owner A-09/A-10 intentionally retired development transports/desktop formats; it did not retire native semantics. |
| A-07 native presentation | Actual square/intersection/hand/large-board renderers, shared complete-choice interaction, keyboard/readable metadata and native effects. Accepted minimum-window and accessibility qualifications remain explicit. |

The original four MSH-47 criteria remain unchanged. All nine prerequisites
26/39/41/42/43/44/45/46/108 are Done, but that is not a waiver of an unresolved
required failure contract. MSH-26's [exact completeness](../rules/chess-completeness.md)
and MSH-39's [closed M3 evidence](../review/m3-closure.md) remain accepted at
their historical revisions. The required full-game and shared scopes were
reconstructed from original criteria and the preserved
[delivery plan](delivery-plan.md)/[implementation map](implementation-map.md),
then checked against actual consumers rather than only final task summaries.

### Six games and seven immutable profiles

The current native/search responsibility map is source-qualified in
[MSH-108](../review/architecture-health-108.md); this acceptance sampled the
actual implementations/tests and the original game criteria independently.

| Game/profile | Complete behavior and initial state | Accepted detailed source/evidence |
| --- | --- | --- |
| Chess — `mshogi.chess.match.v1` |32 initial pieces/all six kinds, castling/EP/four promotions, native king safety and derived effects; repetition/claims/intended claims, clocks and actor-aware match events.26 adds exact cooperative Article-3 dead/side-specific mating reachability. | [Coverage](../rules/chess-coverage.md), [completeness](../rules/chess-completeness.md), native `chess/{action,legal,game,adjudication,matchplay,record}.rs`, core `tests/completeness.rs`; [R6](../review/architecture-acceptance-121.md) and current88 regressions preserve actual desktop consumers. |
| Shogi — `mshogi.shogi.csa36.v1` |40-piece setup, all original/promoted kinds, capture demotion/hands, optional/forced promotion, every drop category, nifu/dead-rank/pawn-drop-mate rules, king safety; repetition/checker loss, declarations/false declaration, resignation and320-action precedence. | [Complete delivery/8AC](shogi-delivery-review.md), [independent source review](evidence/msh75/rules-review.md), native `shogi/{action,legal,game,record}.rs` and actual board/choice/history/declaration fixtures. |
| Xiangqi — `mshogi.xiangqi.wxf2018.v1` |32 pieces/90 intersections/all seven kinds; screens/legs/eyes/river/palaces/General safety, no promotion/drop; actual identity/conduct/cycle warnings/obligations, claimant counts, natural-win proof/Unknown and complete event prefixes. | [Delivery acceptance](xiangqi/delivery-acceptance.md), [evidence](xiangqi/delivery-review-evidence.json), native `xiangqi/{action,legal,game,conduct,assessment,exact,record}.rs`, independent WXF examples and exact-universe tests. |
| Tori — `mshogi.tori.modern.v1` |32 pieces/7×7; handed Quails, nine movement tables, Falcon/Swallow forced promotion, demotion/hands, two-Swallow/file/drop-mate restrictions; Phoenix safety, mate/quiet immobility loss and selected must-vary/repetition-forfeit semantics. | [Delivery/consumer accounting](tori/delivery-review.md), [selected profile](tori/profile.md), native `tori/{transition,legal,game,record}.rs` and frozen independently reviewed corpus. |
| Chu — `mshogi.chu.jcsa2019.v1` |92 pieces/12×12,21 original kinds/18 promoted-origin states/29 faces; no drops; ordered Lion/Horned Falcon/Soaring Eagle routes, captures/returns/refusal and Lion protection; surviving King/Prince, capture-based admission, finite immediate mate, pre-fourth obligation and actual draw consent. | [Delivery/8AC](chu/delivery-review.md), [native verification](chu/native-verification.json), native `chu/{position,legal,game,record}.rs` and game/local conformance adapters. |
| Tenjiku — `mshogi.tenjiku.adams1999-modern.v1` |156 pieces/16×16, original/promoted identities and44 faces; complete Lion/Hawk/area routes, ranked jumps, igui, per-original promotion/refusal, simultaneous burn/mover destruction, multiple royals, history admission and native endings. Adams-specific behavior stays profile-qualified. | [Complete profile](tenjiku/profile.md), [native evidence](tenjiku/native-verification.json), [product/8AC](tenjiku/product-verification.json), native `tenjiku/{position,legal,game,record}.rs`. |
| Tenjiku — `mshogi.tenjiku.muller-completed.v1` |Same complete native/product surface with selected Muller distinctions, including Free Eagle/jumping and Fire Demon behavior. Separate whole-profile history, effects, search contexts, files and receipts; no independently configurable rule switches. | Same independently expected rows executed separately for this profile, plus distinguishing effects and genuine both-profile execution/product evidence. |

Imported diagrams/counters are not proofs of historical reachability. Sparse
native diagram endings, capped standard-root play, actual resigned games and
natural completed tactical games retain their different meanings. Chess Full
is exact finite traversal given sufficient resources, not a universal practical
runtime promise. Bounded/cancelled proof remains Unknown and cannot invent a
timeout/resignation result. No strength conclusion follows from functional play.

### Independent conformance reconciliation

The [engine-independent policy](../testing/conformance-policy.md) remains binding:
selected profiles, independently established expectations, complete-set/transition
checks, exact reversal/replay, meaningful property/metamorphic/adversarial tests
and independent review. External engines are optional investigations, not new
build/test/closure gates. These counts are reused original evidence, not tests
newly executed by MSH-47.

| Game | Independent expected evidence and precise qualification |
| --- | --- |
| Chess |68 FEN inputs;49 ordinary sequences/74 after-states;17 candidate sets/274 candidates;29 attack diagrams/155 attacker sets; special-action21 positions/six sequences/25 states/18 rejections;35 legal corpus positions;38 perft totals/14 divides. `tests/completeness.rs:26` independently derives blocked-wall1152 vertices/6240 transitions and compares a differently indexed DFS. Seeded invariants, corruption detection, witness/negative/budget/cancel/mode replay tests supplement these expectations. |
| Shogi |101 board plus68 complete-choice fixtures;21 history ledgers/nine declaration diagrams/seven command cases. Independent75 review recalculated149 prefixes plus nine declaration roots. Start depth2=900, each divide30 is independently derived. Wider169 depth-two/perft and4017-prefix comparator observations retain their observational status. Actual counter-transform/material/sequence/reversal, strict-record mutations and declaration-history tests are present. |
| Xiangqi |105 board fixtures=101 live+four structural-only; initial44 independently enumerated; sparse perft2/3/7 and divide5+2 hand-derived.20 conduct witnesses/11 cycle histories/four natural-count histories. Exact tests independently enumerate108 bare-General plus510 per Advisor color, successor closure, positive witness/negative/resource cases and owner-exchanged witness replay. Historical79666 initial depth3/520 prefixes remain observations. |
| Tori |103 immutable fixtures/713 executed native assertions=202 structural+281 local+230 Game. Nine authored geometry tables;40 board+two resignation transition cases;35 authored rotated probes; perft3/9 with13 visits. Native harness14 pending consumer fields are not native passes: runtime13 fields/82 requirements plus notation1/1 complete the separate14/83 consumer obligation. |
| Chu |250 immutable rows=240 native+10 real-consumer rows;1076 executed native assertions and37 documentary annotations. Hand-counted perft7/42; Lion88/Soaring Eagle41/Horned Falcon42 geometry;583 rotated actual choices/exact reversals. Native aggregate asserts240 passed/zero pending; product consumers are separately reconciled, not relabelled native tests. |
| Tenjiku, each profile |350 immutable rows=204 native+129 documentary/property annotations+17 consumer rows.204 native rows execute621 expected fields **per profile**,1242 field executions across two profiles, not408 independently distinct positions. Full-Game sparse perft4/16, four divides4,21 visits. Adapter `conformance.rs:29–193` derives native observations and compares complete sets; property/owner-exchange/burn-phase/identity/atomicity/undo/redo/strict-record tests remain separate. |

Examples independently sampled include Chess `tests/{sequences,completeness}.rs`,
Shogi `tests/{shogi_legal,shogi_history,shogi_record}.rs`, Xiangqi
`tests/{xiangqi_board,xiangqi_history,xiangqi_exact}.rs`, and Tori/Chu/Tenjiku
native `{local_tests,game_tests,record_tests,game_conformance}` where supplied.
Tests derive actual effects from native code rather than copy expected fields.
One game's valid symmetry is not imposed on every game. Perft is not evidence of
complete history adjudication, correct search, speed, strength or desktop play.

Historical handoff gates are not current omissions. In particular79's Tenjiku
shared record has12 applicable consumer rows plus five then-pending44 rows;
current44/source completes execution, notation/art and normal activation.
The native corpus/earlier reports remain immutable evidence of their own stage.

### Console, functional engines and actual conclusions

`mshogi-console/src/main.rs:239/273` selects real games and isolates machine
protocol output. Game sessions expose complete notation/actions, native commands,
records, strict Load/Save, history/Undo and labelled local diagnostics. Actual
CLI suites include Shogi/Xiangqi/Chu/Tenjiku malformed record/file refusal and
atomic command batches. Required UCI/USI and Xiangqi adapters remain real consumers;
Chu/Tenjiku are not forced into an invented standard-protocol route encoding.

Each concrete search owns a private full-history native Game/match and uses
native legality/submit/undo; independent solved-tree/history/terminal/restoration
tests exist. No heuristic score is an adjudication and no board-only cache merges
incompatible history/profile identities. All six use actual supervised native
dispatch: `mshogi-engine/src/shared.rs::ExecutionContext`, `server.rs::serve_once`
and `bridge.rs::EngineBridge`. Native conclusions, incomplete iterations and
process failures are separate. A result without a completed iteration cannot
fabricate a fallback move/score/outcome; retained completed iterations preserve
their actual depth/PV under the accepted result contract.

Real native terminal examples include Tori subprocess mate, Chu complete compound
dual-royal capture/Finished and both-profile Tenjiku dual-royal capture. These
support native conclusion delivery, not competitive strength or an invented
completed standard-root game. Read-only analysis and exact root-side telemetry,
native PV/leaf validation and observation-before-engine-action admission have
source/test/actual-product evidence. The separate match-runner diagnostics remain
the unresolved finding below; desktop EngineBridge independence is not a blanket
acceptance of every console consumer.

### Desktop, persistence and bounded resources

Actual New/Load, either human side, real engine replies, native choices and
special actions, analysis/cancel/replacement, outcomes/recovery, exact history,
Return to live/Resume, retained Take back, Save/Load/stopped reopening and guarded
native Quit are evidenced by the game records and final88 journeys. The normal
Tenjiku chooser has both complete profiles. Original/promoted pieces, hands,
intersection geometry, captures/burns and multiple royals are native-derived.

One current `mshogi.desktop.document.v1` envelope preserves strict native payloads,
journals, settings, observations, targets and abandoned branches. Load validates
off-thread, checks fresh identity floors/headroom and installs stopped authority;
saved observations never become pending work. Invalid/stale/capacity refusal
preserves accepted state/history/dirty/path and prior-file bytes, with honest
operation receipts. Atomic Save uses exclusive same-directory temporary output,
flush/sync and cancellation before rename; committed Save is not retrospectively
reported failed by elapsed-budget classification.

The [89 capacity table](shared-integration/contract-review-89.md#execution-saveability-and-resource-accounting)
and [108 stress assessment](../review/architecture-health-108.md) remain accepted.
Common16MiB/256-observation/8192-control bounds retain different native replay
models: Chess retained-branch16k, Shogi16k weighted, Xiangqi16k combined,
Tori262144, Chu12M and Tenjiku160M whole-document units. Tenjiku native1MiB/
1024events/512M is separate from its document160M and cooperative15s worker
deadline. Actual ending/Close reservations, including eight native/document
actor combinations and early/final/prevalidated Close, remain unchanged.
Mandatory successful representative journeys are distinct from permitted typed
near-limit refusals; an operation deadline is not a universal latency promise.

Native Chess M2/schema1 and Full/schema2 core meanings remain valid. Console
restore preserves its documented M2 move/history scope; desktop live restore
is untimed M2, with historical bounded Full analysis/certificates separately
supported. Unsupported/lossy restoration rejects explicitly. Development desktop
archives1/2/3 and internal execution1/2/3 remain retired under A-09/A-10; protocol4
and durable observation schema1 are current. No compatibility reader is revived.

### Original six shared-integration criteria

The [46 matrix](shared-integration/README.md) and [independent89 AC1–6 source assessment](shared-integration/contract-review-89.md)
are independently corroborated at current consumers:

| Original46 criterion | Current supporting mechanism and integrated evidence |
| --- | --- |
|1 library/console/execution/session | Native `RulesProfile/CanonicalRecord/dispatch_native`, complete execution context/history/purpose/limits and exact result re-admission; actual six search dispatchers/console sessions and seven-profile subprocess/desktop journeys. |
|2 canonical records/current Save/Load/Chess scopes | Strict native codecs/raw payload identity and current envelope; genuine cross-game files/stopped Load, malformed prior-file preservation, Chess M2/Full qualification and explicit unsupported rejection. |
|3 real presentation/complete choices | Actual square/intersection/hand/12×12/16×16 renderers; one ordered-choice/inspection owner. Partial selection/cancel never commits; native promotions/drops/routes/effects and keyboard/metadata checks retained. |
|4 ownership/cross-game/profile transitions | One `DesktopController/ActiveDocument/RuntimeMetadata`, existing execution/document/frontend/shell/choice owners; real same-host New/Load and delayed-engine seven-profile transitions, plus deterministic obsolete-authority rejection. |
|5 actual packaged three-platform evidence | Exact six receipts and qualified actual interaction/package identities below, not a build-only or synthetic substitution. |
|6 independent contracts/integration | Fresh89 review and46 final reconciliation resolved material shared-contract findings;108 independently reconciles architecture. Their narrower112 dispositions do not waive whole-M4 console failure acceptance. |

Concrete owners remain `desktop-session/src/lib.rs:290/346/1207`,
`persistence.rs:1664 install_document` for current-file Load, and admitted native
New paths under the same controller. Frontend `ProductApp→useGameSession→
ApplicationView/ApplicationShell`, `GameBoard` and
`interaction/useNativeChoiceController` retain sole session/choice lifetimes.
Native semantics remain game-owned; no second frontend legality engine or
universal variant framework exists or is required.

### Exact packaged-platform receipts and qualifications

Live GitHub connector GETs on6 November confirm both runs completed/success at
exact source `bc92bfe5e3b20e9e999df12cbf54b91682c71b8d`, attempt1, with the six
original jobs and all expected successful/platform-skipped steps intact:

| Platform | Rust run42028972679 | Desktop run42028972586 | Actual accepted interaction |
| --- | --- | --- | --- |
| Ubuntu24.04 | [127250225725 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972679/job/127250225725) | [127250225410 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972586/job/127250225410) |49/49 actual installed/packaged normal/QA phases, zero failed phases. |
| Windows2022 | [127250225944 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972679/job/127250225944) | [127250225967 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972586/job/127250225967) |49/49 actual normal/QA installed phases, zero failed phases. |
| macOS15 | [127250225640 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972679/job/127250225640) | [127250226038 PASS](https://github.com/jweathers777/mShogi/actions/runs/42028972586/job/127250226038) |Hosted native/build/package/bundled-engine checks. Actual macOS GUI is separately local27 evidence, not hosted GUI. |

The unchanged [MSH-88 master](shared-integration/verification.json), SHA256
`097118de9a7c9507900f554da7c91dde97ad35729bce73c9f491b52a7a9cd985`, retains202
hashed evidence files, full original job/runner/step receipts, environments,
commands/package inventories, actual reports and historical failures. Windows
and Linux raw report SHA/source/49phase results and normal host/engine hashes
are checked against retained package-smoke receipts. Their13 normal phases per
platform agree with the normal package; QA installer/extracted-package receipts
remain distinct. Normal bundles exclude instrumentation.

Actual seven-profile same-host New/Load, real engine observations, stopped Load,
retained branches and eight exact delayed-process retirements supplement the
baseline. Retiring old processes is not proof that an obsolete reply was
delivered; deterministic native tests separately submit stale authority. The
labelled synthetic phase does not supply real-game acceptance.

Local macOS27.0.1/26A434 arm64 actual seven-profile normal/QA journeys include
real engine/analysis, files/history/cancel/replacement/Close and process absence.
All four local normal bundle files match its later local tested-source rebuild.
This is not byte equivalence between macOS27 local binaries and hosted macOS15
binaries. Fixed normal/QA identities, ad-hoc/sealed signature qualifications,
excluded observer attempts and no-prompt observations retain their actual scope.
No release signing/notarization or comprehensive screen-reader claim follows.

All production/test/frontend/host/observer/workflow inputs at reviewed07f1bac
differ from testedbc92bfe only by four neutral `GameKind` rustdoc descriptions.
No binary execution is relabelled as occurring at this documentation revision.
Prior superseded/cancelled/failed runs, capacity cancellations, driver/fixture
corrections and unsuccessful runner assertions remain preserved. The current
six successful jobs establish those campaign results, not an MSH-112 repair.

### Material finding F47-01: unresolved runner failure-contract mismatch

**Blocks whole-M4 closure, not accepted native game/shared architecture delivery.**
This is an existing failed correctness assertion, not a new requirement created
from task counts or a speculative performance concern.

The production `mshogi-match run` path calls `run_experiment`
(`crates/mshogi-runner/src/main.rs:64–67`). The [runner contract](../match-runner.md)
records distinct start failure, unexpected exit, invalid action, missing response
and timeout with diagnostic evidence and no invented score. Current
`crates/mshogi-runner/tests/runner.rs:153–180` explicitly requires an exit fixture to be
`UnexpectedExit` with a non-success coded exit, not MissingResponse/exitNone.
MSH-112's original AC2 requires preserving these meaningful distinctions and
forbids accepting either label or raising timeouts to hide the mismatch.

Repeated recorded observations instead show the nonzero-exit fixture reporting
MissingResponse/handshake/Broken pipe with no exit status:

| Historical source/campaign | Preserved observation |
| --- | --- |
|6dec5ab, MSH-102 Rust40874033735 |Original runner.rs172 expectedUnexpectedExit/actualMissingResponse; task112 records actual source and failure. |
|eb027be7…, MSH-119 Linux41131548408/job124323454077 |Same unchanged-runner assertion, handshake Brokenpipe32/exitNone. [Recurrence evidence](../review/evidence/msh119/runner-112-recurrence.json). |
|136f8926…, MSH-86 Linux41536948188/job125591097628 |Same assertion/signature; [Chu native record](chu/native-verification.json) retains actual failure and source. |
|9749eec0…, MSH-87 Linux41816309261/job126527815548 |Explicit exit7 expectedUnexpectedExit, observedMissingResponse/exitNone; [Tenjiku native record](tenjiku/native-verification.json). Later unchanged-source Linux job126537522942 passed; this is not a fix. |

The [R6 failure](../review/evidence/msh121/checks/native176-linux-failure.log.gz)
and the complete112 history remain additional preserved evidence. Current
`crates/mshogi-runner/src/run.rs:295–318` samples `observe_exit()` once after a write error; absent
an observed non-success status, a non-timeout error becomes MissingResponse.
`crates/mshogi-runner/src/process.rs:224` has a separate bounded10ms EOF/status path. This is consistent
with an observation-ordering explanation; it is **not** a newly controlled
reproduction, proved root cause or decision whether the correction belongs in
product or fixture synchronization. MSH-112 explicitly owns that diagnosis.

Evidence demonstrates the category/exit-diagnostic mismatch. It does **not**
demonstrate a fabricated score, corrupt native game, broken cleanup, missing
ordinary desktop play or a need for architectural redesign. Failed experiments
remain unscored. These bounds prevent exaggerating the defect; they do not
make the required diagnostic assertion optional.

MSH-112's original exemption was no M2 reopening or gate on unaffected Xiangqi
work. MSH-89/46 correctly found no material shared **application-contract** defect;
108 found no **architectural redesign** blocker. Those narrower conclusions stay
accepted. They do not supply whole-milestone acceptance for the actual headless
consumer. No recorded owner decision waives this required distinction. Original
47AC3 explicitly bars closure with required failed checks/material findings.

**Narrow correction:** execute existing112's controlled write-error/EOF/exit
diagnosis; resolve the product or fixture defect under the unchanged failure
contract; preserve meaningful categories/unscored outcomes/bounded cleanup;
obtain focused independent process/regression and actual-platform evidence.
Then resume47 at its unchanged original criteria. Do not implement112 in this
review, weaken the assertion, rerun until green, move it to M5/optional research,
archive it, or reopen completed M2/native game acceptance.

### Other limitations and milestone classifications

| Issue | Classification and consequential rationale |
| --- | --- |
| MSH-112 |Separately owned **unresolved functional failure contract and current whole-M4 blocker**, for the reasons above. Not M5 tuning, an architectural advisory or solved by current green CI. |
| Historical failures/retries |Keep original provenance. Corrected source/verifier findings retain successful executed evidence; an unchanged retry establishes that run only. No generic claim that every historical defect is fixed. |
| Tenjiku small kanji/secondary badges at760×650 |Explicit owner-accepted limitation in [design visual decision](../design.md#owner-acceptance-of-tenjiku-visuals-and-shared-identification-2026-11-05): board fits, not every label comfortably readable; about960×1000 recommendation only. No new aesthetic gate or removed game. |
| Comprehensive screen-reader operation |Not certified by keyboard, labels or native AX metadata. Existing required57/M6 accessibility/product scope, not an invented passed check or newly universal M4 certification gate. |
| Release signing/notarization/licensing/minimum OS/update distribution |Existing58/D-10/M6 scope. Real development packages/bundled engines establish functional platform delivery; they are not notarized release acceptance. |
| Competitive strength/learned evaluator |Explicit48–55/104 M5 obligations. Bounded functional legal/history-aware search is delivered; no measured strength target or model improvement is claimed here. |
| 108F01–F04 replay/cooperation/inspection/helper recommendations |Advisory architecture/performance investigations with concrete verification expectations. No demonstrated original M4 contract failure or required refactor established by those recommendations. |
| Future large historical variants |Deferred owner direction outside current six-game M4/M5; no new rules-acquisition, generalization or product gate. |

### Task, dependency and milestone integrity

Original47 dependencies remain26/39/41/42/43/44/45/46/108. Original46 retains all16
prerequisites39/41–45/69/72/73/76–79/88/89/90; game-specific ownership remains41–45.
Research/kernel/shared/consumer splits retain their original full-game obligations.
No completed task's evidence, criteria, milestone membership or owner is removed.
The nine Done prerequisites do not prove the wider failed-check obligation solved.

MSH-112 remains To Do/unassigned in `m-4`, with its original criteria and evidence.
Its existing required behavior is not newly invented because it is assigned there;
that assignment is retained rather than silently moved to improve the percentage.
The unrelated133 documentation review has no milestone and remains untouched.
The historical-access worktree is preserved; this task creates/removes none.

Pinned Backlog1.55.1 `milestone list --show-completed --plain` reports M4 **active,
75/77 Done** at this checkpoint. Completed M2/M3 still have their milestone files
in the active milestone directory. The supported CLI derives completion from
task statuses; it has no separate completion mutation. `milestone archive`
only moves a milestone file to the archive and is not a substitute acceptance
decision. No milestone file, assignment or status is changed to conceal112.

### Independent assessment and focused verification

Two fresh read-only researchers without integrator chat history independently
examined game/conformance coverage (`msh47_game_conformance`) and original scope/
open-issue/formal-workflow eligibility (`msh47_scope_blockers`). The first supports
the complete game deliveries with the precise qualifications above. The second
identified the broader112 failure-contract blocker and distinguished it from
earlier narrower nonblocking conclusions. Neither changed files or ran a product
campaign. A third fresh read-only whole-report reviewer,
`msh47_final_acceptance`, **approved the closure-withheld verdict** after
independently challenging original scope, real runner consumers, required failure
assertions, earlier narrower verdicts, seven-profile coverage and platform/hash
evidence. It found no additional material factual/coverage/disposition gap.
Its source was07f1bac; candidate hashes and actual commands/environment are
recorded in the index. A later locator-only clarification spells the complete
runner paths; it changes no finding. This review is not acceptance of the
unresolved requirement or of M4.

[evidence/msh47/verification.json](evidence/msh47/verification.json) records actual
current-source evidence/hash/link/dependency checks and the live GitHub receipt
reconciliation. Detailed native/platform evidence stays in its existing records.
No new native conformance, comprehensive frontend, packaged GUI, CI, strength,
benchmark or controlled112 reproduction ran. Required new CI jobs for this
documentation review: none;112's later correction has its own verification scope.

### Original MSH-47 acceptance and completion verdict

| Original criterion | Current disposition |
| --- | --- |
|1 preserved26/39 and complete41–46 obligations |Supported: preserved native completeness/M3, original game8AC/DoD and shared6AC evidence, not task statuses alone. No previously completed behavior is revoked. |
|2 independent six-game/native/conformance/console/search/record/desktop trace |Supported by concrete current consumers, independently expected fixtures/reviews and qualified actual product evidence. The cross-cutting failed diagnostic remains explicitly identified rather than hidden. |
|3 reproducible integrated/platform evidence; no required failed checks/material findings |Platform/source/package portion supported; **not complete** because112's existing failed runner contract remains unresolved. Current green jobs do not discharge this obligation. |
|4 durable final closure verdict/scope/limitations/handoff, with no premature closure |Supported as an explicit independently reviewed **blocked closure verdict** with the112 corrective handoff. This checkbox does not mean a positive M4 acceptance; no unfinished functional requirement is transferred into M5/M6. |

MSH-47 remains **In Review**; M4 remains **open/active**. AC1/2/4 have evidence;
AC3 remains unchecked. DoD1/2 remain unchecked while required acceptance and the
material finding are unresolved. DoD3 covers the current documentation/112
handoff; DoD4 is supported by the actual normal review-evidence integration/push
at `a666763ff65d8a30c0e6f9f10d8438d3cda4c643` and durable task summary.
A completed report is not completed milestone acceptance.
Independent review of this blocked verdict does not accept the still-failing
requirement.

### Corrective and M5/M6 handoff

The immediate narrow prerequisite to a new closure decision is **existing
MSH-112**, not another planning hierarchy or broad game campaign. After its
controlled correction/independent verification, resume47, reconcile exact source
and affected evidence, and evaluate formal closure again. Preserve original
accepted rules, schemas, complete actions, budgets, ending/Close reserves and
minimum-window policy throughout.

The actual next M5 assignment after M4 acceptance is **MSH-48**, the existing
dependency-ready reproducible strength/learning experiment pilot. It is To Do,
unassigned and unstarted. MSH-109 still depends on108 and48 and is not yet
dependency-satisfied; it precedes substantial game-strength/optimization work.
M6's57/58 retain required polish/accessibility/distribution, with optional59
research separate. None is started here. STOP after this47 acceptance review.
