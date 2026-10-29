// @vitest-environment jsdom
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { App } from "./App";
import { mockSession } from "./__tests__/session";
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
it("delegates only native close confirmation to the enclosing product shell", () => {
  const dirty = { ...standard, document: { ...standard.document, dirty: true } };
  vi.mocked(useNativeSession).mockReturnValue({ ...mockSession(dirty, command), closeRequest: dirty });
  const { rerender } = render(<App externalCloseGuard />);
  expect(screen.queryByRole("dialog")).toBeNull();
  // Standalone Chess still owns its established native quit guard.
  rerender(<App />);
  expect(screen.getByRole("dialog")).toBeTruthy();
});
it("keeps Save and Save As available while capacity pauses play and analysis", () => {
  session({ ...standard, capacity: { id: 257, status: "paused", message: "Application capacity: 256 observations are already preserved." }, play_paused: true });
  render(<App />);
  expect(screen.getByRole("alert").textContent).toContain("Application limit reached");
  expect((screen.getByRole("button", { name: "Save" }) as HTMLButtonElement).disabled).toBe(false);
  expect((screen.getByRole("button", { name: "Save as…" }) as HTMLButtonElement).disabled).toBe(false);
  expect((screen.getByRole("button", { name: "New game" }) as HTMLButtonElement).disabled).toBe(false);
  expect(screen.queryByRole("button", { name: "Resume play" })).toBeNull();
  expect(screen.queryByRole("button", { name: "Retry engine turn" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect((screen.getByRole("button", { name: "Start move analysis" }) as HTMLButtonElement).disabled).toBe(true);
});
function session(snapshot: Snapshot) {
  vi.mocked(useNativeSession).mockReturnValue(mockSession(snapshot, command));
}
it("uses the shared document, native engine budget and live/history footer", () => {
  session({ ...standard, document: { ...standard.document, path: null, dirty: false } });
  const { container, rerender } = render(<App />);
  expect(container.querySelector("[data-document-state]")?.textContent).toBe("Unsaved");
  expect(container.querySelector("[data-play-limits]")?.textContent).toBe("Play limit: depth 2 · 20,000 nodes");
  expect(container.querySelector("[data-position-status]")?.textContent).toBe("Your turnLive position · 0 plies");
  session({ ...standard, document: { ...standard.document, path: "/games/chess.mshogi", dirty: false }, view: { ...standard.view, historical: true, ply: 1 } });
  rerender(<App />);
  expect(container.querySelector("[data-document-state]")?.textContent).toBe("Unmodified");
  expect(container.querySelector("[data-position-status]")?.textContent).toBe("Read-only history · return to live to playSelected position · 1 ply");
});
it("persists native orientation independently of human color and binds new-game document revision", () => {
  session(standard);
  const { rerender } = render(<App />);
  fireEvent.click(screen.getByRole("button", { name: /Flip board/ }));
  expect(command).toHaveBeenLastCalledWith({
    kind: "preferences",
    orientation: "black",
    analysis_defaults: standard.document.settings.analysis_defaults,
  });
  session({
    ...standard,
    document: {
      ...standard.document,
      settings: { ...standard.document.settings, orientation: "black" },
    },
  });
  rerender(<App />);
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
  expect(command).toHaveBeenLastCalledWith({
    kind: "new_game",
    version: standard.version,
    document_revision: standard.document.revision,
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

it("uses the native Chess Take back capability with a single versioned standalone command", () => {
  const snapshot = { ...standard, can_take_back: true };
  session(snapshot); const { rerender } = render(<App />);
  const button = screen.getByRole("button", { name: "Take back" });
  expect(button.closest("[data-shell-section]")?.getAttribute("data-shell-section")).toBe("actions");
  fireEvent.click(button); expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "take_back", version: snapshot.version });
  session({ ...snapshot, view: { ...snapshot.view, historical: true } }); rerender(<App />);
  expect((button as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(button); expect(command).toHaveBeenCalledTimes(1);
  session({ ...snapshot, can_take_back: false }); rerender(<App />);
  expect((button as HTMLButtonElement).disabled).toBe(true);
});
it("binds product Chess Take back to the full shared identity rather than the legacy move path", () => {
  session({ ...standard, can_take_back: true }); const restore = vi.fn().mockResolvedValue({});
  render(<App onTakeBack={restore} />);
  fireEvent.click(screen.getByRole("button", { name: "Take back" }));
  expect(restore).toHaveBeenCalledOnce(); expect(command).not.toHaveBeenCalled();
});
