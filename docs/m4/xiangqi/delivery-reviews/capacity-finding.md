# F01 — control-only refusal reports nonexistent observations

Disposition: P2, **MSH-114 Ready**, blocks final MSH-103 acceptance.
First found by `/root/review103_archive`, independently confirmed by
`/root/review103_rules` and reproduced by root at541fc303. Native admission,
complete Save, stop reserve and chess-family outcome semantics remain correct.
No production correction was made during this review.

Native `shared_xiangqi_archive.rs::check_capacity` returns this combined reason
for exceeding either controls-with-reserve or observations:

```text
The archive retains256 total ordinary/exact observations and8192 controls including stop/close.
```

Shogi `shared_archive.rs::counts` has the same combined256/8192 form.
`NativeSessionPanels.tsx::NativeRecovery` tests `/256/` before8192 and therefore
selects the observation explanation even at zero observations. The current
native `control_replay_and_native_count_boundaries_reserve_complete_stop_and_terminal_paths`
test builds8191 active controls plus stop reserve, rejects one extra control,
then successfully closes/encodes/decodes the complete8192-control document.
It passed in the independent archive campaign. This establishes actual native
control-boundary behavior; the following probe establishes its frontend meaning.

Temporary file `desktop/src/product/Capacity103.review.test.tsx`:

```tsx
// @vitest-environment jsdom
import { expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { NativeRecovery } from "./NativeSessionPanels";
import type { XiangqiSnapshot } from "./native";
import native from "../__fixtures__/shared-xiangqi-standard.json";
it("control-only refusal must not invent 256 observations", () => {
  const message = "Application capacity: The archive retains256 total ordinary/exact observations and8192 controls including stop/close.";
  const snapshot = { ...native, observations: [],
    capacity: { id: 1, status: "paused", message } } as unknown as XiangqiSnapshot;
  expect(snapshot.xiangqi.exact_observations).toHaveLength(0);
  render(<NativeRecovery snapshot={snapshot} error="" disabled={false} command={vi.fn()} />);
  expect(screen.getByRole("alert").textContent)
    .not.toContain("256 recorded engine observations");
});
```

Executed `npm --prefix desktop test -- src/product/Capacity103.review.test.tsx`
using Node24.19.0. **FAIL1, exit1**, as expected for this truthful expectation:

```text
Received: Application limit reachedThis game has reached the limit of 256 recorded engine observations. Save the complete current game, then start a new game. Saving and reopening does not reset this limit.
```

The exact [failure log](capacity-failure.log) is retained. The temporary probe
was removed after execution; no red test or assertion of incorrect behavior
was added to production. MSH-114 must add durable correct-behavior regressions
for both consumers and actual diagnostic variants, preserve real256→257
admission/stop headroom, verify packaged interaction and obtain independent
review. No raising constants, discarding history, rules change or layout redesign
is required by this finding.
