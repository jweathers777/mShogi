# Tenjiku functional product coverage — MSH-44 working checkpoint

This maps the original eight MSH-44 acceptance criteria to the evidence available
at the functional visual checkpoint. It does not check criteria or declare final
acceptance. The single MSH-44 execution/evidence record is
[product-verification.json](product-verification.json); this document is its
requirements map, not another results ledger.

Production implementation is integrated on `codex/rewrite` at
`aecdccd6152df9cef8736e83ef74d8bc81ae39b2`. The final pushed production checkpoint is
`2c00e2107ac74334bce5d55813e5be25666d7c46`, including the test-only host
assertion correction, profile-specific engine cases and reviewed native effect
descriptions shared by console and accessible Moves history. See the genuine
[macOS gallery](evidence/msh44-macos/gallery.md) and its source/package manifest.
Normal product activation remains off pending the owner's visual/readability
approval. MSH-44 remains unfinished; in particular AC7, AC8 and the definition of
done are not complete.

Both owner-selected whole profiles remain required:
`mshogi.tenjiku.adams1999-modern.v1` and
`mshogi.tenjiku.muller-completed.v1`. A game has one immutable profile; fresh New
may select the other, and Load preserves the saved profile. Neither a live rule
switch nor reinterpretation of a record is part of this checkpoint.

## Original acceptance criteria

| Original criterion | Evidence and implementation available | Completion boundary |
| --- | --- | --- |
| **AC1 — complete sourced, identified, versioned rules** | MSH-83's [profile](profile.md), [machine profile](profile.json), [decisions](decisions.md), [sources](sources.md), [fixtures](fixtures.md) and [source verification](verification.json) ratify both coherent whole profiles. MSH-87's [native handoff](native-v1.md) implements their setup, all movement/promotion/effect categories, refusal rights, repetition and royal/bare endings. | Preserve both profiles and the recorded interpretations. No Chess self-check, invented no-action draw, omitted rare rule or replacement historical profile is introduced by functional search/UI work. Final product acceptance remains separate. |
| **AC2 — one authoritative rules/history owner, exact reversal and separate identities** | Native `tenjiku::{Position,LocalPosition,Game,Action,Effects,GameRecord}` owns legality, full actions, derived phases, history and conclusions. [Native evidence](native-verification.json) and [shared evidence](shared-verification.json) cover atomic refusal, undo/replay and fresh authority. Search reconstructs a private full-history Game; the existing controller/admission worker remains the sole document installer. | Keep frontend, console, engine and training consumers subordinate to native authority. Runtime, saved-state, repetition and search identities retain their distinct contracts. |
| **AC3 — independent expectations and reproducible conformance** | MSH-87 executed 204 distinct native fixture rows and 621 expected fields per profile, plus negative, reversal, sequence, record and transformation checks. Its [verification record](native-verification.json) preserves the independent provenance. MSH-44 adds hand-counted full-Game perft/divide, solved minimax/royal diagrams, profile-sensitive Demon leaves and explicit [profile-play tests](../../../crates/mshogi-engine/tests/tenjiku_profile_play.rs). | Reuse earlier evidence only for unchanged scope. New search/adapter tests establish their respective boundaries; they do not replace rules conformance. External engines remain optional investigations under [conformance policy](../../testing/conformance-policy.md). |
| **AC4 — independently usable console and complete notation** | [Notation/console contract](notation.md), core `tenjiku::notation`, `tenjiku_session.rs` and [CLI tests](../../../crates/mshogi-console/tests/tenjiku_cli.rs) cover explicit whole-profile selection, standard/custom roots, complete ordered travel/returns, igui target, original/promoted identity, promotion choice, exact history/undo/outcomes, canonical Save/Load and read-only analysis. Native full-Game perft/divide preserves history/endings. Human shell and structured `internal-search` remain separate. | Labels are a documented mShogi adapter, not a claimed historical standard or a replacement canonical record. Typed refusal never becomes a partial exact diagnostic or an inferred result. |
| **AC5 — functional native engine, supervised safety and reproducible smoke** | [Engine contract and measurements](engine-v1.md), [search tests](../../../crates/mshogi-search/src/tenjiku/tests.rs), [protocol tests](../../../crates/mshogi-engine/tests/tenjiku_protocol.rs), [routing tests](../../../crates/mshogi-engine/tests/tenjiku_routing.rs) and actual [child/controller journeys](../../../crates/mshogi-console/tests/tenjiku_internal.rs) cover full history/profile, legal complete PV, native terminal scores, zero/partial work, limits, cancellation and stale contexts. [Self-play tests](../../../crates/mshogi-console/tests/tenjiku_selfplay.rs) repeat a bounded four-ply standard-root smoke and an independently solved tactical complete game for both profiles in release. | The standard-root smoke is not a completed standard-root game or strength evidence. Debug functional journeys explicitly use an independent sparse diagram; mandatory release execution uses all 156 initial standard pieces on every CI platform. No test is silently skipped. MSH-53/M5 retains playing-strength ownership. |
| **AC6 — lossless canonical records and replay** | Native schema1 and the existing desktop envelope preserve whole profile/edition, initial state, rights, complete commands/actions/effects, settings, exact selected history, observations and retired Take back branches. [Document execution tests](../../../crates/mshogi-desktop-session/src/tenjiku_document/execution_tests.rs) require an engine observation immediately followed by its exact Play; analysis records no Play. Genuine contexts/PVs are re-admitted and charged before installation. Existing file and controller tests cover atomic failure, fresh stopped Open and prior-file integrity. | Current-format saveability and ending/Close reserves remain mandatory. Unsupported profiles, foreign contexts, forged PV/score and lossy data reject. Common schema migration stays within MSH-46. |
| **AC7 — actual desktop play, inspection, analysis, actions, readable presentation and recovery** | Native-backed 16×16 rendering, [licensed traditional artwork](desktop-art.md), actual engine/analysis capabilities, complete-choice entry and shared document/history controls are implemented. The genuine macOS QA captures linked below exercise real native documents and the bundled engine. They supplement component/native tests. | **Open.** Minimum-size reading comfort and route-badge usability are unresolved; owner visual approval, normal activation and final six-game product acceptance remain outstanding. Current six-job technical CI passed. Fit, font coverage and screenshots alone do not establish usable reading size. |
| **AC8 — complete coverage and independent review, visible integration findings** | This map links the original scope to the three earlier accepted evidence records and the single current [product verification record](product-verification.json). That record identifies bounded independent reviews of search, engine/context/PV, document admission, console/notation and presentation fixes. | **Open.** Final technical CI/gallery receipts and source reviews are reconciled; resolve the owner readability decision, complete authorized activation/acceptance, and keep shared findings visible in MSH-46/MSH-47 and the existing MSH-88/89 campaign. No required behavior is relabeled optional or deferred to strength work. |

Repository links above are paths relative to this document. Detailed commands,
source identities, passed/failed/not-run outcomes and platform receipts belong in
`product-verification.json` and the earlier records, rather than being copied here.

## Genuine macOS functional checkpoint

These are actual captures from the fixed QA application slot, not component
screenshots or a substitute board. Scenario diagrams are explicitly native custom
roots; standard-root captures retain the real 156-piece setup. Neighboring
`.ax.txt` and `-layout.json` files retain accessible UI observations and actual
layout measurements where available. The fixed application identity and scenario
selection procedure follow [packaged GUI verification](../../desktop/packaged-gui-verification.md).

| Requested checkpoint | Reviewable examples and scope |
| --- | --- |
| Both complete profiles, standard density, both orientations, three sizes | Adams [1180](evidence/msh44-macos/adams-standard-1180.png), [960](evidence/msh44-macos/adams-standard-960.png), [760](evidence/msh44-macos/adams-standard-760.png); Muller [1180](evidence/msh44-macos/muller-standard-1180.png), [960](evidence/msh44-macos/muller-standard-960.png), [760](evidence/msh44-macos/muller-standard-760.png). Gote examples: [Adams 1180](evidence/msh44-macos/adams-standard-gote-1180.png), [Muller 760](evidence/msh44-macos/muller-standard-gote-760.png); the gallery contains the remaining recorded size/orientation combinations. |
| All 44 movement faces, all 30 once-promoted original identities, both royals and ownership | Native all-faces custom roots: [Adams 1180](evidence/msh44-macos/adams-all-faces-1180.png), [Adams 760](evidence/msh44-macos/adams-all-faces-760.png), [Muller 1180](evidence/msh44-macos/muller-all-faces-1180.png), [Gote 760](evidence/msh44-macos/muller-all-faces-gote-760.png). Assets, original/face distinctions and non-color promotion marks are documented in [desktop-art.md](desktop-art.md). |
| Genuine human/engine play and read-only analysis | [Adams human/engine](evidence/msh44-macos/adams-human-engine-1180.png), [Muller Gote view](evidence/msh44-macos/muller-human-engine-gote-view-1180.png), [Adams analysis](evidence/msh44-macos/adams-readonly-analysis-1180.png), [Muller analysis](evidence/msh44-macos/muller-readonly-analysis-gote-view-1180.png) and [movement inspection](evidence/msh44-macos/muller-readonly-inspection-gote-view.png). These are bounded actual journeys, not long-game or strength claims. |
| Cancellation, canonical Save/Open, exact history, Take back and dirty Quit | [Cancelled analysis](evidence/msh44-macos/adams-cancelled-analysis-1180.png), [stopped reopen](evidence/msh44-macos/adams-reopened-stopped-1180.png), [history](evidence/msh44-macos/adams-history-1180.png), [Take back](evidence/msh44-macos/adams-takeback-1180.png), [dirty Quit](evidence/msh44-macos/adams-dirty-quit-guard.png) and [fresh Muller after saved Adams](evidence/msh44-macos/fresh-new-muller-after-saved-adams.png). Automated tests separately assert exact state/identity/dirty/file preservation. |
| Complete compound choice and exact Moves geometry | [Partial Lion route](evidence/msh44-macos/adams-compound-partial-1180.png), [keyboard focus at 760](evidence/msh44-macos/adams-compound-keyboard-focus-760.png) and [completed two-capture action](evidence/msh44-macos/adams-compound-completed-1180.png). Partial selection is not a partial committed turn. |
| Stationary capture and actual native conclusion | [Muller igui target](evidence/msh44-macos/muller-igui-target-selection-1180.png), [confirmed complete igui](evidence/msh44-macos/muller-igui-completed-confirmed-1180.png) and [native resignation](evidence/msh44-macos/muller-native-resignation-outcome-1180.png). Stationary target and movement route remain distinct. |
| Fire Demon route, collateral burn and mover destruction | [Adams partial area route](evidence/msh44-macos/adams-area-burn-partial-1180.png) and [burn/death presentation](evidence/msh44-macos/adams-burn-demon-death-1180.png) exist. The paired actual three-stage Adams/Muller interaction is complete: [Muller partial](evidence/msh44-macos/muller-area-burn-partial-1180.png) and [Muller completed](evidence/msh44-macos/muller-burn-demon-death-1180.png) contrast with Adams at identical route8j→7i→7h→8h. Only Adams burns8g before losing its mover8h. [Final accessible Adams](evidence/msh44-macos/adams-burn-accessible-effects-final-1180.ax.txt) and [Muller](evidence/msh44-macos/muller-burn-accessible-effects-final-1180.ax.txt) name the distinct actual native phases. These are imported contract roots, not standard-root continuations. |

## Readability decision remains open

Measured visibility of all 256 targets and a loaded bundled font establishes fit
and asset availability. It does not establish comfortable reading of dense kanji.
The current measurements expose a material minimum-size problem:

| Actual WebView content | Native cell | Minimum engraving em | Recorded case |
| --- | ---: | ---: | --- |
| 760×618 | 16.548 px | 7.777 px | [standard layout](evidence/msh44-macos/adams-standard-760-layout.json) |
| 760×618 | 16.548 px | 5.295 px | [all-faces layout, including 四天王](evidence/msh44-macos/adams-all-faces-760-layout.json) |
| 1180×788 | 23.750 px | 7.600 px | [all-faces layout](evidence/msh44-macos/adams-all-faces-1180-layout.json) |
| 1728×981 | 35.238 px | 11.276 px | [larger native-window all-faces layout](evidence/msh44-macos/adams-all-faces-gote-native-zoom-1728x1013-layout.json) and [capture](evidence/msh44-macos/adams-all-faces-gote-native-zoom-1728x1013.png) |

These are CSS-pixel/em measurements at device-pixel ratio2. Filename/window labels
such as 760×650, 1180×820 and 1728×1013 must not be substituted for the measured
WebView heights above. Earlier source-derived estimates in `desktop-art.md` remain
estimates for their recorded scope; the actual measurements govern this checkpoint.

A concrete proposal for John is a **Tenjiku-only minimum of 960×1000**,
keeping the other five games' minimums unchanged. It is measured in the actual
app but remains unapproved: WebView960×968 gives a598px board/35.595px cells/
11.390px minimum engraving live, and534px/31.786px/10.171px in History in both
orientations. See the [live and history comparison](evidence/msh44-macos/gallery.md#proposed-taller-window-actual-live-and-historical-measurements).
Independent review found the proposal materially clearer; the owner must decide
reading comfort and the resulting minimum-window policy. No minimum, source name, glyph abbreviation,
rule or acceptance criterion has been changed to conceal this issue.

Independent route review also retains a secondary visual concern: completed
route badges use12SVG-unit numerals (about2.85px em at1180,1.99px at760,
3.81px in the proposed taller History view). Complete coordinates remain in
the selected-route footer and completed Moves, with functioning targets/focus/
controls. Functional entry is supported; badge comfort and final artwork approval
are unresolved. The taller minimum alone does not resolve that concern.

## Capacity, CI and the remaining handoff

Native policy remains 1MiB/1024 events/512M reconstruction units. Whole-document
policy remains 160M aggregate work, an independent 15-second worker deadline,
16MiB outer bytes and 256 observations, including actual ending/Close reserves.
The retained128-action measurement admits four genuine-context/PV-bearing
observations at about153M aggregate work and precisely refuses the fifth. The
measurement uses one genuine search per profile with explicitly synthetic nonce
variants for repeated-context growth; it is not four independently executed
searches. Actual worker Open/Save/install and preserved prior-file/ending paths
are recorded separately in `product-verification.json`.

Final-source campaigns are [Rust CI41925013605](https://github.com/jweathers777/mShogi/actions/runs/41925013605)
and [Desktop CI41925013611](https://github.com/jweathers777/mShogi/actions/runs/41925013611)
for2c00e21. All six required jobs actually executed and passed; both workflows completed
successfully. Exact source, attempts, runners, times and required-step receipts
are reconciled in the current product record. Superseded runs and their successes,
failure and cancellations are preserved there. Earlier MSH-79 evidence and
local macOS captures do not substitute for these executions. Rust explicitly
includes release subprocess and full-standard-root tests on every platform.

The six terminal receipts are reconciled in the single product record. The
working technical checkpoint has bounded independent reviews; the final evidence
verdict is recorded there. John must review the genuine visual checkpoint and
readability findings. Subsequent work implements the approved readability/minimum
decision, completes normal activation, and obtains applicable final game/platform
acceptance. Both named whole-profile New choices and normal product Load remain
inactive until owner approval. Original AC7/AC8 and DoD remain open. Existing
MSH-46/47/88/89 integration ownership is preserved; MSH-53/M5 retains strength work.
