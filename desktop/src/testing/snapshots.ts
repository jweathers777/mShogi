import type { GameSnapshot } from "../native";
// Test support only: small changes of native snapshot fixtures.

/** The snapshot with `side` at the bottom of the board. */
export function oriented<T extends GameSnapshot>(snapshot: T, side: string): T {
  const settings = snapshot.document.settings;
  return { ...snapshot, document: { ...snapshot.document, settings: { ...settings, orientation: { ...settings.orientation, side } } } } as T;
}
