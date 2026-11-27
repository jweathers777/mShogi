// @vitest-environment jsdom
import { renderBoard, square } from "./testing/helpers";
import type { BoardGeometry } from "./testing/TestBoard";
import type { SyntheticSnapshot } from "./native";
import { afterEach, expect, it } from "vitest";
import { cleanup, fireEvent, screen } from "@testing-library/react";
import { TestBoard as Board, displayedSquares } from "./testing/TestBoard";

import nativeJson from "./__fixtures__/synthetic.json";

const fixture = nativeJson as SyntheticSnapshot;
const geometry: BoardGeometry = { ...fixture.geometry, initialFocus: fixture.geometry.initial_focus };
afterEach(cleanup);

function board() {
  return renderBoard(Board, { pieces: fixture.pieces, choices: fixture.choices,
    version: fixture.version, contextKey: fixture.profile, geometry,
    orientation: "white" as const, glyphs: { strider: "S", stone: "●" } });
}
it("uses native geometry for all 256 squares, full two-digit ranks and orientation", () => {
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
  expect(screen.getByText("Complete action here", { selector: "button" })).toBeTruthy();
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
  fireEvent.click(screen.getByText("Complete action here", { selector: "button" }));
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
  expect(screen.queryByText("Complete action here", { selector: "button" })).toBeNull();
  fireEvent.click(square("i9"));
  fireEvent.click(square("j10"));
  const finish = screen.getByText("Complete action here", { selector: "button" });
  finish.focus();
  fireEvent.keyDown(finish, { key: "Escape" });
  expect(document.activeElement).toBe(square("i9"));
  expect(screen.queryByText("Complete action here", { selector: "button" })).toBeNull();
  fireEvent.keyDown(square("i9"), { key: "Enter" });
  fireEvent.keyDown(square("i9"), { key: "ArrowUp" });
  fireEvent.keyDown(square("i10"), { key: "ArrowRight" });
  fireEvent.keyDown(square("j10"), { key: " " });
  fireEvent.click(screen.getByText("Cancel selection", { selector: "button" }));
  expect(document.activeElement).toBe(square("i9"));
  expect(onPlay).not.toHaveBeenCalled();
});
it("retires partial routes on native version, profile, disabled state and orientation changes", () => {
  const { props, rerender, onPlay } = board();
  fireEvent.click(square("i9")); fireEvent.click(square("j10"));
  rerender(<Board {...props} version={props.version + 1} />);
  expect(screen.queryByText("Complete action here", { selector: "button" })).toBeNull();
  fireEvent.click(square("i9")); fireEvent.click(square("j10"));
  rerender(<Board {...props} contextKey="a-different-presentation-context" />);
  expect(screen.queryByText("Complete action here", { selector: "button" })).toBeNull();
  fireEvent.click(square("i9")); fireEvent.click(square("j10"));
  rerender(<Board {...props} disabled />);
  fireEvent.click(square("i9"));
  expect(onPlay).not.toHaveBeenCalled();
  rerender(<Board {...props} />);
  fireEvent.click(square("i9")); fireEvent.click(square("j10"));
  rerender(<Board {...props} orientation="black" />);
  expect(screen.queryByText("Complete action here", { selector: "button" })).toBeNull();
  expect(onPlay).not.toHaveBeenCalled();
});
