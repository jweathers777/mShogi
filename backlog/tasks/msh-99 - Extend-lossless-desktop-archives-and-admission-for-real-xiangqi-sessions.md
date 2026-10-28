---
id: MSH-99
title: Extend lossless desktop archives and admission for real xiangqi sessions
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-28T15:11:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-98
  - MSH-95
  - MSH-73
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 99000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Persist complete native xiangqi semantics plus desktop policy/settings/observations with exact game/profile/schema/transport identity, root provenance and commands/events; strict duplicate-preserving dispatch and native replay own validation. Preserve old chess/shogi documents.
- [ ] #2 Measure actual encoded bytes, native events, controls, observations and replay work; enforce native-owned admission before accepting any work that would destroy a complete save path, including reserved stop/close. Cover exact boundaries and observation256-to257; no silent discard or constant-only fix, preserving MSH-61.
- [ ] #3 Atomic failed/interrupted saves preserve prior valid file and dirty state; corrupt/unsupported/resource-limited opens preserve the live game. Fresh execution identities on reopen, historical observations never pending work, untimed engine stopped until explicit Resume; protect unsaved replacement/quit.
- [ ] #4 Run actual native payload roundtrips, malformed/resource/failure boundaries and historical-prefix provenance tests plus current archives regressions; record independent review. Common migration belongs to MSH-46 while native replay remains game-owned.
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
1. Extend existing tagged archivev2 with explicit Xiangqi dispatch and native-owned settings/journal; retain Chessv1 writes and existing v1/v2 Chess/Shogi reads unchanged. RawValue carries original payloads into strict canonical/native replay; no map normalization or trusted derived conclusions.
2. Add concrete Xiangqi cloned-candidate command/response admission on the existing bounded persistence worker, with actual encoded archive/context/transport sizes, native512-command/2048-event/1MiB limits,256observations/8192controls/16MiB archive, bounded weighted replay and reserved close/stop headroom. Measure realistic long warning/claim/history/observation records before settling the weighted policy; preserve rejected prestate and complete save path.
3. Persist document orientation/analysis defaults, full initial/current canonical ledgers, exact control/observation ordering and history selection. Restore via native replay off controller lock with fresh runtime identities and paused work; reuse atomic write and stale Open/revision guards. Keep normal product Xiangqi UI admission gated until its existing owners while enabling native archive consumers.
4. Retain completed native exact reports only as explicitly historical attachments with source record, exact event/position/goal/limits/identity/provenance; validate strict shape/native target and bounded witness replay without executing expensive graph analysis or installing an outcome. No new desktop exact execution capability.
5. Add native payload/controller fixtures for warnings/fourth occurrence/claims/agreements/resignation/same-board history/fast+exact evidence, save/open/history/Resume and stale responses; exact capacity/atomic/interruption/malformed/compatibility tests plus reproducible size/replay measurement. Independent completed-diff review, focused and relevant shared regressions/CI, normal integration/push. Preserve MSH111 GUI failures; stop99, leave76/101/102 untouched.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup06f629ff181c7bcac0bf5c176965ffa91a6ec080 on clean primary codex/rewrite refreshed origin equal. Dependencies98/95/73Done; no active competing owner. Root sole builder/integrator uses primary checkout and reusable caches; read-only archive99_audit researches limits/fixtures, separate completed-boundary reviewer later. InstalledCLI1.55.0 differs; pinned1.55.1 used for all mutations. Existing98/100 worktrees remain queued app-managed archival, no new ownership there. User-authorized99only; no76/101/102implementation or111driver work.

Implementation in progress: explicit Xiangqi archivev2 payload with engine transport3/native canonical identity, immutable candidate admission on existing worker, exact event journal and historical-only completed native exact evidence. Existing12native98tests and15Chess/Shogi archive regressions passed after adapting acceptance waits to stable async receipts. Native frontend fixture regeneration unchanged. Measurement found real initial-policy flaw:100board-action natural-count record+35observations saved but Open exceeded15-second worker deadline; this is a FAILED measurement, not supported capacity. Retain16k work ceiling, add measured12units per native board action to4/event and skip redundant empty-PV reconstruction. Raw probes charge maximum of command-derived/event-derived counts before strict native replay, and checked per-control prehistory prevents undercharged reconstruction. No constant or deadline raised. Final corrected measurement and independent review pending.

Native implementation and focused evidence ready. Existing v2 dispatch adds explicit Xiangqi payload with required engine transport3/play limits1/128, complete canonical initial/current records, settings, exact ordered event/command/observation bindings and historical native exact evidence. Chessv1 writes/Shogiv2 payloads unchanged. Native candidate replay/transition/capacity checks occur on existing bounded worker; invalid commands preserve running work until commit. Product Open explicitly rejects Xiangqi until UI owners complete; native shared archive APIs admit it.

Measured initial4/event-only policy FAILED100ply+claim/35observations Open at15s despite successful Save. Corrected16k ceiling nowcharges4/event+12/boardaction (plus actual reconstructed prehistory, observation8 and witness actions), no ceiling/deadline raised. Corrected actual pretty archives: warning+40claim-audits89events/32obs/67controls834597bytes reopen1073ms;100boardactions+Redclaim103events/7obs/17controls227553bytes reopen2024ms. Independent reruns1201ms/2046ms. Tests exercise256ordinary and1ordinary+255exact thenreject257,8191active+reservedclose/8192closed,16000vs16001work and16MiBencodedOpen vs+1. Actual pretty canonical wrappers including indentation have separate1MiB guard; raw probes never normalize malformed data.

Independent separate reviewer xiangqi98_review audited losslessness/native replay/RawValue/capacity/close/fresh identities/host compatibility and reproduced P2 using actual supervised hostile child: consumed invalidPV response rejection could restore Thinking forever. Fixed by retaining invalid native PV as annotated unscored rejected observation without applying first move, and retiring any other consumed-response refusal; no fallback outcome or automatic retry. Original public-child repro nowFailed/paused with0plies unchanged+retained rejected evidence. Independent14archive tests+2consumed-response regressions+15existing Chess/Shogi archive tests+6coreExact tests passed; final wrapper test1passed after correcting its test-only normalizing helper to originalRawValue. No unresolved material review finding. Initial full-feature run118passed/1old synchronousReplace/savefalse assertion failed; updated only that test to wait stable99receipt/assert newSave capability. Subsequent121all-feature tests passed before final wrapper guard/test. Final exact-source122tests, strict Clippy/fmt, actual child probe and CI pending. InitialClippy collapsible-if and transient test-field typo corrected; failed attempts not claimed passed. Local macOS host check Rust1.90 passed; unchanged21vendoredTao warnings preserved. Native snapshots regenerated unchanged, releaseengine/native supervised probe passed. Test-author archive99_audit wrote isolated tests, not the independent reviewer. Main source/docs/root integration owned by @codex.

Actual gated native/process/archive probe passed with real release child: play, live/history analysis/cancel, Save at historical selection, acknowledged shutdown, fresh Chess-controller Open as Xiangqi with exact event-index/board-ply/context equality, historical observations and no work, ReturnLive, explicit Resume and continued human/engine play with fresh identities. Independent reviewer separately ran extended probe successfully and approved verification-only change; suggested explicit prefix equality assertions are now added and passed. This is not GUI evidence. Current primary includes separately integrated MSH-111 d85d9e1/f2a7f14 with its reviewed Linux14-phase success; prior failed campaigns remain preserved and unchanged. No MSH-111 implementation performed by99.
<!-- SECTION:NOTES:END -->
