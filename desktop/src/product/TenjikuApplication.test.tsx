// @vitest-environment jsdom
// UI requests are mocked; all rendered game records/choices/results are Rust fixtures.
import { fakeSession } from "../testing/helpers";
import type { TenjikuSnapshot } from "../sessionTypes";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { ProductApp } from "./ProductApp";
import { TenjikuControls } from "../verification/TenjikuControls";
import { useGameSession, type GameSession } from "./useGameSession";
import { GameAnalysis, GameRecovery } from "./GameSessionPanels";

import adamsStandard from "../__fixtures__/shared-tenjiku-adams-standard.json";
import mullerStandard from "../__fixtures__/shared-tenjiku-muller-standard.json";
import adamsIncomplete from "../__fixtures__/shared-tenjiku-adams-incomplete.json";
import mullerIncomplete from "../__fixtures__/shared-tenjiku-muller-incomplete.json";
import adamsEnded from "../__fixtures__/shared-tenjiku-adams-ending-stopped.json";
import mullerEnded from "../__fixtures__/shared-tenjiku-muller-ending-stopped.json";
vi.mock("./useGameSession", () => ({ useGameSession: vi.fn() }));
const adams = adamsStandard as unknown as TenjikuSnapshot, muller = mullerStandard as unknown as TenjikuSnapshot;
let session: GameSession;
beforeEach(() => {
  session = fakeSession(adams);
  vi.mocked(useGameSession).mockImplementation(() => session);
});
afterEach(cleanup);

it.each([adams, muller])("selects normal sixth-game New with the saved $profile as its explicit default", async source => {
  session.snapshot = source;
  render(<ProductApp />);
  expect(screen.queryByRole("complementary", { name: "Tenjiku contract verification" })).toBeNull();
  expect(document.querySelector(`[data-rules-profile="${source.profile}"]`)).toBeTruthy();
  fireEvent.click(screen.getByRole("button", { name: "New game" }));
  const game = screen.getByRole("combobox", { name: "Game" }) as HTMLSelectElement;
  expect(within(game).getAllByRole("option").map(option => (option as HTMLOptionElement).value)).toEqual(["chess", "shogi", "xiangqi", "tori", "chu", "tenjiku"]);
  expect(game.value).toBe("tenjiku");
  const profile = screen.getByRole("combobox", { name: "Rules profile" }) as HTMLSelectElement;
  expect(profile.value).toBe(source.profile);
  expect(within(profile).getAllByRole("option").map(option => option.textContent)).toEqual(["Adams 1999 Modern", "Muller Completed"]);
  fireEvent.click(screen.getByRole("button", { name: "Start game" }));
  await waitFor(() => expect(session.newGame).toHaveBeenCalledExactlyOnceWith(source.open_token, "tenjiku", { game_kind: "tenjiku", side: source.human.id.side }, source.profile, "human_engine"));
});

it.each([adams, muller])("allows a deliberate other-profile fresh game from $profile without live profile mutation", async source => {
  session.snapshot = source;
  const other = source.profile === adams.profile ? muller.profile : adams.profile;
  render(<ProductApp />);
  fireEvent.click(screen.getByRole("button", { name: "New game" }));
  fireEvent.change(screen.getByRole("combobox", { name: "Rules profile" }), { target: { value: other } });
  fireEvent.click(screen.getByRole("radio", { name: "Gote" }));
  expect(session.snapshot?.profile).toBe(source.profile);
  fireEvent.click(screen.getByRole("button", { name: "Start game" }));
  await waitFor(() => expect(session.newGame).toHaveBeenCalledExactlyOnceWith(source.open_token, "tenjiku", { game_kind: "tenjiku", side: "gote" }, other, "human_engine"));
});

it.each([adams, muller])("sends both explicit QA profile New/reset requests from exact $profile replacement authority", source => {
  session.snapshot = source;
  render(<ProductApp verificationControls={TenjikuControls} />);
  expect(useGameSession).toHaveBeenCalledWith();
  const qa = screen.getByRole("complementary", { name: "Tenjiku contract verification" });
  for (const [label, profile] of [["Adams 1999 Modern", "mshogi.tenjiku.adams1999-modern.v1"], ["Muller Completed", "mshogi.tenjiku.muller-completed.v1"]] as const) {
    fireEvent.click(within(qa).getByRole("button", { name: `New ${label} game` }));
    expect(session.newGame).toHaveBeenLastCalledWith(source.open_token, "tenjiku", { game_kind: "tenjiku", side: source.human.id.side }, profile);
  }
  expect(session.command).not.toHaveBeenCalled();
  expect(session.file).not.toHaveBeenCalled();
});

it("retires board selection after an actual fresh-document profile replacement", async () => {
  expect(muller.token.document).toBeGreaterThan(adams.token.document);
  session.snapshot = adams;
  vi.mocked(session.newGame).mockResolvedValue(muller);
  const { container, rerender } = render(<ProductApp verificationControls={TenjikuControls} />);
  const first = adams.choices[0]!;
  expect(first).toBeTruthy();
  const origin = container.querySelector<SVGGElement>(`svg [role="button"][aria-label^="${first.from},"]`)!;
  fireEvent.click(origin);
  expect(origin.getAttribute("aria-label")).toContain(", selected");
  fireEvent.click(screen.getByRole("button", { name: "New Muller Completed game" }));
  await waitFor(() => expect(session.newGame).toHaveBeenCalledExactlyOnceWith(adams.open_token, "tenjiku", { game_kind: "tenjiku", side: adams.human.id.side }, muller.profile));
  session = { ...session, snapshot: muller };
  rerender(<ProductApp verificationControls={TenjikuControls} />);
  expect(container.querySelector('svg [aria-label*=", selected"]')).toBeNull();
  expect(container.querySelectorAll("svg [role=button]")).toHaveLength(256);
  expect(screen.getByRole("complementary", { name: "Tenjiku contract verification" }).textContent).toContain("Muller Completed");
});

it.each([adams, muller])("blocks $profile QA replacement while a modal or game operation owns admission", source => {
  session.snapshot = source;
  const { rerender } = render(<ProductApp verificationControls={TenjikuControls} />);
  fireEvent.click(screen.getByRole("button", { name: "New game" }));
  const qa = screen.getByRole("complementary", { name: "Tenjiku contract verification" });
  for (const button of within(qa).getAllByRole("button")) expect((button as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(within(qa).getByRole("button", { name: "New Adams 1999 Modern game" }));
  expect(session.newGame).not.toHaveBeenCalled();
  fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
  session = { ...session, busy: true };
  rerender(<ProductApp verificationControls={TenjikuControls} />);
  for (const button of within(screen.getByRole("complementary", { name: "Tenjiku contract verification" })).getAllByRole("button")) expect((button as HTMLButtonElement).disabled).toBe(true);
});

it.each([adamsIncomplete, mullerIncomplete])("shows an unscored $profile node-limited suggestion without changing the game", value => {
  const source = value as unknown as TenjikuSnapshot, command = vi.fn();
  expect(source.observations.length).toBeGreaterThan(0);
  const observation = source.observations.at(-1)!.observation.result;
  expect(observation).toMatchObject({ kind: "tenjiku", result: { kind: "choice", telemetry: { score: null, completed_depth: 0 } } });
  const selected = source.search_observations.find(view => view.selected)!;
  expect(selected.variation).toHaveLength(1);
  expect(source.status.concluded).toBe(false);
  expect(source.status.score_half_points).toBeNull();
  const { container } = render(<><GameRecovery snapshot={source} error="" disabled={false} command={command} />
    <GameAnalysis snapshot={source} defaults={source.play_limits} disabled={false} command={command} maxDepth={64} minNodes={1} maxNodes={5000000} /></>);
  expect(screen.queryByText("Search incomplete")).toBeNull();
  expect(screen.queryByRole("button", { name: "Retry engine turn" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect(screen.getAllByText("No completed evaluation").length).toBeGreaterThan(0);
  expect(screen.getByRole("button", { name: "Start move analysis" })).toBeTruthy();
  expect(container.querySelector(".shogi-pv")?.textContent).toContain(selected.variation[0]);
  expect(screen.getByText("No move was played.")).toBeTruthy();
  expect(screen.queryByRole("region", { name: "Game outcome" })).toBeNull();
  expect(command).not.toHaveBeenCalled();
});

it.each([adamsEnded, mullerEnded])("retains the $profile winner after session cancellation without calling the outcome unscored", value => {
  const source = value as unknown as TenjikuSnapshot;
  expect(source.phase).toBe("cancelled");
  expect(source.status.concluded).toBe(true);
  expect(source.status.score_half_points).not.toBeNull();
  render(<GameRecovery snapshot={source} error="" disabled={false} command={vi.fn()} />);
  expect(screen.getByRole("region", { name: "Game outcome" }).textContent).toContain(source.status.label);
  expect(screen.queryByText(/No game result or score is assigned/)).toBeNull();
  expect(screen.queryByText("Work stopped")).toBeNull();
});

it.each([adams, muller])("admits shared-core analysis bounds through the normal $profile application", source => {
  session.snapshot = source;
  render(<ProductApp />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  const depth = screen.getByRole("spinbutton", { name: "Depth" }) as HTMLInputElement;
  const nodes = screen.getByRole("spinbutton", { name: "Node limit" }) as HTMLInputElement;
  expect([depth.min, depth.max]).toEqual(["1", "64"]);
  expect([nodes.min, nodes.max]).toEqual(["1", "5000000"]);
  fireEvent.change(depth, { target: { value: "64" } });
  fireEvent.change(nodes, { target: { value: "5000000" } });
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(session.command).toHaveBeenCalledExactlyOnceWith(source, { kind: "analyze", operation: { depth: 64, nodes: 5000000 } });
});

it.each([adams, muller])("refuses work above the shared-core bounds without changing $profile", source => {
  session.snapshot = source;
  render(<ProductApp />);
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  fireEvent.change(screen.getByRole("spinbutton", { name: "Depth" }), { target: { value: "65" } });
  fireEvent.change(screen.getByRole("spinbutton", { name: "Node limit" }), { target: { value: "5000001" } });
  fireEvent.click(screen.getByRole("button", { name: "Start move analysis" }));
  expect(screen.getByRole("alert").textContent).toContain("depth from 1 to 64");
  expect(session.command).not.toHaveBeenCalled();
  expect(session.snapshot?.profile).toBe(source.profile);
});
