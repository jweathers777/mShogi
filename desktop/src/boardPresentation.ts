// Native presentation identities. Renderers decorate these; they do not parse
// action strings or infer effects. Synthetic verification uses the same choices.
export interface BoardGeometry {
  files: string[];
  ranks: string[];
  label: string;
  initialFocus: string;
}
export type NativeOrigin = { kind: "board"; square: string }
  | { kind: "intersection"; point: string }
  | { kind: "hand"; owner: string; piece_kind: string };
export interface NativeChoice {
  // Native product choices include a category; synthetic origins may be opaque.
  origin?: NativeOrigin;
  action: string;
  from: string;
  steps: string[];
  completion: { label: string; glyph: string | null } | null;
}
export interface NativePiece { square: string; owner: string; kind: string; promoted: boolean; text: string; label: string }
export interface NativeHand { origin: string; owner: string; kind: string; count: number; text: string; label: string }

// MSH-123 supplies this from exact native history. It is deliberately not input
// to the interaction controller. R5 does not derive, remember or draw last moves.
export interface CompletedActionGeometry {
  readonly origin: NativeOrigin;
  readonly steps: readonly string[];
}
