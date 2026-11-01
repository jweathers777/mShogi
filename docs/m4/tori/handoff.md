# Modern Tori native/shared handoff — MSH-81

Selected contract `mshogi.tori.modern.v1`, edition2026-11-01; no native Tori
implementation in this planning task. Inspected actual surviving source owners
at `26a2fa9ce8165b6e7ae713d12f16548b65c110da`, whose production code is unchanged
from completed R6. The historical [handoff](archive/2026-10-31/handoff.md) remains
a dated record; its unratified-profile gates are superseded by [T01–T08](profile.md).

## Concrete ownership, scope and observable acceptance

| Owner / actual current consumer | Minimum extension and independently verifiable acceptance |
| --- | --- |
| MSH-85 / `mshogi-core/src/native.rs` GameKind/RulesProfile/CanonicalRecord/NativeSide and game-owned module | Explicit Tori/profile dispatch and native parser;7×7 state, all9faces, quail handed kinds, hands, forced promotion and complete move/drop actions. Native effects/caches undo exactly; original39 seed coverage plus modern corpus implemented. Invalid/malformed/stale/resource requests leave state unchanged. Structural root/counter is not proven reachability. |
| MSH-85 / native history/adjudication/record | Implement T05/T06 including quiet immobility loss, third occurrence/latest-three/departure responsibility, forced-cycle loss, optional native resignation and unscored admin stop. Root terminal assessment and exact event prefixes; replay recomputes every derived obligation/conclusion. Demonstrate complete forced/unequal cycles, same-board/different-history, capture/drop/promotion undo and strict duplicate rejection. |
| Later MSH-42-owned functional engine / `mshogi-engine/src/shared.rs` ExecutionContext/Purpose/Result, transport4 | Real native history-aware evaluator/search with full context and measured bounded work, cancellation, distinct actions/conclusions/incomplete/failure. Existing supervised process and exact purpose/history/request/session/generation/limits acceptance. No Tori capability advertised before consumer exists; no CSA declaration, Chess Full or Xiangqi exact operation inferred. MSH-51 owns measured strength. |
| MSH-77 / `mshogi-desktop-session/src/lib.rs` DesktopController/RuntimeMetadata/ActiveDocument | Add tagged native Tori document in the one runtime/controller. Preserve explicit command admission, retired work, exact historical prefix, read-only analysis, Take back lineage, fresh stopped reopen/explicit Resume. Cross-game cancellation/replacement cannot accept old results. |
| MSH-77 / `document.rs` and sole `persistence.rs::install_document` | Use current document envelope and native strict replay; choose actual schema/version from real payload, not a speculative second save system. Before acceptance measure encoded bytes, native/control events, observations, replay cost and stop/close reservation using long real Tori histories. Boundaries preserve complete Save/Load and no data loss, including256→257 observation rejection; no unjustified constant increase. |
| MSH-77 / `shared.rs` BoardDescriptor/ChoiceOrigin/PresentedChoice/Completion and native inspection | Existing7×7 square geometry/Board/Hand origins already suffice. Supply native Sente/Gote, allfaces/promotions, handed inventory/counts, exact board/drop choices and forced-result completion. No optional promotion when compulsory. Native read-only opponent inspection, no frontend movement rules. |
| Later MSH-42-owned desktop / ProductApp/useGameSession/ApplicationView/shared shell/one useNativeChoiceController | Game-specific Tori board/pieces/hands in existing slots and theme; complete7×7 visible at supported sizes. Supported-only chooser after playable delivery. Native labels, common inspection wording, two-column actual Moves, exact history-event mapping and origin/destination last-move cues. Shared controls retain positions; no second frontend owner or new application architecture. |

No universal rules framework, common board representation, plugin ABI or future
Chu/Tenjiku mechanism is needed. Existing geometry and choice mechanisms are
reused; new native identities and concrete dispatch/capability cases are added
only with actual consumers. No current format/transport change is made by81.

## Preserved task/dependency mapping

- **81** owns the selected complete playing profile and independently expected
  fixtures/source reconciliation. No Kagawa/Okano acquisition or historical
  edition-equivalence gate remains for this modern identity.
- **85**, depends only81, owns the native handoff. Its original AC4 still requires
  useful native slice refinement/splitting **before Ready**. Suggested bounded
  ownership boundaries for that refinement are independent fixtures/comparator
  admission; state/actions/reversal; complete legality; history/adjudication/record.
  These are acceptance groupings, not new tasks or a detailed future implementation
  plan. Preserve native/fixture independence and define actual child dependencies
  when85 is authorized. Do not claim native conformance from81's static checks.
- **77**, depends69/72/73/85, owns the shared real-consumer extension; no dependency
  on aggregate42 or final46. Current R1–R6 owners replace obsolete early bridges.
- **42**, depends39/72/73/77/81/85, retains **all eight** original delivery criteria.
  Later refinement must cover its standalone console/notation, functional native
  engine/supervision/interoperability, canonical replay, desktop play/analysis/
  outcomes/recovery/inspection/saveability and independent complete-game review.
  Explicit current native handoff alone does not deliver those consumers.
- **88/89/46/47** retain all-game/platform/shared/M4 closure; no task closes here.
  **82/83** remain inactive; no dependency chain through other final game deliveries.

First dependency-unblocked native assignment after81 is **MSH-85**, still To Do
pending its original refinement-before-Ready obligation. Do not start it in81.
Provisional Tori→Chu→Tenjiku priority unchanged. M2/M3, completed Chess/Shogi/
Xiangqi and current R6 evidence, MSH-61 capacity guarantee and future M5 ownership
remain intact.
