// @vitest-environment jsdom
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { ApplicationShell, DocumentState, GamePositionStatus, LiveGameHeading, NativeEnginePanel } from "./ApplicationShell";

afterEach(cleanup);
describe("one application shell for native games", () => {
  for (const game of ["chess", "shogi", "xiangqi"] as const) {
    it(`keeps ${game} common controls and sections in the shared locations`, () => {
      const { container } = render(<ApplicationShell game={game} runtime="Local engine"
        document={<><span>Unsaved</span><h2>Untitled game</h2></>}
        actions={<><button>New game</button><button>Save</button><button>Load</button></>}
        heading={<LiveGameHeading eyebrow="YOUR GAME" status="Your turn" player="You play the first side" flip={<button>Flip board</button>} />}
        board={<div>Native board</div>} analysis={<p>Native observations</p>}
        history={<h2>Game history</h2>} gameActions={<button>Resign</button>}
        support={<button>About mShogi</button>} />);
      const header = container.querySelector(".shell-header")!;
      for (const label of ["New game", "Save", "Load"]) expect(header.contains(screen.getByRole("button", { name: label }))).toBe(true);
      expect(header.querySelector(".shell-document")?.textContent).toContain("UnsavedUntitled game");
      expect(Array.from(container.querySelectorAll("[data-shell-section]")).map(e => e.getAttribute("data-shell-section"))).toEqual(["analysis", "actions", "history"]);
      expect(container.querySelector(".shell-board-panel")?.contains(screen.getByRole("button", { name: "Flip board" }))).toBe(true);
      expect(header.contains(screen.getByRole("button", { name: "About mShogi" }))).toBe(true);
      expect(container.querySelector(".shell-sidebar details")).toBeNull();
      expect(header.textContent).toContain("歩");
      expect(container.querySelector("main")?.classList.contains(`shell-${game}`)).toBe(true);
    });
  }
});

it.each([
  [null, false, "Unsaved", "unsaved"],
  [null, true, "Unsaved", "unsaved"],
  ["/games/game.mshogi", false, "Unmodified", "unmodified"],
  ["/games/game.mshogi", true, "Unsaved changes", "modified"],
] as const)("labels disk identity %s and native dirty %s consistently", (path, dirty, label, kind) => {
  const { container } = render(<DocumentState document={{ path, dirty }} />);
  expect(container.querySelector("[data-document-state]")?.getAttribute("data-document-state")).toBe(kind);
  expect(screen.getByText(label)).toBeTruthy();
});

it("keeps actual play limits between native engine status and observations", () => {
  const { container, rerender } = render(<NativeEnginePanel status="Searching" limits={{ depth: 2, nodes: 5000 }}><p>No completed observation</p></NativeEnginePanel>);
  expect([...container.querySelector(".engine-card")!.children].map(e => e.textContent)).toEqual(["EngineSearching", "Play limit: depth 2 · 5,000 nodes", "No completed observation"]);
  rerender(<NativeEnginePanel status="Play paused" limits={{ depth: 3, nodes: 12345 }}><p>Real observation</p></NativeEnginePanel>);
  expect(screen.getByText("Play limit: depth 3 · 12,345 nodes")).toBeTruthy();
});

it.each([
  [{ historical: false, concluded: false, paused: false, humanTurn: true, thinking: false }, "Your turn"],
  [{ historical: false, concluded: false, paused: false, humanTurn: false, thinking: true }, "mShogi is searching within fixed work limits"],
  [{ historical: false, concluded: false, paused: true, humanTurn: true, thinking: false }, "Play paused · resume explicitly to continue"],
  [{ historical: false, concluded: true, paused: true, humanTurn: true, thinking: false }, "Game concluded"],
  [{ historical: true, concluded: true, paused: true, humanTurn: true, thinking: false }, "History · return to live to play"],
])("preserves footer status precedence for %j", (state, label) => {
  render(<GamePositionStatus {...state} ply={1} />);
  expect(screen.getByText(label)).toBeTruthy();
  expect(screen.getByText(`${state.historical ? "Selected" : "Live"} position · 1 ply`)).toBeTruthy();
});
