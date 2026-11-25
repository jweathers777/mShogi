---
id: MSH-234
title: Give Tenjiku Work Examples Distinct Build Names
status: Ready
assignee: []
created_date: '2026-11-23T12:31:00Z'
updated_date: '2026-11-25T08:51:00Z'
labels:
  - tooling
dependencies: []
priority: high
type: chore
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Workspace builds warn that mshogi-core and mshogi-search both emit the tenjiku_work example executable and debug symbols. Give the examples distinct names and update their callers so workspace builds cannot overwrite one example with the other.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Workspace builds emit no duplicate-output warning for the Tenjiku work examples
- [ ] #2 Both diagnostic examples and their documented callers remain usable
<!-- AC:END -->
