import { expect, it } from "vitest";
import { resourceExplanation } from "./resourcePresentation";
import type { ResourceFact, ResourceFailure } from "../resources";
const events: ResourceFact = { category: "native_events", measurement: "candidate", count: 8192, reserved: 1, limit: 8192, scope: "xiangqi", unit: "events" };
function failure(fact: ResourceFact, diagnostic = "Application capacity: retains256 observations and8192 controls") : ResourceFailure { return { fact, diagnostic }; }
it("uses authoritative event facts despite ambiguous combined native diagnostic text", () => {
  const text = resourceExplanation(failure(events));
  expect(text).toContain("native event capacity"); expect(text).not.toContain("256"); expect(text).not.toContain("observations");
});
it.each([
  ["encoded_document_bytes", "document would exceed"], ["canonical_record_bytes", "record byte"], ["native_events", "native event"], ["replay_work", "replay-work"], ["identity_headroom", "execution-identity"], ["validation_budget", "validation"],
] as const)("keeps %s separate from observation/control exhaustion", (category, text) => {
  const message = resourceExplanation(failure({ ...events, category }));
  expect(message).toContain(text); expect(message).not.toContain("256 recorded"); expect(message).not.toContain("control capacity");
});
it.each([undefined, failure({ ...events, category: "unknown" })])("uses a truthful generic fallback for missing or unknown facts", (payload) => {
  expect(resourceExplanation(payload)).toBe("Your current game can still be saved.");
});
it("presents native Tori replay refusal without inventing a game or engine result", () => {
  const nativeReplay: ResourceFact = { category: "replay_work", measurement: "candidate", count: 262618, reserved: 1040, limit: 262144, scope: "tori", unit: "replay_units" };
  const text = resourceExplanation(failure(nativeReplay, "Application capacity reached. The complete previously accepted document remains saveable."));
  expect(text).toContain("replay-work capacity"); expect(text).toContain("can still be saved");
  expect(text).not.toMatch(/illegal|win|loss|draw|engine failure|256/);
});
