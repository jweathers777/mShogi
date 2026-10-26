// @vitest-environment jsdom
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { ProductApp } from "./ProductApp";
import { useGameSession } from "./useGameSession";
import type { GameSession } from "./useGameSession";
import type { GameSnapshot } from "./native";
import standardJson from "../__fixtures__/shared-shogi-standard.json";
vi.mock("./useGameSession", () => ({ useGameSession: vi.fn() }));
const standard = standardJson as GameSnapshot;
let session: GameSession;
beforeEach(() => {
  session = { snapshot: { ...standard, document: { ...standard.document, dirty: false } }, busy: false, error: "", closeRequested: false, command: vi.fn().mockResolvedValue(standard), file: vi.fn().mockResolvedValue(standard), newGame: vi.fn().mockResolvedValue(standard), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true) };
  vi.mocked(useGameSession).mockImplementation(() => session);
});
afterEach(cleanup);
it.each([["Shogi", "Sente", "shogi", "sente"], ["Shogi", "Gote", "shogi", "gote"], ["Chess", "White", "chess", "white"], ["Chess", "Black", "chess", "black"]])("chooses %s / %s with explicit native game and side identity", async (gameLabel, sideLabel, game, side) => {
  render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "New game" }));
  fireEvent.click(screen.getByRole("radio", { name: new RegExp(gameLabel) }));
  fireEvent.click(screen.getByRole("radio", { name: sideLabel }));
  fireEvent.click(screen.getByRole("button", { name: "Start game" }));
  expect(session.newGame).toHaveBeenCalledExactlyOnceWith(standard.open_token, game, { game_kind: game, side });
});
it("keeps New Game cancelled until unsaved changes are explicitly discarded", () => {
  session.snapshot = standard; render(<ProductApp />);
  fireEvent.click(screen.getByRole("button", { name: "New game" }));
  expect(screen.getByRole("dialog", { name: "Save your changes?" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  expect(screen.queryByRole("dialog")).toBeNull(); expect(session.newGame).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "New game" }));
  fireEvent.click(screen.getByRole("button", { name: "Discard & continue" }));
  expect(screen.getByRole("dialog", { name: "New game" })).toBeTruthy(); expect(session.newGame).not.toHaveBeenCalled();
});
it("cannot replace dirty work after cancelled or unsuccessful Save", async () => {
  session.snapshot = standard; vi.mocked(session.file).mockResolvedValueOnce(standard);
  render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "Open…" }));
  fireEvent.click(screen.getByRole("button", { name: "Save & continue" }));
  expect(await screen.findByRole("alert")).toHaveProperty("textContent", expect.stringContaining("not fully saved"));
  expect(session.file).toHaveBeenCalledExactlyOnceWith(standard, "save"); expect(session.newGame).not.toHaveBeenCalled();
  expect(screen.getByRole("dialog", { name: "Save your changes?" })).toBeTruthy();
});
it("invalidates New Game admission when its exact native Open token changes", () => {
  const { rerender } = render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "New game" }));
  session = { ...session, snapshot: { ...session.snapshot!, open_token: { ...standard.open_token, document_revision: standard.open_token.document_revision + 1 } } };
  rerender(<ProductApp />);
  expect((screen.getByRole("button", { name: "Start game" }) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(screen.getByRole("button", { name: "Start game" })); expect(session.newGame).not.toHaveBeenCalled();
});
it("protects native quit with Save/Discard/Cancel and returns explicitly to play", async () => {
  session.snapshot = standard; session.closeRequested = true; render(<ProductApp />);
  expect(await screen.findByRole("dialog", { name: "Save your changes?" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  expect(session.cancelClose).toHaveBeenCalledTimes(1); expect(session.finishClose).not.toHaveBeenCalled();
});
it("provides keyboard radio navigation and restores focus on cancelling New Game", async () => {
  render(<ProductApp />); const newGame = screen.getByRole("button", { name: "New game" }); newGame.focus(); fireEvent.click(newGame);
  const game = screen.getByRole("radio", { name: /Shogi/ }); game.focus(); fireEvent.keyDown(game, { key: "ArrowLeft" });
  const chess = screen.getByRole("radio", { name: /Chess/ }); expect(document.activeElement).toBe(chess); expect(chess.getAttribute("aria-checked")).toBe("true");
  fireEvent.keyDown(screen.getByRole("dialog", { name: "New game" }), { key: "Escape" });
  expect(screen.queryByRole("dialog")).toBeNull(); await waitFor(() => expect(document.activeElement).toBe(newGame));
});
