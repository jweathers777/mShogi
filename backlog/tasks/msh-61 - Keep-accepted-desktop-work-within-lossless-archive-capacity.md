---
id: MSH-61
title: Keep accepted desktop work within lossless archive capacity
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-25T17:48:00Z'
updated_date: '2026-10-25T18:20:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-35
documentation:
  - docs/desktop/m3-design.md
  - docs/desktop/archive-v1.md
ordinal: 61000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Repeated accepted analysis and engine observations can currently make both Save and Save As permanently fail at 257 observations; saving/reopening retains that count. Byte, event and control growth can similarly destroy the complete save path. Ordinary accepted work must preserve a complete saveable session without deleting evidence or changing chess semantics.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A native-owned admission or lossless storage policy prevents ordinary accepted commands, game changes and observations from exceeding supported byte, event, observation and control persistence limits; complete current history and accepted observations remain saveable without raising a constant alone or silent truncation.
- [ ] #2 Capacity rejection or pause is a distinct application resource state, not an illegal move, engine failure or chess outcome; clear actionable UI preserves Save/Save As and never recommends retrying a structurally impossible save.
- [ ] #3 Focused boundary tests include observation 256-to-257, byte/event/control boundaries, engine and analysis acceptance, save/reopen without resetting count, cancellation/recovery/quit and unchanged-session rejection; actual packaged interaction and independent review support the result.
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
Add transactional native admission around canonical document mutations and result acceptance, including count and exact serialized byte budgets with safe headroom for execution/close identities. Keep work off the interaction path when measurement or reconstruction is expensive. Expose a separate capacity state with an actionable pause/rejection UI; preserve complete accepted evidence and successful Save/Open. Verify exact limit boundaries and actual packaged capacity interaction using isolated instrumentation, independently review, integrate/push and collect native/package CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner @codex, codex/m3-desktop reusable checkout .claude/worktrees/next-task, starting source 7c7df45. Required first item in final M3 batch, before MSH-36/37/38/39. Preserve M2 closure, MSH-27–35 Done and roadmap correction40; M4–M6 remain future To Do.

Implementation: complete immutable candidate admission on a separate bounded worker; host waits for stable caller-owned receipts, not whichever admission is current. Resource capacity is distinct in native snapshots/UI and leaves Save/Save As available. Packaged macOS first pass verified real observation255→256, refused257 before execution, complete Save As/Open retaining256, refusal after reopen, explicit NewGame and guarded Cmd-Q. Independent review found SaveAs bookkeeping race and near-limit persisted watermark consumption; regressions/fixes underway before final verification. See docs/desktop/completion-batch-evidence.md; firstpass is explicitly pre-final-identity-fix evidence.

Final focused evidence: 41 all-feature native tests, 9 real-process console integration tests, strict native Clippy, repeated17snapshot generation byte-identical;56 frontend tests and strict Vite build passed. Independent capacity_review reran40 normal-feature native+9process tests; both SaveAs metadata and identity-watermark findings fixed, no remaining material findings. Final normal macOS package opened256-observation archive, refused new analysis as resource capacity, retained all256 and successfully Saved; package-smoke confirms QA exclusion and actual UCI/internal/full bounded engine. Exact package/archive hashes and screenshots docs/desktop/evidence/m3-completion/capacity-local-macos.json; summary docs/desktop/completion-batch-evidence.md. Await integration CI before Done.

Exact normal macOS source3b427d49fb481c9391992d40f047941ae7f445c2 rebuilt and GUI Open256/refuse/Save/clean Cmd-Q repeated; hashes updated. Host2tests/format passed. Initial Rust CI40220415681 macOS+Windows pass, Linux exposed newtest timing assumption (command API may already commit fast candidate before returning). Test now deterministically closes a validated but uncommitted admission; no product behavior change. Independent exacttest+50repetitions passed. Replacement nativeCI required beforeDone.
<!-- SECTION:NOTES:END -->
