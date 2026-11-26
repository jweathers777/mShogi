// @vitest-environment jsdom
import { expect, it } from "vitest";
import { nativeSnapshotIsNewer, type NativeBoardSnapshot } from "./NativeBoard";
import standard from "./__fixtures__/shared-shogi-standard.json";

it("rejects old frontend snapshots across identical coordinates and semantic undo", () => {
  const old: NativeBoardSnapshot = standard as NativeBoardSnapshot;
  const fresh = { ...old, token: { ...old.token, document: old.token.document + 1, revision: old.token.revision + 5 } };
  expect(nativeSnapshotIsNewer(null, old)).toBe(true); expect(nativeSnapshotIsNewer(fresh, old)).toBe(false);
  expect(nativeSnapshotIsNewer(old, fresh)).toBe(true); expect(nativeSnapshotIsNewer(fresh, fresh)).toBe(false);
});
