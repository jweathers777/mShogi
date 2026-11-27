import type { Color, EngineChoice, ChessResult, Telemetry } from "./sessionTypes";

import { sideLabel } from "./games";
// Chess score contract: material is below MATE_SCORE-MAX_DEPTH.
const MATE_SCORE = 1_000_000;
const MAX_SEARCH_DEPTH = 128;
export function scoreDescription(
  score: number | null,
  turn: Color | null,
): string {
  if (score === null) return "No completed evaluation";
  if (Math.abs(score) >= MATE_SCORE - MAX_SEARCH_DEPTH) {
    const plies = MATE_SCORE - Math.abs(score);
    if (turn === null)
      return `Mate distance: ${plies} plies (perspective unavailable)`;
    const winner = score >= 0 ? turn : turn === "white" ? "black" : "white";
    if (plies === 0) return `Mate for ${sideLabel("chess", winner)} · already checkmate`;
    const moves = Math.ceil(plies / 2);
    return `Mate for ${sideLabel("chess", winner)} in ${moves} ${moves === 1 ? "move" : "moves"} (${plies} ${plies === 1 ? "ply" : "plies"})`;
  }
  return `${score > 0 ? "+" : ""}${(score / 100).toFixed(2)} pawns from ${turn === null ? "the recorded side-to-move" : sideLabel("chess", turn)} perspective`;
}
export function readableAction(action: string): string {
  const match = /^([a-h][1-8])([a-h][1-8])([qrbn])?$/.exec(action);
  return match
    ? `${match[1]} → ${match[2]}${match[3] ? ` = ${match[3].toUpperCase()}` : ""}`
    : action;
}
export function choiceDescription(choice: EngineChoice): string {
  return choice.kind === "move"
    ? readableAction(choice.action)
    : `Claim draw${choice.claim.intended ? `, intending ${readableAction(choice.claim.intended)} (unplayed)` : " in the current position"}`;
}
export function resultTelemetry(result: ChessResult | null): Telemetry | null {
  return result === null || result.kind === "failure" ? null : result.telemetry;
}
