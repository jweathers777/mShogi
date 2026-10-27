---
id: MSH-93
title: >-
  Establish complete local xiangqi legal actions and exact transition
  conformance
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-27T18:28:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-91
  - MSH-92
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 93000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Expose the complete local legal action contract using the authoritative native state, including both sides, every piece/geography restriction, captures, cannon screens, facing generals, king safety/pins and all evasions. Preserve mate versus stalemate reason although both no-action states lose under the selected profile.
- [x] #2 Run independently expected fixtures, deterministic legal sequences, applicable established perft/divide diagnostics and admitted-reference complete-set comparisons. Board legality remains separate from history-dependent permission or concluded matches.
- [x] #3 Verify exact apply/undo restoration for complete choices and atomic malformed/stale/illegal rejection. Include cannon-screen removal/insertion and horse-leg evasions rather than relying on chess sliding-check assumptions.
- [x] #4 Independently review discrepancies and conformance coverage against the selected clauses. Record actual commands/source/reference revisions, failures and limitations; no expected-data edits solely to match either engine.
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
Root@codex remains sole native builder/integrator on primary codex/rewrite; prerequisites91/92 integrated/pushed at16042f2 and Done at22c10c1. 1.Add explicit static live-query admission separate from structural diagnostics; implement native geometric threats, pseudo actions, complete legal filtering, local checked/unchecked no-action reasons and checked public resolution/apply with exact undo. Counters/history must not prune board legality. 2.Execute frozen independent105-case corpus, counter and malformed/stale/illegal atomicity, both-side geometry/evasion/capture fixtures; enumerate every choice make/unmake during deterministic sequences and preserve F002 distinct-history seeds. 3.Add bounded developer-only native probe and board-only perft/divide; compare complete sets and observed counts against admitted pinned reference, keeping paper totals2/3/7 distinct from reference observations. 4.Obtain separate completed-boundary review, resolve findings, run focused then workspace fmt/clippy/regressions and actual normal native CI, integrate/push with exact evidence before Done. No94/105 or product consumer implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH91 and92 satisfied with independent review and pushed integration. MSH93 begins only now. Four deliberate General-threat/capture diagrams remain structural-only: exercise explicit live-query rejection, not a silent skip. Preserve foreign reference counter exclusion; raw declared counters remain native.

Completed local-rule implementation and separate-agent boundary review PASS. 16 unit and5 independent fixture/sequence integration tests pass:105 frozen fixtures (101 live,4 explicit structural rejections), all-choice exact reversal,3x64 native sequences, F002 prefixes,8100 public coordinate-pair admissions, stale/counter/domain atomicity and pinned geometric attackers. Workspace fmt/clippy PASS; native workspace430 passed with2 existing opt-in deep Chess tests ignored; desktop-session all-features92 passed. Broad actual native/reference105-fixture/520-prefix comparison PASS; depth3 initial Red/Black79666 are observations, sparse2/3/7 divide5+2 independent. Exact final-source frozen executable rerun/evidence is being retained before integration. Docs: local-legality.md, local-legality-evidence.md, reference/native-*.json, reviews/legality-review.md. Final committed native CI remains required before Done. No later scope implemented.

Final exact-source reference rerun PASS against immutable example SHA256 aced652a098c28abf9d63a1842414a301a9bb45e5aab0f761965c36ab7acd7af. Full report SHA2561802c7daa30b426d88f9b3537a1b74c878f89a6e1ac5a44f128f98217aef4185;105 fixtures/4 explicit reference exclusions/520 prefixes/0 failures. Source hashes unchanged through execution, independently verified in reviews/final-exact-source-review.json. Both opening depth3 observations79666; independent P0017/divide5+2;15 raw developer probe refusal cases PASS. Earlier pre-documentation-build binary/report supersession preserved transparently. Independent completed-boundary verdict PASS, no material finding. Foundations native CI40557737936 passed actual Windows/macOS/Linux at16042f2; final93 committed CI remains pending before closure.

Final source a71d27b1bca2d75d789ead1acc2f8a3132dcdc81 integrated/pushed; all required native CI checks PASS on3 actual systems, run40559507815. Committed source hashes independently match exact frozen executable provenance. No unresolved material review finding. All acceptance and completion items now supported; subsequent source unchanged, closure commit is Backlog-only.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented the complete native local Xiangqi board boundary for mshogi.xiangqi.wxf2018.v1: explicit static root admission, geometric threats, full pseudo/legal actions, General safety, checked/unchecked no-action reasons, native checked resolution/apply and exact reversal, bounded board-only perft/divide and a separate developer comparison probe. No history adjudication or consumer capability was added.

Source integrated and pushed to codex/rewrite at a71d27b1bca2d75d789ead1acc2f8a3132dcdc81. Executed final frozen-binary observations match all nine committed source/build-input hashes. Evidence: docs/m4/xiangqi/local-legality-evidence.md, local-legality.md, reference/native-*.json and reviews/legality-review.md plus final-exact-source-review.json. Independent method: a separate agent reviewed source/official rule pages/fixture coverage, independently ran16 unit+5 integration tests, perft/counter queries and verified all520 exact differential prefixes; final verdict PASS with documentation finding resolved.

All105 frozen independent fixtures pass natively, including4 explicit structural-only live-query rejections;101 admitted cases and520 deterministic reference prefixes agree. Three64-ply native walks probe every generated choice and reverse every prefix. All8100 coordinate-pair admissions, stale values, limits, counter overflow and royal exposure are covered atomically. Independent perft depths1/2/3=2/3/7 and divide5+2 pass; opening depth3=79666 for each side is a differential observation, not independently expected data. Fifteen actual developer-probe failure cases pass. The native raw24-ply check counter remains distinct from foreign adjusted20.

Local workspace fmt/clippy pass; workspace430 passed with2 existing opt-in deep Chess tests ignored, and desktop-session all-features92 passed. Required final native CI run40559507815 succeeded on actual Ubuntu24.04, macOS15 and Windows2022, including fmt/lint/tests/QA constructors/snapshot checks/build/console smoke: https://github.com/jweathers777/mShogi/actions/runs/40559507815 . Foundation source16042f2 also passed three-platform run40557737936. Separately automatic desktop package run40559507879 is still in progress at closure; no new packaged GUI evidence is asserted or required for this native-only slice, and unchanged earlier Chess/Shogi/desktop evidence is reused.

Next dependency-ready native item is MSH-94, still To Do pending its G-CYCLE/G-NOWIN source/fixture gates; MSH-105 owns bounded exact no-win work after94/95. Neither was started. No console, engine, protocol, archive or desktop Xiangqi implementation began. MSH-45 and M4 remain open. Only primary codex/rewrite checkout exists; no temporary task worktree remains. This completion record changes only Backlog evidence after the tested source revision.
<!-- SECTION:FINAL_SUMMARY:END -->
