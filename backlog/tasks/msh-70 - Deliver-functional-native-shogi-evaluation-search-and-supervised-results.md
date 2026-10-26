---
id: MSH-70
title: Deliver functional native shogi evaluation search and supervised results
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T16:19:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-67
  - MSH-69
  - MSH-64
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/shogi-engine.md
type: task
ordinal: 70000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Independent solvable positions/trees, same board differing histories, zero/finite work,cancellation/replacement and actual internal-process smoke. Evaluate shogi defaults instead of copying chess depth/node constants.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A transparent native shogi evaluator includes hands/promoted pieces and named score perspective/units; bounded functional search uses the authoritative full legal set and history/adjudication,not a second rules approximation.
- [x] #2 No-action,selected-profile history outcomes and valid declaration choices are consumed before static leaves; cancellation/resource exhaustion is incomplete work,not an invented game result or fallback move.
- [x] #3 The supervised structured engine returns complete legal shogi choices and real observations under exact context/purpose/session/generation, preserving one execution owner and stale-result safety.
- [x] #4 Independent small search/terminal fixtures and real child-process play for both human sides establish functional support,with recorded work limits and root restoration. No competitive-strength target or learned model is claimed.
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
1. Implement a transparent native shogi material evaluator (board/promoted identities/both hands) with named fixed values, units and native side perspective; no positional/learned terms or declaration-point reuse.
2. Add private bounded iterative-deepening full-width minimax on native Game clones, no cache. Consume native conclusions before static leaves, submit eligible declarations as explicit terminal choices at every node including leaves, score native winners in a stable root-side perspective so unchanged-turn declarations and perpetual-check losses do not invert incorrectly. Count command-edge distance separately from board ply.
3. Preserve exact unwind/root equality for completed, limited, stopped and failed traversal; propagate native invariant diagnostics unscored. Use a small private tree seam for independently specified minimax/perspective and descendant-failure tests, not public game abstractions.
4. Extend existing v3 SharedResult with actual native shogi choices/conclusions/telemetry and game-specific bounded limits. Keep strict raw records, exact purpose/target/history/request/session/generation/limits and one supervised owner; validate native target side/event-prefix/known-board-actions after replay. Advertise only engine-turn/move-analysis once implemented; chess Full remains explicitly unsupported for shogi, v2 chess and archives unchanged.
5. Verify independent arithmetic/native solvable fixtures, drops/promotions/declaration leaves/history/no-action, zero/finite work and cancellation/replacement/root restoration. Exercise real subprocess play for both human assignments with native acceptance and recorded observations/limits. Measure representative initial/hand-heavy positions to choose documented practical defaults/caps instead of chess defaults.
6. Separate completed independent review, proportionate native/shared/subprocess/desktop regressions and normal CI, integrate/push with own acceptance/evidence record, archive checkout and stop70. Do not start71/72 or shared desktop/archive work;41–47/M4 stay open.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH68 completed separately: reviewed8a60b28 integratedada6592, three-platform native40344086656 and desktop40344086642 PASS, durable completion51f4556 pushed and managed console checkout archived. Now primary builder/integrator @codex on codex/shogi-engine at managed /Users/johnweathers/.codex/worktrees/shogi-engine/mShogi based51f4556e42b377e549a398b002d9bc90462b1cf3. Checked native Game/history and existing chess search/shared v3 consumers. Dependencies67/69/64 Done; no competing builder. Read-only independent arithmetic/native fixture audit delegated before product tests. Provisional conservative shogi budgets will be measured before finalizing; no rules or product decision unresolved.

Implemented transparent native shogi material evaluator (Pawn100,Lance/Knight300,Silver400,Gold500,Bishop700,Rook800; promoted minors500,Horse900,Dragon1000; hands base values, kings0), evaluator mshogi.shogi.material.v1 and full-width minimax mshogi.shogi.full-width.v1. Native Game/complete legal choices/history/conclusions remain authoritative, no cache. Stable root-side scores use native winner and1m command-edge distance, including same-turn declarations at leaves. Exact private native undo/root equality is checked after complete/limited/stopped/failed iterations. Invariant errors remain unscored native diagnostics; test-only descendant injection is not a legal mutual-check fixture.

Actual v3 ShogiChoice/ShogiResult/telemetry and native primitive serde reuse existing USI/conclusion forms, without changing canonical record encoding. Native validated-record side/conclusion accessors avoid host history replay for acceptance. Engine-turn and read-only analysis are implemented/advertised; exact event-prefix/known-action/side/purpose/session/request/generation/limits remain bound. Chess Full on shogi stays explicit unscored UnsupportedMode. One existing EngineBridge supervises both games/versions; v2/archives and later desktop/USI duties unchanged. MSH69 temporary unsupported tests evolved to actual consumer behavior; its historical evidence preserved.

Independent read-only fixture audit supplied expected arithmetic, capture/demotion/promotion/drop mate, declared winning root/leaf, no-action precedence and same-position/history-dependent best choices before product search. Board-only corroboration corrected a blocked-bishop-ray oversight before final leaf expectation1i1h; native rules unchanged. First same-position test incorrectly compared complete Position equality including imported-origin provenance; corrected comparison to equal SFEN/repetition identity while keeping full contexts distinct and all score/move expectations. Initial syntax/closure and collapsible-if lint errors corrected; final focused tests/lint pass.

Native workspace318 tests PASS before finalcap reduction (algorithm unchanged); all-targets workspace clippy PASS. Native8integration+2private-tree tests,6v3protocol tests and3actualchild tests PASS. Actual process evidence docs/m4/shogi-engine-process.json contains both human assignments,4complete returned request/result observations/limits and native finalgames. Independently measured source/binary hashes and release process timings retained in shogi-engine-measurements.json; exploratory100k configuration (20.213s dropcase) retained separately and explicitly unsupported by final contract. Final bounds depth1..4,nodes0..20000; defaults2/5000. Final medians standard2/5000:962visits27.226ms; sentehand2494/220.944ms; bothhands5000 partialdepth1/1006.002ms; bothhands20000 partialdepth1/4042.591ms. Native node counts exclude parse/replay, elapsed includes them. Existing watchdog15s/cleanup250ms preserved; measurements are not cross-platform time guarantees.

Separate completed-source reviewer msh70_final_review inspected native minimax/history/serialization/v3/supervision and final docs/evidence; no actionable findings. Independently built/ran native8 + private2 + protocol6 + realprocess3 tests on Darwinarm64 Rust1.89 via separate/tmp target, allPASS. Documented read-only example independently produceddepth2/962visits/score0/PV1g1f1a1b. Final measured source hashes match reviewed source (search9a1a5f8941bd77d047876ecdd7b38a7e3e4b2f2c2efee24159e4cbb15bf4ee93, engineShogi b894ad8f655b11b16067b4c4938567486dabdabf730646d457fe8635f8a00f11). This is AI source review and independent native execution, not human/cross-platform review. Final native regression/normalCI/integration remain pending.

Final reviewed20k configuration: cargo test --workspace --locked PASS318 tests, cargo clippy --workspace --all-targets --locked -- -D warnings PASS, cargo fmt/diff checks PASS. Measurements source fingerprints remain current. Independent final review closed with no actionable findings and19 focused tests plus exact README/docs analysis example PASS. Evidence JSON now explicitly records actual direct-process timeouts (exploratory45s/final15s),3runs/no warmups, and actual child-smoke bridge watchdog30s/cleanup1s, distinct from unchanged product bridge defaults15s/250ms. No source/algorithm changes after review.

Reviewed sourcee88a19d46459e4516d0fb918cbb5e1b8152ad6e3 normally integrated/pushed as36e570f8396c6fae1a70d88c8427e5ca8ca4665b. Exact-source normal native CI40352695690 and Desktop CI40352695658 are running. Local integrated Tauri all-targets clippy PASS; desktop-session --all-features PASS46; regenerated native frontend fixtures zero diff. MSH68 remains Done with separate source/review/CI/completion record. MSH70 remains In Review pending current CI/finalization;71/72 unstarted,41–47/M4 open.

Exact-source native CI40352695690 PASS allthree platforms; local integrated Tauri tests PASS2 and desktop-session QA-feature tests PASS46, fixtures unchanged. Desktop40352695658 attempt1 Windows/macOS PASS. Linux setup121676739188 spent over21minutes downloading native APT dependencies before reaching any product compile/test. After opaque in-progress logs, cancelled that outstanding attempt and reran only Linux on a fresh runner (same36e570f source; no CI/source/criteria changes). Retrieved cancelled logs confirm very slow azure.archive.ubuntu.com package downloads, last observed25.5MB libwebkit2gtk package at16:07:17Z, cancellation16:09:45Z. This was a cancelled incomplete setup, not a failed product test or a passed Linux desktop check. Attempt2 Linux-only verification is pending; preserve original attempts/platform successes in final evidence.

Final exact-source CI on36e570f8396c6fae1a70d88c8427e5ca8ca4665b complete: native https://github.com/jweathers777/mShogi/actions/runs/40352695690 PASS Linux/macOS/Windows. Desktop https://github.com/jweathers777/mShogi/actions/runs/40352695658 PASS allthree at attempt2: Linux retry completed dependency installation, locked frontend/native checks, normal/verification packages, bundled engine and installed GUI interactions; macOS/Windows successes retained on the same source. Attempt1 Linux dependency download was cancelled after over21minutes, no product checks reached; it is not claimed as passed or a product failure. No code/CI/criterion change for retry. Existing M3 macOS GUI/native termination/Tao patch evidence remains applicable and distinct from macOS package smoke.

Acceptance mapping:AC1 independent arithmetic + all native board/promoted/hand evaluation, complete native legal/history search;AC2 actual native winners/no-action/history/length/declaration leaves and same-turn scoring, finite/zero/stop/error private-root restoration tests and completed-iteration-only results;AC3 actualv3engine-turn/read-onlyanalysis, strict native serde/raw boundary, exact native target/context/purpose/limits/runtime identities and common supervisor/cancel/crossgame replacement;AC4 independent solvable trees/native fixtures plus recorded real child play for both human assignments,19 independently rerun focused checks,318 final native tests and normal CI. Separate read-only completed-source review had no actionable findings. Local final Tauri tests2/QA-session46/clippy and unchanged snapshots PASS; frontend78/platform packages verified by current desktop CI. No competitive strength, learned model, shogiFull/USI or desktop shogi claim.

Reviewed sourcee88a19d46459e4516d0fb918cbb5e1b8152ad6e3 integrated/pushed36e570f8396c6fae1a70d88c8427e5ca8ca4665b; final task-only evidence update follows. MSH68 remains independently Done (8a60b28->ada6592, own CI/review/completion51f4556). MSH71/72 now dependency-eligible but unstarted;41–47 and M4 remain open. Completed engine checkout is clean/integrated and will be archived after evidence push. Stop after70.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed MSH70 after separately completing68. Added untrained native material evaluation (board/promoted identities/both hands), bounded full-width private-history minimax and actual supervisedv3shogi engine-turn/read-onlyanalysis. Native conclusions precede leaves; valid formal declarations are explicit terminal choices, including at leaves without board-turn advancement. Stable native-root perspective handles checking-mover losses correctly. No board-only cache, partial-iteration score/fallback, invented game outcome or shogiFull operation.

Defaults depth2/5000 nodes; supported depth1..4/nodes0..20000, chosen from actual initial and drop-heavy measurements. Named evaluator/search identities, root-side hundredths-of-pawn material and1m command-edge terminal-distance convention are documented in docs/m4/shogi-engine.md. Actual both-human-side child requests/results/native finalgames and binary/source fingerprints retained in shogi-engine-process.json; final and explicitly superseded exploration measurements preserved.

Reviewed sourcee88a19d46459e4516d0fb918cbb5e1b8152ad6e3 integrated/pushed via36e570f8396c6fae1a70d88c8427e5ca8ca4665b. Independent AI source/docs review and19 focused native/protocol/actual-process checks PASS, no actionable findings. Final native318, QA-session46 and Tauri2 tests PASS; source-native CI40352695690 and desktop CI40352695658 PASS Linux/macOS/Windows. Desktop Linux needed one unchanged-source retry after cancelled slow dependency setup; no failed product check hidden. Chessv2/records/observations, accepted67profile/history/proof, MSH61 persistence and roadmap intact. MSH71 USI and72 shared desktop sessions are newly eligible and unstarted; stop70, M4/41–47 open.
<!-- SECTION:FINAL_SUMMARY:END -->
