import { expect, it } from "vitest";
import { resourceExplanation } from "./resourcePresentation";
import type { ResourceFact, ResourceFailure } from "../resources";
const control: ResourceFact = { category: "controls", measurement: "candidate", count: 8192, observations: null, reserved: 1, limit: 8192, scope: "xiangqi", unit: "controls" };
function failure(facts: ResourceFact[], diagnostic = "Application capacity: retains256 observations and8192 controls") : ResourceFailure { return { facts, diagnostic, omitted_facts: 0 }; }
it("uses authoritative control facts despite ambiguous combined native diagnostic text", () => {
  const text = resourceExplanation(failure([control]));
  expect(text).toContain("control capacity"); expect(text).not.toContain("256"); expect(text).not.toContain("observations");
});
it("preserves256 accepted mixed ordinary/exact observations without claiming257 were recorded", () => {
  const observation: ResourceFact = { ...control, category: "combined_observations", count: 256, measurement: "accepted", observations: { ordinary: 255, exact: 1 }, reserved: 0, limit: 256, unit: "observations" };
  expect(resourceExplanation(failure([observation]))).toContain("limit of 256 recorded engine observations");
  expect(resourceExplanation(failure([observation]))).toContain("255 ordinary and 1 exact");
  const candidate = resourceExplanation(failure([{ ...observation, count: 257, measurement: "candidate" }]));
  expect(candidate).toContain("would exceed"); expect(candidate).not.toContain("257 recorded");
});
it.each([
  ["encoded_document_bytes", "document would exceed"], ["encoded_observation_bytes", "observation byte"], ["canonical_record_bytes", "record byte"], ["native_events", "native event"], ["replay_work", "replay-work"], ["identity_headroom", "execution-identity"], ["validation_budget", "validation"],
] as const)("keeps %s separate from observation/control exhaustion", (category, text) => {
  const message = resourceExplanation(failure([{ ...control, category }]));
  expect(message).toContain(text); expect(message).not.toContain("256 recorded"); expect(message).not.toContain("control capacity");
});
it.each([undefined, failure([]), failure([control, { ...control, category: "combined_observations" }]), { ...failure([control]), omitted_facts: 1 }, failure([{ ...control, category: "unknown" }])])("uses a truthful generic fallback for missing/multiple/omitted/unknown facts", (payload) => {
  expect(resourceExplanation(payload)).toBe("Your current game can still be saved.");
});
it("presents native Tori replay and observation refusals without inventing a game or engine result", () => {
  const nativeReplay: ResourceFact = { category: "replay_work", measurement: "candidate", count: 262618, observations: null, reserved: 1040, limit: 262144, scope: "tori", unit: "replay_units" };
  const text = resourceExplanation(failure([nativeReplay], "Application capacity reached. The complete previously accepted document remains saveable."));
  expect(text).toContain("replay-work capacity"); expect(text).toContain("can still be saved");
  expect(text).not.toMatch(/illegal|win|loss|draw|engine failure|256/);
  const observations: ResourceFact = { ...nativeReplay, category: "ordinary_observations", count: 257, reserved: 0, limit: 256, unit: "observations" };
  expect(resourceExplanation(failure([observations]))).toContain("would exceed");
  expect(resourceExplanation(failure([observations]))).not.toContain("257 recorded");
});
