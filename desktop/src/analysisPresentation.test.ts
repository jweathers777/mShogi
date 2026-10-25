import { expect, it } from "vitest";
import {
  readableAction,
  reachabilityDescription,
  scoreDescription,
} from "./analysisPresentation";
it("converts native material scores to signed pawn units with the recorded perspective", () => {
  expect(scoreDescription(137, "white")).toBe(
    "+1.37 pawns from White perspective",
  );
  expect(scoreDescription(-50, "black")).toBe(
    "-0.50 pawns from Black perspective",
  );
  expect(scoreDescription(0, "white")).toBe(
    "0.00 pawns from White perspective",
  );
  expect(scoreDescription(null, "black")).toBe("No completed evaluation");
});
it("reports mate distance and the mating side for positive and negative root scores", () => {
  expect(scoreDescription(999_997, "white")).toBe(
    "Mate for White in 2 moves (3 plies)",
  );
  expect(scoreDescription(-999_998, "black")).toBe(
    "Mate for White in 1 move (2 plies)",
  );
  expect(scoreDescription(-1_000_000, "white")).toBe(
    "Mate for Black · already checkmate",
  );
  expect(readableAction("a7a8n")).toBe("a7 → a8 = N");
});
it("does not turn a resource-limited reachability search into a proof", () => {
  const work = { positions: 1, expanded: 1, transitions: 0, static_leaves: 0 };
  const description = reachabilityDescription(
    { unknown: { reason: "position_limit", work } },
    "either",
  );
  expect(description.title).toBe("Unknown · exploration incomplete");
  expect(description.explanation).toContain(
    "No mating conclusion was established",
  );
  expect(
    reachabilityDescription({ possible: { witness: ["g6g7"], work } }, "white")
      .explanation,
  ).toContain("not a forced win");
});
