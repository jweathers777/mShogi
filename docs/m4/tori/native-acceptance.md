# MSH-85 — independent native Tori acceptance

**Verdict: PASS for the complete selected native kernel and real core-consumer
handoff.** Selected `mshogi.tori.modern.v1`, edition
`MSH81-MODERN-2026-11-01`. Reviewed checkout
`558402b4426a1d8faff3931f65643bcda95dd2fe`; unchanged production source
`994521bec0637d6f8f17103f1106c0025d950753`. This acceptance adds documentation and
Backlog evidence; it does not implement a new game consumer or change rules.

Root Codex integrates from the clean primary `codex/rewrite`, aligned with origin.
All direct prerequisites81/126/127/128/129 are Done with supported criteria and
independent reviews. Backlog1.55.1 matches the project baseline. No other active
task owner or task worktree was claimed; unrelated worktrees remain intact.

## Original acceptance criteria

| MSH-85 criterion | Current source and independently examined evidence | Verdict |
| --- | --- | --- |
| AC1 authoritative native state/actions/effects/reversal/history/records | `tori::Position` structural roots; `LocalPosition` complete local rules and immutable inspection; `Game` full-history permission, context, terminal/repetition semantics and reversal; `GameRecord` strict replay and bounded serialization. Actual native and external-consumer tests below. | PASS |
| AC2 independent rules/transition/history fixtures | Unchanged103 specifications and ownership manifest; all nine geometry tables, both owners/handed identities,40board+2resignation transition cases, independently authored complete-choice/defense/terminal/history examples;713 actual native assertions plus meaningful property/metamorphic/adversarial checks. | PASS |
| AC3 stable native payload/capability handoff without duplicate rules | Actual typed `native.rs` routing and external core consumer use native Tori choices/effects/inspection/context/history/records, reject wrong identities, and preserve existing games. Engine routing explicitly unsupported/unscored/unadvertised. Real host session/document/presentation/execution integration remains77 and full product delivery42. | PASS at approved native boundary |
| AC4 bounded child dependencies/verification before Ready | Existing reviewed native-plan/native-plan-review and independent component126→127→128→129 evidence;85depends81andallfourcomponents, no component depends85/77/42. Original planning record and criterion preserved. | PASS (reused) |

AC3's phrase “shared extension consumes” is interpreted using this task's approved
description, [native refinement](native-plan.md), prior refinement review and the
owner's current explicit instruction: a **real native-core consumer** consumes the
stable tagged payloads/capabilities. It does not mean MSH-77's actual application
runtime/document integration is already implemented. No acceptance wording or
dependency was changed, no final game task was closed and no circular dependency
on85's consumer was introduced. This is acceptance of delivered126–129, not another
planning pass or a false claim of desktop completion.

## Native source assessment against T01–T08

The source and expected assertions were read directly, independently of completion
reports. The [selected profile](profile.md), [fixture corpus](fixtures.json) and
[complete ownership](native-fixture-ownership.json) remain byte-identical.

| Profile / consequential cases | Actual implementation boundary and review conclusion |
| --- | --- |
| T01 setup, side and topology | Native bounded7×7 square, Sente/Gote forward rotation and32-piece standard setup; structural diagrams/imports have explicit provenance, missing history and declared counters. All49coordinates and owner exchanges are tested. |
| T02 movement and handed quails | Private `transition::movement` implements each of nine authored tables. Pheasant/Goose jumps differ from rays; Eagle's length2 diagonal is blocked and captures only at destination. Quail handedness survives capture/drop; changing owner rotates movement, not kind. Geometry, blocking/capture and rotation expectations are independent data. |
| T03 promotion/capture/hands | Native effects derive actual victims, demotion, resource ownership and forced resulting face. Origin OR destination zone triggers Falcon/Swallow promotion; drops do not promote. Drop/wait/within/zone-exit cases confirm no hidden timer or optional nonpromotion. Both owner rotations and exact reversal pass. |
| T04 drops and mate defenses | Only own unpromoted Swallows count toward the two-per-file restriction; enemy Swallow and own Goose do not. Swallow last-rank/direct-check mate refuses, while legal royal/nonroyal captures and escapes defeat the mate prohibition. Defender hands are included. A direct adjacent Swallow check cannot be blocked by a drop, so the finite safety-defense test is sound without recursive drop-mate or history permission. Other-kind mate/last-rank drops remain legal. |
| T05 royal safety/endings | Pinned geometric attackers still defend squares. Self-check and Phoenix capture refuse atomically. Local admission rejects adjacent/nonmoving-attacked royals and dead/file-illegal roots, while moving-side check and terminal roots are allowed. Game installs mate/quiet immobility loss at root or after a board action before repetition. Live resignation works out of turn; agreement/impasse are unsupported. Administrative failure never invents a result. |
| T06 complete histories | Board/owned native faces, both hands and turn form the repetition key. Root is occurrence1/index0; declared counters do not infer history. Latest three indices select actual complete first departures; responsibility is the departure side. Arrival is accepted, forbidden departures refuse without mutation, legal variation consumes obligation, and all-forbidden nonempty local choices lose. Stops do not advance board indices. |
| T07 records/context/reversal | Full root/event semantics plus unique monotonic runtime authority validate commands. Exact undo/redo restores caches and semantic fields while retiring old work; replay reconstructs fresh authority. Historical views/prefix records exclude future events/obligations and do not mutate the live game. Duplicate-preserving visitor/RawValue routing precedes typed replay and final redundant-field equality; unsupported/forged/missing/tampered data reject. Capacity is checked on a fully encoded candidate before installation. |
| T08 handoff/notation boundary | Explicit game/profile/edition/schema/envelope identities and native action coordinates are authoritative. Quail labels and forced faces remain distinct. Human notation, engine/search/score semantics and actual session/process acceptance stay with later named consumers; no9×9 USI/SFEN or Chess Full/Xiangqi exact capability is inferred. |

Tori permits at most one endpoint capture per board move; drops never capture.
This profile has no compound multi-capture route. AC2's relevant compound-action obligation is met by preserving
complete native action identity without inventing a Tori compound rule. Later Chu/
Tenjiku semantics are not generalized from these APIs.

Structural `Position` remains explicitly limited. Geometry/structural mutations
are crate-private; there is no public mutable structural state or pseudo-legal
application escape hatch. `LocalPosition` checks complete board rules but grants
no history/terminal permission. `Game::submit` is the live command boundary; its
position/local getters are immutable. The public compile-fail test and separate
external token/consumer probes support this boundary. Opponent inspection returns
native complete hypothetical actions/effects and never grants live authority.

The full forced8 fixture retains the sole locally legal Phoenix4a→5a defense at
ply8, yet Game concludes Sente repetition-forfeit because that departure repeats.
It is not checkmate or a perpetual-check penalty. In unequal16, the actual prefix10
forbids5g/4f; the latest[4,10,16] later forbids4f/3f and permits5g again. Root indices,
obligations and exact prefix records were examined, not only final diagrams.

Independent expected fixtures establish rules; generator/enumeration consistency,
roundtrip and rotation checks supplement them. The recorded distant-Phoenix perft
counts3/9 and13visited are independently justified; no unreviewed generated count
or external engine agreement was promoted to rule authority.

## Exhaustive remaining-work audit

Fresh native execution: **103fixtures,713executed/713passed/0failed/14pending**.
Owners127=202passes,128=281passes,129=230passes. No in-scope native assertion remains
pending. Four documentary proofs are annotations. The corpus retains its original
declarative metadata; the generic harness's overall completion flag/data-only CLI
are not rewritten to pretend later consumers have passed.

[Complete machine-readable audit](evidence/native-acceptance-audit.json) enumerates
**every one** of14pending assertions and83manifest consumer requirements, with exact
fixture/owner/field or scope. It checks exact live-report equality against the
manifest and prior pending evidence; no entry is dropped, retagged or fabricated.
Separate read-only review examined their semantics, not merely their labels.

| Pending actual assertions | Count | Native part already available / genuine later owner |
| --- | ---: | --- |
| C-IDENTITY: accept_late_A_for_B; undo_back_to_same_diagram_does_not_restore_A |2| Native full context/retirement exists.77 must verify actual purpose/request/session/process/limits/replacement/cancellation acceptance with the real engine owner42. |
| C-NOTATION: display_is_not_action_authority |1| Native complete action and strict profile/coordinates exist.42 must implement/test the actual human notation adapter. |
| O-ADMIN: native_conclusion; record_reason_separately; scored |3| Native Stop tests cover all four reasons, no conclusion, exact replay and unchanged board/occurrences. The fixture explicitly supplies runner_termination;77 must verify actual supervised execution termination/observations. |
| H-REPLAY-UNDO: runtime_work_never_revived |1| Native semantic/runtime separation exists.77 must verify real pending execution cannot revive after undo/reopen/selection. |
| REJECT-DUPLICATE-SQUARE, MISSING-ROYAL, ADJACENT-ROYALS, DEAD-SWALLOW, THREE-SWALLOWS, EXCESS-INVENTORY, STRICT: live_document_unchanged |7| Native rejection/parsing is covered.77 must protect the actual active document/installation path. |

Thus14pending =13owned77 +1owned42. All83separate requirements also remain pending:

- **77entries:**35board/terminal-admission fixture pairs (base plus each exact
  -ROTATED counterpart =70) and seven root/strict rejection fixtures. They require
  actual desktop document/token/execution preservation or retirement, beyond native
  checked action/reversal. Every ID is enumerated in the audit JSON.
- **3entries:**C-HISTORY, H-IDENTITY, H-REPLAY-UNDO — exact native history/prefix/context
  through actual shared contracts, fresh stopped reopening and execution retirement.
- **1entry:**C-IDENTITY — full real execution identity and replacement.
- **1entry:**O-ADMIN — actual supervised cancellation/failure/exhaustion unscored.
- **1entry:**C-NOTATION —42's human notation/display adapter.

Totals77+3+1+1+1=83;82owned77 and1owned42. No native deficiency is dismissed as
future consumer work. An overall “require complete” guard still correctly fails
until those real consumer facets exist. Native acceptance does not pass them.

## Native resource and persistence acceptance

Limits are **524288 encoded native bytes,512commands/events,65536replay units**.
Root charges1; each accepted command charges1, and a board action additionally
compares every prior native key. These are deterministic native replay accounting
units, not CPU time, search visits or a desktop control/observation quota. Fixed
7×7 local legality remains bounded separately. Root counters and runtime authority
have explicit checked overflow errors rather than manufactured no-action outcomes.

`Game::commit` constructs the complete candidate, derives native facts, serializes
its exact native payload and checks bytes/events/work before `*self=candidate`.
Ordinary live work reserves2events,8192B and2units; Stop reserves1event,4096B and1unit;
conclusion can consume the remaining headroom. A conservative constructed maximum
metadata test proves4096B per non-board event, including up to513indices/two complete
departures and changed current nullable fields. It is a bound proof, not a fabricated
legal game or independently expected board position.

The fresh independent run reproduced these actual accepted histories:

| Native history | Board actions | Events | Encoded bytes | Replay units |
| --- | ---: | ---: | ---: | ---: |
| Deterministic real ordinary record |360|360|156573|65341|
| After reserved Stop |360|361|156804|65342|
| After reserved Resign |360|362|157044|65343|
| Ordinary event boundary via Stop/Play |255|510|165461|33151|
| After reserved Stop511/Resign512 |255|512|165930|33153|

The360actions include104captures,103drops,16promotions. The next ordinary move's
replay charge would be65703, beyond65534remaining ordinary ceiling; refusal preserves
exact native bytes/context/caches and stays unscored. The510event path specifically
refuses ordinary work as Events; Stop511 and Resign512 still encode/replay. Exact
policy boundaries separately cover bytes/events/work with zero/one/two reservations.
Actual parser input at512KiB succeeds (valid record plus whitespace); one byte more
refuses before parsing. The byte policy is tested at its exact boundary, while the
attainable long-game admission examples hit work/events first; no claim that a real
512KiB Game was constructed. `u64::MAX` declared counter refuses board commit without
turning legal queries into mate, while serialization and reserved non-board work fit.

The observed local debug run took4726ms for generation plus candidate probes and
2662ms for strict parsing plus two replays; timings are observations, not guarantees.
MSH-129's prior measurements and new review both support the policy. No constant
was raised, data discarded or reopening-as-reset advice introduced. Existing outer
canonical envelope/request/response limits remain unchanged.

These are **native** saveability guarantees. There is no Tori desktop file codec,
atomic file installation or historical observation acceptance yet. MSH-77 must
measure actual outer encoded bytes, events/control journals, replay work and
combined observations before accepting them, preserve256→257refusal and reserve
real stop/close paths under MSH-61. Saving/reopening must not reset structural usage
or resume pending saved observations. A native cap cannot substitute for outer
admission, and native parsing/replay must use the existing bounded worker path
where it could block the application event loop.

## Concrete shared-consumer handoff

| Need | Delivered authoritative API / consumer obligation |
| --- | --- |
| Identity/root/native terminology | native GameKind::Tori, RulesProfile::ToriModernV1, NativeSchema::ToriV1; tori::Side, Face, Square, RootMetadata and Position getters. Preserve seven files/handed faces/Sente-Gote and declared versus known counters. |
| Live actions / effects | Game::legal_actions and context-checked submit; native Play/Resign/Stop/unsupported declaration distinction. LocalPosition::inspect is advisory; never use inspection or local membership alone as live permission. Effects include actual captured face and demoted hand change. |
| History and target | Game::events/snapshot/event_prefix/record_prefix retain full native prefix, not only current board/ply. Use exact selected event history and metadata for observations; no future moves/obligations. |
| Repetition versus saved/execution identity | RepetitionKey excludes metadata/runtime, while GameContext includes complete root/events and fresh runtime. Do not use diagram/key equality for accepting work. |
| Outcome/incomplete | Native Conclusion and StopReason remain distinct from GameError/Resource and engine failures. No evaluation/absence-of-proof result, CSA declaration or Chess draw convention. |
| Reversal / retirement | Native undo/redo is semantic restoration with fresh authority. select_prefix/retire_context retire native requests.77 must additionally retire its actual host request/session/process/selection identities. |
| Canonical record and strict admission | GameRecord from_game/from_json/to_game/to_game_with_undo plus RawValue CanonicalRecord; schema1/modeuntimed and every-prefix replay. Wrong game/profile/schema/edition rejects explicitly. No map ahead of strict native parsing. |
| Resource information | capacity_usage returns actual encoded_bytes/events/replay_work, and native constants/errors expose supported limits.77 measures actual outer candidate, journals/observations/settings and reserved lifecycle space before commit. |
| Execution and capabilities | Existing structured server parses real Tori context but returns unscored UnsupportedMode; execution_capabilities(Tori) is empty.77/42 must add a real engine consumer before advertising play/search; shared tagging alone grants no capability. |
| Presentation / application ownership |77 projects native geometry, piece/ownership/promotion, hands/counts, complete choices and event/outcome context through existing R1–R6 owners.42 supplies actual Tori engine/console/notation/product UI. Keep accepted shell/choice ownership; no frontend movement rules or invented universal board semantics. |

The actual external Rust core probe (not a fake desktop consumer) calls typed
NativeCommandTarget/Command/Reply, uses real native choices/inspection and imported
history records, verifies complete prefix/context/replay, rejects stale work and
wrong game/profile/schema/edition and retains existing Chess/Shogi/Xiangqi behavior.
The native API is sufficient for77 without another native architecture or duplicate
rules. Normal consumer-specific integration remains real work: no completed Tori
session/document/engine/UI is implied, and77 must refine its own bounded scope
before implementation rather than copy this document as a speculative plan.

## Verification, independent method and provenance

Fresh review at558402b, macOS27.0.1 build26A434 arm64, Rust1.99.1, debug Cargo:

- `tori85_kernel_review`, separate read-only source/profile/fixture examination:
  `cargo test --locked -p mshogi-core tori:: --lib -- --nocapture` →38PASS,0failed,
  no Tori ignored;713native assertions and actual long capacity cases above.
  `cargo test --locked -p mshogi-core --test native_tori_envelope --test tori_local`
  →3+1PASS. `cargo test --locked -p mshogi-core --doc` →7normal+1compilefail PASS.
  No native mutation escape hatch, new rule or unresolved finding.
- `tori85_pending_review`, separate read-only complete manifest/assertion/native
  dispatch/consumer audit: actual GameAdapter report713/14/0 and83requirements,
  all103manifest entries; exact live/manifest/past-pending equality.
  `cargo test --locked -p mshogi-core --test native_tori_envelope --test native_envelope --test native_xiangqi_envelope`
  →13PASS (3Tori+10existing compatibility). `cargo test --locked -p mshogi-console --test tori_internal`
  →1actual subprocess PASS, exact context, UnsupportedMode and no move/score/outcome.
- Root integrator inspected actual public/private native API, selected profile and
  component/consumer records; reconciled each original85criterion and all pending
  ownership, source differences, CI provenance and this handoff. Author self-review
  alone is not the independent acceptance evidence above.

The fresh kernel transcript is retained as a clearly labeled transcription of
actual tool output, [not a fabricated test log](evidence/native-acceptance-kernel-review.txt).
One initial invalid invocation mixed --doc and --test; Cargo rejected it before any
test ran. Separate valid commands above then passed. Historical126Windows LF-hash,
unchanged runner,129rustdoc and first Tauri package failures remain in their original
component evidence; no new passing run is represented as a fix for unrelated code.

[Current acceptance evidence](evidence/native-acceptance-audit.json) records exact
source/report/corpus/manifest hashes and all pending entries. Reproduce the actual
report and metadata check without any external game engine:

```sh
MSH_TORI_129_REPORT=/tmp/tori85-report.json cargo test --locked -p mshogi-core tori::game_conformance:: --lib -- --nocapture
python3 docs/m4/tori/evidence/verify-native-acceptance.py /tmp/tori85-report.json
# pinned Backlog1.55.1:
backlog doctor
git diff --check
```

The optional standard-library Python audit checks metadata/ownership, not native
rules; Python or another game engine is not a product or native conformance dependency.
The standalone data-only harness still reports0executed/727pending without a native
adapter. No new expected data, runtime architecture or product implementation.

Existing nativeCI41343787560 and Desktop packages41343787588 are both **completed
SUCCESS** at994521b, confirmed by fresh remote queries. [Actual current CI metadata](evidence/native-acceptance-ci.json)
identifies each run and platform. Previous129Windows package status was honestly
pending at its capture; this acceptance records its later success without rewriting
that evidence. Reuse is valid because the production diff from994521b to current
acceptance is empty. Existing package success does not establish Tori GUI support.
No expensive full workspace/platform/GUI campaign was rerun for documentation-only
acceptance. Two old deep Chess opt-ins are not rerun or claimed passed here.

## Remaining delivery and stopping point

No unresolved required native finding or rules-profile decision remains.85 accepts
the native kernel and handoff only. MSH-77 becomes dependency-unblocked after85,
while its recorded To Do status/refinement remains its owner's next assignment;
no status, criteria, dependencies or plan of77/42 are changed here. MSH-42 still
owns all eight complete-game criteria, including functional engine, console,
notation, desktop Save/Load/play/analysis and final game-delivery assessment;
MSH-51/M5 owns measured strength. Native and shared/game persistence obligations
remain distinct. All six games, accepted shell, MSH-61, R1–R6, completed history
and M4 closure requirements remain intact. Stop after MSH-85.


## Final independent document disposition

Both separate read-only reviewers PASS this acceptance matrix and handoff, with no
unresolved material finding. Kernel review directly challenged rules, expected
fixtures, native mutation boundaries, replay, runtime and measured admission;
pending/handoff review independently reproduced the complete metadata audit and
confirmed all14+83entries. Its audit object equals the stored JSON. The sole minor
wording finding was corrected to make clear that a board move captures at most one
endpoint victim and drops never capture. No rule, fixture, acceptance requirement
or production code changed. Final task completion uses the same original criteria;
MSH-77/42 remain unchanged and no subsequent work starts.

## Recorded integration and finalization

Reviewed acceptance and evidence were integrated and normally pushed to
`codex/rewrite` at `2d4737603bd851534ee24c4b21cd70f4dc3229d2`.
Production remains `994521bec0637d6f8f17103f1106c0025d950753`; no product
source, immutable fixture/profile or other task record changed. All four original
MSH-85 acceptance criteria and four completion items are supported and checked;
MSH-85 is Done. This final documentation/task bookkeeping follows that actual
integration, without inventing a future commit reference.

MSH-77 remains To Do and unassigned: its dependencies are now satisfied, but its
implementation/refinement has not started. MSH-42 and M4 remain open. The primary
checkout stays on `codex/rewrite`; no temporary worktree was created for this
acceptance and unrelated worktrees were preserved.
