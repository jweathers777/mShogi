---
id: MSH-30
title: Connect the bundled engine through a structured supervised interface
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T13:07:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-27
  - MSH-21
  - MSH-22
documentation:
  - docs/desktop/m3-design.md
ordinal: 30000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
UCI cannot faithfully encode all native claim and incomplete-search decisions; desktop play needs a lossless boundary and isolated process execution.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A small versioned internal interface reuses existing native search and coordination, carries complete game/request/version/generation/limits context, and distinguishes board action, claim, concluded game, incomplete search and failure; UCI remains available.
- [x] #2 Process supervision keeps launch/I/O/wait/cleanup off the interaction path, bounds queues and streams, supports cancellation/replacement/shutdown and reports launch, exit, malformed, missing, timeout and search failures without chess scores.
- [x] #3 Focused protocol and actual subprocess tests verify typed results, bounded input/output, failures, cancellation and context echo; independent review and native CI evidence are recorded.
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
Add a typed versioned one-shot JSON search mode using existing native coordination; add a bounded single-worker process bridge with cancellation, deadlines and stream limits; verify native subprocess boundaries and UCI compatibility.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner codex + /root/desktop_engine; branch codex/m3-desktop; checkout .claude/worktrees/next-task. MSH-27 contract independently reviewed and integrated.

Author verification: 12 new engine/protocol/process tests and32 unchanged CLI/UCI tests pass; focused clippy alltargets -Dwarnings passes. Actual subprocess checks include cancellation/replacement, published-result retirement, bounded streams, missing/nonzero/malformed/context mismatch/timeouts and shutdown. One-shot internal-search preserves native results and complete GameRecord; Full refused before replay. Root full workspace lint/tests pass. Independent process review and native CI/integration pending.

Independent review /root/desktop_foundation reran12 new tests, all passed, and found no actionable bundled-engine boundary issues. Exact context, strict JSON, Full refusal, typed results, cancellation/publication and off-path stream/process bounds checked. Direct-child cleanup scope and possible detached pipe helpers after deadline are explicit in docs/desktop/internal-engine-v1.md; no arbitrary descendant containment is claimed.

Integrated source 074fe515b9c9b9ab65dfc9a256dcbf0d07eb3132 on codex/rewrite. Native Rust CI https://github.com/jweathers777/mShogi/actions/runs/40196021766 succeeded on ubuntu-24.04 (121176901775), macos-15 (121176901942), windows-2022 (121176901969): fmt, workspace clippy, tests, build and console smoke. Existing M2 evidence reused; desktop package CI remains MSH-28 evidence.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added the lossless versioned internal-search protocol and bounded off-path bundled-engine supervisor while preserving UCI. Twelve new boundary/process tests, existing CLI/UCI checks, independent review and all three native Rust CI jobs pass. Direct-child cleanup scope is documented.
<!-- SECTION:FINAL_SUMMARY:END -->
