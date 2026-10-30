import { describe, expect, it } from "vitest";
import type { ChessSnapshot } from "./product/native";
import { nativeSnapshotIsNewer } from "./NativeBoard";
import { claimName, conclusionName } from "./presentation";
import standardJson from "./__fixtures__/shared-chess-standard.json";
import claimJson from "./__fixtures__/shared-chess-claim.json";
import checkmateJson from "./__fixtures__/shared-chess-checkmate.json";
const standard = standardJson as ChessSnapshot;
const claim = claimJson as ChessSnapshot;
const checkmate = checkmateJson as ChessSnapshot;
describe("real native snapshot semantics", () => {
  it("recognizes actual native external enum shapes and keeps claims separate from conclusions", () => {
    expect(standard.status.label).toBe("White to move");
    expect(conclusionName(checkmate.chess.live_conclusion!)).toBe("Checkmate — Black wins");
    expect(claim.chess.live_conclusion).toBeNull();
    expect(claim.chess.claims.length).toBeGreaterThan(0);
    expect(claim.status.label).toBe("White to move");
    expect(claimName(claim.chess.claims[0]!)).toContain("(move not played)");
    expect(conclusionName({ chess: { draw: { claimed: "fifty_move" } } })).toBe(
      "Draw — claimed fifty-move rule",
    );
    expect(
      conclusionName({ unresolved: { actor: "black", trigger: "flag_fall" } }),
    ).toContain("Unresolved — unscored");
  });
  it("keeps the newer snapshot when an older async response arrives", () => {
    const newer = { ...standard, token: { ...standard.token, revision: standard.token.revision + 2 } };
    expect(nativeSnapshotIsNewer(newer, standard)).toBe(false);
    expect(nativeSnapshotIsNewer(standard, newer)).toBe(true);
  });
});
