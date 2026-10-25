import { describe, expect, it } from "vitest";
import type { Snapshot } from "./native";
import { newestSnapshot } from "./native";
import { claimName, conclusionName, gameStatus } from "./presentation";
import standardJson from "./__fixtures__/standard.json";
import claimJson from "./__fixtures__/claim.json";
import checkmateJson from "./__fixtures__/checkmate.json";
const standard = standardJson as Snapshot;
const claim = claimJson as Snapshot;
const checkmate = checkmateJson as Snapshot;
describe("real native snapshot semantics", () => {
  it("recognizes actual native external enum shapes and keeps claims separate from conclusions", () => {
    expect(gameStatus(standard)).toBe("White to move");
    expect(gameStatus(checkmate)).toBe("Checkmate — Black wins");
    expect(claim.conclusion).toBeNull();
    expect(claim.claims.length).toBeGreaterThan(0);
    expect(gameStatus(claim)).toBe("White to move");
    expect(claimName(claim.claims[0]!)).toContain("(move not played)");
    expect(conclusionName({ chess: { draw: { claimed: "fifty_move" } } })).toBe(
      "Draw — claimed fifty-move rule",
    );
    expect(
      conclusionName({ unresolved: { actor: "black", trigger: "flag_fall" } }),
    ).toContain("Unresolved — unscored");
  });
  it("keeps the newer snapshot when an older async response arrives", () => {
    const newer = { ...standard, revision: 5, version: 4, history: ["e2e4"] };
    expect(newestSnapshot(newer, standard)).toBe(newer);
    expect(newestSnapshot(standard, newer)).toBe(newer);
  });
});
