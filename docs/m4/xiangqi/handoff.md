# Xiangqi native/shared delivery handoff

Planning only, baseline `14b9643`, MSH-80. All new tickets are **To Do**.
MSH-80 owns ratification/reconciliation; no product implementation is authorized
by this refinement. Existing MSH-45/76/84 identities and original criteria remain.
There is one durable integrator, root@codex, in the primary `codex/rewrite`
checkout. No temporary worktree is required for this documentation-only batch.

## Concrete current consumers and necessary changes

These are inspected code boundaries, not speculative generic API signatures.
The implementation task must inspect then-current code and choose its plan.

| Current boundary | Actual requirement from Xiangqi | Owner |
| --- | --- | --- |
| `crates/mshogi-core/src/native.rs`: `GameKind`, `RulesProfile`, `NativeSchema`, `CanonicalRecord`, strict RawValue-based native dispatch | Add explicit Xiangqi tags only after authoritative native records exist. Keep immutable governing profile, native schema and wire version separate. Preserve duplicate fields into the native parser, reject conflicts rather than fall through. No common board storage or universal rules framework. | MSH-94 native;95 shared routing |
| `crates/mshogi-engine/src/shared.rs`: typed purpose/context/results and capability dispatch | Real Xiangqi child with own evaluator/search, native Red/Black score and selected match policy. Context binds full known prefix/root, exact target, purpose, limits, request/session/process identity. No Shogi declaration or Chess Full operation advertised. Routing first returns explicit unsupported/unscored until the actual consumer exists. |95 routing;97 real engine |
| `crates/mshogi-desktop-session/src/shared.rs`: native session and `BoardDescriptor { files,ranks,label,initial_focus }` | Existing dimensions alone do not encode an intersection board. Add the minimum native geometry/palace/river/piece metadata needed by a real Xiangqi consumer, retain native sides/complete choices and serialize all result acceptance. A board action's local legality is not a history conclusion. |98 under76 |
| `shared_inspection.rs`: immutable exact-token/prefix native inspection | Add a Xiangqi-native read-only movement-inspection adapter; retain historical prefix/provenance and distinct non-actionable markers. No poll, game mutation or frontend movement engine. |98; product101 |
| `shared_archive.rs`: candidate admission, encoded bytes, observations/controls/events/replay work and reserved stop/close capacity | Measure actual Xiangqi payloads including full check/chase/warning histories. Preserve the MSH-61 guarantee before every accepted mutation/observation; Save/Open cannot reset a structural limit. Fresh runtime identities and stopped engine on reopen; historical observations remain historical. Do not raise constants or discard accepted history. |99; native94 |
| `desktop/src/product/ApplicationShell.tsx`: dark common page, document/live/player/board/footer slots and sidebar analysis→history→actions→technical | Extend the game discriminator/content, preserve actual composition, theme, control locations, document vocabulary, engine-limit fields and common status footer. Red/Black content differs; unrelated controls do not migrate. |101/102 |
| `desktop/src/NativeBoard.tsx`: current square-board adapter | Use an appropriate Xiangqi intersection renderer. Render river, palaces and native pieces; do not stretch the chess/shogi square renderer into a rules model. Complete board visible at supported normal/minimum sizes without scrolling/zooming. Existing selection mechanics may be reused only where semantics fit. |98 semantic metadata;101 rendering |
| Console/protocol/runner existing game dispatch | Own console selection/notation and actual child/protocol/match consumer. Pin engine dialects and validate replies against native rules/history; protocol outcomes do not govern the game. |96 console;97 engine;100 protocol/runner |

Limits are not chosen from abstract game count. Current outer native-envelope
limit is1MiB; desktop archive limits include16MiB,256 observations,8192 controls
and16000 replay-work units, with game-specific event caps. These are inspected
baseline facts, not approved Xiangqi capacities.95/99 must account for actual
encoded envelope+payload+diagnostics and test exact boundaries/reserves. An
unsupported or resource-limited operation is explicit and unscored; it cannot
commit a partially saveable game. Bounded queues, process cleanup/cancellation
and retired execution identities remain mandatory through all adapters.

No concrete requirement justifies revisiting Chess or Shogi native architecture
or replacing their canonical records. No trait hierarchy, plugin ABI, universal
side/promotion/hand/declaration/notation representation is proposed.

## Refined executable ownership graph

Arrows in the prerequisite column name actual Backlog dependencies. The existing
aggregate handoffs are completion records, not builder tasks that depend on
their own later consumer. Every future ticket contains observable acceptance
and verification expectations, no speculative implementation plan.

| Ticket | Bounded outcome | Prerequisites |
| --- | --- | --- |
| MSH-80 | Reconciled sources, selected complete named profile, owner decisions and this refinement |62 |
| MSH-91 | Independent board corpus and actually built/admitted restricted reference |80 |
| MSH-92 | Native state, complete move/capture action, reversal and scoped codecs |80 |
| MSH-93 | Complete local legality, sequences/perft/divide and admitted comparisons |91,92 |
| MSH-94 | Complete selected history/adjudication and strict canonical native records, including R-NOWIN |80,93 |
| MSH-95 | Real-record typed shared context/result routing, honest unsupported execution |94,69 |
| MSH-84 (preserved) | Independently verified authoritative native handoff, consumed by95 |80,91–95 |
| MSH-96 | Independently usable console/notation/diagnostics/record adapters |84,95 |
| MSH-97 | Own functional native evaluator/search and structured child consumer |84,95 |
| MSH-98 | Serialized native session, real semantic geometry/pieces/choices and read-only inspection |84,95,97,72 |
| MSH-99 | Complete lossless desktop archive/reopen/admission boundary |98,95,73 |
| MSH-76 (preserved) | Actual common typed/session/presentation/archive extension, not finished UI |69,72,73,84,95,98,99 |
| MSH-100 | External dialect/runner and actual bounded self-play/opponent match evidence |96,97 |
| MSH-101 | Actual untimed human-engine Xiangqi, intersection UI in accepted shell |76,97,96 |
| MSH-102 | Analysis/outcomes/recovery/save/history/live/resume and access journeys |101,99,97 |
| MSH-103 | Fresh independent all-eight-criteria game delivery review |100,102,84,76,96,97 |
| MSH-45 (preserved) | Original complete Xiangqi delivery, original eight criteria unchanged |39,72,73,76,80,84,103 |

MSH-84 does **not** depend on76;95 does **not** depend on84. Thus the native
handoff has an actual typed-record consumer without a cycle through the full
session or game delivery. MSH-76 adds shared95/98/99 but does not duplicate
native94, engine97 or product101/102. MSH-46 remains their aggregate shared
owner; MSH-88 actual all-game three-platform packaged acceptance and MSH-89
fresh shared review remain in the final46/47 chain. There is no new dependency
on Tori, Chu, Tenjiku final delivery or their source research.

## Preserved MSH-45 acceptance mapping

| Original criterion | Required evidence owners |
| --- | --- |
|1 complete sourced immutable profile |80 decisions/reconciliation;94 executable history;103 coverage review |
|2 authoritative native state/actions/undo/history |92 state;93 legality;94 full match semantics;84 handoff |
|3 independent fixtures/reversal/perft/reference/records |91 independently expected corpus/admitted reference;93 local comparisons;94 history/records;103 independent reconciliation |
|4 standalone console |96 actual native console/adapters;100 protocol is additional, not a console substitute |
|5 own functional engine |97 native search/child;100 actual self-play/external smoke;54 retains measured M5 strength |
|6 complete canonical save/replay |94 native semantics;95 common dispatch;99 common desktop format/admission;102 actual journeys |
|7 actual desktop |98 real native session/presentation;99 documents;101 real UI;102 full journeys;76 shared boundary. All-game/platform closure stays with46/88/47, not assumed from local GUI. |
|8 complete ledger and independent review |103 maps all evidence/findings;45 may close only with every original criterion. Shared findings remain46/47 blockers. |

## First bounded implementation queue and present stop

1. Finish **MSH-80**: resolve R-EDITION/R-NOWIN and material XQ-P1–P3,
   independently review resulting profile and fixture expectations. R-CYCLE
   interval/anchor policy and examples must be precise before ratification;
   R-CLASS corpus instantiation continues in its actual fixture/history owner.
2. Only then authorize **MSH-91 + MSH-92**, independent corpus/reference and
   native state/actions/reversal. One integrator; no competing builders.
3. **MSH-93** follows both. **MSH-94** is the next separate history/record batch,
   not hidden inside local legality.95 and later real consumers follow the graph.

At this decision boundary **no Xiangqi product ticket is dependency-ready**.
MSH-80 remains In Review with its incomplete criteria recorded. Independent
source acquisition under81–83 remains possible but does not block80; those
partial passes have their own unresolved texts/interpretations. This planning
batch stops here; it neither begins91 nor closes45,46,47 or M4.
