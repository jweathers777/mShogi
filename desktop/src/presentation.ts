import { sideLabel } from "./games";
import type { ChessOutcome, Claim, DrawReason, MatchConclusion } from "./sessionTypes";

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
    ? `Checkmate — ${sideLabel("chess", outcome.checkmate.winner)} wins`
    : `Draw — ${drawName(outcome.draw)}`;
}
export function conclusionName(conclusion: MatchConclusion): string {
  if ("chess" in conclusion) return chessOutcome(conclusion.chess);
  if ("unresolved" in conclusion) return "Unresolved — unscored";
  if ("win" in conclusion)
    return `${sideLabel("chess", conclusion.win.winner)} wins — ${conclusion.win.trigger === "flag_fall" ? "time expired" : "resignation"}`;
  return `Draw — ${conclusion.draw.trigger === null ? "agreement" : "opponent cannot mate"}`;
}
export function gameStatus(snapshot: { conclusion: MatchConclusion | null; turn: string; in_check: boolean }): string {
  if (snapshot.conclusion !== null) return conclusionName(snapshot.conclusion);
  return `${snapshot.turn} to move${snapshot.in_check ? " · check" : ""}`;
}
