// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { ApplicationView } from "./ApplicationView";
import type { ChessSnapshot, GameSnapshot } from "./native";
import type { GameSession } from "./useGameSession";
import standardJson from "../__fixtures__/shared-chess-standard.json";
import claimJson from "../__fixtures__/shared-chess-claim.json";
import currentClaimJson from "../__fixtures__/shared-chess-current-claim.json";
import claimedJson from "../__fixtures__/shared-chess-claimed.json";
import shogiJson from "../__fixtures__/shared-shogi-standard.json";
import xiangqiJson from "../__fixtures__/shared-xiangqi-standard.json";
const standard = standardJson as ChessSnapshot;
afterEach(cleanup);
function app(snapshot: GameSnapshot = standard) {
  const session: GameSession = { snapshot, busy: false, error: "", closeRequested: false, command: vi.fn().mockResolvedValue(snapshot), file: vi.fn().mockResolvedValue(snapshot), newGame: vi.fn().mockResolvedValue(snapshot), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true), requestClose: vi.fn().mockResolvedValue(undefined) };
  const props = { snapshot, session, onNewGame: vi.fn(), onOpen: vi.fn(), blocked: false };
  return { ...render(<ApplicationView {...props} />), props, session };
}
it.each([["intended", claimJson], ["current", currentClaimJson]])("submits the complete %s native draw claim without a board action", (_name, fixture) => {
  const snapshot = fixture as ChessSnapshot; const { session } = app(snapshot);
  expect(screen.getByText("Draw claim available")).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Claim selected draw" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "chess_claim", claim: snapshot.chess.claims[0] });
});
it("renders native coordinates, side artwork and only complete native board actions", () => {
  const { session } = app(); const square = (name: string) => screen.getByRole("button", { name: new RegExp(`^${name},`) });
  expect(square("e2").querySelector("text.piece")?.getAttribute("class")).toContain("white");
  expect(square("e7").querySelector("text.piece")?.getAttribute("class")).toContain("black");
  fireEvent.click(square("e2")); fireEvent.click(square("e4"));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(standard, { kind: "chess_play", action: "e2e4" });
});
it("persists orientation independently of the human side and displays the readable native conclusion", () => {
  const { session, props, rerender } = app();
  fireEvent.click(screen.getByRole("button", { name: /Flip board/ }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(standard, { kind: "chess_preferences", orientation: "black", analysis_defaults: standard.document.settings.settings.analysis_defaults });
  rerender(<ApplicationView {...props} snapshot={claimedJson as ChessSnapshot} />);
  expect(document.querySelector("[data-game-status]")?.textContent).toContain("Draw — claimed");
});
it.each([standardJson, shogiJson, xiangqiJson])("uses the same actual common section and board-slot ownership", fixture => {
  const { container } = app(fixture as GameSnapshot);
  expect(container.querySelectorAll(".shell-header")).toHaveLength(1);
  expect([...container.querySelectorAll("[data-shell-section]")].map(n => n.getAttribute("data-shell-section"))).toEqual(["analysis", "history", "actions"]);
  expect([...container.querySelector(".shell-game-area")!.children].map(n => n.getAttribute("data-game-slot"))).toEqual(["upper-player", "board", "lower-player", "footer"]);
  for (const name of ["New game", "Save", "Save as…", "Open…"]) expect(screen.getByRole("button", { name }).closest(".shell-header")).toBeTruthy();
  expect(screen.getByRole("heading", { name: "Untitled game" })).toBeTruthy();
  expect(container.querySelector("[data-document-state]")?.textContent).toBe("Unsaved");
  expect(container.querySelector("[data-position-status]")?.textContent).toBe("Your turnLive position · 0 plies");
});
