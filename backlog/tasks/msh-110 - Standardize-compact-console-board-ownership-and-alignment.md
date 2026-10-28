---
id: MSH-110
title: Standardize compact console board ownership and alignment
status: Done
assignee:
  - '@codex'
created_date: '2026-10-28T09:50:00Z'
updated_date: '2026-10-28T10:01:00Z'
labels:
  - console
  - presentation
milestone: m-4
dependencies:
  - MSH-90
  - MSH-96
ordinal: 110000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner requests consistent Chess-style case-based human console boards for Chess, Standard Shogi and Xiangqi. Previous Shogi colon tokens and Xiangqi owner prefixes unnecessarily expand these small-game boards. Future variants with multi-character abbreviations should retain abbreviation capitalization and use a lowercase native owner prefix without a colon, such as sLg/gLg or sFK/gFK. Record that convention without implementing future variants; preserve completed history, native rules, action notation and canonical records.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Chess retains its case-based board; Standard Shogi uses uppercase Sente/lowercase Gote with +promotion and stable aligned cells; Xiangqi uses uppercase Red/lowercase Black with aligned native coordinates and river. Empty cells remain dots.
- [x] #2 Current help and documentation explain common ownership, Shogi promotion alignment and the future lowercase-owner-prefix-without-colon convention for longer abbreviations, without claiming future game implementation.
- [x] #3 Focused actual-process checks cover all three games, promotions for both owners, alignment and unchanged records/action notation; proportionate independent review, format/lint checks and normal integration/push are recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Change only human Shogi/Xiangqi token formatting: fixed two-character Shogi cells reserve promotion room; one-character Xiangqi tokens use case; Chess byte-compatible.
2. Update current help/docs and recorded owner convention, retaining historical task/evidence records; no future-game renderer/framework.
3. Verify actual console outputs, promotion/coordinate/river alignment and query non-mutation; console regressions, strict fmt/Clippy, independent bounded review, integration/push and relevant native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Primary root@codex on clean sole primary codex/rewrite atb33630348c37298510b761350e391b1f96759c0c. Refreshed origin equal; no active owners. PinnedCLI1.55.1 resolves PATH1.55.0 mismatch. Initial create refused Gitmetadata lock EPERM without creating a task; retry with authorized Gitmetadata access. Future98/100 remain unstarted.

Implemented only human Shogi/Xiangqi show formatting and help/currentdocs. Shogi uppercaseSente/lowercaseGote, +prefix, fixed2-character right-aligned cells/3-character stride; Xiangqi uppercaseRed/lowercaseBlack,1-character cells, native river/coordinates retained. Chesssource/output unchanged. docs/console-presentation.md plus design/currentgame links preserve future lowercase-native-owner+multi-character-abbreviation convention withoutcolon; no futurevariantcode. macOSarm64 Rust1.89: fmtcheck, consolealltargetClippy-Dwarnings, cargo test--locked-pmshogi-console PASS101/0fail, releasebinarybuildPASS; actualrelease show inspectedall3. Existingtestsupdated forowner-selectednewhumancontract, actionlabels and recordquerynonmutation retained. Separate reviewer console_style_review independentlyreviewedsource/docs and executed28focusedsubprocesstests PASS, no materialfindings, diffcheckPASS. Reviewbaseb33630348c37298510b761350e391b1f96759c0c. No native rules/search/record/transport/desktop source changed; reuse MSH97 platform/GUI evidence for unchanged behavior, relevant nativeCI follows sourcepush.

Integrated/pushed testedsource74a63915c1331439b95172fc77500b9d6d27458c on codex/rewrite. Native CI40667848500 PASSmacos-15/windows-2022/ubuntu-24.04 (fmt/lint/fullworkspace/QA/snapshot/build/consolechecks): https://github.com/jweathers777/mShogi/actions/runs/40667848500 . Local101consoletestsPASS, separate reviewer28focusedPASS/no findings, release showoutputs inspectedall3, doctor/diffcheckPASS. AC1 actualcompact/promoted/alignedboards; AC2 currenthelp/docs/designfutureconvention; AC3 nativequery/notation/recordregressions/review/integration. No desktop interaction changed or newlyclaimed; reuse MSH97 desktop CI40661642327 evidence for unchangedUI/records/engine. Automatically launched desktoppackage40667848503 is supporting only, not a requirement for this human-console formatting boundary; its actual state is checked separately, not inferredpassed. No temporaryworktreecreated; primaryrewrite retained. Future98/100unstarted;M4open.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Standardized current human console boards on case-based ownership: Chess unchanged, StandardShogi uppercaseSente/lowercaseGote with +promotion in fixed2-character right-aligned cells, Xiangqi uppercaseRed/lowercaseBlack retaining native river/intersections. Updated currenthelp/docs/design; recorded future native lowercaseowner prefixes withoutcolons for multi-character abbreviations, without implementing futuregames. Verboseactionlabels, rules, history, canonicalrecords, protocols and desktop unchanged; historical90/96/97 evidence preserved. Verified101console tests, strictfmt/Clippy, releasebinary/all3actualshows; separate source/docs reviewer executed28focusedtestsPASS/no materialfindings. Tested/pushed source74a63915c1331439b95172fc77500b9d6d27458c; nativeCI40667848500 all3platformsPASS, doctor/diffcheckPASS. No blockers.98/100 remain next dependency-ready but unstarted;M4staysopen.
<!-- SECTION:FINAL_SUMMARY:END -->
