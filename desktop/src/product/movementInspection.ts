import type { InspectionContext, MovementInspection } from "../native";

export type InspectMovement = (origin: string) => Promise<MovementInspection | null>;
// Common player-facing copy; side/piece terminology stays in the native label.
// Accessible read-only explanations and action admission are separate concerns.
export function inspectionStatus(label: string): string {
  return `Inspecting ${label} · Esc clears`;
}
export function inspectionContextKey(context: InspectionContext): string {
  const { token, game } = context;
  return `${context.profile ? `${game}:${context.profile}` : game}:${token.document}:${token.version}:${token.revision}:${token.selection}`;
}
