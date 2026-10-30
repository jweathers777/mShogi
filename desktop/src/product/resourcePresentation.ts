import type { ResourceFailure } from "../resources";

// Resource identity and counts come from native admission. Diagnostic text is
// deliberately not an input: it may describe several supported bounds at once.
export function resourceExplanation(resource: ResourceFailure | null | undefined): string {
  if (!resource || resource.omitted_facts > 0 || resource.facts.length !== 1)
    return "Your current game can still be saved.";
  const fact = resource.facts[0]!;
  const limit = fact.limit?.toLocaleString();
  switch (fact.category) {
    case "ordinary_observations":
    case "exact_observations":
    case "combined_observations": {
      const kind = fact.category === "exact_observations" ? "exact analysis observations" : "engine observations";
      const count = fact.measurement === "accepted" ? fact.count : null;
      const complete = count !== null && fact.limit !== null && count >= fact.limit;
      const heading = complete ? `This game has reached the limit of ${limit} recorded ${kind}.`
        : `This operation would exceed this game's ${limit ? `${limit}-observation` : "recorded-observation"} capacity.`;
      const split = fact.category === "combined_observations" && fact.observations
        ? ` ${fact.measurement === "accepted" ? "Recorded" : fact.measurement === "candidate" ? "Proposed document" : "Native accounting"}: ${fact.observations.ordinary.toLocaleString()} ordinary and ${fact.observations.exact.toLocaleString()} exact observations.` : "";
      return heading + split;
    }
    case "controls": return "This game has reached its application control capacity. Your current game can still be saved.";
    case "encoded_document_bytes": return "The complete saved document would exceed the supported byte capacity. Your current game can still be saved.";
    case "encoded_observation_bytes": return "This operation would exceed the saved-observation byte capacity. Your current game can still be saved.";
    case "canonical_record_bytes": return "This operation would exceed the native record byte capacity. Your current game can still be saved.";
    case "native_events": return "This operation would exceed the native event capacity. Your current game can still be saved.";
    case "replay_work": return "This operation would exceed the supported replay-work capacity. Your current game can still be saved.";
    case "identity_headroom": return "This operation would exceed the reserved execution-identity capacity. Your current game can still be saved.";
    case "validation_budget": return "The native validation work budget was reached. Your current game can still be saved.";
    case "unknown": return "Your current game can still be saved.";
  }
}
