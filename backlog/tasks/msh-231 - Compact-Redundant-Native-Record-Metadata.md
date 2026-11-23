---
id: MSH-231
title: Compact Redundant Native Record Metadata
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-23T11:05:00Z'
updated_date: '2026-11-23T14:43:00Z'
labels:
  - simplification
  - persistence
milestone: m-6
dependencies: []
priority: high
ordinal: 238000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Split from MSH-229: native.rs plus per-game record codecs repeatedly store game/profile/edition/schema identities, policy strings and replay-derived current positions, effects and conclusions. A saved game needs an unambiguous profile, a complete root and player history; repeated summaries are currently rechecked during every decode. Cut redundant encoded data and reconciliation where no real consumer needs it. Preserve root history/provenance that affects rules and explicitly recorded format compatibility (including the Tori v1 reader); coordinate native consistency findings in MSH-209.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 For each game, identify essential root/command data versus derived or repeated metadata, and remove redundant data in separate reviewed steps
- [ ] #2 New records round-trip all supported profiles, claims, offers and conclusions through native rules; recorded compatibility and independent rules tests remain valid
- [ ] #3 Record encoded size, replay time and source-line changes; pass required Rust/frontend/affected-platform persistence checks after each step
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inventory essential data and capture baseline sizes/replay measurements. Preserve explicit Chess schema 1/2 and Tori schema 1 compatibility.
2. Compact non-Chess native records to versioned profile/root and ordered actor/command data. Keep native replay, provenance, rights and actual command inputs; remove derived summaries. Review and run required checks.
3. Remove redundant outer document/native labels, update consumers and contract documentation, then review and run required checks including affected-platform persistence.
4. Record measurements, merge into main, close the task and remove task outputs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-230 inspection: desktop DocumentEnvelope.native_schema_version is a hardcoded 1 game-payload label, distinct from the nested CanonicalRecord actual native schema (Tori was already 2 before MSH-230). Consider removing or renaming that redundant misleading label when compacting metadata. MSH-230 leaves the unchanged desktop payload label intact and documents the distinction.

Started from main 8df7bf3. Branch codex/msh-231-compact-records. First map essential versus derived data for all six games and inspect the explicit compatibility paths; measure baseline record sizes/replay before changes.

Inventory: Chess history retains FEN, history origin, claim/adjudication policy, ordered moves and real command/clock observations; schema 1 bytes and schema 2 configuration have explicit compatibility promises (docs/rules/chess-history-record-v1.md and chess-completeness.md). Shogi retains root category/SFEN/counter source and actor/command history; policies, score and after-position/event effects are derived. Xiangqi retains PositionRoot, claim policy and every CommandEntry actor/command (including admitted rejected claims); result/event_end/events/current position/assessment are derived. Tori retains root diagram, hands, declared counter and provenance plus actor/command history; Tori v1 reader must remain. Chu retains NativeRoot including Lion protection and actor/command history. Tenjiku retains StructuralState including whole profile, original-piece rights and provenance plus actor/command history. Current conclusions, offers, obligations, effects and work summaries in the last three are native replay results. MSH-209 coordination: this task changes record representations only; resource-admission architecture remains MSH-232 and Xiangqi replay optimization remains MSH-227.

Checkpoint: baseline measurements captured for 155 existing canonical records in runs/msh-231/baseline.csv (base 8df7bf3, Apple Silicon, release -j2). First bounded implementation step is Chu schema 3: profile/edition/schema + complete NativeRoot + actor/command events. Cached effects/current state/conclusions/offer/obligations and constant format/game/mode removed. Native corpus expectations unchanged; tampered-effect fixture now injects the tampered effect as forbidden input. GUI checks use existing typed Chu presentation for effects/conclusions. Fresh review /root/review_231_chu found no issues and independently compared 29 changed fixture records: all roots and actor/command sequences preserved. Python checks: 176 pass. Native tests, generated fixtures and required full checks in progress; other games and outer envelopes remain next. Branch codex/msh-231-compact-records.

Chu step d081761 validation: 1182 workspace release tests passed, 11 ignored; 603 frontend and 176 Python tests passed; TypeScript and fmt passed; required workspace/all-target Clippy passed in routine CI. Targeted persistence CI 45186903614 passed Linux/macOS/Windows. Required debug workspace command reached the same 4 MSH-199 Chu timing failures (30.44s), with unchanged limits/assertions. Fresh review found no issues. The current writer regenerated fixtures; an independent normalized comparison confirmed all 29 roots/actor-command histories and all other fixture data unchanged. 24 Chu measurement samples: canonical bytes 91795 -> 40653 (-55.7%); single-thread release replay geometric time ratio 0.599, repeated after other local tests ended. Baseline records/CSV preserved at gs://mshogi-experiments-216675633801/runs/msh-231/. Next step: Tori/Tenjiku command-only records plus shared strict parsing/command entry primitive; Chess compatibility, Shogi/Xiangqi and outer metadata remain pending.

Second-step checkpoint: Tori schema 4 and Tenjiku schema 3 retain complete roots and actor/command events; effects/current state/conclusions/offers are replay-derived. Tenjiku profile remains explicit in its root. Shared RecordedCommand and strict duplicate visitor remove repeated parser code across Chu/Tori/Tenjiku. Cached compact event-byte accounting and exact undo-size checks remain native. Fresh review found one stale hostile-effect test; fixed by injecting the native effect before mutation and reviewer confirmed. Core library: 196 pass; explicit Tori v1 compatibility: 5 pass; frontend 603/Python176/TypeScript/fmt pass. All 113 regenerated records preserve exact roots/commands and other fixture data. Full workspace and latest envelope rechecks running; platform CI next. Remaining: Shogi/Xiangqi metadata and shared outer envelopes; Chess byte compatibility retained.

Step 2 follow-up: full release validation exposed stale negative probes in tori_cli (duplicating removed game field was a no-op) and tenjiku_consumer (replacing removed conclusion, indexing absent effects). Probes now inject forbidden data or duplicate required edition, retain rejection/live-state assertions, and check that mutations change input. Full release is now running with --no-fail-fast to collect remaining consumers. Latest envelope tests pass, including all 5 Tori v1 cases. Current persistence run 45191048985: Linux/Windows/routine pass; macOS queued. 23 Tori samples are 59395 -> 28961 bytes (-51.2%); 77 Tenjiku samples 654966 -> 324416 (-50.5%); idle replay timing pending. Core/desktop checks of maximum Tori incremental transport remain safe: conservative 1024-full-event bound 488538 bytes < 524288. Next-step Shogi/Xiangqi and reduced envelopes prepared but NOT applied under runs/msh-231/step3, with regeneration scripts there; existing checkout still only Chu/Tori/Tenjiku changes. Branch codex/msh-231-compact-records.

Step 2 final local results: full release workspace 1182 passed/11 ignored; required debug workspace again only reaches the same four MSH-199 Chu timing failures (30.41s). Final envelope tests and five Tori v1 compatibility tests pass. Fresh review also confirmed the corrected tori_cli/tenjiku_consumer probes, with no further findings. Idle release replay ratios versus 8df7bf3: Tori 0.591 across 23 samples; Tenjiku 0.491 across 77. GitHub macOS could not acquire a hosted runner after multiple attempts (capacity notice); Linux/Windows and routine checks passed. macOS-only retry 45194057670 pending. Now applying the remaining Shogi/Xiangqi codecs and envelope compaction; prior saved-format steps remain in commits d081761, ec7c958 and test correction f28b9f5.

Step 3 checkpoint: Shogi schema 2 and Xiangqi schema 2 now store complete roots and actor/command inputs; shared canonical envelope v2 has only version/game/payload, desktop v2 has format/game/payload and stores native records directly. Chess native bytes remain unchanged; legacy Tori1 wrapper reader retained. Fresh review /root/review_231_final found no production defects and independently confirmed all 155 regenerated native roots and command histories unchanged. Four stale generated presentation differences were removed from the fixture update. Consumer migration and full release checks are running; frontend603 pass. Step2 macOS retry also failed to acquire a runner; Linux/Windows and routine checks passed, macOS still pending. Branch codex/msh-231-compact-records; uncommitted step3 work still needs final checks, measurements, platform CI and integration.
<!-- SECTION:NOTES:END -->
