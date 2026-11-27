import { inspectionContextKey, type InspectMovement } from "./movementInspection";
import type { InspectionContext, MovementInspection, MovementInspectionQuery } from "../sessionTypes";
import { useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

// Stable game query/identity validator only. The shared choice controller is
// the sole owner of pending inspection, supersession, retirement and unmount.
// This adapter has no selection state or independent request lifetime.
export function useMovementInspection(context: InspectionContext | null): InspectMovement {
  const key = context === null ? null : inspectionContextKey(context);
  return useCallback(async (origin: string) => {
    if (context === null) return null;
    try {
      const result = await invoke<MovementInspection>("game_inspection", {
        queryJson: JSON.stringify({ expected: context.token, origin } satisfies MovementInspectionQuery),
      });
      return result.read_only === true && result.origin === origin && inspectionContextKey(result) === key ? result : null;
    } catch {
      // Stale/rejected/resource-limited queries never become game actions.
      return null;
    }
  }, [key]);
}
