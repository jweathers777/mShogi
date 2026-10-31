// @vitest-environment jsdom
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { ProductApp } from "./ProductApp";
import { useGameSession, type GameSession } from "./useGameSession";
import type { GameSnapshot, ShogiSnapshot } from "./native";
import shogiJson from "../__fixtures__/shared-shogi-standard.json";
import xiangqiJson from "../__fixtures__/shared-xiangqi-standard.json";
import chessJson from "../__fixtures__/shared-chess-standard.json";
vi.mock("./useGameSession", () => ({ useGameSession: vi.fn() }));
const standard = shogiJson as ShogiSnapshot;
const fixtures = [chessJson, shogiJson, xiangqiJson] as GameSnapshot[];
let session: GameSession;
function deferred<T>() { let resolve!: (value: T) => void; const promise = new Promise<T>(done => { resolve = done; }); return { resolve, promise }; }
function dirty(fixture: GameSnapshot) { return { ...fixture, close_requires_save: true, document: { ...fixture.document, dirty: true } } as GameSnapshot; }
beforeEach(() => {
  session = { snapshot: dirty(standard), busy: false, error: "", closeRequested: false, command: vi.fn().mockResolvedValue(standard), file: vi.fn().mockResolvedValue(standard), newGame: vi.fn().mockResolvedValue(standard), cancelClose: vi.fn().mockResolvedValue(true), finishClose: vi.fn().mockResolvedValue(true), requestClose: vi.fn().mockResolvedValue(undefined) };
  vi.mocked(useGameSession).mockImplementation(() => session);
});
afterEach(cleanup);
it.each([["Shogi", "Sente", "shogi", "sente"], ["Shogi", "Gote", "shogi", "gote"], ["Chess", "White", "chess", "white"], ["Chess", "Black", "chess", "black"], ["Xiangqi", "Red", "xiangqi", "red"], ["Xiangqi", "Black", "xiangqi", "black"]])("chooses %s / %s directly from dirty work with exact native identity", async (_gameLabel, sideLabel, game, side) => {
  render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "New game" }));
  fireEvent.change(screen.getByRole("combobox", { name: "Game" }), { target: { value: game } });
  fireEvent.click(screen.getByRole("radio", { name: sideLabel }));
  fireEvent.click(screen.getByRole("button", { name: "Start game" }));
  expect(session.newGame).toHaveBeenCalledExactlyOnceWith(standard.open_token, game, { game_kind: game, side });
  await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
  expect(session.file).not.toHaveBeenCalled();
});
it.each(fixtures)("keeps $game installed while New game is cancelled or rejected", async fixture => {
  const source = dirty(fixture); session.snapshot = source; vi.mocked(session.newGame).mockResolvedValue(null);
  render(<ProductApp />); const newGame = screen.getByRole("button", { name: "New game" }); newGame.focus(); fireEvent.click(newGame);
  expect(screen.getByRole("dialog", { name: "New game" })).toBeTruthy();
  expect(session.newGame).not.toHaveBeenCalled(); expect(session.file).not.toHaveBeenCalled();
  fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
  await waitFor(() => expect(document.activeElement).toBe(newGame));
  expect(screen.queryByRole("dialog")).toBeNull(); expect(session.snapshot).toBe(source);
  fireEvent.click(newGame); fireEvent.click(screen.getByRole("button", { name: "Start game" }));
  await waitFor(() => expect(session.newGame).toHaveBeenCalledOnce());
  expect(screen.getByRole("dialog", { name: "New game" })).toBeTruthy(); expect(session.snapshot).toBe(source);
});
it.each(fixtures)("invalidates $game New Game admission on exact native token changes", fixture => {
  session.snapshot = fixture; const { rerender } = render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "New game" }));
  session = { ...session, snapshot: { ...fixture, open_token: { ...fixture.open_token, document_revision: fixture.open_token.document_revision + 1 } } };
  rerender(<ProductApp />);
  expect((screen.getByRole("button", { name: "Start game" }) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(screen.getByRole("button", { name: "Start game" })); expect(session.newGame).not.toHaveBeenCalled();
});
it.each(fixtures)("opens $game Load directly and preserves cancelled/rejected native replacement", async fixture => {
  const source = dirty(fixture); session.snapshot = source; vi.mocked(session.file).mockResolvedValue(null);
  render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "Load" }));
  expect(session.file).toHaveBeenCalledExactlyOnceWith(source, "open");
  expect(screen.queryByRole("dialog")).toBeNull(); expect(session.newGame).not.toHaveBeenCalled();
  await waitFor(() => expect(session.snapshot).toBe(source));
});
it.each(fixtures)("protects dirty $game native quit with concise Cancel and exact Discard", async fixture => {
  session.snapshot = dirty(fixture); session.closeRequested = true; render(<ProductApp />);
  const dialog = await screen.findByRole("dialog", { name: "Save Game" });
  expect(within(dialog).getAllByRole("button").map(button => button.textContent)).toEqual(["Save", "Discard", "Cancel"]);
  fireEvent.click(within(dialog).getByRole("button", { name: "Cancel" }));
  await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
  expect(session.cancelClose).toHaveBeenCalledOnce(); expect(session.finishClose).not.toHaveBeenCalled();
});
it.each(fixtures)("closes dirty $game with Discard only after native exact-token admission", async fixture => {
  const source = dirty(fixture); session.snapshot = source; session.closeRequested = true; render(<ProductApp />);
  fireEvent.click(await screen.findByRole("button", { name: "Discard" }));
  await waitFor(() => expect(session.finishClose).toHaveBeenCalledExactlyOnceWith(source));
  expect(session.file).not.toHaveBeenCalled(); expect(session.cancelClose).not.toHaveBeenCalled();
});
it.each(fixtures)("closes clean $game without an unsaved warning", async fixture => {
  const source = { ...fixture, close_requires_save: false, document: { ...fixture.document, dirty: false } } as GameSnapshot;
  session.snapshot = source; session.closeRequested = true; render(<ProductApp />);
  await waitFor(() => expect(session.finishClose).toHaveBeenCalledExactlyOnceWith(source));
  expect(screen.queryByRole("dialog")).toBeNull(); expect(session.file).not.toHaveBeenCalled();
});
it.each(fixtures)("keeps dirty $game and quit guard after cancelled, failed or stale Save", async fixture => {
  session.snapshot = dirty(fixture); session.closeRequested = true;
  vi.mocked(session.file).mockResolvedValueOnce(null).mockResolvedValueOnce(dirty(fixture));
  render(<ProductApp />); const dialog = await screen.findByRole("dialog", { name: "Save Game" });
  const save = within(dialog).getByRole("button", { name: "Save" });
  fireEvent.click(save); await screen.findByText("The game was not saved. Your game is still open.");
  expect(session.finishClose).not.toHaveBeenCalled(); expect(screen.getByRole("dialog")).toBe(dialog);
  fireEvent.click(save); await waitFor(() => expect(session.file).toHaveBeenCalledTimes(2));
  expect(session.finishClose).not.toHaveBeenCalled(); expect(session.snapshot?.document.dirty).toBe(true);
});
it.each(fixtures)("uses accepted clean $game Save receipt to close once", async fixture => {
  const source = dirty(fixture);
  const saved = { ...source, token: { ...source.token, revision: source.token.revision + 1 }, open_token: { ...source.open_token, version: source.open_token.version + 1 }, document: { ...source.document, dirty: false } } as GameSnapshot;
  session.snapshot = source; session.closeRequested = true; const receipt = deferred<GameSnapshot | null>(); vi.mocked(session.file).mockReturnValue(receipt.promise);
  render(<ProductApp />); const save = within(await screen.findByRole("dialog", { name: "Save Game" })).getByRole("button", { name: "Save" });
  fireEvent.click(save); fireEvent.click(save); expect(session.file).toHaveBeenCalledExactlyOnceWith(source, "save");
  receipt.resolve(saved); await waitFor(() => expect(session.finishClose).toHaveBeenCalledExactlyOnceWith(saved));
});
it("cannot close a replaced document using a stale clean Save receipt", async () => {
  session.snapshot = dirty(standard); session.closeRequested = true;
  vi.mocked(session.file).mockResolvedValue({ ...standard, token: { ...standard.token, document: standard.token.document + 1 }, document: { ...standard.document, dirty: false } });
  render(<ProductApp />); fireEvent.click(within(await screen.findByRole("dialog")).getByRole("button", { name: "Save" }));
  await screen.findByText("The game was not saved. Your game is still open."); expect(session.finishClose).not.toHaveBeenCalled();
});
it("blocks stale dirty quit admission and keeps a failed Cancel reservation guarded", async () => {
  session.closeRequested = true; vi.mocked(session.cancelClose).mockResolvedValue(false);
  const { rerender } = render(<ProductApp />); await screen.findByRole("dialog", { name: "Save Game" });
  session = { ...session, snapshot: { ...standard, open_token: { ...standard.open_token, document_revision: standard.open_token.document_revision + 1 } } };
  rerender(<ProductApp />);
  expect((screen.getByRole("button", { name: "Discard" }) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(screen.getByRole("button", { name: "Cancel" })); await waitFor(() => expect(session.cancelClose).toHaveBeenCalledOnce());
  expect(screen.getByRole("dialog")).toBeTruthy(); expect(session.finishClose).not.toHaveBeenCalled();
});
it("releases a refused clean close reservation without an unsaved warning", async () => {
  session.snapshot = { ...standard, close_requires_save: false, document: { ...standard.document, dirty: false } }; session.closeRequested = true;
  vi.mocked(session.finishClose).mockResolvedValue(false); render(<ProductApp />);
  await waitFor(() => expect(session.cancelClose).toHaveBeenCalledOnce()); expect(screen.queryByRole("dialog")).toBeNull();
});
it("replaces an uncommitted New Game chooser with the dirty native quit guard", async () => {
  const { rerender } = render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "New game" }));
  session = { ...session, closeRequested: true }; rerender(<ProductApp />);
  expect(await screen.findAllByRole("dialog")).toHaveLength(1); expect(screen.getByRole("dialog", { name: "Save Game" })).toBeTruthy(); expect(session.newGame).not.toHaveBeenCalled();
});
it("offers only implemented games and preserves side radio keyboard navigation", async () => {
  render(<ProductApp />); fireEvent.click(screen.getByRole("button", { name: "New game" }));
  expect(screen.getByRole("combobox", { name: "Game" }).tagName).toBe("SELECT");
  expect(screen.getAllByRole("option").map(option => [option.textContent, (option as HTMLOptionElement).value])).toEqual([["Chess", "chess"], ["Shogi", "shogi"], ["Xiangqi", "xiangqi"]]);
  expect(screen.getAllByRole("radio").map(radio => radio.textContent)).toEqual(["Sente", "Gote", "Random"]);
  const sente = screen.getByRole("radio", { name: "Sente" }); sente.focus(); fireEvent.keyDown(sente, { key: "ArrowRight" });
  const gote = screen.getByRole("radio", { name: "Gote" }); expect(document.activeElement).toBe(gote); expect(gote.getAttribute("aria-checked")).toBe("true");
});
it("retains Save at capacity with Load and no ordinary Save As", () => {
  session.error = "Application capacity: retained lineage replay budget."; render(<ProductApp />);
  expect(screen.getByRole("alert").textContent).toContain("The action could not complete");
  expect((screen.getByRole("button", { name: "Save" }) as HTMLButtonElement).disabled).toBe(false);
  expect(screen.getByRole("button", { name: "Load" })).toBeTruthy(); expect(screen.queryByRole("button", { name: /Save as/ })).toBeNull();
});
it.each(fixtures)("honors native $game clean-close authority despite a stop-only dirty journal", async fixture => {
  const source = { ...fixture, close_requires_save: false, document: { ...fixture.document, dirty: true, path: "/games/saved.mshogi" } } as GameSnapshot;
  session.snapshot = source; session.closeRequested = true;
  render(<ProductApp />);
  await waitFor(() => expect(session.finishClose).toHaveBeenCalledExactlyOnceWith(source));
  expect(screen.queryByRole("dialog")).toBeNull(); expect(session.file).not.toHaveBeenCalled();
  // Do not falsify canonical disk equality merely to suppress a prompt.
  expect(session.snapshot?.document.dirty).toBe(true);
});
it.each(fixtures)("guards newer native $game dirty-close authority after an earlier saved session", async fixture => {
  const source = { ...fixture, close_requires_save: true, document: { ...fixture.document, dirty: true, path: "/games/saved.mshogi" } } as GameSnapshot;
  session.snapshot = source; session.closeRequested = true; render(<ProductApp />);
  expect(await screen.findByRole("dialog", { name: "Save Game" })).toBeTruthy();
  expect(session.finishClose).not.toHaveBeenCalled(); expect(session.file).not.toHaveBeenCalled();
});

it.each(fixtures)("keeps Save enabled in the never-saved $game native Quit guard even without canonical changes", async fixture => {
  session.snapshot = { ...fixture, close_requires_save: true, document: { ...fixture.document, dirty: false, path: null } } as GameSnapshot;
  session.closeRequested = true; vi.mocked(session.file).mockResolvedValue(null);
  render(<ProductApp />);
  expect(await screen.findByRole("dialog", { name: "Save Game" })).toBeTruthy();
  const save = within(screen.getByRole("dialog", { name: "Save Game" })).getByRole("button", { name: "Save" }); expect((save as HTMLButtonElement).disabled).toBe(false);
  fireEvent.click(save); expect(await screen.findByText("The game was not saved. Your game is still open.")).toBeTruthy();
  expect(session.finishClose).not.toHaveBeenCalled(); expect(session.snapshot.document.path).toBeNull(); expect(session.snapshot.document.dirty).toBe(false);
});
