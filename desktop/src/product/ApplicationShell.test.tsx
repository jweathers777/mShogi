// @vitest-environment jsdom
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/react";
import { ApplicationShell, LiveGameHeading } from "./ApplicationShell";

afterEach(cleanup);
describe("one application shell for native games", () => {
  for (const game of ["chess", "shogi"] as const) {
    it(`keeps ${game} common controls and sections in the shared locations`, () => {
      const { container } = render(<ApplicationShell game={game} runtime="Local engine"
        document={<><span>Unsaved</span><h2>Untitled game</h2></>}
        actions={<><button>New game</button><button>Save</button><button>Save as…</button><button>Open…</button></>}
        heading={<LiveGameHeading eyebrow="YOUR GAME" status="Your turn" player="You play the first side" flip={<button>Flip board</button>} />}
        board={<div>Native board</div>} analysis={<p>Native observations</p>}
        history={<h2>Game history</h2>} gameActions={<button>Resign</button>}
        technical={<p>Native technical details</p>} />);
      const header = container.querySelector(".shell-header")!;
      for (const label of ["New game", "Save", "Save as…", "Open…"]) expect(header.contains(screen.getByRole("button", { name: label }))).toBe(true);
      expect(header.querySelector(".shell-document")?.textContent).toContain("UnsavedUntitled game");
      expect(Array.from(container.querySelectorAll("[data-shell-section]")).map(e => e.getAttribute("data-shell-section"))).toEqual(["analysis", "history", "actions"]);
      expect(container.querySelector(".shell-board-panel")?.contains(screen.getByRole("button", { name: "Flip board" }))).toBe(true);
      expect(container.querySelector("details")?.open).toBe(false);
      expect(container.querySelector("main")?.classList.contains(`shell-${game}`)).toBe(true);
    });
  }
});
