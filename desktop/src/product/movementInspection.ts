import type { BoardChoice } from "../Board";
import type { NativeBoardSnapshot } from "../NativeBoard";
import type { Side } from "./native";

export interface InspectionContext {
  token: NativeBoardSnapshot["token"];
  game: NativeBoardSnapshot["game"];
}
export interface MovementInspection extends InspectionContext {
  origin: string;
  side: Side;
  label: string;
  choices: BoardChoice[];
  read_only: true;
}
export type InspectMovement = (origin: string) => Promise<MovementInspection | null>;
// Common player-facing copy; side/piece terminology stays in the native label.
// Accessible read-only explanations and action admission are separate concerns.
export function inspectionStatus(label: string): string {
  return `Inspecting ${label} · Esc clears`;
}
export function inspectionContextKey(context: InspectionContext): string {
  const { token, game } = context;
  return `${game}:${token.document}:${token.version}:${token.revision}:${token.selection}`;
}
