# MSH-124 independent source/accessibility review

Reviewer `/root/shell124_review`; read-only production/Backlog. Reviewed the working production diff against `97b9470943107ae842c8184081c5e8e4cd26ef32` on 2026-10-31. This is an actual independent source review plus separately executed frontend checks, not the author's self-review. Integration/source hash must be supplied once committed. Actual packaged visual/focus acceptance remains a later review, not established here.

## Current source verdict

**PASS for the bounded source contract, after the keyboard finding below was corrected. No unresolved material source finding.**

- One ApplicationShell owns Analysis → Game Actions → Moves for Chess/Shogi/Xiangqi. Header/document/player/board/footer slots and accepted dark palette remain shared; specialized boards/hands/intersections and R5 controller remain intact.
- ProductApp changes replace only game cards with labeled native SELECT and exactly Chess/Shogi/Xiangqi options. Existing side button markup/classes/radio arrows/native game-side request identity survive. No future variants appear. New Game source-token checks, Save/Save As/Open, dirty replacement/close guards, direct operation semantics and Resign/Return behavior are unchanged. No native Rust, current document/archive, engine, capacity or useGameSession/useNativeChoiceController code changed.
- Shared 歩 brand and header About entry are appropriate. Support dialog retains full native context/token/profile/diagnostics/resource/observation evidence and bundled font notice/download/license across all games. Native/application close still invokes existing requestClose, and outside close blocking dismisses About rather than competing with the current guard.
- Copy removal retains material Chess claim intention, Shogi formal declaration, Xiangqi repetition obligations/pending-unscored outcomes, incomplete/cancelled/failed execution, structured capacity categories/counts and saveability/reopen-nonreset warning. The exact accepted inspection status helper is unchanged; fallback board instructions are shorter without changing pointer/keyboard action behavior.
- XiangqiAssessment puts ordinary fast result/Unknown meanings and historical exact-report meanings in Analysis, not solely in About. Exact positive witness remains existential, not forced; historical results never become current outcomes. Failed/cancelled/incomplete reports remain unscored. Technical source/provenance stays in native diagnostics. Small suggestion to explicitly label the fast proof was accepted as `Sound fast proof`.
- Dialog trap now includes disclosure summaries, omits contents of closed details, respects hidden/inert ancestors and follows DOM order. Escape/initial focus/invoker restoration remain shared. Actual native summary activation and full ring clipping require package evidence.
- Focus repair retains existing outward3px/offset3px indicator, adding8px nested Moves/sidebar space and non-board ancestor scroll reveal. It does not hide/inset focus or change board interaction. The physical result at760x650 must be checked against every overflow ancestor and viewport in actual package.

## Independent execution and resolved finding

1. `npm test -- --run src/product/ApplicationShell.test.tsx src/product/ProductApp.test.tsx src/product/ApplicationChess.test.tsx src/product/ApplicationShogi.test.tsx src/product/ApplicationXiangqi.test.tsx src/product/NativeCapacity.test.tsx src/product/NativeSessionPanels.test.tsx src/product/Moves.test.tsx src/CompletedActionCue.test.tsx`: **133PASS /9files**, Node24, local macOS. `/private/tmp/msh124-independent-source-tests.log`.
2. `npm run check`: **PASS**, `/private/tmp/msh124-independent-typescript.log`.
3. Separate SupportDialog/XiangqiAssessment run initially **7PASS/1FAIL**: after opening Font license, next Tab reached Quit instead of license link. Failure retained in `/private/tmp/msh124-independent-support-tests.log` and `/private/tmp/msh124-independent-support-tests-final.log`. A test-simulation change alone did not resolve it; reviewer reported the repeat failure rather than waiving it.
4. Builder diagnosed selector-list enumeration ordering in this test environment and explicitly sorts filtered trap controls by DOM position. Reviewer inspected that production correction and independently reran exact new tests: **8PASS /2files**, `/private/tmp/msh124-independent-support-tests-corrected.log`. This corrects and exercises meaningful focus order, including the open-license link, rather than deleting the assertion.
5. `git diff --check`: **PASS** at review. Focused counts total141 checks; not a full frontend/native campaign claim. The author retains responsibility for final full-run/CI/package hashes and task completion.

## Remaining verification boundary

No package screenshots were yet provided at this source review. Need actual normal/minimum all3 game composition, chooser/native side buttons, About keyboard disclosures/Escape/focus return, unclipped Moves focus and boards, existing notation/cues. Driver now probes actual focus outline against nested clipping ancestors and keeps read-only DOM observations; native diagnostics are accessed through ordinary public About UI. No claim of final GUI or platform success follows from these source checks alone.

## Exact checkpoint reconciliation

Reviewed committed checkpoint `de5209d3ac0d987d42396a5b2e3bf4325800784d` against baseline97b9470 after source freeze. Its production diff is the reviewed boundary; only associated focused tests, external driver/tests, Backlog plan/evidence and post-R4 design handoff accompany it. Primary checkout clean at this checkpoint. The141 independently executed checks and TypeScript result above apply to this production source, including the corrected DOM-order trap; no later unreviewed production change is implied. Root separately reports full272 frontend checks and71 external-driver checks; those are author-run evidence, not misreported as this reviewer's executions.

## Final source follow-up

Reviewed production source `ead30e672f2a2cf1747c02c8daa508babed3b7c2`: only a one-line About paragraph wrapping rule follows de5209d; no additional semantic/interaction ownership change. Actual package follow-up in `/private/tmp/msh124-review-visual.md` resolves F-V01 and supports macOS normal/minimum visual acceptance. Source verdict remains PASS; CI and integration/finalization remain root's separate obligations.

## Proportionate shared minimum-size correction

Initial CI exposed genuine same-shell Analysis heights: Linux Chess200/Xiangqi221 at236px card width; Windows Chess200/Xiangqi229.046875 at221px card width (read actual failure `/private/tmp/msh124-windows-initial-failed.log:526`). Independently inspected the resulting actual one-line production delta in max-height700: `.game-shell .shell-analysis { min-height:200px; }` →232px. The232 decision is based on the largest observed229.046875px, rather than the earlier proposed222px that would not cover Windows. Source correction **PASS**: one shared class/query across all3games, no game-specific alternate composition, retained gameplay Unknown assessment, unchanged card order, native/lifecycle/controller behavior, and unchanged strict card-layout comparison. Actual package minimum captures and successful platform comparison remain needed for final evidence.

## Final aligned source and driver review

Reviewed `4a624d6c4f129ce774d385192fbbe6a9bdcf547a` against ead30e6. The only production change is the reviewed shared min-height232px. Two external driver/test-file changes replace stale ordinary-copy expectations: historical notice now requires exact current heading/meaning AND historical=true/native selected event/explicit enabled Return; capacity evidence requires exact new summary and255→256→rejected257 accounting. The correction strengthens relevant native state/count probes rather than relaxing geometry, move/event identity, resource semantics or strict shared-card layout equality. No injected IPC/input, production instrumentation, native semantics or lifecycle change.

Independently executed `python3 desktop/scripts/test_gui_smoke.py`: **73PASS**, `/private/tmp/msh124-independent-driver-final.log`. No production JS changed after the reviewed141 focused checks/TypeScript result. Final package-platform CI remains root's separate audit.

## Platform-found support glyph follow-up

Actual4a Linux screenshot review found Unicodeⓘ rendered as tofu (F-V03), despite green input/semantic probes. Reviewed91eb13e2451af77150b82b62a448bf97e9d5ac58 correction: exact same button/accessible name/title/callback/disabled state, decorative unfocusable/aria-hidden18px SVG circle/stem/dot and grid centering only. Independent5Support tests+strictTS PASS; logs `/private/tmp/msh124-independent-svg-support.log` and `...-typescript.log`. Source PASS; actual Linux recapture remains necessary. Full28platform review is `/private/tmp/msh124-review-platform.md`; no hidden native/lifecycle change or assertion weakening.

## Final independent disposition

Source91eb13e2451af77150b82b62a448bf97e9d5ac58 and identified final package evidence now support **PASS, no unresolved material MSH124 source/accessibility/visual finding**. Independent74-image/artifact/focus/layout review and actual final Desktop/native all3-success metadata are in `/private/tmp/msh124-review-platform.md`; root's GUI execution and complete artifact hash audit remain separately attributed. Proportionate `docs/review/shell-124.md` acceptance mapping review finds no evidence/scope conflict. Historical failed tests/GUI/native races stay preserved. No MSH125/native/rules/archive/controller implementation drift or subsequent task activation observed.
