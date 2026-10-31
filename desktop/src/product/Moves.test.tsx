// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { NativeHistory } from "./NativeSessionPanels";
import type { GameSnapshot, MovesPresentation } from "./native";
import shogiJson from "../__fixtures__/shared-shogi-standard.json";
import chessJson from "../__fixtures__/shared-chess-standard.json";
import xiangqiJson from "../__fixtures__/shared-xiangqi-standard.json";
afterEach(cleanup);
const moves: MovesPresentation = {
  root: { label: "Starting position", numbering: "Numbering begins at the imported counter; earlier moves are unknown." },
  entries: [
    { event_index: 4, board_ply: 1, number: 23, column: 1, notation: "H8+7", action: "b9c7", command_start: 3, command_end: 5 },
    { event_index: 6, board_ply: 2, number: 24, column: 0, notation: "C2=5", action: "h2e2", command_start: 5, command_end: 6 },
  ], last_action: null,
};
function snapshot(extra: Partial<GameSnapshot> = {}): GameSnapshot {
  return { ...xiangqiJson, moves, event_index: 4, events: [
    { index: 4, board_ply: 1, label: "Black board action" },
    { index: 5, board_ply: 1, label: "Black repetition warning" },
    { index: 6, board_ply: 2, label: "Red board action" },
    { index: 7, board_ply: 2, label: "Draw offered" },
  ], ...extra } as unknown as GameSnapshot;
}
it("uses native pair numbers and side columns without inventing an absent first-player move", () => {
  const command = vi.fn(); render(<NativeHistory snapshot={snapshot()} disabled={false} command={command} />);
  const rows = screen.getAllByRole("row");
  expect(rows).toHaveLength(3); expect(within(rows[0]!).getAllByRole("columnheader").map(node => node.textContent)).toEqual(["Move number", "Red", "Black"]);
  expect(within(rows[1]!).getAllByRole("cell")[0]?.textContent).toBe("");
  const blackMove = screen.getByRole("button", { name: "23. Black: H8+7" });
  expect(blackMove.getAttribute("aria-current")).toBe("step"); fireEvent.click(blackMove);
  expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "inspect", event_index: 4 });
  expect(screen.getByText(moves.root.numbering)).toBeTruthy();
  expect(screen.queryByRole("button", { name: /Black board action/ })).toBeNull();
});
it("selects action and warning as exact different prefixes, never their shared board ply or command end", async () => {
  const command = vi.fn(), state = snapshot({ event_index: 5 });
  const { rerender } = render(<NativeHistory snapshot={state} disabled={false} command={command} />);
  const warning = screen.getByRole("button", { name: /Black repetition warning/ });
  expect(warning.getAttribute("aria-current")).toBe("step");
  expect(screen.getByRole("button", { name: "23. Black: H8+7" }).getAttribute("aria-current")).toBeNull();
  expect(document.querySelectorAll('[data-associated-selection="true"]')).toHaveLength(1);
  expect(document.querySelectorAll(".move-event-count")).toHaveLength(1);
  const user = userEvent.setup(); warning.focus(); await user.keyboard("{Enter}");
  expect(command).toHaveBeenLastCalledWith({ kind: "inspect", event_index: 5 });
  fireEvent.click(screen.getByRole("button", { name: "23. Black: H8+7" }));
  expect(command).toHaveBeenLastCalledWith({ kind: "inspect", event_index: 4 });
  rerender(<NativeHistory snapshot={{ ...state, event_index: 6 }} disabled={false} command={command} />);
  expect(screen.getByRole("button", { name: "24. Red: C2=5" }).getAttribute("aria-current")).toBe("step");
  fireEvent.click(screen.getByRole("button", { name: "Starting position" })); expect(command).toHaveBeenLastCalledWith({ kind: "inspect", event_index: 0 });
});
it.each([["chess", chessJson, "White", "Black"], ["shogi", shogiJson, "Sente", "Gote"], ["xiangqi", xiangqiJson, "Red", "Black"]])("shares the empty Moves structure with native %s side labels", (_game, value, first, second) => {
  render(<NativeHistory snapshot={{ ...value, events: [], moves: { root: { label: "Starting position", numbering: "Known history only." }, entries: [], last_action: null } } as unknown as GameSnapshot} disabled={false} command={vi.fn()} />);
  expect(screen.getByRole("region", { name: "Moves" })).toBeTruthy();
  expect(screen.getAllByRole("columnheader").map(node => node.textContent)).toEqual(["Move number", first, second]);
  expect(screen.getByText("No moves in known history.")).toBeTruthy();
});
it("keeps native events selectable without creating move rows and disables every target during an operation", () => {
  const command = vi.fn(); render(<NativeHistory snapshot={snapshot()} disabled command={command} />);
  expect(screen.getAllByRole("row")).toHaveLength(3);
  expect(screen.getByText("Position events (2)")).toBeTruthy();
  for (const button of screen.getAllByRole("button", { hidden: true })) { expect((button as HTMLButtonElement).disabled).toBe(true); fireEvent.click(button); }
  expect(command).not.toHaveBeenCalled();
});
