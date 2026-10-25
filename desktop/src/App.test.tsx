// @vitest-environment jsdom
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { App } from "./App";
import { useNativeSession } from "./useNativeSession";
import type { Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
import claimJson from "./__fixtures__/claim.json";
vi.mock("./useNativeSession", () => ({ useNativeSession: vi.fn() }));
const standard = standardJson as Snapshot;
const nativeClaim = claimJson as Snapshot;
const command = vi.fn();
beforeEach(() => {
  command.mockReset();
});
afterEach(cleanup);
function session(snapshot: Snapshot) {
  vi.mocked(useNativeSession).mockReturnValue({
    snapshot,
    error: "",
    connected: true,
    busy: false,
    command,
  });
}
it("keeps orientation independent of human color and dispatches rendered native versions", () => {
  session(standard);
  render(<App />);
  fireEvent.click(screen.getByRole("button", { name: /Flip board/ }));
  expect(
    screen
      .getByRole("group", { name: /Chess board/ })
      .getAttribute("aria-label"),
  ).toContain("black at bottom");
  expect(screen.getByText(/You play White/)).toBeTruthy();
  fireEvent.change(screen.getByLabelText("Your color for a new game"), {
    target: { value: "black" },
  });
  fireEvent.click(screen.getByRole("button", { name: "New game" }));
  expect(command).toHaveBeenCalledWith({
    kind: "new_game",
    version: standard.version,
    human: "black",
  });
});
it("presents an intended claim as an opportunity and sends the full claim without playing its move", () => {
  session(nativeClaim);
  render(<App />);
  expect(screen.getByText("White to move")).toBeTruthy();
  expect(screen.getByText("Draw claim available")).toBeTruthy();
  expect(screen.getByText(/An intended move is not played/)).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Claim selected draw" }));
  expect(command).toHaveBeenCalledWith({
    kind: "claim",
    version: nativeClaim.version,
    claim: nativeClaim.claims[0],
  });
});
it("labels last real observations with their searched version and side, while process failures remain separate", () => {
  const snapshot: Snapshot = {
    ...standard,
    version: 9,
    engine: {
      status: "failed",
      last: {
        kind: "move",
        action: "e7e5",
        telemetry: {
          score: 37,
          completed_depth: 2,
          nodes: 420,
          principal_variation: [{ kind: "move", action: "e7e5" }],
          status: "complete",
        },
      },
      last_turn: "black",
      searched_version: 6,
      message: "Engine response timed out. No chess outcome was assigned.",
    },
  };
  session(snapshot);
  render(<App />);
  expect(
    screen.getByText(/Last searched position · version 6 · Black perspective/),
  ).toBeTruthy();
  expect(screen.getByText("+0.37 pawns from Black perspective")).toBeTruthy();
  expect(screen.getByRole("alert").textContent).toContain("No chess outcome");
  expect(screen.getByText("White to move")).toBeTruthy();
});
