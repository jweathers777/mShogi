// @vitest-environment jsdom
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { ProductApp } from "./ProductApp";
import { useGameSession } from "./useGameSession";
import type { GameSession } from "./useGameSession";
import type { ChessShogiSnapshot as GameSnapshot } from "./native";
import standardJson from "../__fixtures__/shared-shogi-standard.json";
import xiangqiJson from "../__fixtures__/shared-xiangqi-standard.json";
import type { XiangqiSnapshot } from "./native";
import chessJson from "../__fixtures__/standard.json";
import { mockSession } from "../__tests__/session";
import { useNativeSession } from "../useNativeSession";
import type { Snapshot } from "../native";
vi.mock("./useGameSession", () => ({ useGameSession: vi.fn() }));
vi.mock("../useNativeSession", () => ({ useNativeSession: vi.fn() }));
const standard = standardJson as GameSnapshot;
let session: GameSession;
beforeEach(() => {
  session = { snapshot: { ...standard, document: { ...standard.document, dirty: false } }, busy: false, error: "", closeRequested: false, command: vi.fn().mockResolvedValue(standard), file: vi.fn().mockResolvedValue(standard), newGame: vi.fn().mockResolvedValue(standard), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true) };
  vi.mocked(useGameSession).mockImplementation(() => session);
  vi.mocked(useNativeSession).mockReturnValue(mockSession(chessJson as Snapshot));
});
afterEach(cleanup);
it.each([["Shogi", "Sente", "shogi", "sente"], ["Shogi", "Gote", "shogi", "gote"], ["Chess", "White", "chess", "white"], ["Chess", "Black", "chess", "black"], ["Xiangqi", "Red", "xiangqi", "red"], ["Xiangqi", "Black", "xiangqi", "black"]])("chooses %s / %s with explicit native game and side identity", async (gameLabel, sideLabel, game, side) => {
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
it("owns one close guard across a Shogi-to-Chess view replacement", async () => {
  const { rerender } = render(<ProductApp />);
  session = { ...session, snapshot: { ...standard, game: "chess", token: { ...standard.token, document: standard.token.document + 1 } }, closeRequested: true };
  const chess = { ...(chessJson as Snapshot), document: { ...(chessJson as Snapshot).document, dirty: true } };
  vi.mocked(useNativeSession).mockReturnValue({ ...mockSession(chess), closeRequest: chess });
  rerender(<ProductApp />);
  expect(await screen.findAllByRole("dialog")).toHaveLength(1);
  expect(screen.getByRole("dialog", { name: "Save your changes?" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Discard & continue" }));
  await waitFor(() => expect(session.finishClose).toHaveBeenCalledExactlyOnceWith(session.snapshot));
});
it("replaces an uncommitted New Game chooser with the native quit guard", async () => {
  const { rerender } = render(<ProductApp />);
  fireEvent.click(screen.getByRole("button", { name: "New game" }));
  expect(screen.getByRole("dialog", { name: "New game" })).toBeTruthy();
  session = { ...session, closeRequested: true };
  rerender(<ProductApp />);
  expect(await screen.findAllByRole("dialog")).toHaveLength(1);
  expect(screen.getByRole("dialog", { name: "Close mShogi?" })).toBeTruthy();
  expect(session.newGame).not.toHaveBeenCalled();
});
it("provides keyboard radio navigation and restores focus on cancelling New Game", async () => {
  render(<ProductApp />); const newGame = screen.getByRole("button", { name: "New game" }); newGame.focus(); fireEvent.click(newGame);
  const game = screen.getByRole("radio", { name: /Shogi/ }); game.focus(); fireEvent.keyDown(game, { key: "ArrowLeft" });
  const chess = screen.getByRole("radio", { name: /Chess/ }); expect(document.activeElement).toBe(chess); expect(chess.getAttribute("aria-checked")).toBe("true");
  fireEvent.keyDown(screen.getByRole("dialog", { name: "New game" }), { key: "Escape" });
  expect(screen.queryByRole("dialog")).toBeNull(); await waitFor(() => expect(document.activeElement).toBe(newGame));
});

it("protects the Xiangqi document during failed Save-and-open and native close", async () => {
  const xiangqi = xiangqiJson as unknown as XiangqiSnapshot;
  session.snapshot = xiangqi; vi.mocked(session.file).mockResolvedValue(xiangqi);
  const { rerender } = render(<ProductApp />);
  fireEvent.click(screen.getByRole("button", { name: "Open…" }));
  fireEvent.click(screen.getByRole("button", { name: "Save & continue" }));
  expect(await screen.findByRole("alert")).toHaveProperty("textContent", expect.stringContaining("not fully saved"));
  expect(session.file).toHaveBeenCalledExactlyOnceWith(xiangqi, "save");
  fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  session = { ...session, closeRequested: true }; rerender(<ProductApp />);
  expect(await screen.findByRole("dialog", { name: "Save your changes?" })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
  expect(session.cancelClose).toHaveBeenCalledOnce(); expect(session.finishClose).not.toHaveBeenCalled();
});
it("routes product Chess Take back through the full shared rendered token and common operation", () => {
  const chess = { ...(chessJson as Snapshot), can_take_back: true };
  const source = { ...standard, game: "chess" as const, chess, capabilities: { ...standard.capabilities, take_back: true } };
  session = { ...session, snapshot: source };
  render(<ProductApp />);
  fireEvent.click(screen.getByRole("button", { name: "Take back" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(source, { kind: "take_back" });
  expect(vi.mocked(useNativeSession).mock.results.at(-1)?.value.command).not.toHaveBeenCalled();
});

it.each(["The game changed; stale request.", "Application capacity: retained lineage replay budget."])("shows native product Chess Take back refusal: %s", (error) => {
  const chess = { ...(chessJson as Snapshot), can_take_back: true };
  session = { ...session, snapshot: { ...standard, game: "chess", chess, capabilities: { ...standard.capabilities, take_back: true } }, error };
  render(<ProductApp />);
  const alert = screen.getByRole("alert");
  expect(alert.textContent).toContain(error.startsWith("Application capacity:") ? "Application limit reached" : "Command or connection problem");
  expect((screen.getByRole("button", { name: "Save" }) as HTMLButtonElement).disabled).toBe(false);
});
