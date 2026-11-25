---
id: MSH-268
title: One Native Record Size Limit
status: To Do
assignee: []
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-25T12:23:00Z'
labels:
  - core
  - needs-john
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 7880
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Native record byte limits differ by game with no recorded reason: Tori and Chu 512 KiB (tori/record.rs, chu/record.rs), Shogi, Xiangqi and Tenjiku 1 MiB (shogi/record.rs MAX_RECORD_BYTES, xiangqi/record.rs, tenjiku/record.rs), Chess has no limit of its own. Every record must also fit in the shared envelope of 1 MiB (native.rs MAX_RECORD_ENVELOPE_BYTES), so a game record of 1 MiB cannot be wrapped. Xiangqi also refuses a command when the record is within 4 KiB of its limit. A user can reach these limits in a very long game, so a change needs John's approval.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 John's choice is recorded in the notes.
- [ ] #2 One limit rule applies to all games and is checked in one place; the per-game constants are removed; tests and documents state the limit.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John (MSH-209, 2026-11-25): may the per-game native record limits become one limit? Recommendation: yes. Use one limit for every game, equal to the room for the payload in the shared envelope (1 MiB minus the envelope header), and check it in one place in the shared record code. Tori and Chu games can then grow about twice as long before a refusal; Shogi, Xiangqi and Tenjiku lose only the header room; no game rule changes. Set the task to Ready when John approves.
<!-- SECTION:NOTES:END -->
