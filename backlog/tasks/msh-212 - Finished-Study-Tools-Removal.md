---
id: MSH-212
title: Finished Study Tools Removal
status: To Do
assignee: []
created_date: '2026-11-20T22:29:00Z'
labels:
  - tools
milestone: m-5
dependencies: []
ordinal: 219000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: remove tools that only served finished studies and whose inputs were in evidence directories removed by MSH-211 (Git History Cleanup): the MSH-161 Chess study runner (tools/experiments/chess.py, the study parts of cli.py and core.py, tests/test_chess.py, status-settings.json, quick_status.py if unused), tools/strength/extract_blunders.py, make_openings.py and request_overhead/summarize.py. Keep tools/experiments core.py and gcp.py, which the strength tools use.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 No tool or test refers to a path under docs/*/evidence/
- [ ] #2 The remaining strength cloud tools import and their unit tests pass
<!-- AC:END -->
