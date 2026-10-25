// @vitest-environment jsdom
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { App } from "./App";
import { useNativeSession } from "./useNativeSession";
import { mockSession } from "./__tests__/session";
import type { Snapshot } from "./native";
import standardJson from "./__fixtures__/standard.json";
import claimJson from "./__fixtures__/claim.json";
import claimedJson from "./__fixtures__/claimed.json";
import analysisJson from "./__fixtures__/analysis-moves.json";
vi.mock("./useNativeSession", () => ({ useNativeSession: vi.fn() }));
const standard = standardJson as Snapshot;
const claim = claimJson as Snapshot;
const claimed = claimedJson as Snapshot;
const analyzed = analysisJson as Snapshot;
const command = vi.fn();
beforeEach(() => command.mockReset().mockResolvedValue(null));
afterEach(cleanup);
function session(snapshot: Snapshot, busy = false) {
  vi.mocked(useNativeSession).mockReturnValue({ ...mockSession(snapshot, command), busy });
}
function thinking(): Snapshot {
  return { ...standard, play_paused: true,
    analysis: { ...standard.analysis, status: "thinking" } };
}
it("recovers focus from a removed Cancel analysis control to the surviving analysis start", () => {
  session(thinking());
  const { rerender } = render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  const cancel = screen.getByRole("button", { name: "Cancel analysis" });
  cancel.focus(); fireEvent.click(cancel);
  expect(command).toHaveBeenCalledWith({ kind: "cancel_analysis", version: standard.version });
  session({ ...thinking(), analysis: { ...standard.analysis, status: "cancelled" } });
  rerender(<App />);
  expect(document.activeElement).toBe(screen.getByRole("button", { name: "Start move analysis" }));
});
it("recovers to an enabled inspector control while a cancelled command is settling", () => {
  session(thinking());
  const { rerender } = render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  screen.getByRole("button", { name: "Cancel analysis" }).focus();
  const cancelled = { ...thinking(), analysis: { ...standard.analysis, status: "cancelled" as const } };
  session(cancelled, true); rerender(<App />);
  const inspector = screen.getByRole("button", { name: "Analyze position" });
  expect(document.activeElement).toBe(inspector);
  expect(screen.getByRole("button", { name: "Start move analysis" }).hasAttribute("disabled")).toBe(true);
  session(cancelled); rerender(<App />);
  expect(document.activeElement).toBe(inspector);
});
it("does not steal focus from a work-budget input when analysis finishes", () => {
  session(thinking());
  const { rerender } = render(<App />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  const input = screen.getByLabelText("Node budget"); input.focus();
  session(analyzed); rerender(<App />);
  expect(document.activeElement).toBe(input);
});
it("recovers engine-cancel focus to Retry, then to the board when the retry control disappears", () => {
  session({ ...standard, human: "black", engine: { ...standard.engine, status: "thinking" } });
  const { rerender } = render(<App />);
  screen.getByRole("button", { name: "Cancel search" }).focus();
  session({ ...standard, human: "black", engine: { ...standard.engine, status: "cancelled" } });
  rerender(<App />);
  const retry = screen.getByRole("button", { name: "Retry engine turn" });
  expect(document.activeElement).toBe(retry);
  session(standard); rerender(<App />);
  expect(document.activeElement).toBe(document.querySelector('.square[tabindex="0"]'));
});
it("focuses the actual conclusion after a claim removes its controls", () => {
  session(claim);
  const { rerender } = render(<App />);
  const button = screen.getByRole("button", { name: "Claim selected draw" });
  button.focus(); fireEvent.click(button);
  session(claimed); rerender(<App />);
  expect(document.activeElement).toBe(document.querySelector("[data-game-status]"));
  expect(document.activeElement?.textContent).toContain("Draw");
});
it("restores board focus after Resume and after a native selection identity remount", () => {
  session({ ...standard, play_paused: true });
  const { rerender } = render(<App />);
  screen.getByRole("button", { name: "Resume play" }).focus();
  session(standard); rerender(<App />);
  const original = document.querySelector<SVGGElement>('.square[tabindex="0"]')!;
  expect(document.activeElement).toBe(original);
  session({ ...standard, version: standard.version + 1,
    view: { ...standard.view, selection_id: standard.view.selection_id + 1 } });
  rerender(<App />);
  const current = document.querySelector('.square[tabindex="0"]');
  expect(current).not.toBe(original);
  expect(document.activeElement).toBe(current);
});
it("labels a rejected engine move as returned evidence without saying it was played", () => {
  session({ ...standard, human: "black", engine: { ...standard.engine, status: "failed",
    message: "Engine returned an illegal choice. No move was accepted.", last_turn: "white",
    searched_version: standard.version,
    last: { kind: "move", action: "e2e5", telemetry: { score: 0, completed_depth: 1,
      nodes: 21, principal_variation: [], status: "complete" } } } });
  render(<App />);
  expect(screen.getByText("Move returned: e2 → e5")).toBeTruthy();
  expect(screen.queryByText(/Played e2e5/)).toBeNull();
  expect(screen.getByRole("alert").textContent).toContain("No engine move or chess result was added");
  expect(screen.getByText("White to move")).toBeTruthy();
});
