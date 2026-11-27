// @vitest-environment jsdom
import { gameSnapshotIsNewer } from "./sessionState";
import type { GameSnapshot } from "./sessionTypes";
import { expect, it } from "vitest";

import standard from "./__fixtures__/shared-shogi-standard.json";

it("rejects old frontend snapshots across identical coordinates and semantic undo", () => {
  const old: GameSnapshot = standard as GameSnapshot;
  const fresh = { ...old, token: { ...old.token, document: old.token.document + 1, revision: old.token.revision + 5 } };
  expect(gameSnapshotIsNewer(null, old)).toBe(true); expect(gameSnapshotIsNewer(fresh, old)).toBe(false);
  expect(gameSnapshotIsNewer(old, fresh)).toBe(true); expect(gameSnapshotIsNewer(fresh, fresh)).toBe(false);
});
