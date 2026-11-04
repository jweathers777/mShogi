// @vitest-environment jsdom
import { cleanup, render } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
import { TenjikuBoard } from "./TenjikuBoard";
import type { TenjikuSnapshot } from "./native";
import adams from "../__fixtures__/shared-tenjiku-adams-promoted-hawk.json";
import muller from "../__fixtures__/shared-tenjiku-muller-promoted-hawk.json";
import adamsFaces from "../__fixtures__/shared-tenjiku-adams-all-faces.json";
import mullerFaces from "../__fixtures__/shared-tenjiku-muller-all-faces.json";
import profile from "../../../docs/m4/tenjiku/profile.json";

afterEach(cleanup);
it.each([adams, muller])("uses selected native $profile identities and leaves accessible labels on the shared squares", value => {
  const snapshot = structuredClone(value) as unknown as TenjikuSnapshot;
  const hawk = snapshot.pieces.find(piece => piece.square === "8h")!;
  // Technical diagnostic lettering is deliberately not the artwork's source.
  hawk.text = "diagnostic"; hawk.kind = "wrong-face"; hawk.promoted = false;
  const { container, rerender } = render(<TenjikuBoard snapshot={snapshot} disabled inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} />);
  const square = container.querySelector('[role="button"][aria-label^="8h,"]')!;
  const piece = square.querySelector('[data-tenjiku-face="LHk"]')!;
  expect(piece.textContent).toBe("獅鷹");
  expect(piece.getAttribute("data-tenjiku-piece")).toBe("Ln");
  expect(piece.getAttribute("data-promoted")).toBe("true");
  expect(square.getAttribute("aria-label")).toBe(hawk.label);
  expect(piece.getAttribute("aria-hidden")).toBe("true");
  expect(container.querySelectorAll('[data-royal="true"]')).toHaveLength(snapshot.tenjiku.selected.royals.length);
  expect(container.textContent).not.toContain("diagnostic");
  expect(container.querySelectorAll('svg [role="button"]')).toHaveLength(256);
  expect(container.querySelector('[data-game-slot="footer"] [data-position-status]')).toBeTruthy();
  expect(piece.getAttribute("transform")).toBeNull();
  snapshot.document.settings.settings.orientation = "gote";
  rerender(<TenjikuBoard snapshot={snapshot} disabled inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} />);
  expect(square.querySelector('[data-tenjiku-face="LHk"]')?.getAttribute("transform")).toBe("rotate(180 50 50)");
});

it.each([adamsFaces, mullerFaces])("renders all sourced $profile identities from the explicitly imported native artwork diagram", value => {
  const snapshot = value as unknown as TenjikuSnapshot;
  const identities = snapshot.tenjiku.selected.identities;
  expect(snapshot.tenjiku.root).toMatchObject({ kind: "imported", source: "Artwork inspection diagram: all44faces and30once-promoted identities, not reached play" });
  expect(identities).toHaveLength(67);
  expect(new Set(identities.map(identity => identity.piece.id)).size).toBe(67);
  expect([...new Set(identities.map(identity => identity.face))].sort()).toEqual(profile.pieces.map(piece => piece.face).sort());
  expect(identities.filter(identity => identity.piece.promoted)).toHaveLength(30);
  const { container } = render(<TenjikuBoard snapshot={snapshot} disabled inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} />);
  expect(container.querySelectorAll("[data-tenjiku-piece]")).toHaveLength(67);
  expect(container.querySelectorAll(".tenjiku-promotion-mark")).toHaveLength(30);
  expect(container.querySelectorAll('[data-royal="true"]')).toHaveLength(3);
  expect(container.querySelectorAll('svg [role="button"]')).toHaveLength(256);
  for (const identity of identities) {
    const square = container.querySelector(`[role="button"][aria-label^="${identity.square},"]`)!;
    const piece = square.querySelector("[data-tenjiku-face]")!;
    expect(piece.getAttribute("data-tenjiku-face")).toBe(identity.face);
    expect(piece.getAttribute("data-tenjiku-piece")).toBe(identity.piece.base);
    expect(piece.getAttribute("data-promoted")).toBe(String(identity.piece.promoted));
    expect(piece.getAttribute("transform")).toBe(identity.piece.side === "gote" ? "rotate(180 50 50)" : null);
    expect(square.getAttribute("aria-label")).toBe(snapshot.pieces.find(piece => piece.square === identity.square)!.label);
  }
  expect(container.querySelector('[data-tenjiku-face="HT"]')?.textContent).toBe("四天王");
});
