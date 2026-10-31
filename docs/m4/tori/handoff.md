# Tori native/shared handoff — planning only

Inspected current source at `6a9d24dbff8e883fdc23dabf74216b91cc194ac0`, after R6.
No executable source, format, profile enum, game selector or dependency changed.
MSH-42 remains the complete delivery record; MSH-51/M5 owns measured strength.

## Actual surviving consumers

| Current owner | Concrete Tori requirement / boundary |
| --- | --- |
| `mshogi-core/src/native.rs` GameKind/RulesProfile/CanonicalRecord/NativeSide; strict RawValue native replay | Future explicit Tori dispatch with its selected immutable edition and game-owned parser. Keep left/right quail kinds, own side, hands, promotions, root provenance and full obligations; no Shogi/Chess fallback. Native implementation owns legality and history. |
| `mshogi-engine/src/shared.rs` ExecutionContext/Purpose/Result; current transport4 | Reuse exact purpose/history/request/session/process/limits acceptance and the supervised child. Add a native result specialization only with an actual Tori engine. Board actions, concluded/incomplete/failure stay distinct. No CSA declaration, Chess Full or Xiangqi exact operation inferred from another game. |
| `mshogi-desktop-session/src/lib.rs` one DesktopController/RuntimeMetadata/ActiveDocument | Add one native-tagged Tori document, never another global session/controller. Native command admission and work retirement preserve history browsing, Take back lineage, fresh stopped reopen and stale result safety. |
| `document.rs` current `mshogi.desktop.document.v1`, `persistence.rs` sole installation | Future Tori payload remains game-owned and replay-authoritative. Duplicate-preserving dispatch, exact event prefixes, retained observations and actual-byte/event/control/observation/replay/headroom admission apply before acceptance. Measure real Tori records before choosing capacities or any version change; no format change here. |
| `shared.rs` BoardDescriptor/Orientation/ChoiceOrigin/PresentedChoice/Completion | Existing metadata already supports7×7 square geometry and distinct hand origins. Supply native handed-piece identities and complete move/drop/promotion choices; range/jump paths are native facts, not a TS rules engine or invented compound capture. Resolve P02 before exposing completion alternatives. |
| `shared_inspection.rs`, one `useNativeChoiceController`, ProductApp/ApplicationView | Extend native read-only movement inspection/semantic renderer, preserving the shared shell, supported-only chooser and exact common inspection wording. Full board visible at normal/minimum sizes, native hands/kanji/owner orientation, actual two-column Moves and selected history. No generic layout redesign or per-game frontend owner. |

## Existing task ownership and gates

- **MSH-81** owns source/profile/independent seed resolution. P01–P06 in
  [profile](profile.md) name unfinished obligations. No complete profile is claimed.
- **MSH-85** depends only on81. It owns future native state/actions/reversal,
  legality, history/adjudication and canonical record handoff. Before Ready,
  refine/split useful independent native slices from the *selected* rules,
  preserving independently expected fixtures. Do not begin42's implementation
  or create speculative implementation tickets from unresolved interpretations.
- **MSH-77** retains full dependencies69/72/73/85; shared ownership belongs to46.
  It consumes the real Tori handoff through the current R1–R6 contracts above.
  It must not resurrect obsolete context transports, archive readers, parallel
  frontend/session owners or Standard-Shogi-coded legality in shared code.
- **MSH-42** retains39/72/73/77/81/85 and all eight original criteria. Later
  refinement must visibly cover console/notation, functional search/adapter,
  archives, actual desktop and independent complete-game review;85 alone is
  insufficient. No dependency on another later game's final delivery is added.
- **MSH-88/89/46/47** retain final six-game/platform/integration closure; M4 stays
  open. No task's completion is inferred from this handoff.

## Independent fixture contract

`fixtures.json` is research data, not runnable engine conformance. Supported rows
have direct source locators and independently expected geometry/restrictions;
gated rows state alternatives or required missing evidence. Neither legal-set
counts/perft nor a comparator's output are fabricated. Only a later ratified
kernel may turn reviewed seeds into complete positions and apply/undo/history
fixtures. Include both orientations, captures/demotion, distinct quail hand
identities, jump blockers, complete promotion effects, atomic rejection and
same-diagram/different-history pairs. No external implementation is admitted by
this research; audit a pinned comparator's exact operations/profile first.

First bounded continuation: complete81's targeted P02–P04 source reconciliation
and TORI-P1 owner checkpoint. Only after81 is genuinely Done can85's native
implementation planning become dependency-ready.77 and aggregate42 remain gated.
