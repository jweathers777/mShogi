// @vitest-environment jsdom
import {
  cleanup,
  fireEvent,
  render,
  screen,
  within,
} from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { App } from "./App";
import { mockSession } from "./__tests__/session";
import { useNativeSession } from "./useNativeSession";
import type { Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
import movesJson from "./__fixtures__/analysis-moves.json";
import mateJson from "./__fixtures__/analysis-mate.json";
import matedJson from "./__fixtures__/analysis-mated.json";
import possibleJson from "./__fixtures__/analysis-possible.json";
import impossibleJson from "./__fixtures__/analysis-impossible.json";
import unknownJson from "./__fixtures__/analysis-unknown.json";
import cancelledJson from "./__fixtures__/analysis-cancelled.json";
vi.mock("./useNativeSession", () => ({ useNativeSession: vi.fn() }));
const standard = standardJson as Snapshot;
const moves = movesJson as Snapshot;
const command = vi.fn();
beforeEach(() => {
  command.mockReset();
});
afterEach(cleanup);
function session(snapshot: Snapshot) {
  vi.mocked(useNativeSession).mockReturnValue(mockSession(snapshot, command));
}
function inspect(snapshot: Snapshot) {
  session(snapshot);
  const view = render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  return view;
}
it("viewing analysis alone never starts work; bounded move analysis sends its explicit read-only purpose", () => {
  inspect(standard);
  expect(command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({
    kind: "analyze",
    version: standard.version,
    operation: { kind: "moves", depth: 4, nodes: 100000 },
  });
});
it("rejects unsupported work limits without silently clamping them", () => {
  inspect(standard);
  fireEvent.change(screen.getByLabelText("Search depth"), {
    target: { value: "9" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(command).not.toHaveBeenCalled();
  expect(screen.getByRole("alert").textContent).toContain("depth from 1 to 8");
  fireEvent.change(screen.getByLabelText("Search depth"), {
    target: { value: "4" },
  });
  fireEvent.change(screen.getByLabelText("Node budget"), {
    target: { value: "5000001" },
  });
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(command).not.toHaveBeenCalled();
});
it("keeps advanced cooperative reachability a distinct bounded operation", () => {
  inspect(standard);
  fireEvent.click(screen.getByText("Advanced: Full mating reachability"));
  fireEvent.change(screen.getByLabelText("Mating side"), {
    target: { value: "white" },
  });
  fireEvent.change(screen.getByLabelText("Position budget"), {
    target: { value: "50" },
  });
  fireEvent.change(screen.getByLabelText("Transition budget"), {
    target: { value: "100" },
  });
  fireEvent.click(
    screen.getByRole("button", { name: "Run Full reachability" }),
  );
  expect(command).toHaveBeenCalledExactlyOnceWith({
    kind: "analyze",
    version: standard.version,
    operation: {
      kind: "full_reachability",
      target: "white",
      max_positions: 50,
      max_transitions: 100,
    },
  });
  expect(screen.getByText(/not forced-win search/)).toBeTruthy();
});
it("displays real final native observations without applying suggested moves or resuming play", () => {
  inspect(moves);
  expect(screen.getByText(/Suggested move:/)).toBeTruthy();
  expect(screen.getByLabelText("Analysis result").textContent).toContain(
    "not played",
  );
  expect(screen.getByLabelText("Analysis result").textContent).toContain(
    moves.analysis.target!.label,
  );
  expect(command).not.toHaveBeenCalled();
  const source = screen.getByRole("button", { name: /^e2,/ });
  fireEvent.click(source);
  fireEvent.click(screen.getByRole("button", { name: /^e4,/ }));
  expect(command).not.toHaveBeenCalled();
  expect(source.getAttribute("aria-disabled")).toBe("true");
  fireEvent.click(screen.getByRole("button", { name: "Resume play" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({
    kind: "resume",
    version: moves.version,
  });
});
it("cancels and replaces analysis through separate commands while keeping play paused", () => {
  const working: Snapshot = {
    ...moves,
    analysis: { ...moves.analysis, status: "thinking", result: null },
  };
  inspect(working);
  fireEvent.click(screen.getByRole("button", { name: "Cancel analysis" }));
  expect(command).toHaveBeenLastCalledWith({
    kind: "cancel_analysis",
    version: working.version,
  });
  fireEvent.click(
    screen.getByRole("button", { name: "Replace with move analysis" }),
  );
  expect(command).toHaveBeenLastCalledWith({
    kind: "analyze",
    version: working.version,
    operation: working.document.settings.analysis_defaults,
  });
  expect(screen.getByRole("button", { name: "Resume play" })).toBeTruthy();
});
it("does not automatically resume cancelled analysis or offer engine retry as an ownership shortcut", () => {
  inspect(cancelledJson as Snapshot);
  expect(
    screen.getByText(/Analysis cancelled. No conclusion was established/),
  ).toBeTruthy();
  expect(command).not.toHaveBeenCalled();
  expect(
    screen.queryByRole("button", { name: "Retry engine turn" }),
  ).toBeNull();
});
it("withholds stale target results and never presents a rejected wrong-purpose result as valid analysis", () => {
  const stale: Snapshot = { ...moves, version: moves.version + 1 };
  const { rerender } = inspect(stale);
  expect(
    screen.getByText(/belongs to an earlier position or request/),
  ).toBeTruthy();
  expect(screen.queryByText(/Suggested move:/)).toBeNull();
  const rejected: Snapshot = {
    ...moves,
    analysis: {
      ...moves.analysis,
      status: "failed",
      result: (possibleJson as Snapshot).analysis.result,
      message: "Wrong operation returned",
    },
  };
  session(rejected);
  rerender(<App />);
  expect(screen.getByText(/Analysis process failed/)).toBeTruthy();
  expect(screen.queryByText("A cooperative mating route exists")).toBeNull();
  expect(screen.getByText(/No accepted analysis observation/)).toBeTruthy();
});
it("identifies positive and negative native mate scores using the target's actual side", () => {
  const { rerender } = inspect(mateJson as Snapshot);
  expect(screen.getByLabelText("Analysis result").textContent).toContain(
    "Mate for White in 1 move (1 ply)",
  );
  session(matedJson as Snapshot);
  rerender(<App />);
  expect(screen.getByLabelText("Analysis result").textContent).toContain(
    "Mate for White · already checkmate",
  );
});
it("distinguishes Full cooperative witnesses, impossibility proofs and work-limited unknown results", () => {
  const { rerender } = inspect(possibleJson as Snapshot);
  expect(
    within(screen.getByLabelText("Analysis result")).getByText(
      "A cooperative mating route exists",
    ),
  ).toBeTruthy();
  expect(
    within(screen.getByLabelText("Analysis result")).getByText(
      /not a forced win against best defense/,
    ),
  ).toBeTruthy();
  session(impossibleJson as Snapshot);
  rerender(<App />);
  expect(
    within(screen.getByLabelText("Analysis result")).getByText(
      "Neither side can reach mate",
    ),
  ).toBeTruthy();
  session(unknownJson as Snapshot);
  rerender(<App />);
  expect(
    within(screen.getByLabelText("Analysis result")).getByText(
      "Unknown · exploration incomplete",
    ),
  ).toBeTruthy();
  expect(
    within(screen.getByLabelText("Analysis result")).getByText(
      /No mating conclusion was established/,
    ),
  ).toBeTruthy();
});
it("finishes analysis of a concluded game without offering to reopen it", () => {
  const ended = matedJson as Snapshot;
  inspect(ended);
  expect(screen.getByRole("button", { name: "Finish analysis" })).toBeTruthy();
  expect(screen.queryByRole("button", { name: "Resume play" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Finish analysis" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({
    kind: "resume",
    version: ended.version,
  });
  expect(
    within(screen.getByLabelText("Game outcome")).getByText(/Checkmate/),
  ).toBeTruthy();
});
