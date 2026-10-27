import { useCallback, useLayoutEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { inspectionContextKey, type InspectionContext, type MovementInspection, type InspectMovement } from "./movementInspection";

// This query never enters the command/result-acceptance path. Exact native
// selection and request freshness are checked again before presentation.
export function useMovementInspection(context: InspectionContext | null): InspectMovement {
  const key = context === null ? null : inspectionContextKey(context);
  const current = useRef({ key, generation: 0, mounted: false });
  useLayoutEffect(() => {
    current.current = { key, generation: current.current.generation + 1, mounted: true };
    return () => { current.current.mounted = false; current.current.generation++; };
  }, [key]);
  return useCallback(async (origin: string) => {
    if (context === null || !current.current.mounted || current.current.key !== key) return null;
    const generation = ++current.current.generation;
    try {
      const result = await invoke<MovementInspection>("game_inspection", {
        queryJson: JSON.stringify({ expected: context.token, origin }),
      });
      if (!current.current.mounted || current.current.generation !== generation || current.current.key !== key
        || result.read_only !== true || result.origin !== origin || inspectionContextKey(result) !== key) return null;
      return result;
    } catch {
      // Stale/rejected/resource-limited inspection never changes game state.
      return null;
    }
  // Token primitives form the immutable query context. Snapshot polling alone
  // must not retire an inspection when its exact context remains unchanged.
  }, [key]);
}
