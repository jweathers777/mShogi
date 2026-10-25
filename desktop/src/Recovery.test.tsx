// @vitest-environment jsdom
import {
  cleanup,
  fireEvent,
  render,
  screen,
  within,
} from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { App } from "./App";
import { mockSession } from "./__tests__/session";
import { useNativeSession } from "./useNativeSession";
import type { Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
import claimJson from "./__fixtures__/claim.json";
import currentClaimJson from "./__fixtures__/current-claim.json";
import claimedJson from "./__fixtures__/claimed.json";
import unresolvedJson from "./__fixtures__/unresolved.json";
vi.mock("./useNativeSession", () => ({ useNativeSession: vi.fn() }));
const standard = standardJson as Snapshot;
const command = vi.fn();
beforeEach(() => {
  command.mockReset();
});
afterEach(cleanup);
function session(snapshot: Snapshot) {
  vi.mocked(useNativeSession).mockReturnValue(mockSession(snapshot, command));
}
function engineTurn(phase: Snapshot["engine"]["status"]): Snapshot {
  return {
    ...standard,
    human: "black",
    engine: { ...standard.engine, status: phase, message: null },
  };
}
it("requires explicit resignation confirmation during an engine turn and preserves its native version", () => {
  const snapshot = engineTurn("thinking");
  session(snapshot);
  render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Resign game…" }));
  const dialog = screen.getByRole("dialog", { name: "Resign as Black?" });
  expect(command).not.toHaveBeenCalled();
  expect(within(dialog).getByText(/unresolved and unscored/)).toBeTruthy();
  expect(document.activeElement).toBe(
    within(dialog).getByRole("button", { name: "Keep playing" }),
  );
  fireEvent.click(
    within(dialog).getByRole("button", { name: "Confirm resignation" }),
  );
  expect(command).toHaveBeenCalledExactlyOnceWith({
    kind: "resign",
    version: snapshot.version,
  });
});
it("lets keyboard users cancel resignation without changing game history", async () => {
  session(standard);
  render(<App />);
  const trigger = screen.getByRole("button", { name: "Resign game…" });
  fireEvent.click(trigger);
  await userEvent.setup().keyboard("{Escape}");
  expect(screen.queryByRole("dialog")).toBeNull();
  expect(document.activeElement).toBe(trigger);
  expect(command).not.toHaveBeenCalled();
});
it("does not silently upgrade a resignation confirmation when an engine response changes the game", () => {
  session(engineTurn("thinking"));
  const { rerender } = render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Resign game…" }));
  session({ ...standard, version: 9, revision: 10 });
  rerender(<App />);
  expect(
    (
      screen.getByRole("button", {
        name: "Confirm resignation",
      }) as HTMLButtonElement
    ).disabled,
  ).toBe(true);
  expect(screen.getByRole("alert").textContent).toContain("The game changed");
  fireEvent.click(screen.getByRole("button", { name: "Confirm resignation" }));
  expect(command).not.toHaveBeenCalled();
});
it("distinguishes native current and intended claim opportunities from a concluded claimed draw", () => {
  session(currentClaimJson as Snapshot);
  const { rerender } = render(<App />);
  expect(screen.getByText("Current-position claim")).toBeTruthy();
  expect(screen.queryByLabelText("Game outcome")).toBeNull();
  session(claimJson as Snapshot);
  rerender(<App />);
  expect(screen.getByText(/^Intended-move claim/)).toBeTruthy();
  expect(screen.getByText(/unplayed intention/)).toBeTruthy();
  session(claimedJson as Snapshot);
  rerender(<App />);
  expect(screen.getByLabelText("Game outcome").textContent).toContain(
    "Draw — claimed fifty-move rule",
  );
  expect(screen.queryByText("Draw claim available")).toBeNull();
  expect(screen.queryByRole("button", { name: "Resign game…" })).toBeNull();
});
it("presents the actual native unresolved resignation without inventing a winner or a draw", () => {
  session(unresolvedJson as Snapshot);
  render(<App />);
  const result = screen.getByLabelText("Game outcome");
  expect(result.textContent).toContain("White resigned");
  expect(result.textContent).toContain("without an established win or draw");
  expect(result.textContent).toContain("no score is assigned");
  expect(
    screen.queryByRole("button", { name: "Retry engine turn" }),
  ).toBeNull();
  expect(screen.getByRole("button", { name: "New game" })).toBeTruthy();
});
for (const phase of ["cancelled", "incomplete", "failed"] as const) {
  it(`keeps ${phase} engine work separate from a chess conclusion and retries the native history`, () => {
    const snapshot = {
      ...engineTurn(phase),
      version: 7,
      history: ["e2e4", "e7e5"],
    };
    session(snapshot);
    render(<App />);
    expect(screen.getByText(/The game is still ongoing/)).toBeTruthy();
    expect(screen.queryByLabelText("Game outcome")).toBeNull();
    expect(screen.queryByText(/game unscored/)).toBeNull();
    expect(screen.getByText("e2e4")).toBeTruthy();
    expect(screen.getByText("e7e5")).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "Retry engine turn" }));
    expect(command).toHaveBeenCalledExactlyOnceWith({
      kind: "retry",
      version: 7,
    });
    expect(snapshot.history).toEqual(["e2e4", "e7e5"]);
  });
}
it("cancels active engine work explicitly without offering a chess result", () => {
  const snapshot = engineTurn("thinking");
  session(snapshot);
  render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Cancel search" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({
    kind: "cancel",
    version: snapshot.version,
  });
  expect(screen.queryByLabelText("Game outcome")).toBeNull();
});
