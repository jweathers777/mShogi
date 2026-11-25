---
id: MSH-268
title: Remove Game Size and Engine Request Limits
status: Ready
assignee: []
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-25T12:46:00Z'
labels:
  - core
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 3800
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John (2026-11-25): no limit on game size or on requests to the engine; the limits are arbitrary and came from early overengineering. Remove: the per-game record byte caps (Tori and Chu 512 KiB; Shogi, Xiangqi and Tenjiku 1 MiB); the 1 MiB saved record wrapper cap (core native.rs, console record_io.rs); the 1 MiB engine request cap (mshogi-engine MAX_REQUEST_BYTES in server.rs and bridge.rs, runner experiments check); the 16 MiB desktop saved document cap (MAX_ARCHIVE_BYTES); and the 4,096-byte input line cap where it limits a game record or a UCI/USI position command (console MAX_LINE_BYTES). Check the current code for any other cap on the length or bytes of a game and remove it. Keep game rules that end a game (for example the Shogi move-count rule), cancellation, time limits, search limits, and limits that do not depend on game length (diagnostic text, position strings, root metadata).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 No code path refuses a game, a saved file or an engine request because of its length or byte size; the listed constants, capacity errors and the capacity messages in the user interface that become unused are removed; production line counts before and after are recorded.
- [ ] #2 Tests show that a game past each old limit can be played, saved, reopened, sent to the engine and entered over UCI or USI where that protocol exists; tests of the removed limits are removed.
- [ ] #3 Game rules and their expected test results are unchanged; documents (docs/native-resource-admission.md, docs/design.md and affected contracts) state that there is no size limit; workspace, frontend and affected-platform checks pass.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John (MSH-209, 2026-11-25): may the per-game native record limits become one limit? Recommendation: yes. Use one limit for every game, equal to the room for the payload in the shared envelope (1 MiB minus the envelope header), and check it in one place in the shared record code. Tori and Chu games can then grow about twice as long before a refusal; Shogi, Xiangqi and Tenjiku lose only the header room; no game rule changes. Set the task to Ready when John approves.

Finding, 2026-11-25: both 1 MiB caps are our own choices (MSH-68, MSH-69); no recorded reason for the number. The engine request cap (mshogi-engine MAX_REQUEST_BYTES) is also 1 MiB and a full request carries the record plus overhead, so a record just under 1 MiB can be saved but may be refused by the engine. Revised recommendation, waiting for John: one record limit of 1 MiB for all games; derive the saved wrapper and engine request caps from it plus a fixed overhead allowance.

John, 2026-11-25: I prefer no limit on the game size or the requests to the engine. This replaces the earlier question and recommendation. Accepted consequence: a very large or damaged file or input can use much memory; loading stays cancellable. Not in this task: the 2 MiB engine response cap and the 4,096-byte stderr and diagnostic caps. Very long games load slowly until MSH-250, MSH-251 and MSH-227 are done.
<!-- SECTION:NOTES:END -->
