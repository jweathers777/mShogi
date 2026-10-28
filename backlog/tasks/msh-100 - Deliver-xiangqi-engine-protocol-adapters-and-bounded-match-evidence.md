---
id: MSH-100
title: Deliver xiangqi engine protocol adapters and bounded match evidence
status: In Review
assignee:
  - '@codex-protocol'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-28T12:55:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-96
  - MSH-97
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 100000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Choose and document the minimum real external dialect(s) from admitted opponents; pin rank/alphabet/FEN/options differences (Pikafish UCI0-9 versus Fairy generic UCI1-10/UCCI0-9). Preserve structured native functionality and do not advertise unsupported conversions.
- [ ] #2 Actual supervised handshake/position/go/stop/quit and bounded runner uses native match authority and complete known history. Explicitly report invalid replies, unsupported adjudication policy, missing responses, timeout/process exit and unscored unresolved work separately from native game outcomes.
- [ ] #3 Record own-engine self-play and a bounded match against an independently built/admitted Xiangqi engine with exact source/binary/network/options/work policy and replayable commands/moves/events. Deterministic replay is required; rerun equality only under documented deterministic settings/limits, never promised for timed observations.
- [ ] #4 Execute protocol-negative/cancel/cleanup/replay tests and independent review. Audit/licensing/source facts alone do not admit an opponent or establish playing strength.
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
1. Audit current admitted opponent sources/binaries/network and current UCI/USI process/runner seams; select the minimum evidenced real Xiangqi dialect, documenting rank/alphabet/FEN/options and unsupported conversions without changing selected WXF rules.
2. Add bounded native-game-owned external protocol adapter and runner, using full known native history, actual measured97limits and explicit claim/no-win policy; supervised handshake/position/go/stop/quit with cancel, generation/identity retirement, bounded I/O/diagnostics and no fallback result.
3. Record source/binary/network/options/work provenance for actual own-engine self-play and an independently built/admitted opponent match. Native commands/outcomes own results; replay exact commands/moves/events and separate replay determinism from timed rerun claims.
4. Run protocol-negative/cancel/cleanup/replay/nativefixtures and relevant compatibility checks; independent review, then root integrates completed98/100 branches after both reviews. No desktop/session/inspection/native-rule/search redesign or product UI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Before source root inspected full98/100 records/deps/ownership/fileboundaries. Parallel independent consumers:100 has96/97Done and no98edge;98 has84/95/97/72Done. Isolatedworker @codex-protocol in appmanaged /Users/johnweathers/.codex/worktrees/xiangqi-protocol100/mShogi on codex/msh100-xiangqi-protocol; root soleintegrator and98builder separatebranch. Own consoleexternalprotocol/mainselectors, runnernewXQmodules/productionbinary entry/tests, protocol-specificdocs/evidence. Do not edit desktop-session/coreinspection/sharedengine/search or globalhandoff/coverage/task98. Escalate actualsharedboundaryneed tointegrator before touching; no dependencies changed forparallelism. Sourcebasee923cfd5712698d4a8b059ffb3b37b0e31de8f8b; pinnedBacklog1.55.1. Refinecurrentplan afterworker inspection beforecoding ifnecessary; root owns finalCI/Done/integration/push. Stop100, no later tasks.

Implemented isolated Pikafish-compatible UCI0-9 and native Xiangqi runner; no98/core/search/sharedengine changes. Seven actual console protocol and seven runner tests pass, reusing native94/97/105 expectations (Red/Black stalemate, history-sensitive warning/fourth loss, claimant-specific natural count). Full console+runner compatibility tests and strict Clippy passed on macOS arm64. Runner rejects unsupported live non-board histories/provenance/counters rather than losing context; terminal canonical records retain all native commands. Actual independent unmodified Pikafish9d7833aa build succeeded with fetched50,241,164byte NNUE sha256cd396741e9e0ffd22043216c5f8aed5d8960be6422ddefc4a5bc09a5344fec14 verified against upstream release asset684051879. Runtime admission/match evidence and independent review pending; not yet complete.

Independent reviewer /root/xiangqi100_review found replay accepted contradictory retired bestmove evidence even though runtime rejected it; fixed accepted-generation replay with ordered phase/command/token state machine and six new tamper cases. All8runner tests pass. Reviewer independently executed real ownengine selfplay+replay, matching native canonical games/actions/context digests under finite deterministicsettings. First pinnedOct26Pikafish actual opponent match failed cleanly/unscored because mutableOct28network architecture differed; preserved exact report/config under protocol/exploratory/incompatible-network*. Investigated raw headers/hash and historical release network. Selected immutable officialSep06release source4c17cee11f888ae1d48a9494f2e2239f019f0a1f plus its matching archive NNUE; independently rebuilt clean source, actual search now loads64MiBnetwork and returnslegal b2e2, cleanexit0. Same UCI0-9/FEN bounds and no selectable WXFprofile; added exact source/archive/binary/network/license manifest, no product rules/search changes. Full runtime admission/matches and finalreview stillpending.

Follow-up independent review reproduced missing option advertisements accepted on replay although runtime rejected them. Resolved with the same advertised-name/type/value validation as runtime and missing/invalid-advertisement regression. All9runner+2retained-recorded tests pass; seven actual UCI subprocess tests previously pass. Reviewer independently reran actual selfplay and compatible-source opponent4games, confirming canonical games/actions/context/position/go equality with retained reports. Verification helper finalverdict now requires every replay pass and no game failures, and admission script explicitly verifies native played acknowledgement plus canonical acceptedcommand/boardevent. No native rules/search changes; final docs/review/integration pending.

Branch implementation/review complete, pending root integration+nativeCI and Done. Final production4d6481bffbb7c83ee9461e4036ccaff71057b6ee; actual main matches source8d6a0dd1851966ef1df3515155730f1abf4c74dd reused after replay-onlyadvertisement fix. Final macOS arm64 cargoClippy alltargets -Dwarnings PASS; cargo test console+runner153tests/25suites PASS incl7newactualUCI+9runner+3recorded suites; releaseproduction build/fmt/diffcheck PASS. docs/m4/xiangqi/protocol/review.md records independent reviewer method, two resolved material replay findings, independent own/opponent actual reruns and no remaining material findings. Exact source/network/license/provenance in protocol/opponent-source.json and opponent-admission.md; finalactualadmission5probes PASS. Twelve successful bounded own/opponent/repeat games33acceptedactions:5native stalematewins,7unscoredcutoffs; replayallPASS and own semanticrepeatPASS. Two actual ownzero-work games at4d6481b correctlyincomplete_work/noaction/noscore/replayPASS. Preserved initial incompatible-network3unscoredfailures+1nativeconclusion and prereviewselfplay/admission as exploratoryevidence. NativeCI/combinedintegration/push/ACDoD finalchecks are sole rootintegrator work; no100branch merge/push. No98/core/sharedengine/search/Cargo.lock production changes. All owned runtimeengine/testprocessesreaped, no neededignored worktreefiles beyond reproducibletargetcache; external admittedsource/network/archive retained outsideworktree under/private/tmp/msh100-* for reproduction.

Root sole integration:2ae336819284ac24df8ec3a86c1c1b605ac73bd6 into codex/rewrite from reviewed clean98873ec5b8586b42500a4e5cf84deb73c853d674 after98 source16c6827; no conflict or dependency change. Reviewer independently approved final988 including third retained-record test3/3, with production unchanged4d6481bffbb7c83ee9461e4036ccaff71057b6ee. All current production work reviewed; final combined-source verification and native/desktop CI pending. Task remains In Review.
<!-- SECTION:NOTES:END -->
