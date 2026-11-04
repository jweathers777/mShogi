// @vitest-environment jsdom
// Native-generated controller snapshots supply every complete action and effect.
// These tests establish frontend consumption, not new rules or search conformance.
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { TenjikuBoard } from "./TenjikuBoard";
import { GameBoard } from "./GameBoard";
import type { TenjikuSnapshot } from "./native";
import type { MovementInspection } from "./movementInspection";
import adamsCompound from "../__fixtures__/shared-tenjiku-adams-compound.json";
import mullerCompound from "../__fixtures__/shared-tenjiku-muller-compound.json";
import adamsPlayed from "../__fixtures__/shared-tenjiku-adams-compound-played.json";
import mullerPlayed from "../__fixtures__/shared-tenjiku-muller-compound-played.json";
import adamsPromotion from "../__fixtures__/shared-tenjiku-adams-promotion.json";
import mullerPromotion from "../__fixtures__/shared-tenjiku-muller-promotion.json";
import adamsPromoted from "../__fixtures__/shared-tenjiku-adams-promoted.json";
import mullerPromoted from "../__fixtures__/shared-tenjiku-muller-promoted.json";
import adamsDeclined from "../__fixtures__/shared-tenjiku-adams-declined.json";
import mullerDeclined from "../__fixtures__/shared-tenjiku-muller-declined.json";
import adamsIgui from "../__fixtures__/shared-tenjiku-adams-igui.json";
import mullerIgui from "../__fixtures__/shared-tenjiku-muller-igui.json";
import adamsIguied from "../__fixtures__/shared-tenjiku-adams-iguied.json";
import mullerIguied from "../__fixtures__/shared-tenjiku-muller-iguied.json";
import adamsBurned from "../__fixtures__/shared-tenjiku-adams-burned.json";
import mullerBurned from "../__fixtures__/shared-tenjiku-muller-burned.json";
import adamsSelected from "../__fixtures__/shared-tenjiku-adams-selected-root.json";
import mullerSelected from "../__fixtures__/shared-tenjiku-muller-selected-root.json";
import adamsInspection from "../__fixtures__/shared-tenjiku-adams-inspection.json";
import mullerInspection from "../__fixtures__/shared-tenjiku-muller-inspection.json";
import adamsHawk from "../__fixtures__/shared-tenjiku-adams-hawk.json";
import mullerHawk from "../__fixtures__/shared-tenjiku-muller-hawk.json";
import adamsHawkPlayed from "../__fixtures__/shared-tenjiku-adams-hawk-played.json";
import mullerHawkPlayed from "../__fixtures__/shared-tenjiku-muller-hawk-played.json";
import adamsPromotedHawk from "../__fixtures__/shared-tenjiku-adams-promoted-hawk.json";
import mullerPromotedHawk from "../__fixtures__/shared-tenjiku-muller-promoted-hawk.json";
import adamsArea from "../__fixtures__/shared-tenjiku-adams-area.json";
import mullerArea from "../__fixtures__/shared-tenjiku-muller-area.json";
import adamsAreaPlayed from "../__fixtures__/shared-tenjiku-adams-area-played.json";
import mullerAreaPlayed from "../__fixtures__/shared-tenjiku-muller-area-played.json";
import adamsJump from "../__fixtures__/shared-tenjiku-adams-ranked-jump.json";
import mullerJump from "../__fixtures__/shared-tenjiku-muller-ranked-jump.json";
import adamsJumpPlayed from "../__fixtures__/shared-tenjiku-adams-ranked-jump-played.json";
import mullerJumpPlayed from "../__fixtures__/shared-tenjiku-muller-ranked-jump-played.json";
import adamsBlocked from "../__fixtures__/shared-tenjiku-adams-ranked-blocked.json";
import mullerBlocked from "../__fixtures__/shared-tenjiku-muller-ranked-blocked.json";

const fixtures = [
  { label: "Adams Modern", compound: adamsCompound, played: adamsPlayed, promotion: adamsPromotion, promoted: adamsPromoted, declined: adamsDeclined, igui: adamsIgui, iguied: adamsIguied, burned: adamsBurned, selected: adamsSelected, inspection: adamsInspection, hawk: adamsHawk, hawkPlayed: adamsHawkPlayed, promotedHawk: adamsPromotedHawk, area: adamsArea, areaPlayed: adamsAreaPlayed, jump: adamsJump, jumpPlayed: adamsJumpPlayed, blocked: adamsBlocked },
  { label: "Muller Completed", compound: mullerCompound, played: mullerPlayed, promotion: mullerPromotion, promoted: mullerPromoted, declined: mullerDeclined, igui: mullerIgui, iguied: mullerIguied, burned: mullerBurned, selected: mullerSelected, inspection: mullerInspection, hawk: mullerHawk, hawkPlayed: mullerHawkPlayed, promotedHawk: mullerPromotedHawk, area: mullerArea, areaPlayed: mullerAreaPlayed, jump: mullerJump, jumpPlayed: mullerJumpPlayed, blocked: mullerBlocked },
];
const snapshot = (value: unknown) => value as TenjikuSnapshot;
afterEach(cleanup);
function square(key: string) {
  const node = document.querySelector<SVGGElement>(`svg [role="button"][aria-label^="${key},"]`);
  if (!node) throw new Error(`Missing native Tenjiku coordinate ${key}`);
  return node;
}
function board(value: unknown) {
  const props = { snapshot: snapshot(value), disabled: false, inspectionDisabled: false, onPlay: vi.fn(), onInspect: vi.fn() };
  return { ...render(<TenjikuBoard {...props} />), props };
}
function returningChoices(value: TenjikuSnapshot) {
  return value.choices.filter(choice => choice.steps.length === 2 && choice.steps[1] === choice.from);
}

it.each(fixtures)("renders the actual $label 16×16 board in both orientations without advertising engine play", fixture => {
  const { container, props, rerender } = board(fixture.compound);
  expect(container.querySelectorAll("svg [role=button]")).toHaveLength(256);
  expect(container.querySelector("svg")?.getAttribute("viewBox")).toBe("0 0 1680 1680");
  expect(container.querySelector("svg [role=button]")?.getAttribute("aria-label")).toMatch(/^16a,/);
  expect(container.querySelectorAll(".native-coordinate-frame .coordinate")).toHaveLength(32);
  expect(container.querySelectorAll(".square .coordinate")).toHaveLength(0);
  expect(container.querySelector('[data-game-slot="footer"] [data-position-status]')).toBeTruthy();
  expect(container.querySelector('[data-game-slot="upper-player"]')?.textContent).toContain("Native side");
  expect(container.querySelector(".shogi-hand")).toBeNull();
  expect(props.snapshot.hands).toEqual([]);
  expect(props.snapshot.capabilities.engine_turn).toBe(false);
  expect(props.snapshot.capabilities.move_analysis).toBe(false);
  expect(square("1a").querySelector("[data-tenjiku-piece]")?.getAttribute("transform")).toBe("rotate(180 50 50)");
  fireEvent.keyDown(square("8h"), { key: "ArrowRight" });
  expect(document.activeElement).toBe(square("7h"));
  const flipped = { ...props.snapshot, document: { ...props.snapshot.document, settings: { ...props.snapshot.document.settings, settings: { orientation: "gote" as const } } } };
  rerender(<TenjikuBoard {...props} snapshot={flipped} />);
  expect(container.querySelector("svg [role=button]")?.getAttribute("aria-label")).toMatch(/^1p,/);
  expect(square("1a").querySelector("[data-tenjiku-piece]")?.getAttribute("transform")).toBeNull();
  fireEvent.keyDown(square("8h"), { key: "ArrowRight" });
  expect(document.activeElement).toBe(square("9h"));
  expect(props.onPlay).not.toHaveBeenCalled();
});

it.each(fixtures)("selects different native $label captures returning to the same endpoint by pointer and keyboard", fixture => {
  const source = snapshot(fixture.compound);
  const returns = returningChoices(source).filter(choice => ["8g", "9g"].includes(choice.steps[0]!));
  expect(new Set(returns.map(choice => choice.steps[0])).size).toBeGreaterThan(1);
  for (const [index, choice] of returns.slice(0, 2).entries()) {
    const { container, props, unmount } = board(source);
    if (index === 0) { fireEvent.click(square(choice.from)); fireEvent.click(square(choice.steps[0]!)); }
    else { fireEvent.keyDown(square(choice.from), { key: "Enter" }); fireEvent.keyDown(square(choice.steps[0]!), { key: " " }); }
    expect(props.onPlay).not.toHaveBeenCalled();
    expect(container.querySelector("[data-selected-route]")?.getAttribute("data-selected-route")).toBe(`${choice.from}>${choice.steps[0]}`);
    expect(screen.getByRole("group", { name: "Partial route selection" }).closest('[data-game-slot="footer"]')).toBeTruthy();
    if (index === 0) fireEvent.click(square(choice.from));
    else fireEvent.keyDown(square(choice.from), { key: "Enter" });
    expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(choice.action);
    expect(container.querySelector("[data-selected-route]")).toBeNull();
    unmount();
  }
});

it.each(fixtures)("requires explicit completion of a native $label terminal prefix and recovers focus on Escape", fixture => {
  const source = snapshot(fixture.compound), route = returningChoices(source)[0]!;
  const short = source.choices.find(choice => choice.from === route.from && choice.steps.length === 1 && choice.steps[0] === route.steps[0])!;
  expect(short).toBeTruthy();
  const { props, container, rerender } = board(source);
  fireEvent.click(square(route.from)); fireEvent.click(square(route.steps[0]!));
  expect(props.onPlay).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Complete action here" }));
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(short.action);
  expect(document.activeElement).toBe(square(route.steps[0]!));
  // A changed native selection retires old submission authority; actions remain
  // the exact generated choices, with no local movement reconstruction.
  rerender(<TenjikuBoard {...props} snapshot={{ ...source, token: { ...source.token, selection: source.token.selection + 1 } }} />);
  fireEvent.keyDown(square(route.from), { key: "Enter" }); fireEvent.keyDown(square(route.steps[0]!), { key: " " });
  const cancel = screen.getByRole("button", { name: "Cancel selection" }); cancel.focus();
  fireEvent.keyDown(cancel, { key: "Escape" });
  expect(document.activeElement).toBe(square(route.from));
  expect(container.querySelector("[data-selected-route]")).toBeNull();
  expect(screen.queryByRole("group", { name: "Partial route selection" })).toBeNull();
  expect(props.onPlay).toHaveBeenCalledTimes(1);
});

it.each(fixtures)("keeps $label promotion and decline as explicit complete native actions", fixture => {
  const source = snapshot(fixture.promotion);
  const promote = source.choices.find(choice => choice.completion?.label === "Promote")!;
  const decline = source.choices.find(choice => choice.from === promote.from && JSON.stringify(choice.steps) === JSON.stringify(promote.steps) && choice.completion?.label === "Do not promote")!;
  expect(promote).toBeTruthy(); expect(decline).toBeTruthy();
  const { props } = board(source);
  fireEvent.click(square(promote.from)); fireEvent.click(square(promote.steps[0]!));
  const dialog = screen.getByRole("dialog");
  expect(props.onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(dialog, { key: "Escape" });
  expect(document.activeElement).toBe(square(promote.steps[0]!));
  expect(props.onPlay).not.toHaveBeenCalled();
  fireEvent.click(square(promote.from)); fireEvent.click(square(promote.steps[0]!));
  fireEvent.click(screen.getByRole("button", { name: "Do not promote" }));
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(decline.action);
  const rights = snapshot(fixture.declined).tenjiku.selected.identities;
  expect(rights.some(identity => identity.piece.right === "first_declined_next_move" && !identity.piece.promoted)).toBe(true);
});

it.each(fixtures)("commits $label promotion only on its selected native completion", fixture => {
  const source = snapshot(fixture.promotion), promote = source.choices.find(choice => choice.completion?.label === "Promote")!;
  const command = vi.fn();
  render(<GameBoard snapshot={source} disabled={false} inspectionDisabled={false} command={command} />);
  fireEvent.keyDown(square(promote.from), { key: "Enter" }); fireEvent.keyDown(square(promote.steps[0]!), { key: " " });
  expect(command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Promote" }));
  expect(command).toHaveBeenCalledExactlyOnceWith({ kind: "tenjiku", actor: source.human.id.side, command: { kind: "play", action: JSON.parse(promote.action) } });
});

it.each(fixtures)("selects a $label stationary target without creating a movement route", fixture => {
  const source = snapshot(fixture.igui), choice = source.choices.find(choice => choice.tenjiku?.kind === "stationary_capture")!;
  expect(choice).toBeTruthy();
  const action = JSON.parse(choice.action) as { kind: string; route: unknown[]; target: unknown };
  expect(action.kind).toBe("igui"); expect(action.route).toEqual([]);
  expect(choice.steps).toHaveLength(1);
  const { props, container } = board(source);
  fireEvent.click(square(choice.from));
  expect(props.onPlay).not.toHaveBeenCalled();
  expect(container.querySelector("[data-selected-route]")).toBeNull();
  fireEvent.click(square(choice.steps[0]!));
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(choice.action);
  expect(container.querySelector("[data-selected-route]")).toBeNull();
});

it.each(fixtures)("decorates source-correct $label burn/death phases independently of the actual mover route", fixture => {
  const source = snapshot(fixture.burned);
  const { container } = render(<TenjikuBoard snapshot={source} disabled inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} completedAction={source.moves.last_action} />);
  const phases = [...container.querySelectorAll("[data-native-effect-phase]")].map(node => [node.getAttribute("data-native-effect-phase"), node.getAttribute("data-effect-phase-index"), node.getAttribute("data-location")]);
  expect(phases).toEqual(fixture.label === "Adams Modern" ? [["burn", "1", "8g"], ["mover_dies", "2", "8h"]] : [["mover_dies", "1", "8h"]]);
  expect(container.querySelector("[data-route]")).toBeNull();
  expect(container.querySelector('[data-last-move="origin"]')?.getAttribute("data-location")).toBe("8j");
  expect(container.querySelector('[data-last-move="destination"]')?.getAttribute("data-location")).toBe("8h");
  expect(container.querySelector('[data-last-move="destination"][data-location="8g"]')).toBeNull();
});

it.each(fixtures)("retains $label compound, promotion and stationary completed geometry without fake travel", fixture => {
  for (const value of [fixture.played, fixture.promoted, fixture.iguied]) {
    const source = snapshot(value), action = source.moves.last_action!;
    const { container, unmount } = render(<TenjikuBoard snapshot={source} disabled inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} completedAction={action} />);
    if (value === fixture.played) {
      expect(action.steps).toHaveLength(2);
      expect(container.querySelector("[data-route]")?.getAttribute("data-route")).toBe(`${action.origin.kind === "board" ? action.origin.square : ""}>${action.steps.join(">")}`);
      expect(container.querySelectorAll('[data-native-effect-phase="capture"]').length).toBeGreaterThan(0);
    } else if (value === fixture.promoted) {
      expect(container.querySelectorAll('[data-native-effect-phase="promotion"]')).toHaveLength(1);
      expect(container.querySelector('[data-native-effect-phase="promotion"]')?.getAttribute("data-location")).toBe(action.steps.at(-1));
      expect(container.querySelector("[data-route]")).toBeNull();
    } else {
      expect(action.steps).toEqual([]);
      expect(container.querySelector("[data-route]")).toBeNull();
      expect(container.querySelector('[data-last-move="destination"]')).toBeNull();
      expect(container.querySelector("[data-native-stationary-target]")?.getAttribute("data-native-stationary-target")).toBe("9h");
      expect(container.querySelector('[data-native-effect-phase="capture"]')?.getAttribute("data-location")).toBe("9h");
    }
    unmount();
  }
});

it.each(fixtures)("keeps exact native $label historical inspection visibly read-only", async fixture => {
  const source = snapshot(fixture.selected), response = fixture.inspection as unknown as MovementInspection;
  expect(source.historical).toBe(true);
  expect(response.token).toEqual(source.token);
  expect(response.profile).toBe(source.profile);
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue(response);
  const { container } = render(<TenjikuBoard snapshot={source} disabled inspectionDisabled={false} onPlay={onPlay} onInspect={onInspect} />);
  fireEvent.click(square(response.origin));
  await vi.waitFor(() => expect(screen.getByText(`Inspecting ${response.label} · Esc clears`)).toBeTruthy());
  expect(container.querySelectorAll(".inspection-marker").length).toBeGreaterThan(0);
  expect(container.querySelectorAll(".move-marker,.capture-marker")).toHaveLength(0);
  fireEvent.click(square(response.choices[0]!.steps[0]!));
  expect(onPlay).not.toHaveBeenCalled();
  fireEvent.keyDown(square(response.origin), { key: "Escape" });
  expect(container.querySelectorAll(".inspection-marker")).toHaveLength(0);
});

it.each(fixtures)("consumes native $label Lion Hawk and promoted-Lion complete two-capture choices", fixture => {
  for (const value of [fixture.hawk, fixture.promotedHawk]) {
    const source = snapshot(value), choice = source.choices.find(choice => choice.from === "8h" && JSON.stringify(choice.steps) === JSON.stringify(["9h", "10h"]))!;
    expect(choice).toBeTruthy();
    const direct = source.choices.find(choice => choice.from === "8h" && JSON.stringify(choice.steps) === JSON.stringify(["10h"]))!;
    expect(direct).toBeTruthy(); expect(direct.action).not.toBe(choice.action);
    const identity = source.tenjiku.selected.identities.find(piece => piece.square === "8h")!;
    expect(identity.face).toBe("LHk");
    expect(identity.piece.base).toBe(value === fixture.hawk ? "LHk" : "Ln");
    expect(identity.piece.promoted).toBe(value === fixture.promotedHawk);
    const { props, unmount } = board(source);
    fireEvent.keyDown(square(choice.from), { key: "Enter" }); fireEvent.keyDown(square("9h"), { key: " " });
    expect(props.onPlay).not.toHaveBeenCalled();
    fireEvent.keyDown(square("10h"), { key: "Enter" });
    expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(choice.action);
    unmount();
    const shortcut = board(source);
    fireEvent.click(square("8h")); fireEvent.click(square("10h"));
    expect(shortcut.props.onPlay).toHaveBeenCalledExactlyOnceWith(direct.action);
    shortcut.unmount();
  }
  const source = snapshot(fixture.hawkPlayed);
  const { container } = render(<TenjikuBoard snapshot={source} disabled inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} completedAction={source.moves.last_action} />);
  expect([...container.querySelectorAll('[data-native-effect-phase="capture"]')].map(node => node.getAttribute("data-location"))).toEqual(["9h", "10h"]);
});

it.each(fixtures)("submits the complete native $label Area3 route only after all three landings", fixture => {
  const source = snapshot(fixture.area), completed = snapshot(fixture.areaPlayed).moves.last_action!;
  const choice = source.choices.find(choice => choice.action === completed.action)!;
  expect(choice).toBeTruthy(); expect(choice.steps).toEqual(["7h", "7g", "6g"]);
  const { props, container } = board(source);
  fireEvent.click(square(choice.from));
  fireEvent.click(square(choice.steps[0]!)); expect(props.onPlay).not.toHaveBeenCalled();
  fireEvent.click(square(choice.steps[1]!)); expect(props.onPlay).not.toHaveBeenCalled();
  expect(container.querySelector("[data-selected-route]")?.getAttribute("data-selected-route")).toBe("8h>7h>7g");
  fireEvent.click(square(choice.steps[2]!));
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(choice.action);
});

it.each(fixtures)("keeps a native $label ranked jump as one landing and never exposes the blocked jump as an action", fixture => {
  const source = snapshot(fixture.jump), played = snapshot(fixture.jumpPlayed), choice = source.choices.find(choice => choice.action === played.moves.last_action!.action)!;
  expect(choice).toBeTruthy(); expect(choice.from).toBe("8h"); expect(choice.steps).toEqual(["11e"]);
  const { props, unmount } = board(source);
  fireEvent.click(square(choice.from)); fireEvent.click(square("11e"));
  if (screen.queryByRole("dialog")) {
    expect(props.onPlay).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: choice.completion!.label }));
  }
  expect(props.onPlay).toHaveBeenCalledExactlyOnceWith(choice.action);
  unmount();
  const rendered = render(<TenjikuBoard snapshot={played} disabled inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} completedAction={played.moves.last_action} />);
  expect(rendered.container.querySelector("[data-route]")).toBeNull();
  expect(rendered.container.querySelector('[data-native-capture="11e"]')).toBeTruthy();
  expect(rendered.container.querySelector('[data-last-move="destination"]')?.getAttribute("data-location")).toBe("11e");
  expect(played.pieces.some(piece => piece.square === "9g")).toBe(true);
  rendered.unmount();
  const blocked = snapshot(fixture.blocked);
  expect(blocked.choices.some(choice => choice.from === "8h" && choice.steps.at(-1) === "11e")).toBe(false);
  const blockedBoard = board(blocked);
  fireEvent.click(square("8h")); fireEvent.click(square("11e"));
  expect(blockedBoard.props.onPlay).not.toHaveBeenCalled();
});

it.each(fixtures)("plays the native $label first quiet return and keeps historical traversal read-only", async fixture => {
  const source = snapshot(fixture.selected), response = fixture.inspection as unknown as MovementInspection;
  const quiet = response.choices.find(choice => choice.from === "8h" && JSON.stringify(choice.steps) === JSON.stringify(["7h", "8h"]))!;
  expect(quiet).toBeTruthy();
  // Repetition includes the side to move: this first quiet return is a complete
  // native choice, even though the physical board returns to its prior layout.
  expect(snapshot(fixture.compound).choices.some(choice => choice.action === quiet.action)).toBe(true);
  const live = board(fixture.compound);
  fireEvent.click(square("8h")); fireEvent.click(square("7h"));
  expect(live.props.onPlay).not.toHaveBeenCalled();
  fireEvent.click(square("8h"));
  expect(live.props.onPlay).toHaveBeenCalledExactlyOnceWith(quiet.action);
  live.unmount();
  const onPlay = vi.fn(), onInspect = vi.fn().mockResolvedValue(response);
  render(<TenjikuBoard snapshot={source} disabled inspectionDisabled={false} onPlay={onPlay} onInspect={onInspect} />);
  fireEvent.click(square(response.origin));
  await vi.waitFor(() => expect(screen.getByText(`Inspecting ${response.label} · Esc clears`)).toBeTruthy());
  fireEvent.click(square("7h"));
  expect(screen.queryByRole("button", { name: "Complete action here" })).toBeNull();
  fireEvent.click(square("8h"));
  expect(onPlay).not.toHaveBeenCalled();
  expect(screen.queryByRole("dialog")).toBeNull();
});
