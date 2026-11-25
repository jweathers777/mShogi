import { expect, it } from "vitest";
import { resourceExplanation } from "./resourcePresentation";
import type { ResourceFact, ResourceFailure } from "../resources";
const events: ResourceFact = { category: "native_root_bytes", measurement: "candidate", count: 8192, reserved: 1, limit: 8192, scope: "xiangqi", unit: null };
function failure(fact: ResourceFact, diagnostic = "Application capacity: retains 256 observations and 8192 controls") : ResourceFailure { return { fact, diagnostic }; }
it("uses authoritative event facts despite ambiguous combined native diagnostic text", () => {
  const text = resourceExplanation(failure(events));
  expect(text).toContain("root byte"); expect(text).not.toContain("256"); expect(text).not.toContain("observations");
});
it.each([
  ["native_root_bytes", "root byte"], ["identity_headroom", "execution-identity"], ["validation_budget", "validation"],
] as const)("keeps %s separate from observation/control exhaustion", (category, text) => {
  const message = resourceExplanation(failure({ ...events, category }));
  expect(message).toContain(text); expect(message).not.toContain("256 recorded"); expect(message).not.toContain("control capacity");
});
it.each([undefined, failure({ ...events, category: "unknown" })])("uses a truthful generic fallback for missing or unknown facts", (payload) => {
  expect(resourceExplanation(payload)).toBe("Your current game can still be saved.");
});
