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
  // Input targets are not movement: igui selects a victim while route stays empty.
  tenjiku?: { kind: "travel"; route: string[] } | { kind: "stationary_capture"; target: string };
}
export interface NativePiece { square: string; owner: string; kind: string; promoted: boolean; text: string; label: string }
export interface NativeHand { origin: string; owner: string; kind: string; count: number; text: string; label: string }

// MSH-123 supplies this from exact native history. It is deliberately not input
// to the interaction controller. R5 does not derive, remember or draw last moves.
export interface CompletedActionGeometry {
  readonly action?: string;
  readonly event_index?: number;
  readonly origin: NativeOrigin;
  readonly steps: readonly string[];
  // Ordered native victims, including intermediate Lion captures. Never inferred
  // from the board currently displayed or from an action spelling.
  readonly captures?: readonly { readonly square: string; readonly owner: unknown; readonly original_kind: string; readonly movement_face: string; readonly promoted: boolean }[];
  readonly effects?: readonly { readonly from: string; readonly to: string }[];
  readonly native_effects?: { readonly kind: "tenjiku"; readonly stationary_target: string | null; readonly phases: readonly TenjikuEffectPhase[] };
}

export interface TenjikuEffectPiece {
  readonly square: string; readonly face: string;
  readonly piece: { readonly id: number; readonly side: "sente" | "gote"; readonly base: string; readonly promoted: boolean; readonly right: string };
}
export type TenjikuEffectPhase = { readonly phase: "capture"; readonly victim: TenjikuEffectPiece }
  | { readonly phase: "burn"; readonly victims: readonly TenjikuEffectPiece[] }
  | { readonly phase: "mover_dies"; readonly mover: TenjikuEffectPiece }
  | { readonly phase: "promotion"; readonly before: TenjikuEffectPiece; readonly after: TenjikuEffectPiece };
