// Native admission facts. R4 consumes these in the shared application notices;
// callers must never derive a category or count from diagnostic text.
export interface ResourceFact {
  category: "encoded_document_bytes" | "canonical_record_bytes" | "native_events"
    | "replay_work" | "identity_headroom"
    | "validation_budget" | "unknown";
  measurement: "accepted" | "candidate" | "unknown";
  count: number | null;
  reserved: number | null;
  limit: number | null;
  scope: "application" | "chess" | "shogi" | "xiangqi" | "tori" | "chu" | "tenjiku" | null;
  unit: "bytes" | "events" | "replay_units"
    | "identities" | "milliseconds" | null;
}
export interface ResourceFailure {
  fact: ResourceFact;
  diagnostic: string;
}
