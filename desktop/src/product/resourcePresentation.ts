import type { ResourceFailure } from "../resources";

// Resource identity and counts come from native admission. Diagnostic text is
// deliberately not an input: it may describe several supported bounds at once.
export function resourceExplanation(resource: ResourceFailure | null | undefined): string {
  if (!resource)
    return "Your current game can still be saved.";
  const fact = resource.fact;
  switch (fact.category) {
    case "encoded_document_bytes": return "The complete saved document would exceed the supported byte capacity. Your current game can still be saved.";
    case "canonical_record_bytes": return "This operation would exceed the native record byte capacity. Your current game can still be saved.";
    case "native_events": return "This operation would exceed the native event capacity. Your current game can still be saved.";
    case "replay_work": return "This operation would exceed the supported replay-work capacity. Your current game can still be saved.";
    case "identity_headroom": return "This operation would exceed the reserved execution-identity capacity. Your current game can still be saved.";
    case "validation_budget": return "Native validation reached its time budget. Your accepted game has been kept. Keep it open if Save cannot complete.";
    case "unknown": return "Your current game can still be saved.";
  }
}
