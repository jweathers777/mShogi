import { vi } from "vitest";
import type { Snapshot } from "../native";
import type { useNativeSession } from "../useNativeSession";
export function mockSession(
  snapshot: Snapshot,
  command = vi.fn(),
): ReturnType<typeof useNativeSession> {
  return {
    snapshot,
    documentEpoch: 0,
    error: "",
    connected: true,
    busy: false,
    command,
    save: vi.fn().mockResolvedValue(snapshot),
    saveAs: vi.fn().mockResolvedValue(snapshot),
    open: vi.fn().mockResolvedValue(snapshot),
    closeRequest: null,
    cancelClose: vi.fn().mockResolvedValue(snapshot),
    finishClose: vi.fn().mockResolvedValue(true),
    requestClose: vi.fn().mockResolvedValue(undefined),
  };
}
