// @vitest-environment jsdom
// UI requests are mocked; all rendered native records/choices/results are Rust fixtures.
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { ProductApp } from "./ProductApp";
import { TenjikuControls } from "../verification/TenjikuControls";
import { useGameSession, type GameSession } from "./useGameSession";
import { NativeAnalysis, NativeRecovery } from "./NativeSessionPanels";
import type { TenjikuSnapshot } from "./native";
import adamsStandard from "../__fixtures__/shared-tenjiku-adams-standard.json";
import mullerStandard from "../__fixtures__/shared-tenjiku-muller-standard.json";
import adamsUnsupported from "../__fixtures__/shared-tenjiku-adams-unsupported.json";
import mullerUnsupported from "../__fixtures__/shared-tenjiku-muller-unsupported.json";
import adamsEnded from "../__fixtures__/shared-tenjiku-adams-ending-stopped.json";
import mullerEnded from "../__fixtures__/shared-tenjiku-muller-ending-stopped.json";
vi.mock("./useGameSession", () => ({ useGameSession: vi.fn() }));
const adams = adamsStandard as unknown as TenjikuSnapshot, muller = mullerStandard as unknown as TenjikuSnapshot;
let session: GameSession;
beforeEach(() => {
  session = { snapshot: adams, busy: false, error: "", closeRequested: false,
    command: vi.fn().mockResolvedValue(adams), file: vi.fn().mockResolvedValue(adams),
    newGame: vi.fn().mockResolvedValue(adams), cancelClose: vi.fn().mockResolvedValue(true),
    finishClose: vi.fn().mockResolvedValue(true), requestClose: vi.fn().mockResolvedValue(undefined) };
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
  await waitFor(() => expect(session.newGame).toHaveBeenCalledExactlyOnceWith(source.open_token, "tenjiku", { game_kind: "tenjiku", side: source.human.id.side }, source.profile));
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
  await waitFor(() => expect(session.newGame).toHaveBeenCalledExactlyOnceWith(source.open_token, "tenjiku", { game_kind: "tenjiku", side: "gote" }, other));
});

it.each([adams, muller])("sends both explicit QA profile New/reset requests from exact $profile replacement authority", source => {
  session.snapshot = source;
  render(<ProductApp verification verificationControls={TenjikuControls} />);
  expect(useGameSession).toHaveBeenCalledWith();
  const qa = screen.getByRole("complementary", { name: "Tenjiku contract verification" });
  for (const [label, profile] of [["Adams 1999 Modern", "mshogi.tenjiku.adams1999-modern.v1"], ["Muller Completed", "mshogi.tenjiku.muller-completed.v1"]] as const) {
    fireEvent.click(within(qa).getByRole("button", { name: `New ${label} game` }));
    expect(session.newGame).toHaveBeenLastCalledWith(source.open_token, "tenjiku", { game_kind: "tenjiku", side: source.human.id.side }, profile);
  }
  expect(session.command).not.toHaveBeenCalled();
  expect(session.file).not.toHaveBeenCalled();
});

it("retires board selection after an actual native fresh-document profile replacement", async () => {
  expect(muller.token.document).toBeGreaterThan(adams.token.document);
  session.snapshot = adams;
  vi.mocked(session.newGame).mockResolvedValue(muller);
  const { container, rerender } = render(<ProductApp verification verificationControls={TenjikuControls} />);
  const first = adams.choices[0]!;
  expect(first).toBeTruthy();
  const origin = container.querySelector<SVGGElement>(`svg [role="button"][aria-label^="${first.from},"]`)!;
  fireEvent.click(origin);
  expect(origin.getAttribute("aria-label")).toContain(", selected");
  fireEvent.click(screen.getByRole("button", { name: "New Muller Completed game" }));
  await waitFor(() => expect(session.newGame).toHaveBeenCalledExactlyOnceWith(adams.open_token, "tenjiku", { game_kind: "tenjiku", side: adams.human.id.side }, muller.profile));
  session = { ...session, snapshot: muller };
  rerender(<ProductApp verification verificationControls={TenjikuControls} />);
  expect(container.querySelector('svg [aria-label*=", selected"]')).toBeNull();
  expect(container.querySelectorAll("svg [role=button]")).toHaveLength(256);
  expect(screen.getByRole("complementary", { name: "Tenjiku contract verification" }).textContent).toContain("Muller Completed");
});

it.each([adams, muller])("blocks $profile QA replacement while a modal or native operation owns admission", source => {
  session.snapshot = source;
  const { rerender } = render(<ProductApp verification verificationControls={TenjikuControls} />);
  fireEvent.click(screen.getByRole("button", { name: "New game" }));
  const qa = screen.getByRole("complementary", { name: "Tenjiku contract verification" });
  for (const button of within(qa).getAllByRole("button")) expect((button as HTMLButtonElement).disabled).toBe(true);
  fireEvent.click(within(qa).getByRole("button", { name: "New Adams 1999 Modern game" }));
  expect(session.newGame).not.toHaveBeenCalled();
  fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
  session = { ...session, busy: true };
  rerender(<ProductApp verification verificationControls={TenjikuControls} />);
  for (const button of within(screen.getByRole("complementary", { name: "Tenjiku contract verification" })).getAllByRole("button")) expect((button as HTMLButtonElement).disabled).toBe(true);
});

it.each([adamsUnsupported, mullerUnsupported])("shows actual $profile incomplete observations without a fabricated score or PV", value => {
  const source = value as unknown as TenjikuSnapshot, command = vi.fn();
  expect(source.observations.length).toBeGreaterThan(0);
  expect(source.observations.at(-1)!.observation.result).toMatchObject({ kind: "tenjiku", result: { kind: "incomplete", telemetry: { score: null, completed_depth: 0, principal_variation: [] } } });
  expect(source.status.concluded).toBe(false);
  expect(source.status.score_half_points).toBeNull();
  const { container } = render(<><NativeRecovery snapshot={source} error="" disabled={false} command={command} />
    <NativeAnalysis snapshot={source} defaults={source.play_limits} disabled={false} command={command} maxDepth={2} minNodes={1} maxNodes={2000} /></>);
  expect(screen.getByText("Search incomplete")).toBeTruthy();
  expect(screen.queryByRole("button", { name: "Retry engine turn" })).toBeNull();
  fireEvent.click(screen.getByRole("button", { name: "Analyze position" }));
  expect(screen.getAllByText("No completed evaluation").length).toBeGreaterThan(0);
  expect(screen.getByRole("button", { name: "Start move analysis" })).toBeTruthy();
  expect(container.querySelector(".shogi-pv")).toBeNull();
  expect(screen.queryByRole("region", { name: "Game outcome" })).toBeNull();
  expect(command).not.toHaveBeenCalled();
});

it.each([adamsEnded, mullerEnded])("retains the native $profile winner after a concluding Stop without calling the outcome unscored", value => {
  const source = value as unknown as TenjikuSnapshot;
  expect(source.tenjiku.selected.incomplete).not.toBeNull();
  expect(source.status.concluded).toBe(true);
  expect(source.status.score_half_points).not.toBeNull();
  render(<NativeRecovery snapshot={source} error="" disabled={false} command={vi.fn()} />);
  expect(screen.getByRole("region", { name: "Game outcome" }).textContent).toContain(source.status.label);
  expect(screen.queryByText(/No game result or score is assigned/)).toBeNull();
  expect(screen.queryByText("Work stopped")).toBeNull();
});
