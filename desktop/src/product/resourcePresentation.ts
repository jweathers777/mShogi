import type { ResourceFailure } from "../resources";

// Resource identity and counts come from native admission. Diagnostic text is
// deliberately not an input: it may describe several supported bounds at once.
export function resourceExplanation(resource: ResourceFailure | null | undefined): string {
  if (!resource)
    return "Your current game can still be saved.";
  const fact = resource.fact;
  switch (fact.category) {
    case "canonical_record_bytes": return "This operation would exceed the native root byte capacity. Your current game can still be saved.";
    case "identity_headroom": return "This operation would exceed the reserved execution-identity capacity. Your current game can still be saved.";
    case "validation_budget": return "Native validation reached its time budget. Your accepted game has been kept. Keep it open if Save cannot complete.";
    case "unknown": return "Your current game can still be saved.";
  }
}
