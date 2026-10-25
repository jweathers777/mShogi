import type {
  ChessOutcome,
  Claim,
  DrawReason,
  MatchConclusion,
  Snapshot,
} from "./native";
export const colorName = (color: string) =>
  color === "white" ? "White" : "Black";
export function claimName(claim: Claim): string {
  const rule =
    claim.reason === "threefold" ? "threefold repetition" : "fifty-move rule";
  return claim.intended === null
    ? `Claim draw: ${rule}, current position`
    : `Claim draw: ${rule}, intending ${claim.intended} (move not played)`;
}
function drawName(reason: DrawReason): string {
  if (typeof reason !== "string")
    return `claimed ${reason.claimed === "threefold" ? "threefold repetition" : "fifty-move rule"}`;
  return {
    stalemate: "stalemate",
    dead_position: "proven dead position",
    fivefold: "fivefold repetition",
    seventy_five_move: "seventy-five-move rule",
  }[reason];
}
function chessOutcome(outcome: ChessOutcome): string {
  return "checkmate" in outcome
    ? `Checkmate — ${colorName(outcome.checkmate.winner)} wins`
    : `Draw — ${drawName(outcome.draw)}`;
}
export function conclusionName(conclusion: MatchConclusion): string {
  if ("chess" in conclusion) return chessOutcome(conclusion.chess);
  if ("unresolved" in conclusion)
    return `Unresolved — unscored (${colorName(conclusion.unresolved.actor)} ${conclusion.unresolved.trigger === "flag_fall" ? "ran out of time" : "resigned"}; mating possibility not established)`;
  if ("win" in conclusion)
    return `${colorName(conclusion.win.winner)} wins — ${conclusion.win.trigger === "flag_fall" ? "time expired" : "resignation"}`;
  return `Draw — ${conclusion.draw.trigger === null ? "agreement" : "opponent cannot mate"}`;
}
export function gameStatus(snapshot: Snapshot): string {
  if (snapshot.conclusion !== null) return conclusionName(snapshot.conclusion);
  return `${colorName(snapshot.turn)} to move${snapshot.in_check ? " · check" : ""}`;
}
