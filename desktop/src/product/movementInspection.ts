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
export function inspectionContextKey(context: InspectionContext): string {
  const { token, game } = context;
  return `${game}:${token.document}:${token.version}:${token.revision}:${token.selection}`;
}
