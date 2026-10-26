// @vitest-environment jsdom
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { NativeBoard, nativeSnapshotIsNewer, type NativeBoardSnapshot } from "./NativeBoard";
import standard from "./__fixtures__/shared-shogi-standard.json";
import drops from "./__fixtures__/shared-shogi-drops.json";
import promotion from "./__fixtures__/shared-shogi-promotion.json";
import forced from "./__fixtures__/shared-shogi-forced.json";
afterEach(cleanup);
function square(name: string) {
  const element = document.querySelector<SVGGElement>(`svg [role="button"][aria-label^="${name},"]`);
  if (!element) throw new Error(`Missing ${name}`);
  return element;
}
function board(snapshot: NativeBoardSnapshot = standard as NativeBoardSnapshot) {
  const onPlay = vi.fn();
  const props = { snapshot, bottomSide: snapshot.orientations[0]!.side.key, onPlay };
  return { ...render(<NativeBoard {...props} />), props, onPlay };
}
it("uses native semantic identities, both hands and explicit Sente/Gote ordering", () => {
  const { props, rerender } = board();
  expect(document.querySelectorAll("svg [role=button]")).toHaveLength(81);
  expect(screen.getByRole("group", { name: /Sente at bottom/ })).toBeTruthy();
  expect(document.querySelector("svg [role=button]")?.getAttribute("aria-label")).toMatch(/^9a, Gote L/);
  expect(screen.getByRole("button", { name: "Sente hand P, 0" })).toBeTruthy();
  expect(screen.getByRole("button", { name: "Gote hand R, 0" })).toBeTruthy();
  const owner = square("9g").querySelector(".piece")?.getAttribute("data-owner");
  expect(square("9g").querySelector(".piece")?.classList.contains("native-first-owner")).toBe(true);
  rerender(<NativeBoard {...props} bottomSide={props.snapshot.orientations[1]!.side.key} />);
  expect(document.querySelector("svg [role=button]")?.getAttribute("aria-label")).toMatch(/^1i, Sente L/);
  expect(square("9g").querySelector(".piece")?.getAttribute("data-owner")).toBe(owner);
  expect(square("9g").querySelector(".piece")?.classList.contains("native-first-owner")).toBe(true);
  fireEvent.keyDown(square("5g"), { key: "ArrowDown" });
  expect(document.activeElement).toBe(square("5f"));
});
it("cancels a native hand origin with keyboard and submits only a complete drop", async () => {
  const { onPlay } = board(drops as NativeBoardSnapshot); const user = userEvent.setup();
  const pawn = screen.getByRole("button", { name: "Sente hand P, 1" });
  pawn.focus(); await user.keyboard("{Enter}");
  expect(onPlay).not.toHaveBeenCalled(); expect(pawn.getAttribute("aria-pressed")).toBe("true");
  fireEvent.keyDown(pawn, { key: "ArrowDown" }); expect(document.activeElement).toBe(square("5g"));
  fireEvent.keyDown(square("5g"), { key: "Escape" }); expect(document.activeElement).toBe(pawn);
  expect(onPlay).not.toHaveBeenCalled();
  await user.keyboard("{Enter}"); fireEvent.click(square("5e"));
  expect(onPlay).toHaveBeenCalledExactlyOnceWith("P*5e");
});
it("keeps optional promotion/decline separate and cancels without a partial action", () => {
  const { onPlay } = board(promotion as NativeBoardSnapshot);
  fireEvent.click(square("5d")); fireEvent.click(square("5c")); expect(onPlay).not.toHaveBeenCalled();
  expect(screen.getByRole("button", { name: "Promote" })).toBeTruthy();
  expect(screen.getByRole("button", { name: "Do not promote" })).toBeTruthy();
  fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" }); expect(document.activeElement).toBe(square("5c"));
  expect(onPlay).not.toHaveBeenCalled(); fireEvent.click(square("5d")); fireEvent.click(square("5c"));
  fireEvent.click(screen.getByRole("button", { name: "Do not promote" })); expect(onPlay).toHaveBeenCalledExactlyOnceWith("5d5c");
});
it("offers only native forced promotion and retires selection across replacement", () => {
  const { props, rerender, onPlay } = board(forced as NativeBoardSnapshot);
  fireEvent.click(square("5b")); fireEvent.click(square("5a"));
  expect(screen.queryByRole("button", { name: "Do not promote" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Promote" })); expect(onPlay).toHaveBeenCalledExactlyOnceWith("5b5a+");
  fireEvent.click(square("5b"));
  const replacement = { ...props.snapshot, token: { ...props.snapshot.token, document: props.snapshot.token.document + 1 } };
  rerender(<NativeBoard {...props} snapshot={replacement} />); fireEvent.click(square("5a")); expect(onPlay).toHaveBeenCalledTimes(1);
  rerender(<NativeBoard {...props} snapshot={{ ...replacement, capabilities: { board_action: false } }} />);
  fireEvent.click(square("5b")); fireEvent.click(square("5a")); expect(onPlay).toHaveBeenCalledTimes(1);
});
it("rejects old frontend snapshots across identical coordinates and semantic undo", () => {
  const old: NativeBoardSnapshot = standard as NativeBoardSnapshot;
  const fresh = { ...old, token: { ...old.token, document: old.token.document + 1, revision: old.token.revision + 5 } };
  expect(nativeSnapshotIsNewer(null, old)).toBe(true); expect(nativeSnapshotIsNewer(fresh, old)).toBe(false);
  expect(nativeSnapshotIsNewer(old, fresh)).toBe(true); expect(nativeSnapshotIsNewer(fresh, fresh)).toBe(false);
});
