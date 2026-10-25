// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { Board, displayedSquares, type BoardGeometry, type BoardChoice } from "./Board";
import type { BoardPiece } from "./native";
import nativeJson from "./__fixtures__/synthetic.json";

interface Fixture {
  profile: string;
  version: number;
  geometry: Omit<BoardGeometry, "initialFocus"> & { initial_focus: string };
  pieces: BoardPiece[];
  choices: BoardChoice[];
}
const fixture = nativeJson as Fixture;
const geometry: BoardGeometry = { ...fixture.geometry, initialFocus: fixture.geometry.initial_focus };
afterEach(cleanup);
function square(name: string) {
  return screen.getByRole("button", { name: new RegExp(`^${name},`) });
}
function board() {
  const onPlay = vi.fn();
  const props = { pieces: fixture.pieces, choices: fixture.choices,
    version: fixture.version, profile: fixture.profile, geometry,
    orientation: "white" as const, disabled: false,
    glyphs: { strider: "S", stone: "●" }, onPlay };
  return { ...render(<Board {...props} />), props, onPlay };
}
it("uses native geometry for all256 squares, full two-digit ranks and orientation", () => {
  const { props, rerender } = board();
  expect(screen.getAllByRole("button")).toHaveLength(256);
  expect(square("p16")).toBeTruthy();
  expect(displayedSquares("white", geometry)[0]).toBe("a16");
  expect(displayedSquares("black", geometry)[0]).toBe("p1");
  expect(screen.getByRole("group").textContent).toContain("16");
  square("p16").focus();
  fireEvent.keyDown(square("p16"), { key: "ArrowRight" });
  expect(document.activeElement).toBe(square("p16"));
  fireEvent.keyDown(square("p16"), { key: "ArrowDown" });
  expect(document.activeElement).toBe(square("p15"));
  rerender(<Board {...props} orientation="black" />);
  fireEvent.keyDown(square("p15"), { key: "ArrowUp" });
  expect(document.activeElement).toBe(square("p14"));
});
it("preserves distinct native routes sharing origin and endpoint without committing a prefix", () => {
  const { props, rerender, onPlay } = board();
  fireEvent.click(square("i9"));
  fireEvent.click(square("j10"));
  expect(onPlay).not.toHaveBeenCalled();
  expect(screen.getByText(/Selected route: i9 → j10/)).toBeTruthy();
  expect(screen.getByRole("button", { name: "Complete action here" })).toBeTruthy();
  fireEvent.click(square("i9"));
  expect(onPlay).toHaveBeenLastCalledWith("136>153>136");
  rerender(<Board {...props} version={props.version + 1} />);
  fireEvent.click(square("i9"));
  fireEvent.click(square("h9"));
  expect(onPlay).toHaveBeenCalledTimes(1);
  fireEvent.click(square("i9"));
  expect(onPlay).toHaveBeenLastCalledWith("136>135>136");
  expect(onPlay).toHaveBeenCalledTimes(2);
});
it("requires explicit completion of a terminal prefix and submits the supplied short native choice", () => {
  const { onPlay } = board();
  fireEvent.click(square("i9"));
  fireEvent.click(square("j10"));
  expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Complete action here" }));
  expect(onPlay).toHaveBeenCalledExactlyOnceWith("136>153");
});
it("cancels stages by Escape or Cancel and restores source focus without an action", () => {
  const { onPlay } = board();
  fireEvent.click(square("i9"));
  fireEvent.keyDown(square("i9"), { key: "Escape" });
  expect(onPlay).not.toHaveBeenCalled();
  fireEvent.click(square("i9"));
  fireEvent.click(square("j10"));
  fireEvent.keyDown(square("j10"), { key: "Escape" });
  expect(document.activeElement).toBe(square("i9"));
  expect(screen.queryByRole("button", { name: "Complete action here" })).toBeNull();
  fireEvent.click(square("i9"));
  fireEvent.click(square("j10"));
  const finish = screen.getByRole("button", { name: "Complete action here" });
  finish.focus();
  fireEvent.keyDown(finish, { key: "Escape" });
  expect(document.activeElement).toBe(square("i9"));
  expect(screen.queryByRole("button", { name: "Complete action here" })).toBeNull();
  fireEvent.keyDown(square("i9"), { key: "Enter" });
  fireEvent.keyDown(square("i9"), { key: "ArrowUp" });
  fireEvent.keyDown(square("i10"), { key: "ArrowRight" });
  fireEvent.keyDown(square("j10"), { key: " " });
  fireEvent.click(screen.getByRole("button", { name: "Cancel selection" }));
  expect(document.activeElement).toBe(square("i9"));
  expect(onPlay).not.toHaveBeenCalled();
});
it("retires partial routes on native version, profile, disabled state and orientation changes", () => {
  const { props, rerender, onPlay } = board();
  fireEvent.click(square("i9")); fireEvent.click(square("j10"));
  rerender(<Board {...props} version={props.version + 1} />);
  expect(screen.queryByRole("button", { name: "Complete action here" })).toBeNull();
  fireEvent.click(square("i9")); fireEvent.click(square("j10"));
  rerender(<Board {...props} profile="a-different-presentation-context" />);
  expect(screen.queryByRole("button", { name: "Complete action here" })).toBeNull();
  fireEvent.click(square("i9")); fireEvent.click(square("j10"));
  rerender(<Board {...props} disabled />);
  fireEvent.click(square("i9"));
  expect(onPlay).not.toHaveBeenCalled();
  rerender(<Board {...props} />);
  fireEvent.click(square("i9")); fireEvent.click(square("j10"));
  rerender(<Board {...props} orientation="black" />);
  expect(screen.queryByRole("button", { name: "Complete action here" })).toBeNull();
  expect(onPlay).not.toHaveBeenCalled();
});
