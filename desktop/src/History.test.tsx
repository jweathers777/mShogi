// @vitest-environment jsdom
import {
  act,
  cleanup,
  fireEvent,
  render,
  screen,
  within,
} from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { App } from "./App";
import { useNativeSession } from "./useNativeSession";
import { mockSession } from "./__tests__/session";
import { SavedObservations } from "./History";
import type { Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
import movesJson from "./__fixtures__/analysis-moves.json";
import claimedJson from "./__fixtures__/claimed.json";
vi.mock("./useNativeSession", () => ({ useNativeSession: vi.fn() }));
const standard = standardJson as Snapshot;
const moves = movesJson as Snapshot;
afterEach(cleanup);
it("selects native event boundaries, including a non-move claim, rather than constructing positions", () => {
  const snapshot = claimedJson as Snapshot;
  const session = mockSession(snapshot);
  vi.mocked(useNativeSession).mockReturnValue(session);
  render(<App />);
  const timeline = screen.getByRole("list", { name: "Recorded game events" });
  const event = snapshot.view.timeline.at(-1)!;
  fireEvent.click(within(timeline).getAllByRole("button").at(-1)!);
  expect(session.command).toHaveBeenCalledExactlyOnceWith({
    kind: "inspect",
    version: snapshot.version,
    event_index: event.index,
  });
  expect(snapshot.history).toHaveLength(0);
});
it("renders the native historical board separately from live history and requires explicit Return to live", () => {
  const snapshot: Snapshot = {
    ...moves,
    history: ["e2e4", "e7e5"],
    view: {
      ...standard.view,
      historical: true,
      label: "Initial native prefix",
      selection_id: 90,
    },
  };
  const session = mockSession(snapshot);
  vi.mocked(useNativeSession).mockReturnValue(session);
  render(<App />);
  expect(screen.getByLabelText("Historical position")).toBeTruthy();
  expect(screen.getByText(/live game remains after 2 plies/)).toBeTruthy();
  expect(
    screen.getByRole("button", { name: /^e2,/ }).getAttribute("aria-disabled"),
  ).toBe("true");
  expect(
    (screen.getByRole("button", { name: "Resume play" }) as HTMLButtonElement)
      .disabled,
  ).toBe(true);
  const returnLive = screen.getByRole("button", { name: "Return to live" });
  expect(returnLive.closest(".shell-history-notice-heading")).not.toBeNull();
  fireEvent.click(returnLive);
  expect(session.command).toHaveBeenCalledExactlyOnceWith({
    kind: "return_live",
    version: snapshot.version,
  });
});
it("binds analysis to the selected native prefix and withholds a same-version result for a different selection", () => {
  const snapshot: Snapshot = {
    ...moves,
    view: {
      ...moves.view,
      historical: true,
      label: "Earlier prefix",
      selection_id: moves.analysis.target!.selection_id + 1,
    },
  };
  const session = mockSession(snapshot);
  vi.mocked(useNativeSession).mockReturnValue(session);
  render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect(
    screen.getByText(/Selected: Earlier prefix · historical prefix/),
  ).toBeTruthy();
  expect(screen.queryByText(/Suggested move:/)).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith({
    kind: "analyze",
    version: snapshot.version,
    operation: moves.document.settings.analysis_defaults,
  });
});
it("keeps restored play stopped until explicit Resume and restores saved analysis defaults without resetting typed budgets on polls", () => {
  const snapshot: Snapshot = {
    ...standard,
    play_paused: true,
    document: {
      ...standard.document,
      path: "/games/saved.mshogi",
      settings: {
        ...standard.document.settings,
        analysis_defaults: { kind: "moves", depth: 3, nodes: 4321 },
      },
    },
  };
  const session = mockSession(snapshot);
  vi.mocked(useNativeSession).mockReturnValue(session);
  const { rerender } = render(<App />);
  expect(session.command).not.toHaveBeenCalled();
  expect(
    screen.getByRole("button", { name: /^e2,/ }).getAttribute("aria-disabled"),
  ).toBe("true");
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect(
    (screen.getByLabelText("Search depth") as HTMLInputElement).value,
  ).toBe("3");
  expect((screen.getByLabelText("Node budget") as HTMLInputElement).value).toBe(
    "4321",
  );
  fireEvent.change(screen.getByLabelText("Node budget"), {
    target: { value: "9999" },
  });
  vi.mocked(useNativeSession).mockReturnValue({
    ...session,
    snapshot: { ...snapshot, revision: snapshot.revision + 1 },
  });
  rerender(<App />);
  expect((screen.getByLabelText("Node budget") as HTMLInputElement).value).toBe(
    "9999",
  );
  vi.mocked(useNativeSession).mockReturnValue({ ...session, documentEpoch: 1 });
  rerender(<App />);
  expect((screen.getByLabelText("Node budget") as HTMLInputElement).value).toBe(
    "4321",
  );
  fireEvent.click(screen.getByRole("button", { name: "Resume play" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith({
    kind: "resume",
    version: snapshot.version,
  });
});
it("labels saved observations historical with named score perspective, and hides rejected results outside technical details", () => {
  const accepted = moves.observations.at(-1)!;
  const { rerender } = render(
    <SavedObservations observations={[{ ...accepted, historical: true }]} />,
  );
  fireEvent.click(screen.getByText(/Recorded search observations \(/));
  expect(screen.getByText(/Saved historical observation/)).toBeTruthy();
  expect(screen.getByText(/pawns from White perspective/)).toBeTruthy();
  expect(
    screen.getByText(/No action was applied by this analysis/),
  ).toBeTruthy();
  rerender(
    <SavedObservations
      observations={[
        {
          ...accepted,
          historical: true,
          application_error: "Illegal returned action",
        },
      ]}
    />,
  );
  expect(
    screen.getByText(/Rejected response: Illegal returned action/),
  ).toBeTruthy();
  expect(screen.queryByText(/pawns from White perspective/)).toBeNull();
  expect(screen.queryByText(/Read-only suggestion:/)).toBeNull();
});

it("restores keyboard focus to the live SVG board after Return to live removes its button", async () => {
  const historical: Snapshot = {
    ...standard,
    play_paused: true,
    view: { ...standard.view, historical: true, selection_id: 90 },
  };
  const session = mockSession(historical);
  vi.mocked(useNativeSession).mockReturnValue(session);
  const { rerender } = render(<App />);
  const trigger = screen.getByRole("button", { name: "Return to live" });
  trigger.focus();
  const live: Snapshot = {
    ...standard,
    version: 2,
    play_paused: true,
    view: { ...standard.view, selection_id: 91 },
  };
  vi.mocked(session.command).mockResolvedValue(live);
  await act(async () => {
    fireEvent.click(trigger);
  });
  vi.mocked(useNativeSession).mockReturnValue({ ...session, snapshot: live });
  rerender(<App />);
  expect(
    screen.queryByRole("button", { name: "Return to live" }),
  ).toBeNull();
  expect(document.activeElement).toBe(
    screen.getByRole("button", { name: /^e2,/ }),
  );
});
it("returns keyboard focus to its SVG square after cancelling a native Quit prompt", async () => {
  const snapshot: Snapshot = {
    ...standard,
    document: { ...standard.document, dirty: true },
  };
  const session = mockSession(snapshot);
  vi.mocked(useNativeSession).mockReturnValue(session);
  const { rerender } = render(<App />);
  const square = screen.getByRole("button", { name: /^e2,/ });
  square.focus();
  expect(document.activeElement).toBe(square);
  const close: Snapshot = {
    ...snapshot,
    version: 2,
    revision: 3,
    play_paused: true,
  };
  vi.mocked(useNativeSession).mockReturnValue({
    ...session,
    snapshot: close,
    closeRequest: close,
  });
  rerender(<App />);
  const cancel = screen.getByRole("button", { name: "Cancel" });
  expect(document.activeElement).toBe(cancel);
  await act(async () => {
    fireEvent.click(cancel);
  });
  expect(session.cancelClose).toHaveBeenCalledExactlyOnceWith();
  expect(screen.queryByRole("dialog")).toBeNull();
  expect(document.activeElement).toBe(square);
});
