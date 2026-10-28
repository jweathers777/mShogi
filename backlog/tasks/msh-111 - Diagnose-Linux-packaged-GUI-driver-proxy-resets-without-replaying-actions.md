---
id: MSH-111
title: Diagnose Linux packaged GUI driver proxy resets without replaying actions
status: To Do
assignee: []
created_date: '2026-10-28T13:58:00Z'
labels:
  - ci
  - verification
milestone: m-4
dependencies: []
references:
  - https://github.com/jweathers777/mShogi/actions/runs/40692691108
  - https://github.com/jweathers777/mShogi/actions/runs/40698788452
documentation:
  - docs/m4/xiangqi/session-contract.md
ordinal: 111000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Linux tauri-driver2.0.6 intermittently loses the downstream connection with Hyper SendRequest/ECONNRESET. Baseline74a639 and integrated2b50ac2/6f6ea69 show this on screenshots, DOM reads and New game clicks, followed by successful same-session captures. Aborted journeys cause consequential missing-save errors. Native, frontend/host and package checks pass. Pooling is a hypothesis, not an established cause. Diagnose separately without gating unaffected Xiangqi delivery or changing game semantics; MSH-98 owns native session contracts rather than this infrastructure campaign.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Concretely diagnose or reproduce the failures with exact driver, WebKit, package, source and environment provenance; distinguish proxy failures from application/native IPC failures and retain original failed evidence.
- [ ] #2 Any correction is external to normal distributables, bounded and independently reviewed. Never automatically replay ambiguously delivered mutations, input or lifecycle actions; semantic errors and exhausted observations remain fatal and all journey assertions remain intact.
- [ ] #3 Record a complete successful actual Linux installed/packaged Chess/Shogi GUI campaign across all current phases, including play, analysis/cancellation, claims, recovery, Save/Open, history and native close/quit. Distinguish driver tests, package checks and actual GUI evidence.
- [ ] #4 Run focused boundary tests and appropriate compatibility checks for shared verification changes; record exact tested/integrated revisions and CI including failures, without inferring other-platform success.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
