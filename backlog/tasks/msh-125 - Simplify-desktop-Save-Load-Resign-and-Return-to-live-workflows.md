---
id: MSH-125
title: Simplify desktop Save Load Resign and Return to live workflows
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-31T09:02:00Z'
updated_date: '2026-10-31T15:38:00Z'
labels:
  - desktop
  - owner-approved-ux
milestone: m-4
dependencies:
  - MSH-119
documentation:
  - docs/design.md
  - docs/desktop/post-r4-ux.md
  - docs/architecture-migration-plan.md
priority: medium
type: enhancement
ordinal: 125000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved post-MSH-119 lifecycle/interaction changes (2026-10-31). Current repeated unsaved replacement guards, Save As and resignation confirmation add friction; history browsing also creates an unexplained second Resume step. Own the bounded common/native admission changes and shared workflow copy while preserving atomic file replacement, exact execution retirement and truthful outcomes. This intentionally permits losing the replaced unsaved game only after successful explicit New Game or Load. It does not remove the Quit safeguard, weaken saveability or change native game rules. Read the explicit browse-origin versus pre-existing-stop policy in post-r4-ux.md. Shell styling/game dropdown and Moves projection/cues have separate owners; R5 remains unchanged. Future work only: unassigned, no speculative implementation plan.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Ordinary file controls are Save and Load, with no ordinary Save As button. Save without a path chooses a file; later Save writes the current path. Cancellation/failure does not clear dirty state; interrupted/failed writes preserve a prior valid save. Underlying atomic Save As mechanism may remain internal; current document envelope, full observations/history/lineage, capacity admission and reserved close/stop headroom remain intact.
- [ ] #2 New Game and Load intentionally replace without a separate unsaved-game confirmation. The live document remains installed until a valid replacement is admitted atomically; cancelling either chooser/picker, malformed/unsupported/resource-limited load, new-game failure or stale token/replacement leaves the original document/history/dirty state intact. A successfully replaced unsaved game is intentionally discarded without an automatic Save claim. In-flight save/engine/file/close operations cannot authorize replacement from an obsolete snapshot.
- [ ] #3 Unsaved native Quit/window-close retains a concise safeguard with Save, Discard and Cancel. Remove Save & continue/Discard & continue and redundant explanatory paragraphs; Save failure/cancellation retains the game and guard, stale dirty Save receipts cannot authorize Quit, and Cancel cancels native close reservation safely. Already-clean quit does not require an unsaved warning. Native menu/window/macOS termination protection and cleanup remain valid.
- [ ] #4 Resign submits once without a confirmation dialog, using the correct native actor and exact rendered admission context with busy/double-submission protection. Present concise native conclusion or unresolved-unscored meaning; never infer a loss, draw or score from resignation/evaluation/failure. Other native claim/declaration/offer semantics and required confirmations are not indiscriminately removed.
- [ ] #5 History browsing from otherwise running live play records native browse-origin intent while retiring old execution. One Return to live restores live view and fresh eligible play admission, including a new engine-turn request when appropriate, without a second unexplained Resume. Pre-existing stopped reopen/Take back/explicit cancellation/failure remains stopped with appropriate Resume/Retry; intervening explicit stop, newer stop reason, conclusion or capacity refusal takes precedence. Return does not revive retired identities, automatically retry failures, resume saved pending work or start unsupported analysis.
- [ ] #6 Native/controller/frontend tests cover all three games: running-human/running-engine history return, already-paused/concluded/history-analysis cases, cancellation/replacement races, same-board/different-context and complete request/session/process freshness; intentional successful replacement versus cancelled/failed/stale replacement; first/subsequent/cancelled/failed/interrupted Save; dirty Quit and stale receipts; direct native Resign including unresolved Chess outcome. Preserve exact history and Take back evidence with no frontend rules.
- [ ] #7 Actual identified packaged all-three-game journeys verify New Game/Load without extra guard, picker cancellation/failure preservation, Save first/subsequent, dirty Quit Save/Discard/Cancel, one-step running history Return versus stopped reopen/Take back Resume, Resign and focus/keyboard/native close. Difficult states use separately labeled QA builds; normal instrumentation exclusion is verified. Independent review focuses on atomicity, intentional-loss boundary, retirement/admission and outcome honesty; resolve findings and record revisions/platforms, actual checks/GUI, applicable CI and normal integration/push.
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
1. Root Codex owns integration in the clean primary codex/rewrite checkout at07c0be6; pinned Backlog1.55.1 resolves global1.55.0 mismatch. Keep one shared ProductApp/ApplicationView workflow, with a bounded frontend worker owning only those presentation/hook/action files and tests. Preserve current shell, selector, Moves, R5, native rules and archive format.
2. Inspect and extend authoritative native command admission with runtime browse-origin intent: history entry retires execution; one ReturnLive restores eligible previously running intent with fresh work only. Pre-existing/intervening Load, Take back, cancellation, failure, conclusion, capacity or stop authority overrides it. Preserve historical analysis retirement, full context/history and saveability admission. Strengthen post-picker Save/New/Open/close identity boundaries where concrete inspection finds gaps.
3. Simplify shared Save/Load/New/Quit and direct Resign: intentional replacement only after native admission, first/subsequent Save and cancellation receipts remain truthful, clean Quit avoids a prompt, dirty Quit Save/Discard/Cancel retains native close reservations. Add all-three-game native/controller/frontend regressions for exact tokens, stale receipts, atomic disk/replacement and running versus stopped return.
4. Verify identified normal/separate-QA packaged all-three-game workflows on macOS, update external GUI probes for changed approved behavior without weakening semantics, verify normal instrumentation exclusion. Obtain independent review focused on losslessness, deliberate replacement, runtime browse origin, stale execution and honest outcomes; resolve material findings, run focused/native/desktop CI, record source/package evidence, normal integration/push and stop125.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup: clean codex/rewrite07c0be6 aligned0/0 with origin, unrelated three worktrees preserved, no In Progress/In Review owner displaced. MSH-119 direct dependency Done;120/123/124 remain Done. Read task, owner attachment, design, post-r4 browse policy and migration plan. Actual shared ProductApp currently guards dirty New/Open and clean Quit, GameActions confirms Resign; native ReturnLive pauses all3. Root primary builder, no other task started.

Implementation checkpoint (uncommitted): shared header Save/Load with intentional New/Load admission and direct Resign; frontend318/26 PASS plus strictTS. Host post-picker Save/Open now rechecks exact source token; cancellation is an explicit rejected receipt, native Load title; host12 PASS. Native transient browse-running intent preserves named stopped recovery, retires history analysis and uses normal journaled Resume under the same atomic capacity admission. New eight all3 native journey tests PASS, including fresh request/process identities, retired responses through correct native handler, Load/Take back/cancel/failure/capacity/conclusion stops, stopped replay and clean-close authority. Original187 native regressions PASS before final focused additions. New test found Chess Resume accepted a concluded session; native admission now rejects it without changing its outcome. Close guard exempts only exact stop-only token from warning, never marks canonical close controls disk-saved; newer dirty token requires guard. Independent review and packaged verification remain pending; no AC/DoD checked.

Source verification ready: full native-session199 tests PASS, full frontend319/26 PASS, strictTS PASS, external driver/adapters91 PASS, host12 PASS at initial host boundary (final native rebuild pending). Independent review F01 found older stopped recovery overriding newer cancelled/failed historical analysis; clear both stale browse intent/stop and preserve newer meaning, with strengthened positive all3 regression PASS. Preserve concluded Chess ordinary/Full Finish analysis without reopening game; reject Resume of concluded Idle and hide that unavailable control. Two new near-control-limit tests prove Return+Resume controls and close reserve are admitted atomically, otherwise complete current archive unchanged. Clippy initially found adjacent minified if formatting; source formatting corrected, rerun pending. Actual package/GUI and native/Desktop CI remain required, no criteria checked. Scope unchanged; MSH-121 unstarted.

Packaged checkpoint871a1fa4: actual normal macOS Chess/Shogi/Xiangqi Save/Load/New/Resign/history/close journeys captured; normal package smoke PASS. First native CI all3 passed tests and QA checks but failed fixture generation because the fixture writer attempted Resume on a concluded game; correct generator to preserve concluded Idle. First desktop CI exposed never-saved initial Chess(path=None, canonical dirty=false): native Quit authority now includes missing saved path, without falsifying canonical dirty state. Add native untouched-root regression and all3 frontend Save-in-Quit tests. Driver now uses native close_requires_save and bounded keyboard reveal before pointer history selection; original failed Linux/Windows observations retained, final CI/package pending. Corrected full frontend322 PASS, strictTS PASS, native11 workflow PASS. No criteria checked before final review/verification.

Corrected source58a5d7ad95cba1724118a91d8ad4958fb1c287ed integrated/pushed. Final local native all-features212 PASS(192.16s), full frontend322/26 PASS, TS/clippy PASS; independent reviewer regenerated23 identicalfixtures, 50frontend/92verifier checks and neverSavednative PASS. Rebuilt normal/QA macOS packages identified by hashes: actual untouchedChess Quit→Save(enabled)→pickerCancel→guardCancel preserves game; first/subsequentSave succeeds; clean bareCmdQ PID91838 exits. QA actual Cocoa terminate Cancel preserves Chess, all3 minimum760x650 boards captured, Cocoa terminate→Discard onXq exitsPID92029/no packagedengine remains. Independent reviewer viewed9final+6earliercaptures, verified8packagehashes/16normalexclusionmarkers, PASS with no unresolved material finding. Final CI41214429663/648 pending; reports distinguish earlier871 journeys from corrected58 packages. MSH121 unstarted.

Final-source58 desktopLinux14/15 phases PASS; late normalChess Resign probe failed because it incorrectly required board context to change. Actual native match accepted White resignation, unresolved/unscored, with unchanged canonical board context. Correct external verifier against the separate authoritative Chess match ledger: one terminal event, exact unchanged prefix/profile/root, correct actor/version, accepted unscored conclusion and unchanged board/Moves. Captured actual native payload is a reproducible regression fixture;94 driver/adapters PASS including wrong actor/stale/rejected/duplicate/invented-board/winner negatives. This is verification-only; production remains58. Preserve failed run41214429648 and rerun actual GUI after independent probe review. Native58 matrix Mac/Windows PASS; LinuxMSH112 failed-job retry underway at identical source.
<!-- SECTION:NOTES:END -->
