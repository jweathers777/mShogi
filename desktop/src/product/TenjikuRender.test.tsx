// @vitest-environment jsdom
// Reproducible contract-render measurements, not native WebView responsiveness.
import { Profiler } from "react";
import { afterEach, expect, it, vi } from "vitest";
import { cleanup, render } from "@testing-library/react";
import { TenjikuBoard } from "./TenjikuBoard";
import type { TenjikuSnapshot } from "./native";
import adamsJson from "../__fixtures__/shared-tenjiku-adams-standard.json";
import mullerJson from "../__fixtures__/shared-tenjiku-muller-standard.json";
afterEach(cleanup);

it.each([adamsJson, mullerJson])("measures dense native $profile render without asserting a timing threshold", value => {
  const source = value as unknown as TenjikuSnapshot;
  expect(source.pieces).toHaveLength(156);
  const commits: { phase: string; actual_ms: number; base_ms: number }[] = [];
  const started = performance.now();
  const { container } = render(<Profiler id={source.profile} onRender={(_id, phase, actual, base) => commits.push({ phase, actual_ms: actual, base_ms: base })}>
    <TenjikuBoard snapshot={source} disabled={false} inspectionDisabled={false} onPlay={vi.fn()} onInspect={vi.fn()} />
  </Profiler>);
  const elapsed = performance.now() - started;
  expect(container.querySelectorAll("svg [role=button]")).toHaveLength(256);
  expect(container.querySelectorAll("[data-tenjiku-piece]")).toHaveLength(156);
  console.info("TENJIKU_RENDER", JSON.stringify({ environment: `Node ${process.versions.node}/jsdom/ReactProfiler contract test; not native WebView`, profile: source.profile,
    pieces: source.pieces.length, board_targets: 256, compact_snapshot_utf8_bytes: new TextEncoder().encode(JSON.stringify(source)).length,
    render_to_settled_dom_ms: elapsed, profiler_actual_total_ms: commits.reduce((sum, commit) => sum + commit.actual_ms, 0), commits }));
});
