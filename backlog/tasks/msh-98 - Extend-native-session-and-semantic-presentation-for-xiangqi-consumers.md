---
id: MSH-98
title: Extend native session and semantic presentation for xiangqi consumers
status: In Review
assignee:
  - '@codex-session'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-28T13:41:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-84
  - MSH-95
  - MSH-97
  - MSH-72
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 98000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Authoritative serialized native session consumes real xiangqi rules/engine contexts and owns command admission, result acceptance and all actions; preserve exact history/purpose/request/session/generation/limits and cancel/replacement/undo/historical-selection protection.
- [ ] #2 Expose native semantic Red/Black pieces/labels, intersection topology, river/palaces, orientation, complete choices, outcomes/claims and analysis provenance without forcing chess squares or shogi promotion/hands/declaration. Extend only fields required by this consumer.
- [ ] #3 Provide native read-only opponent movement inspection against the exact live or selected history prefix; no TypeScript movement rules or actionable enemy markers. Distinguish view changes from live mutation; requests/late observations never migrate to another selected target.
- [ ] #4 Run concrete native session/presentation/race fixtures and shared chess/shogi regressions with independent review. A diagnostic harness is separate from finished product UI and excluded from normal distributables.
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
1. Add a concrete native Xiangqi session adapter to existing explicit shared desktop dispatch, with native record/context/action/claim/outcome ownership, bounded actual engine requests and read-only result acceptance; do not implement product UI or desktop archive99.
2. Present Red/Black semantic identities and explicit intersection/river/palace/orientation topology, complete choices, native warnings/count advice/conclusions/pending and fast/exact provenance; preserve existing Chess/Shogi serialized shapes where unaffected.
3. Add native read-only hypothetical-owner movement inspection, live/history-prefix selection and controller token/request/session/process/limits guards. Keep all real transitions in native Game and retire work on replacement/undo/view/cancel/retry; explicit exact execution stays bounded and separate if exposed.
4. Exercise native session/real child/race/inspection/strict-command/capacity fixtures and existing shared regressions; independent review, relevant CI, source integration only after both98/100 reviews, one root integrator. Diagnostic constructors only in test/verification builds.
5. Keep standard product new-game/file paths unavailable for Xiangqi until their existing owners; expose the native contract for the next actual UI/archive consumer without claiming completed product delivery.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Parallel ownership audit before source:98 deps84/95/97/72 allDone;100 deps96/97 allDone, no mutual edge.98 owns coreXQread-onlyinspection additions, desktop-session/shareddispatch/semanticcontract, tests/nativeexamples and docs/m4/xiangqi/session*.100 owns consoleexternalprotocol and runner modules/tests/docs/m4/xiangqi/protocol*. Both consume unchanged completed97engine/84nativecontracts; neither owns sharedengine redesign. Root sole integrator and98builder;100 isolatedworker. Primarye923cfd5712698d4a8b059ffb3b37b0e31de8f8b clean/refreshedorigin equal; baselineCLI1.55.1 pinned. Appmanagedworktree98 /Users/johnweathers/.codex/worktrees/xiangqi-session98/mShogi,100 /Users/johnweathers/.codex/worktrees/xiangqi-protocol100/mShogi. Globalhandoff/coverage, taskrecords/finalCI and any incidentaloverlap reconciled onlybyroot. No dependency changes; no laterUI/archive/architecturework.

Native consumer implementation ready for independent review. macOS arm64: cargo test -p mshogi-desktop-session --all-features --locked passed103 tests; new focused10 include256 accepted observations then257 refusal. Core relevant44 tests passed (native envelope/board/exact/game-boundary/history/new inspection-prefix). Actual supervised engine probe passed engine turn, read-only live/history analysis, cancel, Return to live and acknowledged cleanup. Clippy all-targets/all-features passed; no normal product Xiangqi admission or archive/UI claim. docs/m4/xiangqi/session-contract.md records native dispatch, immutable intra-command history versus canonical committed prefix and exact-analysis boundary. Source revision and independent findings recorded after commit; CI pending.

Independent bounded source/requirements review by separate agent xiangqi98_review: e52f9c1..055ac634dcda52b6ceaf905a57645b0f1151bc73. Independently ran native10 then11session tests,4coreprefix/inspection tests,18existing Chess/Shogi shared tests and actual supervised-child probe; final paused-capability fixture1 passed. P2 resolved055ac63: currently-submittable claim/resign flags now requireResume, native eligibility remains separate. Author-found admission/receipt retirement correction38d6446 also independently reviewed/tested. No unresolved material finding. Local broad workspace fmt/clippy/test/build and native frontend fixture regeneration passed (base5d1e86e; two subsequent focused fixes passed separately). Final integrated CI pending. Initial local host command used wrong workspace Rust1.89 then corrected to pinned desktop1.90; required bundled resource prepared directly after unrelated Homebrew Node failed dyld startup. These failed attempts are not claimed passed; actual host check outcome recorded separately.

Final local macOS host check passed at055ac63 with pinned desktop Rust1.90 and actual release native engine resource; upstream vendored Tao warnings unchanged. Pinned desktop rustfmt check passed. Added only documentation wording corrections after review (three native games/intersection metadata; remove obsolete unsupported-session comment); no behavioral difference. Native/probe evidence remains separate from product GUI evidence; applicable three-platform CI awaits the combined integration source.

Single-integrator non-destructive source integration:16c6827256c8ea65f63f01016f03d177eb6521b8 into codex/rewrite (reviewed branch5ebf1e77568bd84ff332d9ff3454c33aac29ccdc; behavioral review055ac634dcda52b6ceaf905a57645b0f1151bc73). Protocol100 independently merged afterward2ae336819284ac24df8ec3a86c1c1b605ac73bd6, no source conflict or dependency change. Final combined-source verification/CI pending, task remains In Review.

Exact combined integration source2b50ac2f8c7fea6174be71a479cf532324347a3e, macOS27 arm64/Rust1.89: cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo test --workspace --locked; cargo test --locked -p mshogi-desktop-session --all-features (105 passed); cargo clippy --locked -p mshogi-desktop-session --all-targets --all-features -- -D warnings; native write_frontend_fixtures then git diff --exit-code (unchanged Chess/Shogi fixtures); release mshogi/mshogi-match build; desktop-verification xiangqi_session_probe against actual target/release/mshogi: all passed. Workspace+feature runs aggregate658 passing executions across70 suite summaries, not658 distinct tests; existing two deep opt-in tests remain ignored. Local log/private/tmp/msh98-100-integrated-native.log. This final source also passed fresh actual12game protocol reproduction, separate100 evidence. Native CI40692691079 and desktop CI40692691108 target exactly2b50ac2; pending at this note, no completion inferred. All twelve focused Xiangqi session fixtures plus four core helper tests are included. No product Xiangqi archive/UI, timed play or implicit exact graph introduced.

CI native40692691079 passed all3OS at2b50ac2. Desktop40692691108 attempt1: macOS15 and Windows2022 passed full applicable jobs; Ubuntu normal package/native/frontend checks and12of14GUIphases passed, but normal-shogi aborted at gui-smoke.py791 capture(active-analysis), GET/session/.../screenshot. Hyper proxy SendRequest/ECONNRESET, not native game IPC; same-session subsequent failure capture succeeded with live rendered event2/7g7f/4c4d/active analysis and no product failure outcome. Dependent reopen phase then correctly reported missing unsaved test archive/ENOENT. Independent reviewer xiangqi98_review confirmed source/tracebacks/DOM/png/driver evidence and identical proxy-reset signature at different NewGame-click operation in baseline74a639, with frontend/scripts unchanged. One failed-Linux-job rerun was requested on unchanged2b50ac2, keeping all assertions and raw attempt1 evidence. No source changes or assumed success. Attempt1 job122831286080/artifact12819300392, retained/private/tmp/msh98-100-linux-artifacts.zip; gui-report SHA25661a285e6665177f48a0527506a9d3d53d497a7102e5c33680b390a73f40c7257, failure webview1cf95059138120e02ea0514f02865059442a0e2b4ce1f6742f057a15a0a082e7. Final CI verdict remains pending.100 independently completed atf5ce07c because its protocol/native criteria all passed;98 remains In Review.

Plan amendment after integrated CI attempt 2: correct the external GUI verification client only. Add at most one logged retry to the explicitly read-only DOM observation and screenshot wrappers for proxy transport resets; generic script commands, mutations, input, session creation/deletion and semantic errors stay single-attempt/fatal. Preserve all actual journey assertions. Cover recovery, exhaustion and non-replay with focused Python boundary tests; obtain independent review before pushing and run actual native desktop CI on the resulting revision. Prior failed Linux observations remain retained evidence; production semantics and task scope are unchanged.

Desktop40692691108 attempt2 job122841060757/artifact12819867638:13of14 phases passed, including normal Shogi play/analysis/save/reopen that failed previously. Claim fixture failed before pressing Claim at pure w.text()/read POST execute/sync (old gui-smoke.py1037); proxy SendRequest/ECONNRESET. Immediate same-session failure PNG/DOM succeeded with expected claim opportunity, ongoing game and0plies. Retained/private/tmp/msh98-100-linux-retry-artifacts.zip. No product defect inferred; no blind third unchanged rerun. External-only client correction adds one logged transport retry through explicit DOM-read/screenshot wrappers, preserves generic script/mutation/input/session single-attempt and every journey assertion. Independent separate xiangqi98_review audited every read call for DOM-only observation, reviewed whitelist/error/exhaustion boundaries and independently passed29 Python tests plus diffcheck; no material findings. Production source unchanged from2b50ac2. New actual package CI required before completion.
<!-- SECTION:NOTES:END -->
