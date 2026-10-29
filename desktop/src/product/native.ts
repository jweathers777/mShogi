import type { NativeBoardSnapshot } from "../NativeBoard";
import type { Snapshot } from "../native";
export type GameKind = "chess" | "shogi" | "xiangqi";
export type NativeSide = { game_kind: "chess"; side: "white" | "black" }
  | { game_kind: "shogi"; side: "sente" | "gote" }
  | { game_kind: "xiangqi"; side: "red" | "black" };
export interface Side { id: NativeSide; key: string; label: string; first_mover: boolean }
export interface OpenToken { document: number; version: number; document_revision: number }
export interface SearchView {
  selected: boolean; observation_index: number; purpose: string; event_index: number; board_ply: number;
  perspective: string; engine: string; score: string | null; completed_depth: number | null;
  nodes: number | null; variation: string[]; status: string; historical: boolean;
}
interface GameSnapshotBase extends Omit<NativeBoardSnapshot, "game" | "orientations" | "sides" | "capabilities"> {
  open_token: OpenToken;
  human: Side; turn: Side; sides: Side[];
  orientations: { side: Side; files: string[]; ranks: string[] }[];
  document: { revision: number; dirty: boolean; path: string | null };
  io: { id: number; phase: string; message: string | null };
  capacity: { id: number; status: "available" | "checking" | "paused"; message: string | null };
  capabilities: { board_action: boolean; declaration: boolean; claims: boolean; resign: boolean; undo: boolean; move_analysis: boolean; save: boolean };
  status: { concluded: boolean; label: string; score_half_points: number[] | null };
  events: { index: number; board_ply: number; label: string }[];
  event_index: number; board_ply: number; play_limits: { depth: number; nodes: number }; historical: boolean; paused: boolean;
  phase: "idle" | "thinking" | "cancelled" | "failed" | "incomplete" | "paused";
  diagnostic: string | null;
  execution_purpose: string | null;
  declaration_advice: { actor: Side; eligible: boolean }[];
  context: unknown;
  analysis: Observation | null;
  observations: Observation[];
  search_observations: SearchView[];
  chess: Snapshot | null;
}
export type ChessShogiSnapshot = GameSnapshotBase & ({ game: "chess" } | { game: "shogi" }) & {
  document: GameSnapshotBase["document"] & { settings: { game_kind: "chess" | "shogi"; settings: { orientation: string; analysis_defaults: { depth: number; nodes: number } } } };
};
// Preserve the authoritative flat Xiangqi settings encoding; no native/archive migration.
export type XiangqiSnapshot = GameSnapshotBase & {
  game: "xiangqi";
  xiangqi: {
    terminal_actions: { offer_draw: boolean; accept_draw: boolean; decline_draw: boolean };
    canonical_context_scope: "selected_committed" | "live_with_immutable_selection";
    selected_canonical: unknown | null; selected_events: unknown[]; selected_position: string; root: string;
    claim_policy: "explicit" | "auto_valid_claims";
    count_advice: { claimant: "red" | "black"; known_plies: number; claimant_checks: number; effective_plies: number; eligible: boolean }[];
    warnings: { side: "red" | "black"; fault: "perpetual_check" | "perpetual_chase"; active: boolean; event_index: number; occurrences: number[] }[];
    offered_by: "red" | "black" | null;
    conclusion: unknown | null; pending: { reason: string; occurrences: number[] } | null;
    no_win_assessment: { kind: "proven_neither_can_win" | "proven_natural_win" | "unknown_exact_unavailable" | "unknown_exact_incomplete" | "failure"; winner?: "red" | "black"; witness?: string[]; reason?: string };
    exact_observations: unknown[];
  };
  document: GameSnapshotBase["document"] & { settings: { game_kind: "xiangqi"; orientation: "red" | "black"; analysis_defaults: { depth: number; nodes: number } } };
};
export type GameSnapshot = ChessShogiSnapshot | XiangqiSnapshot;
export interface Observation {
  historical: boolean; application_error: string | null;
  response: { context: { request_id: number; session_version: number; process_generation: number;
    purpose: { kind: string; target?: { selection_id: number; event_index: number; board_ply: number } }; limits: { depth: number; nodes: number } }; result: unknown };
}
export type Operation =
  | { kind: "xiangqi"; command: { kind: "play"; action: string } | { kind: "resign" | "offer_draw" | "accept_draw" | "decline_draw" | "claim_natural_count" } }
  | { kind: "xiangqi_preferences"; settings: { orientation: "red" | "black"; analysis_defaults: { depth: number; nodes: number } } }
  | { kind: "play"; action: string }
  | { kind: "resume" | "retry" | "cancel" | "undo" | "return_live" | "resign" | "declare" }
  | { kind: "inspect"; event_index: number }
  | { kind: "analyze"; operation: { kind: "moves"; depth: number; nodes: number } }
  | { kind: "shogi_preferences"; settings: { orientation: string; analysis_defaults: { depth: number; nodes: number } } };
