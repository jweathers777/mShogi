---
id: MSH-46
title: Deliver shared six-game integration across console engine records and desktop
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-05T21:55:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
  - MSH-41
  - MSH-42
  - MSH-43
  - MSH-44
  - MSH-45
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-76
  - MSH-77
  - MSH-78
  - MSH-79
  - MSH-88
  - MSH-89
  - MSH-90
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/m4/implementation-map.md
  - docs/m4/shared-integration/README.md
  - docs/m4/shared-integration/verification.json
  - docs/m4/shared-integration/contract-review-89.md
  - docs/m4/shared-integration/contract-review-89.json
ordinal: 46000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The five game scopes own their rules and game-specific adapters/tests. This ticket is the single owner of reusable boundary extensions and cross-game integration under A-01 through A-07: game/profile selection and dispatch, complete-action transport, native engine/session coordination, common canonical-record evolution and compatibility, shared board/action presentation and packaging. Do not duplicate those deliverables in each game ticket. Existing chess behavior and MSH-26 evidence remain valid regression obligations.

Delivery may coordinate with game tickets before their completion; dependencies below gate final cross-game acceptance, not permission for speculative framework work. D-08 remains open for concrete public contracts, schema evolution and necessary interoperability adapters. Select protocols only for actual consumers/opponents; no universal DSL, ABI or chess-only record requirement. The current M3 batch is unchanged.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Shared library/console/structured engine/session boundaries select and carry explicit rules profile/version, complete actions and rules-relevant state/history for all six games. They preserve atomicity, claim versus board-action distinctions, identity/cancellation/stale-result safety and bounded reporting without frontend legality or Python/node round trips.
- [x] #2 Common canonical-record/version compatibility and save/reopen plumbing preserve all six games and reject unsupported or lossy conversions explicitly; game-specific encodings and expectations remain owned by MSH-41 through MSH-45. Previously supported chess records and adjudication modes have regression evidence.
- [x] #3 Shared desktop mechanisms support the real square/intersection and large-board presentations, multi-stage complete-action choices, promotions/drops or other profile choices, keyboard and readable textual interaction required by the implemented profiles. Partial selection never commits a partial turn.
- [x] #4 An ownership/coverage matrix ties each shared mechanism to this task and each game-specific adaptation to its game ticket. Cross-game library, console, native engine, saved-record and actual desktop flows run together; chess regressions and transitions between games/profile versions are exercised.
- [x] #5 Installed packaged applications with their bundled native engines demonstrate the required six-game flows on Windows, macOS and Linux, including cancellation/stale work, persistence, outcomes, recovery and accessibility. Actual revisions, environments, commands and failures are recorded; synthetic fixtures or browser-only tests do not establish this acceptance.
- [x] #6 Independent integration review resolves material shared-boundary findings and updates public contracts and coverage honestly. Functional delivery makes no playing-strength claim and does not move M5 targets into this task.
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
Primary integrator Codex in the primary codex/rewrite checkout, clean and origin-aligned at91115368c6e99f344dee002d9264f4d0b62cc76b. Aggregate closure only: (1) reconcile each original46AC1–6/DoD with the existing shared ownership matrix, exact tested runtimebc92bfe5e3b20e9e999df12cbf54b91682c71b8d, MSH88 actual packaged/platform record and MSH89 independent contract assessment; inspect source equivalence, immutable evidence hashes, original criteria and all16 prerequisite identities/statuses. (2) Update the existing shared README as the final delivery record and current108/47 handoffs, retaining all original dependencies, game ownership, retired-format policy, capacities/ending reserves, accessibility/platform qualifications and open MSH112 diagnostic. (3) Obtain a fresh independent read-only completion review; run proportionate hash/link/matrix/DAG/diff checks only, without a repeated broad product/CI campaign. (4) Check only supported criteria/DoD, integrate and push normally to codex/rewrite, then close46 with durable summary. Leave108 and47 unstarted, M4 open; stop after46.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH62 planning refinement2026-10-26: original identity,status,AC and DoD remain intact. This complete-delivery record now depends explicitly on its required bounded component/shared work; docs/m4/delivery-plan.md traces each original criterion and single ownership. No game implementation or delivery completion occurred. Earlier M3-context wording remains historical; current authorization is planning-only. Early shared69/72/73 depend on concrete shogi native consumers, not this final record or all five game deliveries. Later game-specific shared extensions76-79 wait on native handoffs84-87 and profile research80-83, not their aggregate delivery. Final integrated packaged acceptance88 and fresh shared review89 retain all original final obligations; MSH47 still gates M4 closure.

MSH-90 is the required bounded common-console follow-up to MSH-68/69/70, owned once under MSH-46. Added to this final acceptance gate without removing existing dependencies or changing criteria. MSH-71/72 remain unstarted.

MSH88 final shared packaged integration accepted2026-11-05, evidence e68e8e3d1787f2d029e3601259070807bdbab180 pushed to codex/rewrite; tested codebc92bfe5e3b20e9e999df12cbf54b91682c71b8d, all6 required platform jobs pass and Windows/Linux49/49 actualGUI phases plus separately qualified localmacOS27 actualall6/seven-profile journeys. Current ownership/originalAC1–5 mapping: docs/m4/shared-integration/README.md; single exact-source record verification.json in that directory. All current native/library/console/engine/session/document/frontend owners are exercised together; no production rules/limits/architecture or minimum changed, no retired reader revived. Fresh88independent review approved functional/platform evidence. Original46AC6 remains MSH89; no46criteria are checked or narrowed by this handoff,46 remains ToDo/unassigned, and108/47/M4 are not closed.89 is dependency-ready but not begun.

MSH-89 independent contract reconciliation approved 2026-11-05; publication/integration and89DoD4 closure follow. Report docs/m4/shared-integration/contract-review-89.md and compact review index contract-review-89.json independently support every original46AC1–6. Reviewed integration3bcaf013b7e9ea7d37a5a542700de8a1ea041b2b, reused exact tested runtime/platform sourcebc92bfe5e3b20e9e999df12cbf54b91682c71b8d and unchanged88record; two fresh source/evidence reviewers plus distinct findings/DAG reviewer approved. All original46sixcriteria and16dependencies including90 remain intact and unchecked; no46activation or owner claim. Current nativeChessM2/Full/console/live-desktop scopes and deliberately retired development formats are distinguished; all6games/two immutableTenjiku profiles use existing owners, game-specific resource accounting and actualplatform evidence. No material blocker remains for shared acceptance. Only stale rustdoc/Load-install wording corrected.112 remains a separately owned unfixed functional diagnostic race, not M5 optimization or repaired by later retries. After89Done,46 is the next dependency-satisfied M4 task and still requires its own activation, criteria/DoD reconciliation and normal integration;108/47/M4 are not completed here.

MSH-89 is Done. Independent original46AC1–6 contract reconciliation/report and compact review index were normally integrated/pushed on codex/rewrite at22b4f409f738cd7dec45a78ab7ecc4cf9661322b before final89closure. All six obligations have supporting evidence, with no material shared-contract blocker; current source/compatibility/resource/platform qualifications and the separate unfixed112 disposition remain explicit. Original46criteria/dependencies, ToDo status, unassigned ownership and unchecked items are preserved. With89Done all16dependencies are satisfied;46 is the next M4 task and still requires its own authorized activation, final criteria/DoD reconciliation and normal completion.108/47/M4 remain open; none started here.

MSH-46 final reconciliation2026-11-05: all original6AC and DoD1–3 independently supported; durable current record docs/m4/shared-integration/README.md maps every original obligation to source, original88receipts and89assessment, with no criterion/dependency/ownership removed. Fresh fork-none read-only reviewer msh46_completion_review approved at91115368c6e99f344dee002d9264f4d0b62cc76b plus six-file candidate diff SHA25627e8e364e59b2a4b1bab3f09b6225d837cd5b991ea1b55f9d5761cced28c8322, no material blocker; DoD4 remains pending ordinary commit/push. LocalmacOS27arm64 Python3 closure checks pass: unchanged master88SHA097118de9a7c9507900f554da7c91dde97ad35729bce73c9f491b52a7a9cd985/all202filehashes; six raw sourcebc92bfe jobs+attempt1+runners+stepconclusions; originaldecompressedWindows/LinuxreportSHAs49/49passed/no failures; source equivalence (only4neutralrustdoccomments since testedbc);19originalAC/DoD/dependency/non46owner comparisons againstGit9111536; full16prerequisite statuses allDone;113resolvedgraphnodes;86local linktargets; compatible1.55.1doctor;gitdiff--check. Initial audit harness omitted39 from selected dictionary and raisedKeyError; corrected complete prerequisite lookup passes, no application failure or source change. Existing native11/11 and platform/actualGUI checks reused at actual sources; no new broad campaign. Native gameconformance/search/performance/strength/shared integration remain distinct. BothwholeTenjiku IDs, current/retiredformat meanings, resource/actualendingClose reserves, normalQA identity/signature/accessibility limitations and MSH112separatelyunfixedfunctionaldiagnostic disposition preserved.108/47handoffs appended without claiming/starting them;108 remains next after46Done and47stillwaits108. Evidence integration/push and finalDoD4/Done closure follow.

Final integration condition satisfied: approved evidence90e1222d8db27ede3212b794e877a975a068c35d was normally committed and pushed to origin/codex/rewrite before DoD4/Done. Original6AC/4DoD now checked; final metadata/docs commit follows normally. No product source or acceptance change.

Fresh independent final reconciliation approved after actual90e1222d8db27ede3212b794e877a975a068c35d evidence push: reviewer independently confirmed HEAD/origin alignment, durable final summary, all original46/47/108 AC/DoD/dependencies intact,46Done supported,108Ready/unassigned/unstarted,47ToDo/unassigned/unstarted/gated108, no runtime/evidence change;gitdiff--check passed. Reviewed final metadata candidate diff SHA256ea6b5903e79d9a687efa6230e80001041c815d8c123490de3a146e68d4485375. No material blocker; ordinary closure commit/push authorized. No next task started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Closed the original six-game shared-delivery assignment using source-qualified component/game acceptance, actual packaged integration and fresh independent contract/completion review. No implementation, rules, architecture, formats, limits, ending/Close reserves or requirements changed. All six original AC and four DoD items are supported in docs/m4/shared-integration/README.md; every original criterion and all16 dependencies are preserved, with native/game-specific ownership still41–45. Six games include both immutable Tenjiku whole profiles.

Exact tested runtime/platform source bc92bfe5e3b20e9e999df12cbf54b91682c71b8d passed all6 required jobs: Rust42028972679 and Desktop42028972586, attempt1. Windows/Linux actual packaged reports each49/49 pass; localmacOS27 gameplay and four-file package equivalence remain distinct from hostedmacOS15 native/package CI. Existing MSH88verification.json/all202evidence files and MSH89report/index remain unchanged. Current runtime differs from tested source only by four neutral rustdoc comments; no new binary/OS execution is claimed. Current versus retired development formats, supported Chess native/console/live desktop scopes, actual keyboard/accessiblemetadata versus full screen-reader support, dense kanji760x650 limitation and signing/notarization qualifications remain explicit. Functional search is not competitive strength.

Fresh independent read-only msh46_completion_review approved all6AC/DoD1–3, no material blocker; DoD4 followed actual normal evidence commit/push90e1222d8db27ede3212b794e877a975a068c35d on codex/rewrite. New closure checks passed exact-source diff, immutable master/202hashes, six raw jobs+steps/runners/source, original49phase report hashes,19record AC/DoD/dependency/non46owner preservation, all16prerequisitesDone,113resolved graphnodes,86local linktargets, Backlog1.55.1doctor andgitdiff--check. Audit harness first omitted39 from its selecteddictionary; complete prerequisite lookup corrected and passed, no application failure/change. No broad product/conformance/frontend/GUI/CI campaign repeated.

MSH112 remains separately owned ToDo/unfixed functional runner diagnostics with recurrence evidence and nonblocking shared-contract rationale retained; later green retries do not fix it. MSH108/47 handoffs updated without activation:108 is Ready/unassigned/now dependency-satisfied, next independent architecture diagnostic;47 remains ToDo/unassigned and gated by108, owning whole-M4 closure. M4 and M5 are not begun/closed here. Primary integration checkout reused; no worktree created/removed and unrelated historical-access worktree preserved. Stop after46.
<!-- SECTION:FINAL_SUMMARY:END -->
