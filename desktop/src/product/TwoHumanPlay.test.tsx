// @vitest-environment jsdom
import { renderApplication, square } from "../testing/helpers";
import type { GameSnapshot } from "../sessionTypes";
import { afterEach, expect, it } from "vitest";
import { cleanup, fireEvent, screen } from "@testing-library/react";

import chess from "../__fixtures__/shared-chess-standard.json";
import shogi from "../__fixtures__/shared-shogi-standard.json";
import xiangqi from "../__fixtures__/shared-xiangqi-standard.json";
import tori from "../__fixtures__/shared-tori-standard.json";
import chu from "../__fixtures__/shared-chu-routes.json";
import adams from "../__fixtures__/shared-tenjiku-adams-standard.json";
import shogiDrops from "../__fixtures__/shared-shogi-drops.json";
import toriGote from "../__fixtures__/shared-tori-gote.json";
import muller from "../__fixtures__/shared-tenjiku-muller-standard.json";
afterEach(cleanup);
const fixtures = [chess, shogi, xiangqi, tori, chu, adams, muller] as GameSnapshot[];
const app = renderApplication;
it.each(fixtures)("shows two human participants and an explicit side to resign for $profile", fixture => {
  const local = fixture.sides[1]!;
  const snapshot = { ...fixture, play_mode: "two_humans" as const, local_actor: local, turn: local, paused: false,
    capabilities: { ...fixture.capabilities, engine_turn: false, board_action: true, resign: true } } as GameSnapshot;
  const { container, session } = app(snapshot);
  expect(screen.getByText(/Two humans · local play/)).toBeTruthy();
  expect([...container.querySelectorAll(".shell-player-identity strong")].every(node => node.textContent?.startsWith("Human"))).toBe(true);
  expect(container.querySelector("[data-position-status]")?.textContent).toContain(`${local.label} to move`);
  expect(screen.queryByRole("button", { name: "Retry engine turn" })).toBeNull();
  expect(screen.queryByRole("button", { name: "Cancel search" })).toBeNull();
  expect(session.command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: `Resign ${local.label}` }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "resign" });
});
it.each([chu, adams, muller] as GameSnapshot[])("lets the other local human respond to a $profile draw offer before the turn changes", fixture => {
  if (fixture.game !== "chu" && fixture.game !== "tenjiku") throw new Error("draw fixture");
  const offerer = fixture.turn;
  const history = fixture.game === "chu" ? fixture.chu : fixture.tenjiku;
  const snapshot = { ...fixture, play_mode: "two_humans" as const, local_actor: offerer,
    [fixture.game]: { ...history, live: { ...history.live, draw_offer: offerer.id.side } } } as GameSnapshot;
  const { session } = app(snapshot);
  fireEvent.click(screen.getByRole("button", { name: "Decline draw" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "command", command: { kind: "decline_draw" } });
});
it.each([...fixtures, toriGote as GameSnapshot])("plays a complete action for the local actor rather than the fixed human anchor in $profile", fixture => {
  const snapshot = { ...fixture, play_mode: "two_humans" as const, local_actor: fixture.turn,
    human: fixture.sides.find(side => side.key !== fixture.turn.key)!, paused: false,
    capabilities: { ...fixture.capabilities, engine_turn: false, board_action: true } } as GameSnapshot;
  const { session } = app(snapshot);
  const choice = snapshot.choices.find(choice => choice.from !== null && choice.steps.length === 1 && choice.completion === null)!;
  fireEvent.click(square(choice.from!)); fireEvent.click(square(choice.steps[0]!));
  // The session derives the actor; the operation carries only the opaque action.
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "play", action: choice.action });
});

it("lets the local Shogi actor drop from hand independently of the fixed human anchor", () => {
  const fixture = shogiDrops as GameSnapshot;
  const snapshot = { ...fixture, play_mode: "two_humans" as const, local_actor: fixture.turn,
    human: fixture.sides.find(side => side.key !== fixture.turn.key)!, paused: false,
    capabilities: { ...fixture.capabilities, engine_turn: false, board_action: true } } as GameSnapshot;
  const { container, session } = app(snapshot);
  const choice = snapshot.choices.find(choice => choice.origin?.kind === "hand")!;
  const hand = snapshot.hands.find(hand => hand.origin === choice.from)!;
  fireEvent.click(container.querySelector(`[data-hand="${hand.kind}"][data-owner="${snapshot.turn.id.side}"]`)!);
  fireEvent.click(screen.getByRole("button", { name: new RegExp(`^${choice.steps[0]},`) }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "play", action: choice.action });
});
it("uses Xiangqi actor advice to label off-turn draw controls", () => {
  const fixture = xiangqi as unknown as GameSnapshot;
  if (fixture.game !== "xiangqi") throw new Error("Xiangqi fixture");
  const first = fixture.sides[0]!, second = fixture.sides[1]!;
  const snapshot = { ...fixture, play_mode: "two_humans" as const, local_actor: first,
    xiangqi: { ...fixture.xiangqi, terminal_actions: { offer_actor: second, response_actor: second, offer_draw: true, accept_draw: true, decline_draw: true } } };
  const { session } = app(snapshot);
  expect(screen.getByRole("button", { name: `Offer draw for ${second.label}…` })).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: `Accept draw for ${second.label}…` }));
  fireEvent.click(screen.getByRole("button", { name: `Accept draw for ${second.label}` }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(snapshot, { kind: "command", command: { kind: "accept_draw" } });
});

it.each(fixtures)("shows local play details without an automatic engine budget for $profile", fixture => {
  const snapshot = { ...fixture, play_mode: "two_humans" as const, local_actor: fixture.turn } as GameSnapshot;
  const { container, session } = app(snapshot);
  fireEvent.click(screen.getByRole("button", { name: "Play details" }));
  expect(screen.getByText("Both players are local. Engine analysis starts only on request.")).toBeTruthy();
  expect(screen.queryByRole("status", { name: "Engine turn status" })).toBeNull();
  expect(container.querySelector("[data-play-limits]")).toBeNull();
  expect(screen.queryByText(/Thinking time:/)).toBeNull();
  expect(screen.queryByText("No engine search has completed in this game.")).toBeNull();
  expect(session.command).not.toHaveBeenCalled();
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect(screen.getByRole("button", { name: "Start move analysis" })).toBeTruthy();
  expect(session.command).not.toHaveBeenCalled();
});
