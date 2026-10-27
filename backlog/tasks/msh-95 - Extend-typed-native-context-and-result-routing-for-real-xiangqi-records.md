---
id: MSH-95
title: Extend typed native context and result routing for real xiangqi records
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-27T20:48:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-94
  - MSH-69
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/shared-routing.md
  - docs/m4/xiangqi/routing-evidence.md
ordinal: 95000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Route real authoritative xiangqi canonical payloads through explicit game/profile/edition/native-schema/transport identities with RawValue-preserving strict outer/nested parsing. Reject duplicate/unknown/missing/conflicting versions without fallthrough; existing chess/shogi and historical records retain semantics.
- [x] #2 Bind request acceptance to purpose, complete known history/root, target position, limits, request/session/process identity; historical selection and semantic undo cannot revive retired work. Preserve native side/score/operation meanings and distinct action/claim/conclusion/incomplete/failure results.
- [x] #3 Exercise real native records including same board/different history and non-ply terminal events. Unsupported xiangqi execution stays explicit/unscored and unadvertised until the real engine consumer lands; no borrowed Chess Full operation.
- [x] #4 Preserve bounded actual encoded envelopes/diagnostics/queues and supervised cancel/cleanup. Run strict-parsing, race/replacement/cancellation and existing actual chess/shogi subprocess regressions; record independent boundary review.
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
Root@codex is the sole builder/integrator in primary codex/rewrite at0a90ec8e8669b5dff70f37b8ceaec48d1702be1b; only our preserved scope-conflict notes are dirty, no other active worker/worktree.1.Add explicit Xiangqi game/profile/edition/schema/side and authoritative canonical-record routing to existing strict RawValue envelope; expose only small replay-validated native metadata needed by current consumers, and borrow native Game for command admission without rewriting rules.2.Extend v3 context/target validation and native command dispatch while keeping Xiangqi capabilities empty and all execution requests explicit unscored UnsupportedMode; preserve native claims, warning/audit/conclusion semantics and G-NOWIN values without adding evaluator/search/verifier. Keep wire versions and Chess/Shogi payloads compatible.3.Make only compilation/explicit-rejection adaptations in existing consumers; no Xiangqi desktop/session/archive feature. Retain one supervised execution owner, exact echo matching, replacement/cancel cleanup and monotonic retirement across games; measure full encoded payload/envelope/request/response, keep current caps unless concrete evidence requires otherwise.4.Focused real-record strict parsing/identity/history/same-ply command, purpose/limits/request/session/process/selection and cross-game cancellation/late-result tests plus actual Chess/Shogi and unsupported-Xiangqi child entry-point evidence.5.Separate completed-boundary review, resolve findings, relevant native/frontend regressions and normal CI, coherent normal integration/push and evidence; stop after95, leaving105 now dependency-ready only after95 Done and preserving97/84/105 ownership/dependencies.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup for owner MSH95-only request at clean primary codex/rewrite0a90ec8e8669b5dff70f37b8ceaec48d1702be1b; dependencies94/69 Done and no active worker ownership. PATH Backlog1.55.0 kept read-only; pinned1.55.1 matches baseline. Material request/task scope conflict presented before coding: attached request describes evaluator/search and optional real child, but existing95 AC3 requires execution unsupported/unadvertised until real consumer97;97 owns evaluator/search/child and depends84/95, with84 requiring105. Owner asked whether to preserve95 routing-only or explicitly revise affected ownership/graph. No implementation, status, acceptance criterion or dependency changed pending resolution. Read-only consumer audit and baseline boundary checks continue.

Read-only baseline verification at0a90ec8: cargo test --locked -p mshogi-core --test native_envelope PASS5; cargo test --locked -p mshogi-engine --test shared PASS6. Current strict Chess/Shogi routing, native command ownership, real execution, context matching and composed-size baseline all pass. No source edit or implementation plan accepted while the material95-versus97 scope question remains pending.

Owner explicitly resolved the scope conflict: preserve existing repository95routing-only ownership and graph, keep the conflict note as evidence, do not add evaluator/search/engine-turn/analysis execution or start105.95current4AC/DoD govern. MSH94/69 dependencies Done. After95 closure105 becomes dependency-ready;97 remains after required native handoff84 including105. No ownership/dependency changes authorized or made. Initial audit: native envelope/side/command variants currently Chess/Shogi, strict RawValue parsing already preserved. Xiangqi native record is validated immutable JSON but lacks cheap shared event/ply/turn accessors; add only authoritative cached derived metadata to avoid repeated replay during host context validation. Existing consumer exhaustive matches require explicit unsupported compatibility arms, not a new product adapter.

Routing-only implementation complete for review. Native Xiangqi identities/envelope/schema/qualified sides and borrowed Game command dispatch preserve native Submission result+undo, complete warnings/claims/audits/root/provenance and assessment modes. Cached derived event/action/turn getters add no native wire field or rule change. Shared3/legacy2/envelope1 remain compatible. Xiangqi capabilities empty; all valid purpose shapes echo exact context with unscored UnsupportedMode, malformed target/identity staysInvalidRequest; no evaluator/search/verifier or later consumer added. Actual target-side checked and ordinary-limit dispatch exhaustive; unsupported u8/u64 settings are opaque, not borrowed search limits. Minimal desktop enum compatibility plus explicitNew/Replace refusal preserves current document/work.

Separate reviewer routing95_review completed independent bounded source/test/documentary review and actually ran45focused tests, PASS, no material unresolved finding. Reports/docs: docs/m4/xiangqi/shared-routing.md, routing-evidence.md, routing-reviews/{initial-audit,completed-boundary-review,process-verification}. Isolated console test delegate is distinct from reviewer. Local finalfmt/workspaceclippy pass; workspace478pass0fail2existingignored; desktopallfeatures93pass; nativefrontendfixture regenunchanged; pinnedNode24.19.0 strictTS and21files161frontendtests pass; workspacebuild pass. SystemNode26 failedmissingdylib; never counted asprojectcheck, recoveredusingexistingpinnedruntime. Initialtest API/lint issues corrected, nofailedcheckcountedaspassed. Native512command/1024event fixture measured253656native/253832envelope/253999request/254281responsebytes; exact1MiB/+1checks pass and no constants increased. Strictreverseparser assertions independently rerunPASS. Normal source integration/push/CI still pending; do not markDoneuntil recorded.

Reviewed production source6412f483bd3ccdd1e27499cf05cdc47ff2723467 normally committed/pushed to codex/rewrite; native CI40577559325 and packageCI40577559291 running. No105/97 dependency/ownership/source changes. During CI audit discovered older baseline0a90ec8 package run40571798791 failed Linux verification-promotion with Remote end closed connection without response; Windows/macOSpassed, exactcauseunestablished. This is retained as failed baseline automation evidence, not mislabeled passed or attributed to new routing. Current integrated normal workflows must establish compatibility; do not substitute the old successfula822450run for them.

Final type audit correction: the existing shared WorkLimits.depth is u16 (unchanged), not u8 as mistakenly described in initial comments/docs/review. Corrected comment/docs; strengthened test255 to actualu16::MAX65535. No wire/runtimepolicy/capability changed. Root and separate reviewer independently reran engine routing5PASS at actual upper representation bound; measured native253656/envelope253832/request254001/response254283bytes. Earlier253999/254281 figures were validdepth255 measurements, now superseded for the maximum-depth test. Reviewer explicitly preserves/corrects missedu8endorsement, updates shared.rs hash and PASS verdict without inflating45distincttests. No change tograph/105/97 or existing gamebehavior. Also observed external /private/tmp/mshogi-architecture-roadmap worktree on codex/architectural-health-roadmap, not created/owned by this task; preserved untouched.

Final reviewed follow-up integrated/pushed69838067abfdd98771d5b1fdd19984d18fb15b55 (comment/docs and stronger actualu16-upper-bound test; no runtime/wire change). Final native CI40578289309 and packageCI40578289335 running. Initial implementation6412f48 native CI40577559325 completedPASSall3; not substituted for finalrevision checks. Production source hashes remain reviewed, with shared.rs commenthash explicitly superseded in final review. This batch still stops95;105/97 untouched.

Final69838067 native CI40578289309 PASSall3: actual logged workspace478macOS/Linux,475Windows,0failed2existingignored each; isolateddesktopQA93each plusfmt/lint/build/nativefixturecompatibility/consolesmokePASS. FinalpackageCI40578289335 LinuxactualinstalledGUIandmacOSpackagechecksPASS, Windowspending. Earlier6412packageCI40577559291 failed Linuxnormal-shogi-reopened with Remote end closed connection without response whileWindows/macOSpassed; this differs from baseline0a90verification-promotiondisconnect. Exactrootcauseunestablished; preservebothfailedruns asintermittentdriver-connection evidence rather than asserting sourcecause. CurrentfinalLinuxcompleteGUIpassed without harnesschange, no acceptancecriteriaweakened. AwaitfinalWindowsbeforeDone.

Final source 69838067abfdd98771d5b1fdd19984d18fb15b55 is normally integrated and pushed to codex/rewrite. Required native CI https://github.com/jweathers777/mShogi/actions/runs/40578289309 completed SUCCESS on macos-15, windows-2022 and ubuntu-24.04: actual workspace 478 passed on macOS/Linux, 475 passed on Windows, zero failures and two existing intentionally ignored deep-perft tests per platform; desktop-session all-features QA 93 passed on each. Format, workspace/all-feature clippy, native frontend fixture compatibility, build and console smoke passed.

Final package CI https://github.com/jweathers777/mShogi/actions/runs/40578289335 completed SUCCESS on all three target systems. It establishes locked frontend/native checks, normal package inspection/bundled-engine checks and separate verification-package identity; Linux and Windows also executed the existing actual installed/packaged Chess/Shogi GUI journeys. macOS is package/engine evidence, not a new interactive macOS GUI campaign. Earlier accepted macOS GUI/native termination/Tao provenance is reused unchanged. No Xiangqi execution or desktop capability is implied. Prior baseline0a90 and initial6412 Linux driver-disconnect failures remain recorded separately, exact cause unestablished; the final full workflow passed without harness changes or weakened criteria. No failed attempt is relabeled as passed.

All4AC and4DoD mapped to shared-routing.md, routing-evidence.md, independent45-test review and final CI. Native payload semantics and all task ownership/dependencies remain unchanged. The preserved request-scope conflict and owner correction prevented evaluator/search drift into95. MSH105 is now dependency-ready after95 completion but remains To Do/unassigned/unstarted; MSH84 still waits105, then96/97 are eligible under their existing graph. MSH45 and M4 remain open. No temporary worktree was created for95; the external architecture-roadmap worktree belongs to other ongoing work and is untouched.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered routing-only MSH-95 under its original acceptance criteria. Explicit Xiangqi game/profile/edition/native-schema/side and canonical-record variants route the complete authoritative MSH-94 ledger through the existing strict RawValue envelope. Native command dispatch preserves accepted versus rejected-claim audit results, undo and native actor/turn admission. Cheap event/action/turn metadata is derived only from validated native state; native JSON and existing envelope1/shared3/legacy2 versions remain unchanged.

Exact request/response acceptance retains complete root/provenance/history/warnings/claims, purpose, limits, request/session/process and historical-selection identity. Cancellation, replacement and semantic undo/reopen cannot revive retired execution. Xiangqi execution capabilities remain empty: valid engine-turn, move-analysis and Chess-Full-shaped requests return unscored UnsupportedMode with exact context; malformed input/targets remain distinct refusals. No evaluator, search, verifier or later consumer was added. Desktop New/Replace explicitly refuses Xiangqi before retiring the current saveable document/work. Encoded caps were not increased; the native512-command/1024-event fixture measures253656/253832/254001/254283 bytes for native/envelope/request/response with actual u16-max depth settings, and exact1MiB/+1 boundaries pass.

Verification: local workspace478 passed (two existing opt-in deep-perft tests ignored), desktop QA93, strict TypeScript and161 frontend tests; native snapshots unchanged. Independent source/test/documentation review PASS with45 distinct tests actually run; late u8-versus-u16 documentation error corrected and upper-bound test independently rerun. Final tested production integration69838067abfdd98771d5b1fdd19984d18fb15b55 passed normal native and package CI on Windows/macOS/Linux. Earlier Linux driver-disconnect failures are retained as failures, not substituted for final evidence.

MSH105 is now dependency-ready and remains unstarted. Existing queue:105 exact/bounded no-win verification,84 native handoff, then96 console and97 evaluator/search/real child where dependencies permit. Ownership/dependencies unchanged; MSH45/M4 stay open. Stop after95.
<!-- SECTION:FINAL_SUMMARY:END -->
