import type { GameSnapshot, Side } from "./native";

export function nativeSnapshotIsNewer(current: GameSnapshot | null, incoming: GameSnapshot): boolean {
  if (current === null) return true;
  return incoming.token.document > current.token.document || (incoming.token.document === current.token.document && incoming.game === current.game && incoming.profile === current.profile && incoming.token.revision > current.token.revision);
}

export function sessionStatus(snapshot: GameSnapshot): "thinking" | "analyzing" | "idle" {
  if (snapshot.phase !== "thinking") return "idle";
  if (snapshot.execution_purpose === "move_analysis") return "analyzing";
  if (snapshot.execution_purpose === "engine_turn" && snapshot.capabilities.engine_turn) return "thinking";
  return "idle";
}

/** The side that the document settings place at the bottom of the board. */
export function bottomSide(snapshot: GameSnapshot): Side {
  return snapshot.sides.find(side => side.id.side === snapshot.document.settings.orientation.side)!;
}
