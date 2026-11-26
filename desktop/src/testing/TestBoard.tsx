// Test support only: renders the shared Board from colour pieces and a geometry,
// as the Chess snapshot fixtures and the synthetic fixture supply them.
import type { ComponentProps } from "react";
import { Board } from "../Board";
import type { BoardGeometry } from "../boardPresentation";
import type { Color } from "../native";

const chess: BoardGeometry = { files: [..."abcdefgh"], ranks: [..."12345678"], label: "Chess board", initialFocus: "e2" };
const chessGlyphs = { king: "♚", queen: "♛", rook: "♜", bishop: "♝", knight: "♞", pawn: "♟" };
type Shared = Omit<ComponentProps<typeof Board>, "pieces" | "files" | "ranks" | "label" | "initialFocus" | "bottomOwner" | "firstOwner" | "contextKey">;
export interface TestPiece { square: string; color: Color; kind: string; label: string }

/** Square keys in display order for a colour at the bottom. */
export function displayedSquares(orientation: Color, geometry = chess): string[] {
  const files = orientation === "white" ? geometry.files : [...geometry.files].reverse();
  const ranks = orientation === "white" ? [...geometry.ranks].reverse() : geometry.ranks;
  return ranks.flatMap(rank => files.map(file => `${file}${rank}`));
}
export function TestBoard({ pieces, orientation, version = 0, geometry = chess, contextKey = "", glyphs = chessGlyphs, ...props }: Shared & {
  pieces: TestPiece[]; orientation: Color; version?: number; geometry?: BoardGeometry; contextKey?: string;
}) {
  return <Board {...props} pieces={pieces.map(piece => ({ ...piece, owner: piece.color }))} glyphs={glyphs}
    files={orientation === "white" ? geometry.files : [...geometry.files].reverse()}
    ranks={orientation === "white" ? [...geometry.ranks].reverse() : geometry.ranks}
    label={`${geometry.label}, ${orientation} at bottom`} initialFocus={geometry.initialFocus}
    bottomOwner={orientation} firstOwner="white" contextKey={JSON.stringify([version, contextKey])} />;
}
