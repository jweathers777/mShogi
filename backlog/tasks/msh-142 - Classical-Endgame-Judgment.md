---
id: MSH-142
title: Classical Endgame Judgment
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-09T16:23:00Z'
updated_date: '2026-11-09T18:31:00Z'
labels:
  - handle:classical-endgame-judgment
milestone: m-5
dependencies:
  - MSH-141
documentation:
  - docs/m5/chess-strength-assessment-report.md
priority: high
ordinal: 147000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Classical Endgame Judgment. Continue owner-authorized conventional Chess development from MSH-139 (Chess Strength Assessment) and MSH-141 (Chess Frontier Reliability). The assessment exposed a uniquely winning rook-versus-pawn move missed for a draw, optimistic drawn-pawn/rook scores and poor exchanges into pawn endings. Research established specialized endgame evaluation/scaling, implement a coherent useful subset and general independent tests. Native Game remains the sole outcome/history authority; no learned model, external tablebase runtime dependency, rating target or cross-game transfer. Routine local matches share the15minute/task cumulative allowance; substantial studies are reserved for meaningful progress.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Implemented specialized classical endgame knowledge addresses general demonstrated judgment or conversion weaknesses, with documented applicability guards and honest heuristic versus exact distinctions.
- [x] #2 Independent winning/drawing/losing positions, color and side-to-move counterexamples, promotion and history-sensitive guards validate the change; search/restoration/cancellation and native terminal authority are preserved.
- [ ] #3 Focused tactical/endgame and realistic-time comparisons identify gains and regressions without a rating claim; independent review and required native integration evidence support completion.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Root Codex owns clean origin-aligned primary codex/rewrite at d2bfb2b; unrelated historical worktree preserved. Owner explicitly authorizes this task queue. MSH-141 platform-only dependency retry proceeds at frozen22bf634; implementation can progress independently, acceptance still requires its actual macOS receipt. Inspect current static evaluation and preserved endgame failures; research established KPK and material-specific scaling, using native-generated knowledge and independent oracle fixtures with exact versus heuristic applicability stated. Implement compact original endgame knowledge without runtime external engines or replacing native history/adjudication. Verify opposite-turn/color, promotion, fortress counterexamples, search restoration/claims/cancellation and varied quiet-threat/sacrifice/defensive regressions. Compare deterministic and bounded realistic-time positions, review independently, check affected fixture consumers, then required Rust Linux/macOS15/Windows CI at coherent source. No opponent campaign during this increment; retain evidence and integrate normally.

Owner assigned142and143as one queue. Preserve separate implementation commits, local profiles and independent reviews; run one required Rust Linux/macOS15/Windows campaign at the final combined engineering source, exercising both increments.142completion waits on that exact-source integration evidence and141macOS receipt. This reuses unchanged endgame evidence without redundant full-platform campaigns.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Bounded independent read-only source audit during MSH-141 (Chess Frontier Reliability) verification: current evaluator has no side-to-move term, material-specific recognizers or scale factors, so static evaluation cannot distinguish the same-board/opposite-turn MSH-139 opposition pair (draw versus pawn-side win). Established classical references support KPK knowledge and wrong-bishop rook-pawn fortress scaling; KRKP rules are heuristic and must not be claimed to prove the unique Re1 win. Source: official-stockfish/Stockfish sf_11 endgame.cpp and bitbase.cpp. No implementation, runtime tablebase dependency, match campaign or task activation occurred; worker must inspect current source and record the implementation plan on activation.

Implemented original native-generated24KiB KPK advisory bitbase, narrowly guarded wrong-bishop rook-pawn corner scaling and KBNK correct-corner conversion. All3generator/module tests, exhaustive165676valid-state regeneration, independent8KPK oracle byte checks and91release searchlib tests pass. Three bounded native conversion journeys preserve knownwin and reach uncapturable Q/R promotion.4release actual desktop-play tests pass; generated sharedfixtures unchanged; workspace clippy passed. Independent endgame_review approves production/testsource, no material issue. Broader local native tests and sequential timing pending; platform3OS required at final combined142+143source. KRKP and discovered left14noncontact-mate frontier case remain explicitly unresolved for continuedengineering; no opponentgames or ratingclaim.

Final local evidence at50b459a:687native core/search/console tests passed0failed4ignored; finalrelease searchlib91pass2ignored includes safeconversionassertion;4release desktopplay andclippy/fmt passed;sharedfixtures unchanged. Sequential15position1scomparison changes drawnopposition score+260to0, otherdepth/scoresunchanged; noKRKPcorrection/ratingclaim. Source integrated/pushed with combinedf974776; required3OSgate shared from143record, pending.142canonical record docs/m5/evidence/msh142/verification.json.
<!-- SECTION:NOTES:END -->
