import type { ResourceFailure } from "../resources";
import type { AnalysisOperation, AnalysisState, Claim, ChessResult, Color, MatchConclusion, ExecutionObservation } from "../native";
import type { NativeBoardSnapshot } from "../NativeBoard";
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
  edition: string;
  score_meaning: string;
  open_token: OpenToken;
  human: Side; turn: Side; sides: Side[];
  orientations: { side: Side; files: string[]; ranks: string[] }[];
  document: { revision: number; dirty: boolean; path: string | null };
  io: { id: number; phase: string; message: string | null; resource?: ResourceFailure | null };
  capacity: { id: number; status: "available" | "checking" | "paused"; message: string | null; resource?: ResourceFailure | null };
  capabilities: { board_action: boolean; declaration: boolean; claims: boolean; resign: boolean; undo: boolean; take_back: boolean; move_analysis: boolean; chess_full: boolean; save: boolean; drops: boolean; promotion: boolean; engine_turn: boolean; hands: boolean; history: boolean };
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
  chess: ChessPresentation | null;
  chess_match?: unknown | null;
}
export interface ChessPresentation {
  claims: Claim[];
  live_conclusion: MatchConclusion | null;
  selected_conclusion: MatchConclusion | null;
  in_check: boolean;
  analysis: AnalysisState;
  last_engine_result: ChessResult | null;
  last_engine_turn: Color | null;
  searched_version: number | null;
  rules_mode: "m2";
}
export type ChessSnapshot = GameSnapshotBase & {
  game: "chess";
  chess: ChessPresentation;
  document: GameSnapshotBase["document"] & { settings: { game_kind: "chess"; settings: { orientation: Color; analysis_defaults: AnalysisOperation } } };
};
export type ShogiSnapshot = GameSnapshotBase & {
  game: "shogi";
  document: GameSnapshotBase["document"] & { settings: { game_kind: "shogi"; settings: { orientation: "sente" | "gote"; analysis_defaults: { depth: number; nodes: number } } } };
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
export type GameSnapshot = ChessSnapshot | ShogiSnapshot | XiangqiSnapshot;
export interface Observation {
  historical: boolean; application_error: string | null;
  observation: ExecutionObservation;
}
export type Operation =
  | { kind: "chess_play"; action: string }
  | { kind: "chess_claim"; claim: Claim }
  | { kind: "chess_preferences"; orientation: Color; analysis_defaults: AnalysisOperation }
  | { kind: "xiangqi"; command: { kind: "play"; action: string } | { kind: "resign" | "offer_draw" | "accept_draw" | "decline_draw" | "claim_natural_count" } }
  | { kind: "xiangqi_preferences"; settings: { orientation: "red" | "black"; analysis_defaults: { depth: number; nodes: number } } }
  | { kind: "play"; action: string }
  | { kind: "resume" | "retry" | "cancel" | "take_back" | "undo" | "return_live" | "resign" | "declare" }
  | { kind: "inspect"; event_index: number }
  | { kind: "analyze"; operation: AnalysisOperation }
  | { kind: "shogi_preferences"; settings: { orientation: string; analysis_defaults: { depth: number; nodes: number } } };
