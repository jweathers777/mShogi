# MSH-85 bounded native-kernel refinement

Planning source: `6893f1249dff0414cdbe81ce99fac3a3c0c0e7dc`, primary
`codex/rewrite`,2026-11-01. MSH-81 is Done. Its selected
[`mshogi.tori.modern.v1`](profile.md),103 [fixture specifications](fixtures.json)
and source/read/review evidence are unchanged. This is a planning refinement,
not implementation, executed native conformance or a new rules decision.

## Task ownership and order

Use the project's existing flat component-ID pattern (84/91–94), with explicit
`owner-msh-85` labels/references and aggregate dependencies. These are MSH-85's
four native components, not competing final-delivery records. No child depends
on its aggregate85, shared consumer77 or final42.

| Task | Responsibility and independently completable boundary | Complete direct prerequisites |
| --- | --- | --- |
| **MSH-126** | Self-contained executable fixture loader/assertion adapters, independently expected data, diagnostics and property/metamorphic interfaces; no external engine required. No product Tori kernel. |81 |
| **MSH-127** | Game-owned7×7 state/root/side/pieces/hands, complete actions, native movement/capture/drop/promotion effects and exact structural reversal. Helpers private/clearly limited; not full local legality. |126 |
| **MSH-128** | Complete local legal choices/apply, royal safety and root admission, Swallow restrictions/drop-mate, check/mate/quiet status, immutable native inspection and conformance/perft diagnostics. No repetition/game-command permission. |127 |
| **MSH-129** | Full native Game/context/lineage, repetition/forced cycles/endings, command and record replay, bounded saveable native admission and core tagged dispatch/consumer probe. |128 |
| **MSH-85** | Coordinating final native-handoff acceptance, original AC1–3 and DoD after actual components/evidence; AC4 is this reviewed refinement. |81,126,127,128,129 |
| **MSH-77** | Real shared execution/session/document/presentation integration consuming85, retaining original scope/dependencies. |69,72,73,85 |
| **MSH-42** | Complete Tori console, functional engine/search/supervision, notation, desktop, persistence and independent game delivery; MSH-51/M5 owns measured strength. |39,72,73,77,81,85 |

126 is Done ([harness evidence](harness-evidence.md));127 is Done
([structural evidence](native-structural-evidence.md));128 is Done
([local legality and inspection evidence](native-local-evidence.md)).129 is the first
dependency-satisfied Ready slice.85 is scoped Ready and blocked on129,
**not Done**; only original AC4 may be checked by this planning pass. No later
implementation/consumer task is activated by this handoff update. Serial order keeps the independent corpus/harness
contract settled before transition testing, transitions before full legality, and
complete local choices before repetition/record replay. Future bounded execution
can use one integrator without competing owners of the same kernel boundary.

## Actual architecture inspected and smallest native extension

At the planning source, `crates/mshogi-core/src/lib.rs` exported only
chess/shogi/xiangqi and native dispatch. MSH-127 now also exports the structural
`tori` module, while native game dispatch remains unchanged. Shogi's `action.rs` makes structural application private; `legal.rs`
exports complete board legality/inspection and separate repetition identity.
Xiangqi's `legal.rs` separates local root/status from `game.rs` admission/history/
monotonic runtime token and `record.rs` strict replay. Reuse these **ownership
boundaries**, not their piece rules, counter algorithms, repetition or notation.

| Boundary / surviving owner | Concrete Tori deliverable / separation |
| --- | --- |
| Game-owned core module (127/128/129) | Explicit native Sente/Gote,7×7 squares, Ph/Fa/Ea/Cr/Pt/LQ/RQ/Sw/Go, handed hand kinds, forced promotion, root provenance/counter. Native move/drop choices and ordered derived effects. Safe Rust representation remains the implementer's choice. No universal board/rules DSL. |
| Position/action layer127 | Exact structural snapshot/effects/reversal and limited queries. A structurally admissible diagram is not a proven reachable game or full legal root. Private intermediate helpers cannot become desktop admission. |
| Local layer128 | Complete safety/drop-legal choices independent of history; immutable owner-hypothetical movement inspection, including hands. Native geometry supplies effects/inspection facts. Local mate/quiet no-action is an observation;129 installs the actual conclusion. |
| Native Game129 | Full root/known events, board-ply/event-prefix mapping, T06 occurrence/departure obligations and conclusion; local actions can be history-forbidden. Native exact context plus never-rewound lineage prevents stale command/undo revival. Out-of-turn resignation stays a terminal command, not an ordinary board action. |
| Record/identity/dispatch129 | Native explicit format/schema/profile/edition and authoritative prefix replay. Extend `native.rs` GameKind/RulesProfile/NativeSchema/NativeSide/CanonicalRecord and native command dispatch with actual game-owned payloads. Preserve RawValue nested bytes and strict unknown/duplicate/conflict rejection. A core integration probe consumes real native records/choices/history; empty exports or a fake desktop consumer are insufficient. |
| Current shared consumers, later77/42 | `mshogi-engine/src/shared.rs` owns full purpose/request/session/process/limits execution. `DesktopController`/`ActiveDocument` own live document/runtime; `document.rs` and sole `persistence.rs::install_document` own current archive installation; native inspection adapter and one frontend/choice controller remain. Do not implement them in127–129. |

Adding a GameKind variant can require exhaustive arms outside core.129 owns only
necessary compile-preserving typed routing or explicit unsupported branches, with
focused existing-game regressions. No Tori engine capability, New Game option,
DesktopController variant/session, real document codec, frontend or transport
redesign is authorized by those compile adaptations. Actual Tori execution needs
the later real MSH-42 engine; unsupported/incomplete/failure stays unscored.77
owns consumer acceptance and integration of the real native handoff once85 closes.

A complete action may encode its forced result face or derive it from native
state; either must reject contradictory external faces and preserve exact native
action equality for repetition/records. Bounded action/record syntax is selected
from the actual native consumers, not borrowed9×9 SFEN or USI. Human display
notation is later42, not replay authority. No optional promotion alternative or
CSA declaration is invented for reuse.

## Fixture ownership — all103, unchanged expectations

[Machine-readable mapping](native-fixture-ownership.json) enumerates **every ID**,
its original expected fields, corpus owner, primary native owner, supplementary
native checks and later actual consumer facets. Corpus SHA256 remains
`3f9e5ca3b541fc72f562ddd8231408e17e7bd5ba3ccee118e643ce0f543731e3`.

- **126 owns all103 as independent test inputs**, strict registration/adapter
  requirements and useful mismatches. It can complete without an unimplemented
  kernel: execute loader/constructed-observation sentinels and deliberate harness
  failures, report native adapters pending127–129. It must not fabricate103
  passing native tests. Later tasks connect real native execution to the same data.
- Primary native verification: **127=14**, **128=69**, **129=20**. These identify
  the earliest full native boundary for the row, not disjoint permission to ignore
  assertions owned by other layers. Local admission rows also exercise127 effects;
  all42 accepted transition rows require129 full-game reversal as applicable.
- Structural/root versus local-safety checks are explicit. All9 geometry tables,
  both owner rotations and accepted captures/drop/promotion reversal flow through
  the public local boundary before complete native conformance is claimed.
- T06 units with supplied choices are history-algorithm tests, not fabricated
  complete positions; full forced8-ply and unequal16-ply sequences separately test
  integration. Same board/different history and exact undo/prefix replay belong129.
- Cross-layer rows remain honest: `TORI-C-IDENTITY` gives process/session/purpose/
  limits examples.129 verifies real native history/context/lineage projection;
  **77 and the later real42 engine verify full execution acceptance**.129 must
  not add fake process objects merely to report a kernel pass. Similarly actual
  desktop document preservation/reopen/observation quotas belong77, while native
  rejection/replay is129. Actual human notation adapter belongs42. The manifest
  retains these consumer obligations rather than marking them passed in85.

126's harness reports field/boundary-level executed, passed, pending and failed
assertions with documentary annotations and later consumer requirements, with explicit owners; no ignored/stub native assertion may support85Done.
85 completion does not wait on its own consumer77: a real **core** consumer probe
verifies the stable native handoff; actual shared/product acceptance stays77/42.
This preserves original85AC3 without a circular dependency or pretend desktop.

## Independent testing and optional investigations — owner correction2026-11-01

MSH-126 is a self-contained executable harness. The prior mandatory Fairy build/
admission requirement is withdrawn. All native tasks require independent rules/
history examples, deterministic/property/metamorphic/adversarial tests, exact
reversal/replay and review under [project policy](../../testing/conformance-policy.md).
No engine comparison is a completion or CI gate. The [historical source audit](comparator-audit.md)
is retained as optional nonauthoritative investigation only. Perft/divide remains
local; expected counts require independent establishment, never an upstream or
mShogi-generated number alone. No profile or fixture expectation changes.

## Resource/admission and remaining delivery

129 measures real encoded native bytes, accepted events/commands, replay work and
bounded diagnostics before fixing native limits. Admission checks candidates
before committing; reserve space/work for supported concluding/stop operations.
Long accepted histories must still serialize losslessly. Overflow/capacity must
not look like illegal chess/shogi action, zero legal moves, mate or engine failure.
77 later measures actual outer document, observations, controls, replay and close/
stop headroom, including256→257 combined observation refusal and atomic Save/Load.
No native fixture may force77 to inherit an arbitrary unmeasured constant.

MSH-42's original eight criteria remain intact: this kernel does not deliver its
console, functional engine, interoperability/supervision, product UI, save/reopen
or independent whole-game review. M2/M3 and all accepted Chess/Shogi/Xiangqi/R6
source/evidence stay unchanged. No historical acquisition gate, optional exact
mating graph analysis, future variant, M5 optimization or shared refactor is added.
There is no known rules or architectural blocker to126; actual harness build/data-admission
failures must be reported by its implementer, never assumed passed by this plan.

## Planning verification and disposition

[Independent review and actual checks](native-plan-review.md) pass this refinement;
[static evidence](evidence/native-plan-validation.json) verifies the complete
fixture assignment, preserved criteria and acyclic dependency sets. No kernel
implementation or product verification is claimed. Final integration references
are recorded in MSH-85 after normal commit/push.

## Completed MSH-126 handoff

The owner superseded mandatory comparator admission. [Executable harness](harness.md)
provides independent fixture admission, structured observations, staged native
assertions and property/metamorphic interfaces without any external engine. All
103 expected specifications/profile/ownership remain unchanged.126 completion
evidence is separate from the historical planning review and from pending native
127–129 implementation; see [evidence](harness-evidence.md).


## Completed MSH-127 handoff

Native `mshogi-core::tori` structural state/actions/effects and private exact
apply/undo/redo are integrated at ce1801d7801f12e447b625739acd6a0c53aa2640;
[execution, review and three-platform CI evidence](native-structural-evidence.md).
All202127assertions pass across54rows, including all14primary rows;525assertions
and83consumer requirements remain pending. The original42accepted specifications
contain40board transitions and2resignations; the latter stay129. All profile,
fixture and ownership bytes remain unchanged.128 must implement complete local
legality/safety/drop rules/status/inspection and rerun structural evidence through
that public legal boundary.129 retains history/outcomes/strict records/replay/
native dispatch;85 remains coordinating acceptance,77/42 remain later consumers.
No future implementation plan or dependency was changed, and no next task started.


## Completed MSH-128 handoff

Source5c60ac65dc03a7f9afcaa796a2babb252329d361 adds the complete checked local
LocalPosition API and immutable native inspection. [Evidence](native-local-evidence.md)
records all281128assertions plus202127passes, independent rules/property/adversarial
review and actual nativeCI PASS on Windows/macOS/Linux. Structural helpers stay
private; existing game consumers and profile/fixture/ownership bytes are unchanged.
129 consumes local status/complete actions/effects/reversal and retains all T06
history/departure obligations, commands/conclusions, strict records/replay and core
dispatch/capacity measurement. It must preserve exact local inspection on historical
prefixes and separate full-game permission from board membership. No future
implementation plan, dependency, ownership or status was changed by this handoff.


## MSH-129 native Game handoff

The [native game evidence](native-game-evidence.md) records the actual authoritative
T05/T06 Game, exact full context/retirement/reversal, immutable history and complete
prefix records, strict native schema1 replay and typed core dispatch. All713native
assertions execute and pass (202127+281128+230129),14genuinely later assertions and
83real-consumer requirements remain pending. Selected profile/corpus/ownership are
unchanged. Native bounds are measured separately from later desktop quotas;
ordinary admission preserves complete encoding and reserved Stop/Resign headroom.
Tori execution remains explicitly unsupported/unscored and unadvertised; no Tori
session, engine, console, desktop codec/UI or chooser is supplied.85 retains separate
native acceptance;77/42 remain real shared/product consumers. Earlier handoff sections
above are historical checkpoints, superseded only as to delivered native129 behavior.
No later task is started or closed by this evidence update.
