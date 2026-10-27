---
id: MSH-93
title: >-
  Establish complete local xiangqi legal actions and exact transition
  conformance
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-27T18:23:00Z'
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
- [ ] #1 Expose the complete local legal action contract using the authoritative native state, including both sides, every piece/geography restriction, captures, cannon screens, facing generals, king safety/pins and all evasions. Preserve mate versus stalemate reason although both no-action states lose under the selected profile.
- [ ] #2 Run independently expected fixtures, deterministic legal sequences, applicable established perft/divide diagnostics and admitted-reference complete-set comparisons. Board legality remains separate from history-dependent permission or concluded matches.
- [ ] #3 Verify exact apply/undo restoration for complete choices and atomic malformed/stale/illegal rejection. Include cannon-screen removal/insertion and horse-leg evasions rather than relying on chess sliding-check assumptions.
- [ ] #4 Independently review discrepancies and conformance coverage against the selected clauses. Record actual commands/source/reference revisions, failures and limitations; no expected-data edits solely to match either engine.
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
Root@codex remains sole native builder/integrator on primary codex/rewrite; prerequisites91/92 integrated/pushed at16042f2 and Done at22c10c1. 1.Add explicit static live-query admission separate from structural diagnostics; implement native geometric threats, pseudo actions, complete legal filtering, local checked/unchecked no-action reasons and checked public resolution/apply with exact undo. Counters/history must not prune board legality. 2.Execute frozen independent105-case corpus, counter and malformed/stale/illegal atomicity, both-side geometry/evasion/capture fixtures; enumerate every choice make/unmake during deterministic sequences and preserve F002 distinct-history seeds. 3.Add bounded developer-only native probe and board-only perft/divide; compare complete sets and observed counts against admitted pinned reference, keeping paper totals2/3/7 distinct from reference observations. 4.Obtain separate completed-boundary review, resolve findings, run focused then workspace fmt/clippy/regressions and actual normal native CI, integrate/push with exact evidence before Done. No94/105 or product consumer implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH91 and92 satisfied with independent review and pushed integration. MSH93 begins only now. Four deliberate General-threat/capture diagrams remain structural-only: exercise explicit live-query rejection, not a silent skip. Preserve foreign reference counter exclusion; raw declared counters remain native.

Completed local-rule implementation and separate-agent boundary review PASS. 16 unit and5 independent fixture/sequence integration tests pass:105 frozen fixtures (101 live,4 explicit structural rejections), all-choice exact reversal,3x64 native sequences, F002 prefixes,8100 public coordinate-pair admissions, stale/counter/domain atomicity and pinned geometric attackers. Workspace fmt/clippy PASS; native workspace430 passed with2 existing opt-in deep Chess tests ignored; desktop-session all-features92 passed. Broad actual native/reference105-fixture/520-prefix comparison PASS; depth3 initial Red/Black79666 are observations, sparse2/3/7 divide5+2 independent. Exact final-source frozen executable rerun/evidence is being retained before integration. Docs: local-legality.md, local-legality-evidence.md, reference/native-*.json, reviews/legality-review.md. Final committed native CI remains required before Done. No later scope implemented.

Final exact-source reference rerun PASS against immutable example SHA256 aced652a098c28abf9d63a1842414a301a9bb45e5aab0f761965c36ab7acd7af. Full report SHA2561802c7daa30b426d88f9b3537a1b74c878f89a6e1ac5a44f128f98217aef4185;105 fixtures/4 explicit reference exclusions/520 prefixes/0 failures. Source hashes unchanged through execution, independently verified in reviews/final-exact-source-review.json. Both opening depth3 observations79666; independent P0017/divide5+2;15 raw developer probe refusal cases PASS. Earlier pre-documentation-build binary/report supersession preserved transparently. Independent completed-boundary verdict PASS, no material finding. Foundations native CI40557737936 passed actual Windows/macOS/Linux at16042f2; final93 committed CI remains pending before closure.
<!-- SECTION:NOTES:END -->
