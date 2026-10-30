// Native admission facts. R4 consumes these in the shared application notices;
// callers must never derive a category or count from diagnostic text.
export interface ResourceFact {
  category: "encoded_document_bytes" | "encoded_observation_bytes" | "canonical_record_bytes" | "native_events"
    | "controls" | "ordinary_observations" | "exact_observations"
    | "combined_observations" | "replay_work" | "identity_headroom"
    | "validation_budget" | "unknown";
  measurement: "accepted" | "candidate" | "unknown";
  count: number | null;
  observations: { ordinary: number; exact: number } | null;
  reserved: number | null;
  limit: number | null;
  scope: "application" | "chess" | "shogi" | "xiangqi" | null;
  unit: "bytes" | "events" | "controls" | "observations" | "replay_units"
    | "identities" | "milliseconds" | null;
}
export interface ResourceFailure {
  facts: ResourceFact[];
  omitted_facts: number;
  diagnostic: string;
}
