import type { GameSnapshot, Side } from "./sessionTypes";

export function gameSnapshotIsNewer(current: GameSnapshot | null, incoming: GameSnapshot): boolean {
  if (current === null) return true;
  return incoming.token.document > current.token.document || (incoming.token.document === current.token.document && incoming.game === current.game && incoming.profile === current.profile && incoming.token.revision > current.token.revision);
}

export function sessionStatus(snapshot: GameSnapshot): "thinking" | "analyzing" | "idle" {
  const working = snapshot.phase === "thinking";
  if (working && snapshot.execution_purpose === "engine_turn" && snapshot.capabilities.engine_turn) return "thinking";
  if ((working && snapshot.execution_purpose === "move_analysis")
    || (snapshot.game === "chess" && snapshot.chess.analysis.status === "thinking")) return "analyzing";
  return "idle";
}

/** The side that the document settings place at the bottom of the board. */
export function bottomSide(snapshot: GameSnapshot): Side {
  return snapshot.sides.find(side => side.id.side === snapshot.document.settings.orientation.side)!;
}
