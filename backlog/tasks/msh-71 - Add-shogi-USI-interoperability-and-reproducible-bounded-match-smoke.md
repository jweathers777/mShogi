---
id: MSH-71
title: Add shogi USI interoperability and reproducible bounded match smoke
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T18:11:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-70
  - MSH-68
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/shogi-verification.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 71000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Actual USI processes, declared-rule option admission, bounded opponent/self-play transcripts and offline event replay; opponent/source lineage and limitations recorded.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A separate USI adapter exposes real shogi native search with handshake/readiness,SFEN+complete history,move/drop/promotion,resign/win,stop/gameover/quit and a documented supported subset while UCI chess stays available.
- [ ] #2 Host shogi authority validates returned actions and declarations under the recorded profile; protocol draw notification is distinguished from game outcome,score and replay policy. Process/transport failures are distinct from named competition penalties.
- [ ] #3 Bounded self-play and a recorded match against pinned independent opponent candidate YaneuraOu0f421391ae311df9df57dcfcb2b4a78a1972bdf8 (or independently reviewed justified replacement) preserve binary/build/options/profile,commands,limits,observations and native conclusions.
- [ ] #4 Invalid move,false declaration,missing/late/malformed reply,timeout,cancellation and process cleanup have explicit reports and actual process fixtures. Work-limited rerun claims are conditional; recorded event replay is deterministic.
- [ ] #5 Untimed experiments are supported first. Unsupported clock/event profiles fail explicitly; any timed support must define and verify its game-specific timing policy before claiming it. MSH50 retains strength measurement.
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
1. Primary builder/integrator @codex owns clean sole main checkout codex/rewrite at4d0d93b. Dependencies68/70 and common follow-up90 Done, no competing owners. Independent read-only protocol/fixture audit and external pinned-opponent build/admission delegated; no product code delegation or future72+ work.
2. Implement separate native USI adapter behind --game shogi usi (bare usi selects shogi, explicit chess rejects). Preserve existing UCI, human adapters/records/board and raw structured dispatch. Strict startpos/SFEN+full moves admission is atomic; preserve imported counter/provenance and no inferred prehistory.
3. Use one bounded persistent shogi worker with one replaceable pending request/result slot, exact game context/limits/request/version identity, cooperative stop and distinct retirement. isready remains responsive; position/go/newgame/gameover/quit retire old publication. Stop retains legitimate completed iterations; no usable result/native failure emits bounded protocol diagnostic+stderr and exits nonzero without fallback/resign/score. Finite defaults2/5000,caps4/20000; optional infinite holds bounded observations until stop. No clocks/ponder/tsume. Native terminal scores use info strings (not false mate); PV includes only board prefix; root checkmate resignation is native-only.
4. Add separate shogi experiment run/replay commands in existing native runner, reusing bounded subprocess transport and hashes without changing chess contracts. Strict host admission for move/win/resign; false declaration is native concluded loss, illegal/malformed/missing/late/timeout are unscored experiment failures. Root+complete history, options/advertisements, traffic/work/elapsed/cleanup/observations and exact canonical games retained. Experiment move/time cutoffs are incomplete, not rules draws; offline replay validates native events and result.
5. Actual subprocess fixtures cover handshake/options/readiness, supplied history/drop/promotion, declarations/resignation/terminals, malformed/illegal/missing/late replies, stop/replacement/newgame/gameover/quit, zero/insufficient work and watchdog/cleanup. Reuse independently established67/70 roots/expected facts; separate read-only final implementation/runner review.
6. Run bounded paired self-play and admitted pinned YaneuraOu experiments on actual Mac, save complete reports and replay offline; retain build/compiler/binary/options and any failed attempts. Qualify repeatability. Run native/shared/console regressions and normal CI, integrate/push exact source plus separate completion evidence, stop71;72–75 unstarted,41–47/M4 open.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup4d0d93b main clean, only worktree, global CLI1.55.0 differs so using verified pinned1.55.1 path. Read-only protocol reviewer used Shogidokoro/original USI specs and accepted independent history/search fixtures; no profile decision required. External admission built unmodified YaneuraOu0f421391 materiallevel1 APPLEM1 with Appleclang21, binary74eded93711841dbdef41c0d38c8b85de9af77b9925274269c7b42a6d3987f86, actual handshake/options/readiness/search/stop/declaration succeeded. Provenance /private/tmp/msh71-opponent-admission*.json, primary builder will retain durable evidence. Initial sandbox fetch DNS failure followed by authorized escalated successful exact-pin fetch; build first attempt succeeds. This is external opponent/protocol evidence on Mac only, not rules-oracle/strength/other-platform execution.

Implemented concrete USI adapter and shogi-run/shogi-replay in native runner, retaining existing chess interfaces and native rules/search. USI finite defaults2/5000,caps4/20000; genuine no-result emits info execution_failure/stderr and exits1, never fallback/resign. Stop retains last completed work; replacement/newgame/gameover/quit retire exact native history/request/version/limits. isready responsive; go infinite holds bounded work, go mate returns standard checkmate notimplemented, clocks/ponder unsupported. Native decisive values not mislabeledmate; PV truncatesbeforeDeclare; host/native admissibility owns move/win/resign and false declaration loss. One fresh child per runner turn, complete position suffix and process generation prevents untagged stale-result reuse. Common subprocess validation extracted without chess policy coupling; bounded existing transport/cleanup reused.
Actual Mac admission pinned YaneuraOu0f421391 materiallevel1 APPLEM1/Appleclang21, expectedsource tree0844b429 andbinary74eded93711841dbdef41c0d38c8b85de9af77b9925274269c7b42a6d3987f86. All advertised requestedsettings independently checked (CSA27,1thread,64hash,no books/ponder,alllegal,ResignValue99999,MaxMovesToDraw0,drawvalues0). docs/m4/usi/{opponent-admission,opponent-admission-supplement}.json retains actualbuild/compiler/flags/options/traffic/fetchDNSfailure and scope. No replacement or external code bundling.
Final actual selfplay6/opponent6 each contains2standard8-action cutoff/unscored,2nativeSentecheckmate-drop wins and2nativeSenteEnteringKing wins, pairedengineassignments;20acceptedrequests perreport. Native6game repeat canonicalrecords equal; external reruns neither tested norpromised. Source/binary/report hashes/environment in execution-manifest.json. Separate realnativezero-work2, instrumentedlate2 and retirement-capture2 are unscoredfailures with0acceptedcommands;8supported retainedreports includingearlierexploratory total36games replayoffline. Opponentexecution actualMacarm64only; futureCIisproduct/fixture/platformevidence, notexternalopponentexecution.
Independent read-only protocol/fixture audit established USI score/PV/declaration semantics and reused acceptednativefixtures before implementation, executed3baseline sentinels and7finalnativeUSIprobes. Separate source reviewer rebuilt console80+runner31 at an earlierfreeze (111PASS), replayed allthen34games, and found2material boundaryissues. Fixed rawTABvsescapedtranscript replay comparison withactualprocess regression; fixed delayedduplicate afterdrain beforeexit0 byrequiringretirement EOF beforeacceptance. Retained pre-fix reviewcounterexample/probe andfinalrejectedcapture report; no native ruleexpectation changed. Initial stop test wronglyassumedfirstiteration completion after200ms; corrected tohonest completed/incomplete alternatives; initial misplacedcaptureflag bugfixed byfocusedtests. Final runner14/USI11 focusedtests passed; finalworkspace run and finalreview inprogress. Docs/README examples and contract updated; verificationfixture excludedfromnormalpackaging byexistinginspection. MSH72–75 unstarted;41–47/M4open.

Final local verification on macOS 27.0.1 arm64, Rust 1.89.0: cargo test --workspace --locked passed 353 tests in 53 groups, zero failures. Workspace all-targets Clippy -D warnings, cargo fmt --all -- --check and git diff --check passed. Release production engine/runner builds and actual final experiments/replay passed; source and binaries match docs/m4/usi/execution-manifest.json. Updated native CI includes explicit USI handshake smoke; recorded native reports replay cross-platform without opponent execution. Normal package inspection rejects fixture/test-driver paths; new USI fixture is separate from normal engine bundles.
Final independent read-only AI implementation reviewer /root/msh71_boundary_review inspected USI lifecycle/result mapping, native admission, runner deadlines/cleanup, full-history identity, strict replay, settings and evidence. Separate CARGO_TARGET_DIR=/private/tmp/msh71-boundary-review-target cargo test --locked -p mshogi-console -p mshogi-runner passed 113 tests (console80/runner33). Independently replayed all 8 valid reports/36 games, rejected the intentionally invalid pre-fix full report, and reran original TAB/late-cleanup-duplicate probes: corrected expectations passed. No remaining actionable findings. Final reviewed USI SHA256 60dbb25a01988aebcc90d08d60df98a5f4c7cad643e6affeb63f029982f3c6ac; runner shogi.rs e99773a3d118ec59a617c81b6a8c4d4fe6648e6da3bc4944973369956061b6aa. Independent protocol/evidence auditor also verified actual binary/source/options, paired assignments, provenance/conclusions/scores, complete normal traffic, failed-work isolation and conditional repeatability; seven actual native USI probes passed. These are AI reviews plus separate actual execution, not human or additional-platform opponent evidence.
Normal integration/push and current-source CI pending. Primary checkout is the only worktree; no temporary checkout to clean. Preserve MSH90 and all completed inputs unchanged. Stop at71, with72 next in planned desktop sequence.
<!-- SECTION:NOTES:END -->
