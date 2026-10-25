import type {
  Color,
  EngineChoice,
  EngineResult,
  MateReachability,
  ReachabilityTarget,
  Telemetry,
} from "./native";
import { colorName } from "./presentation";
// Native full-width-v1 score contract: material is below MATE_SCORE-MAX_DEPTH.
const MATE_SCORE = 1_000_000;
const MAX_NATIVE_DEPTH = 128;
export function scoreDescription(
  score: number | null,
  turn: Color | null,
): string {
  if (score === null) return "No completed evaluation";
  if (Math.abs(score) >= MATE_SCORE - MAX_NATIVE_DEPTH) {
    const plies = MATE_SCORE - Math.abs(score);
    if (turn === null)
      return `Mate distance: ${plies} plies (perspective unavailable)`;
    const winner = score >= 0 ? turn : turn === "white" ? "black" : "white";
    if (plies === 0) return `Mate for ${colorName(winner)} · already checkmate`;
    const moves = Math.ceil(plies / 2);
    return `Mate for ${colorName(winner)} in ${moves} ${moves === 1 ? "move" : "moves"} (${plies} ${plies === 1 ? "ply" : "plies"})`;
  }
  return `${score > 0 ? "+" : ""}${(score / 100).toFixed(2)} pawns from ${turn === null ? "the recorded side-to-move" : colorName(turn)} perspective`;
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
export function resultTelemetry(result: EngineResult | null): Telemetry | null {
  return result === null ||
    result.kind === "failure" ||
    result.kind === "reachability"
    ? null
    : result.telemetry;
}
export function reachabilityDescription(
  result: MateReachability,
  target: ReachabilityTarget,
): { title: string; explanation: string } {
  const subject = target === "either" ? "At least one side" : colorName(target);
  if ("possible" in result)
    return {
      title: "A cooperative mating route exists",
      explanation: `${subject} can deliver mate along the returned cooperative route. This is not a forced win against best defense and does not change the live game's outcome.`,
    };
  if ("impossible" in result)
    return {
      title:
        target === "either"
          ? "Neither side can reach mate"
          : `${subject} cannot reach mate`,
      explanation:
        "The native search exhausted the reachable graph using sound proofs. This read-only result does not adjudicate the live M2 game.",
    };
  const why = {
    cancelled: "cancelled",
    position_limit: "position budget exhausted",
    transition_limit: "transition budget exhausted",
    resource_limit: "resource limit reached",
  }[result.unknown.reason];
  return {
    title: "Unknown · exploration incomplete",
    explanation: `${why[0]!.toUpperCase()}${why.slice(1)}. No mating conclusion was established. Failure to find a route is not proof that mating is impossible or possible.`,
  };
}
