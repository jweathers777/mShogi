// @vitest-environment jsdom
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { ProductApp } from "./ProductApp";
import { useGameSession } from "./useGameSession";
import type { GameSession } from "./useGameSession";
import type { ShogiSnapshot as GameSnapshot } from "./native";
import standardJson from "../__fixtures__/shared-shogi-standard.json";
import xiangqiJson from "../__fixtures__/shared-xiangqi-standard.json";
import type { XiangqiSnapshot } from "./native";
import chessJson from "../__fixtures__/shared-chess-standard.json";
import type { ChessSnapshot } from "./native";
vi.mock("./useGameSession", () => ({ useGameSession: vi.fn() }));
const standard = standardJson as GameSnapshot;
let session: GameSession;
beforeEach(() => {
  session = { snapshot: { ...standard, document: { ...standard.document, dirty: false } }, busy: false, error: "", closeRequested: false, command: vi.fn().mockResolvedValue(standard), file: vi.fn().mockResolvedValue(standard), newGame: vi.fn().mockResolvedValue(standard), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true), requestClose: vi.fn().mockResolvedValue(undefined) };
  vi.mocked(useGameSession).mockImplementation(() => session);
});
afterEach(cleanup);
it.each([["Shogi", "Sente", "shogi", "sente"], ["Shogi", "Gote", "shogi", "gote"], ["Chess", "White", "chess", "white"], ["Chess", "Black", "chess", "black"], ["Xiangqi", "Red", "xiangqi", "red"], ["Xiangqi", "Black", "xiangqi", "black"]])("chooses %s / %s with explicit native game and side identity", async (_gameLabel, sideLabel, game, side) => {
  render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "New game" }));
  fireEvent.change(screen.getByRole("combobox", { name: "Game" }), { target: { value: game } });
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
  session = { ...session, snapshot: { ...(chessJson as ChessSnapshot), document: { ...(chessJson as ChessSnapshot).document, dirty: true } }, closeRequested: true };
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
it("offers only implemented games in the compact native chooser and preserves side radio navigation", async () => {
  render(<ProductApp />); const newGame = screen.getByRole("button", { name: "New game" }); newGame.focus(); fireEvent.click(newGame);
  const chooser = screen.getByRole("combobox", { name: "Game" });
  expect(chooser.tagName).toBe("SELECT");
  expect(screen.getAllByRole("option").map(option => [option.textContent, (option as HTMLOptionElement).value])).toEqual([["Chess", "chess"], ["Shogi", "shogi"], ["Xiangqi", "xiangqi"]]);
  expect(screen.getAllByRole("radio").map(radio => radio.textContent)).toEqual(["Sente", "Gote", "Random"]);
  const sente = screen.getByRole("radio", { name: "Sente" }); sente.focus(); fireEvent.keyDown(sente, { key: "ArrowRight" });
  const gote = screen.getByRole("radio", { name: "Gote" }); expect(document.activeElement).toBe(gote); expect(gote.getAttribute("aria-checked")).toBe("true");
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
it("routes Chess Take back through the full shared rendered token and common operation", () => {
  const source = { ...(chessJson as ChessSnapshot), capabilities: { ...chessJson.capabilities, take_back: true } };
  session = { ...session, snapshot: source };
  render(<ProductApp />);
  fireEvent.click(screen.getByRole("button", { name: "Take back" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(source, { kind: "take_back" });
});

it("does not classify an untyped command diagnostic as a particular resource", () => {
  session = { ...session, snapshot: chessJson as ChessSnapshot, error: "Application capacity: retained lineage replay budget." };
  render(<ProductApp />);
  expect(screen.getByRole("alert").textContent).toContain("The action could not complete");
  expect((screen.getByRole("button", { name: "Save" }) as HTMLButtonElement).disabled).toBe(false);
  expect((screen.getByRole("button", { name: "Save as…" }) as HTMLButtonElement).disabled).toBe(false);
});
