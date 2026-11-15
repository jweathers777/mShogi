---
id: MSH-176
title: Compare the legacy and present Chu engines
status: Ready
assignee: []
created_date: '2026-11-14T14:14:00Z'
updated_date: '2026-11-15T16:27:00Z'
labels:
  - handle:legacy-runner-adapter
milestone: m-5
dependencies:
  - MSH-173
references:
  - crates/mshogi-runner/src/experiments/mod.rs
  - crates/mshogi-runner/src/experiments/run.rs
  - docs/m4/chu/engine-v1.md
  - docs/m5/strength-roadmap.md
priority: medium
type: task
ordinal: 180000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Legacy Runner Adapter.

**Problem:** It is not known how the present Chu engine compares with the original C++ engine. The present engine is still the first full-width search, limited to depth 2 and 2,000 nodes with material-only evaluation; the legacy engine will probably win now. Chu moves to the shared search core in later work, so the comparison is most useful as a baseline that can be repeated then.

**Objective:** A measured result of the present Chu engine against the legacy engine of MSH-173 (Frozen Legacy Opponent), with commands that repeat it later.

**Approach:** Connect the legacy opponent to the existing experiment runner (`mshogi-match experiment-run`), with the native Chu rules as the authority for every move and result. When the legacy engine proposes an illegal move, rejects a legal one, claims a different result or fails, the game is not scored, and its cause is recorded; it is never a win or a loss. Play both colors. The present engine has only depth and node limits, so measure its time per move at its largest setting and give the legacy engine about the same time; record both. If one engine wins nearly all games, a small sample is enough; if the result is close, follow "Game tests in this direction" in `docs/m5/strength-roadmap.md`. Note the runner limits (at most 16 openings and 128 games for each configuration, and a report size limit); several seeded configurations can be combined.

**Completion:** The result is recorded with its uncertainty and unscored games, and the commands repeat it after Chu moves to the shared core.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The runner plays the legacy engine against the present Chu engine with both colors; the native rules decide every move and result, and each disagreement or failure is an unscored game with its cause
- [ ] #2 The settings make the time per move comparable, and the measured median time per move of both engines is recorded
- [ ] #3 The result records wins, draws and losses, unscored games by cause, an interval and actual compute, and states only what the sample supports
- [ ] #4 Recorded commands repeat the comparison, and the reports replay without either engine binary
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ticket revision, 2026-11-15 (requested by John): this task now delivers the comparison itself, not only a smoke test. It absorbs the needed part of MSH-175 (Opponent Adjudication Policy): disagreements and failures are unscored with their cause. It depends on MSH-173 only. MSH-174 and MSH-175 are archived.
<!-- SECTION:NOTES:END -->
