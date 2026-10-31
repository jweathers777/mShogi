---
id: MSH-125
title: Simplify desktop Save Load Resign and Return to live workflows
status: Ready
assignee: []
created_date: '2026-10-31T09:02:00Z'
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
