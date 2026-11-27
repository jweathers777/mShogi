// @vitest-environment jsdom
// Native-generated inputs exercise shared presentation, never a rules oracle.
// SVG child order is asserted on the rendered DOM. Actual WebView pixel/hit-test
// evidence is supplied separately by the packaged GUI campaign.
import { renderGameBoard, square } from "./testing/helpers";
import type { GameSnapshot, MovementInspection } from "./native";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { GameBoard } from "./product/GameBoard";

import chessJson from "./__fixtures__/shared-chess-standard.json";
import shogiJson from "./__fixtures__/shared-shogi-standard.json";
import shogiDrops from "./__fixtures__/shared-shogi-drops.json";
import xiangqiJson from "./__fixtures__/shared-xiangqi-standard.json";
import xiangqiAgreed from "./__fixtures__/shared-xiangqi-agreed.json";
import toriJson from "./__fixtures__/shared-tori-contract.json";
import chuJson from "./__fixtures__/shared-chu-routes.json";
import chuCaptured from "./__fixtures__/shared-chu-captured.json";
import adamsJson from "./__fixtures__/shared-tenjiku-adams-standard.json";
import mullerJson from "./__fixtures__/shared-tenjiku-muller-standard.json";
import adamsCompound from "./__fixtures__/shared-tenjiku-adams-compound.json";
import mullerCompound from "./__fixtures__/shared-tenjiku-muller-compound.json";
import adamsPromoted from "./__fixtures__/shared-tenjiku-adams-promoted-hawk.json";
import mullerPromoted from "./__fixtures__/shared-tenjiku-muller-promoted-hawk.json";

const { inspect } = vi.hoisted(() => ({ inspect: vi.fn() }));
vi.mock("./product/useMovementInspection", () => ({ useMovementInspection: () => inspect }));
beforeEach(() => inspect.mockReset());
afterEach(cleanup);
const cast = (value: unknown) => value as GameSnapshot;
const cases = [
  { label: "Chess", source: cast(chessJson), first: "e2", name: "e2, White pawn", second: "d2", nextName: "d2, White pawn" },
  { label: "Shogi", source: cast(shogiJson), first: "1g", name: "1g, Sente pawn", second: "1i", nextName: "1i, Sente lance" },
  { label: "Xiangqi", source: cast(xiangqiJson), first: "a0", name: "a0, Red rook", second: "b0", nextName: "b0, Red horse" },
  { label: "Tori", source: cast(toriJson), first: "7c", name: "7c, Sente Falcon", second: "2d", nextName: "2d, Sente Swallow" },
  { label: "Chu", source: cast(chuJson), first: "6f", name: "6f, Sente Lion", second: "12l", nextName: "12l, Sente King" },
  { label: "Adams Tenjiku", source: cast(adamsJson), first: "9m", name: "9m, Sente Great General", second: "5k", nextName: "5k, Sente Dog" },
  { label: "Muller Tenjiku", source: cast(mullerJson), first: "9m", name: "9m, Sente Great General", second: "5k", nextName: "5k, Sente Dog" },
];

function help(container: HTMLElement) {
  return container.querySelector<HTMLElement>('[data-game-slot="footer"] .selected-piece-status')
    ?? container.querySelector<HTMLElement>(".board-help");
}

it.each(cases)("identifies friendly $label pieces in the stable footer with keyboard selection, replacement and Escape", ({ source, first, name, second, nextName }) => {
  const { container, command } = renderGameBoard(source);
  const footer = container.querySelector('[data-game-slot="footer"]') ?? container.querySelector(".board-wrap");
  const firstTarget = square(first, container);
  firstTarget.focus(); fireEvent.keyDown(firstTarget, { key: "Enter" });
  expect(help(container)?.textContent).toBe(`Selected ${name} · Esc clears`);
  expect(footer?.contains(help(container))).toBe(true);
  expect(firstTarget.getAttribute("aria-pressed")).toBe("true");
  expect(within(container).getByRole("status").textContent).toContain(`${name} selected. Choose a highlighted destination.`);
  expect(inspect).not.toHaveBeenCalled();
  fireEvent.click(square(second, container));
  expect(help(container)?.textContent).toBe(`Selected ${nextName} · Esc clears`);
  expect(firstTarget.getAttribute("aria-pressed")).toBe("false");
  fireEvent.keyDown(square(second, container), { key: "Escape" });
  expect(container.querySelector(".selected-piece-status")).toBeNull();
  expect(container.textContent).toContain("Arrows: move focus · Enter/Space: select · Esc: cancel");
  expect(document.activeElement).toBe(square(second, container));
  expect(within(container).getByRole("status").textContent).toBe("Selection cleared.");
  expect(command).not.toHaveBeenCalled();
});

it.each([cast(chuJson), cast(adamsCompound), cast(mullerCompound)])("retains the original $profile piece name during compound entry and clears after completion", source => {
  const route = source.choices.find(choice => choice.steps.length === 2 && choice.steps[1] === choice.from && choice.completion === null)!;
  expect(route).toBeTruthy();
  const name = source.pieces.find(piece => piece.square === route.from)!.label;
  const { container, command } = renderGameBoard(source);
  fireEvent.click(square(route.from, container));
  fireEvent.click(square(route.steps[0]!, container));
  expect(help(container)?.textContent).toBe(`Selected ${name} · Esc clears`);
  expect(container.querySelector(".route-selection,.shogi-route")).toBeTruthy();
  expect(command).not.toHaveBeenCalled();
  fireEvent.click(square(route.steps[1]!, container));
  expect(command).toHaveBeenCalledTimes(1);
  expect(container.querySelector(".selected-piece-status")).toBeNull();
  expect(within(container).getByRole("status").textContent).toBe("Complete native action submitted.");
});

it.each([
  { source: cast(toriJson), square: "7f", name: "7f, Sente Eagle (promoted Falcon)" },
  { source: cast(adamsPromoted), square: "8h", name: "8h, Sente Lion Hawk (promoted Lion)" },
  { source: cast(mullerPromoted), square: "8h", name: "8h, Sente Lion Hawk (promoted Lion)" },
])("keeps the native $source.profile promoted name and original identity", ({ source, square: coordinate, name }) => {
  const { container, command } = renderGameBoard(source);
  fireEvent.click(square(coordinate, container));
  expect(help(container)?.textContent).toBe(`Selected ${name} · Esc clears`);
  expect(command).not.toHaveBeenCalled();
});

it.each([
  { source: cast(shogiDrops), kind: "rook", name: "Sente rook in hand, 2" },
  { source: cast(toriJson), kind: "Cr", name: "Sente Crane in hand, 1" },
])("identifies $source.game hand selections with the existing accessible name", ({ source, kind, name }) => {
  const { container, command } = renderGameBoard(source);
  const hand = container.querySelector<HTMLButtonElement>(`button[data-hand="${kind}"][data-owner="${source.human.id.side}"]`)!;
  expect(hand.getAttribute("aria-label")).toBe(name);
  hand.focus(); fireEvent.keyDown(hand, { key: "Enter" });
  expect(help(container)?.textContent).toBe(`Selected ${name} · Esc clears`);
  fireEvent.keyDown(hand, { key: "Escape" });
  expect(container.querySelector(".selected-piece-status")).toBeNull();
  expect(document.activeElement).toBe(hand);
  expect(command).not.toHaveBeenCalled();
});

it.each(cases)("keeps $label enemy and immobile friendly identification read-only", async ({ source: playingSource }) => {
  // Native Xiangqi agreement and Chu's opponent-to-move prefix also retain
  // inspectable friendly pieces while publishing no actions for those pieces.
  const source = playingSource.game === "xiangqi" ? cast(xiangqiAgreed) : playingSource.game === "chu" ? cast(chuCaptured) : playingSource;
  const { container, command } = renderGameBoard(source);
  const immobile = source.pieces.find(piece => piece.owner === source.human.key && !source.choices.some(choice => choice.from === piece.square))!;
  const enemy = source.pieces.find(piece => piece.owner !== source.human.key)!;
  expect(immobile).toBeTruthy(); expect(enemy).toBeTruthy();
  for (const piece of [immobile, enemy]) {
    const side = source.sides.find(side => side.key === piece.owner)!;
    const label = piece.label.replace(`${piece.square}, `, "");
    const response: MovementInspection = { token: source.token, game: source.game, profile: source.profile,
      origin: piece.square, side, label, read_only: true, choices: [] };
    inspect.mockResolvedValueOnce(response);
    fireEvent.click(square(piece.square, container));
    const status = await screen.findByText(`Inspecting ${label} · Esc clears`);
    expect(status.classList.contains("movement-inspection-status") || status.parentElement?.classList.contains("movement-inspection-status")).toBe(true);
    expect(container.querySelector(".selected-piece-status")).toBeNull();
    expect(within(container).getByRole("status").textContent).toContain("Read-only movement choices; no action can be played.");
    expect(container.querySelector(".capture-marker,.move-marker,.shogi-destination,.xiangqi-destination")).toBeNull();
    fireEvent.keyDown(square(piece.square, container), { key: "Escape" });
    expect(container.querySelector(".movement-inspection-status")).toBeNull();
  }
  expect(inspect).toHaveBeenCalledTimes(2);
  expect(command).not.toHaveBeenCalled();
});

it("retires the computed name on a changed native context and after an ordinary completed move", () => {
  const source = cast(chessJson), command = vi.fn();
  const { container, rerender } = render(<GameBoard snapshot={source} disabled={false} inspectionDisabled={false} command={command} />);
  fireEvent.click(square("e2", container));
  const changed = { ...source, token: { ...source.token, revision: source.token.revision + 1 } };
  rerender(<GameBoard snapshot={changed} disabled={false} inspectionDisabled={false} command={command} />);
  expect(container.querySelector(".selected-piece-status")).toBeNull();
  fireEvent.click(square("e2", container));
  const quiet = square("e3", container).querySelector(".move-marker")!;
  expect(quiet.getAttribute("r")).toBe("12");
  expect(quiet.getAttribute("aria-hidden")).toBe("true");
  expect(quiet.getAttribute("pointer-events")).toBe("none");
  fireEvent.click(square("e3", container));
  expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "play", action: "e2e3" });
  expect(container.querySelector(".selected-piece-status")).toBeNull();
});

it.each([
  { label: "Chess pawn presentation contract", source: cast(chessJson), from: "e2", destination: "e7", art: ".piece", marker: ".capture-marker", contract: true },
  { label: "Shogi pawn presentation contract", source: cast(shogiJson), from: "1g", destination: "1a", art: ".piece-tile", marker: ".shogi-destination", contract: true },
  { label: "Xiangqi cannon", source: cast(xiangqiJson), from: "b2", destination: "b9", art: ".xiangqi-piece", marker: ".xiangqi-destination" },
  { label: "Tori falcon", source: cast(toriJson), from: "7c", destination: "7b", art: ".piece-tile", marker: ".shogi-destination" },
  { label: "Chu lion", source: cast(chuJson), from: "6f", destination: "7e", art: ".piece-tile", marker: ".capture-marker" },
  { label: "Adams dense Great General", source: cast(adamsJson), from: "9m", destination: "9d", art: ".piece-tile", marker: ".capture-marker" },
  { label: "Muller dense Great General", source: cast(mullerJson), from: "9m", destination: "9d", art: ".piece-tile", marker: ".capture-marker" },
])("paints the $label capture outline after the occupied native piece without taking pointer events", ({ source, from, destination, art, marker, contract }) => {
  // These two adapters isolate the plain-text Chess and Shogi art paths. Their
  // supplied choices test paint/hit dispatch only, not game-rule correctness.
  const input = contract ? { ...source, choices: [{ action: "capture-paint-contract", from, origin: { kind: "board" as const, square: from }, steps: [destination], completion: null }] } : source;
  const { container, command } = renderGameBoard(input);
  fireEvent.click(square(from, container));
  const destinationSquare = square(destination, container);
  const artwork = destinationSquare.querySelector(art)!, overlay = destinationSquare.querySelector(marker)!;
  expect(artwork).toBeTruthy(); expect(overlay).toBeTruthy();
  // A later SVG sibling paints above the piece subtree. This fails for the old
  // shared Board implementation even though its capture class already existed.
  expect(artwork.compareDocumentPosition(overlay) & Node.DOCUMENT_POSITION_FOLLOWING).toBeTruthy();
  expect(overlay.closest('svg[role="group"]')).toBe(artwork.closest('svg[role="group"]'));
  expect(overlay.getAttribute("pointer-events")).toBe("none");
  expect(overlay.getAttribute("aria-hidden")).toBe("true");
  expect(destinationSquare.getAttribute("aria-label")).toContain("legal destination");
  expect(destinationSquare.querySelector(".inspection-marker")).toBeNull();
  fireEvent.click(destinationSquare);
  // Depending on native complete-route alternatives this enters a prefix,
  // completes immediately, or opens a native completion dialog. None inspects.
  expect(inspect).not.toHaveBeenCalled();
  expect(destinationSquare.getAttribute("aria-label")?.includes("route step") || command.mock.calls.length > 0 || screen.queryByRole("dialog")).toBeTruthy();
});
