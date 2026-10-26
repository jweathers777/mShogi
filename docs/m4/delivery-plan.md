# M4 delivery and dependency refinement

MSH-62 is a planning-only batch. No shogi/variant implementation has started.
MSH-41–47 remain the complete-delivery/closure records with all their original
acceptance criteria and DoD unchanged and unchecked. M2/M3, MSH-26, MSH-40 and
MSH-61 remain Done with their evidence. The new tickets are native Backlog.md
records, not a replacement status board; this document explains their contracts
and traces obligations.

Standard shogi is first. The provisional later delivery order is **xiangqi,
tori shogi, chu shogi, tenjiku shogi**. This is priority, not a dependency chain
forcing independent source research to wait for completed earlier games. The
[later source inventory](later-game-sources.md) currently provides no evidence
strong enough to change that order. A future change should cite evidence and
preserve all five delivery commitments.

## Standard shogi components and original acceptance coverage

Each component's task contains scope, observable acceptance, dependencies and
verification expectations. Future tasks have no speculative implementation plan.
The next active worker must inspect the then-current code and record its plan.

| Original MSH-41 criterion | Game-owned bounded tickets | Required shared work |
| --- | --- | --- |
| AC1 — complete sourced profile and consequential interpretations | MSH-63 ratifies SHOGI-P1; MSH-67 implements its history/conclusions; MSH-75 checks completeness | No common rules authority; each implementation retains its own profile. |
| AC2 — authoritative state/actions/reversal/history/match | MSH-65 state/complete move-drop actions and reversal; MSH-66 full legality; MSH-67 native history/adjudication | MSH-69 carries exact native context; MSH-72 serializes session acceptance. |
| AC3 — independent fixtures/properties/differential/records | MSH-64 independent board corpus and admitted reference harness; MSH-66 comparisons; MSH-67 outcome/record corpus; MSH-75 review | MSH-69/72/73 verify boundary/context/compatibility and admission. |
| AC4 — standalone console/notation/diagnostics | MSH-68 shogi shell and adapters | MSH-69 owns common game-selection/routing contract. |
| AC5 — own functional native search/cancellation/interoperability | MSH-70 evaluator/search/structured child; MSH-71 USI and bounded self-play/opponent smoke | MSH-69 common envelope/dispatch; MSH-72 acceptance/execution ownership. MSH-50 retains measured strength. |
| AC6 — canonical game-specific records and strict replay | MSH-67 native records; MSH-68 notation adapters | MSH-73 common desktop format/migration/worker/admission; it does not own shogi rules replay. |
| AC7 — actual desktop play/inspection/analysis/persistence | MSH-74 game-specific native/desktop adaptation and running-app journeys | MSH-72 common sides/board/hand-selection/capabilities; MSH-73 Save/Open/history plumbing; final cross-game three-platform acceptance remains MSH-88 under MSH-46. |
| AC8 — complete ledger and independent game review | MSH-75 traces all eight criteria, findings and exact evidence before MSH-41 closes | Required shared MSH-69/72/73 must also be Done. MSH-46/47 retain final cross-game/M4 closure. |

SHOGI-P1 option A was explicitly approved on 2026-10-26 against integration
`6db8500e92129b95e502e7b4d5c4a26072e38652`; MSH-63 records ratification and review. MSH-64/65 do not depend on
it: independently settled board fixtures and state/reversal work need not wait
for conclusion-policy selection. Neither component alone claims full legal or
match conformance. MSH-66 waits for both. **MSH-67 waits for MSH-63** and full
legality; affected history/outcome behavior cannot be accepted before ratification.

After MSH-67 supplies a concrete canonical native consumer, MSH-69 can establish
the shared tagged envelope. Its completion does not advertise unsupported shogi
search: MSH-70 supplies the actual engine handler. Console MSH-68 can proceed
alongside search, followed by USI MSH-71. MSH-72 then supplies the concrete
shared session/hand-selection/capability boundary, MSH-73 the common archive
extension, and MSH-74 the complete shogi desktop consumer. MSH-75 independently
reconciles the whole delivery. No dependency on MSH-41 final completion appears
in this prerequisite chain.

## Single ownership of shared work

MSH-46 remains the final shared complete-delivery record. It keeps its original
dependencies on all five game deliveries. Those final dependencies do not gate
the separately owned early implementation tickets:

| Shared ticket | Concrete ownership | Direct prerequisite boundary |
| --- | --- | --- |
| MSH-69 | Common typed profile/context/result transport, strict parsing/routing and compatible chess adapter | MSH-67 native shogi canonical context; MSH-62 planning |
| MSH-72 | Native session dispatch and shared side/orientation/board/hand-origin/complete-choice/analysis capability presentation | MSH-69 and actual shogi engine MSH-70 |
| MSH-73 | Common desktop envelope/version compatibility, bounded workers, fresh Open identities, history dispatch and lossless admission | MSH-72 and MSH-69 |
| MSH-76 | Real xiangqi typed context/session/archive extension and intersection presentation | Early shared tickets plus native handoff MSH-84 |
| MSH-77 | Real tori typed context/session/archive and 7×7 hands/presentation extension | Early shared tickets plus native handoff MSH-85 |
| MSH-78 | Real chu context/session/archive, compound routes/effects and multiple-royal presentation extension | Early shared tickets plus native handoff MSH-86 |
| MSH-79 | Real tenjiku context/session/archive and 16×16 derived-effect/capability extension | Early shared tickets, MSH-78 mechanisms and native handoff MSH-87; never completed chu delivery MSH-43 |
| MSH-88 | Final cross-game regression and actual six-game packaged acceptance on Windows/macOS/Linux | All five game deliveries and every required shared extension |
| MSH-89 | Fresh independent shared contract/integration review and reconciliation of MSH-46 | MSH-88 |

The shared owner implements common wire/session/persistence/presentation
mechanisms once. Game owners implement their rules, canonical native replay,
search/evaluation, notation and native presentation adapters. For example,
MSH-73 dispatches replay; MSH-67 defines shogi replay. MSH-72 accepts a complete
hand-origin choice; MSH-66 determines whether that drop is legal. No second
ticket owns the same common mechanism under a game name.

Later extension tickets MSH-76–79 are **To Do**, not permission to implement a
framework from synthetic examples. Before activation they must be refined
against accepted profiles and real authoritative native handoffs. They already
name consumer-specific outcomes and regression expectations. The handoffs
MSH-84–87 are game-owned native scopes, also To Do; each must be split into
useful native slices before Ready if needed. This is deliberate unresolved
refinement, rather than claiming that every later game is implementation-ready.
Neither handoff depends on its aggregate game delivery or a shared extension
that consumes it. This preserves an executable acyclic path as later rules settle.

## Later-game research and complete-delivery gates

| Delivery record | Independent profile/source task | Game-owned native handoff | Required shared extension and completion dependencies |
| --- | --- | --- | --- |
| MSH-45 xiangqi | MSH-80 — reconcile WXF/AXF/CXA editions, translations and check/chase history | MSH-84 | MSH-72/73/76, plus MSH-80/84; existing eight criteria still require its own full engine, console, records, actual desktop and independent review. |
| MSH-42 tori | MSH-81 — acquire/ratify historical profile and independent fixtures | MSH-85 | MSH-72/73/77, plus MSH-81/85; all original delivery criteria remain. |
| MSH-43 chu | MSH-82 — reconcile adopted association/historical lion, promotion and royal conventions | MSH-86 | MSH-72/73/78, plus MSH-82/86; all original delivery criteria remain. |
| MSH-44 tenjiku | MSH-83 — acquire/ratify editions, fire effects, generals, routes and endings | MSH-87 | MSH-72/73/79, plus MSH-83/87; all original delivery criteria remain. |

MSH-80–83 can become dependency-ready after planning without waiting for the
other profiles or a finished shogi game. Their output is a ratified profile and
further executable native refinement, not an assumption that a bibliography
settles the rules. Missing historical texts and material rule choices block the
affected game, not unaffected standard-shogi work. Native handoffs and shared
extension tickets are only part of each later delivery; their original AC4–8
must still supply the rest. Future refinement must add those game-specific
bounded tickets before activating the aggregate, retaining the complete record.

## MSH-46 original criteria remain final acceptance

| Original criterion | Required refined acceptance |
| --- | --- |
| AC1 — all-six shared authoritative contexts/actions/cancellation | MSH-69/72, later MSH-76–79 and actual integrated MSH-88 |
| AC2 — all-six canonical compatibility and old chess regression | MSH-73, later typed extensions and MSH-88; game-specific encodings remain with their game owners |
| AC3 — real geometry/routes/promotions/drops/accessibility | MSH-72, real later extensions MSH-76–79 and MSH-88; synthetic evidence alone is insufficient |
| AC4 — single ownership and actual cross-game flows | This ownership map plus actual MSH-88 coverage; code presence or a task graph does not establish delivery |
| AC5 — three-platform installed packaged applications and engines | MSH-88 actual GUI journeys; build/protocol-only reports are supporting evidence |
| AC6 — independent shared integration review/contracts/limits | MSH-89; unresolved required findings block MSH-46 |

MSH-46 now depends explicitly on MSH-69/72/73, MSH-76–79, MSH-88/89 and its
original MSH-39/41–45. MSH-47 is unchanged: it still requires MSH-26, MSH-39,
all five complete game deliveries and MSH-46, then independently reviews the
whole M4 outcome. No early shared ticket depends on MSH-46, MSH-47 or a final
game delivery it helps supply. Task-count percentages do not change these gates.

## First bounded implementation queue after this planning batch

Historical MSH-62 handoff: the owner discussion has now selected option A.
The authorized next batch is MSH-63–66 only; task records track execution.

1. Resolve **SHOGI-P1 / MSH-63** against the sourced proposal. The explicit selection is recorded in the profile and D-13.
2. MSH-64 independent board corpus/reference admission and MSH-65 native state,
   complete actions/reversal are the first bounded implementation work; they may
   run independently under one integrator. MSH-66 full legality follows both.
3. MSH-67 ratified history/adjudication/records follows MSH-66 and MSH-63.
4. MSH-69 shared envelope, MSH-68 console and MSH-70 native functional search,
   then MSH-71 interoperability, MSH-72/73 shared desktop/storage, MSH-74 actual
   shogi journeys and MSH-75 independent complete-game review.

Independent later-source MSH-80–83 may be scheduled without changing that game
delivery order. M5's MSH-50 owns shogi strength targets; functional M4 smoke,
reference move generation and evaluator correctness are distinct evidence.
