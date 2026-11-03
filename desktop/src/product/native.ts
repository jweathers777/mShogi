import type { ResourceFailure } from "../resources";
import type { AnalysisOperation, AnalysisState, Claim, ChessResult, Color, MatchConclusion, ExecutionObservation } from "../native";
import type { NativeBoardSnapshot } from "../NativeBoard";
import type { CompletedActionGeometry } from "../boardPresentation";
export type GameKind = "chess" | "shogi" | "xiangqi" | "tori" | "chu";
export type PlayableGameKind = Exclude<GameKind, "chu">;
export type NativeSide = { game_kind: "chess"; side: "white" | "black" }
  | { game_kind: "shogi"; side: "sente" | "gote" }
  | { game_kind: "xiangqi"; side: "red" | "black" }
  | { game_kind: "tori"; side: "sente" | "gote" }
  | { game_kind: "chu"; side: "sente" | "gote" };
export interface Side { id: NativeSide; key: string; label: string; first_mover: boolean }
export interface OpenToken { document: number; version: number; document_revision: number }
export interface SearchView {
  selected: boolean; observation_index: number; purpose: string; event_index: number; board_ply: number;
  perspective: string; engine: string; score: string | null; completed_depth: number | null;
  nodes: number | null; variation: string[]; status: string; historical: boolean;
}
export interface MovesPresentation {
  root: { label: string; numbering: string };
  entries: { event_index: number; board_ply: number; number: number; column: 0 | 1; notation: string; action: string; command_start: number; command_end: number }[];
  last_action: CompletedActionGeometry | null;
}
interface GameSnapshotBase extends Omit<NativeBoardSnapshot, "game" | "orientations" | "sides" | "capabilities"> {
  edition: string;
  score_meaning: string;
  open_token: OpenToken;
  human: Side; turn: Side; sides: Side[];
  orientations: { side: Side; files: string[]; ranks: string[] }[];
  document: { revision: number; dirty: boolean; path: string | null };
  // Native close admission can exempt only its own unchanged stop journal.
  // The document's canonical dirty state remains independent and authoritative.
  close_requires_save: boolean;
  io: { id: number; phase: string; message: string | null; resource?: ResourceFailure | null };
  capacity: { id: number; status: "available" | "checking" | "paused"; message: string | null; resource?: ResourceFailure | null };
  capabilities: { board_action: boolean; declaration: boolean; claims: boolean; resign: boolean; undo: boolean; take_back: boolean; move_analysis: boolean; chess_full: boolean; save: boolean; drops: boolean; promotion: boolean; engine_turn: boolean; hands: boolean; history: boolean };
  status: { concluded: boolean; label: string; score_half_points: number[] | null };
  events: { index: number; board_ply: number; label: string }[];
  moves: MovesPresentation;
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
    exact_observations: XiangqiExactEvidence[];
  };
  document: GameSnapshotBase["document"] & { settings: { game_kind: "xiangqi"; orientation: "red" | "black"; analysis_defaults: { depth: number; nodes: number } } };
};
export type ToriSide = "sente" | "gote";
export type ToriFace = "phoenix" | "falcon" | "eagle" | "crane" | "pheasant" | "left_quail" | "right_quail" | "swallow" | "goose";
export type ToriAction = { kind: "move"; from: string; to: string; result_face: ToriFace } | { kind: "drop"; face: ToriFace; to: string };
export type ToriStop = "cancelled" | "resource_limit" | "engine_failure" | "work_limit";
export type ToriConclusion = { kind: "checkmate" | "resignation"; winner: ToriSide } | { kind: "stalemate_loss" | "repetition_forfeit"; loser: ToriSide };
export interface ToriHistory {
  event_index: number; board_ply: number; in_check: boolean; occurrences: number[];
  obligation: { side: ToriSide; occurrences: [number, number, number]; forbidden_departures: ToriAction[] } | null;
  conclusion: ToriConclusion | null; incomplete: ToriStop | null;
}
export type ToriSnapshot = GameSnapshotBase & {
  game: "tori";
  tori: { root_kind: "standard" | "diagram" | "imported"; root_source: string | null; declared_root_plies: string;
    earlier_history_unknown: boolean; historical_reachability_proven: boolean; selected: ToriHistory; live: ToriHistory };
  document: GameSnapshotBase["document"] & { settings: { game_kind: "tori"; settings: { orientation: ToriSide } } };
};
// Native-only shared-contract consumer. This does not advertise a playable game.
export interface ChuHistory {
  event_index: number; board_ply: number; declared_total_plies: string;
  identities: { square: string; piece: { side: ToriSide; base: string; promoted: boolean }; face: string }[];
  royals: { identity: ChuHistory["identities"][number]; local_capture_threat: boolean }[];
  capture_all_royals_threat: [boolean | null, boolean | null]; threat_diagnostics: string[];
  lion_rights: unknown; occurrences: number[]; obligations: unknown[];
  conclusion: unknown | null; draw_offer: ToriSide | null; incomplete: string | null; last_effects: unknown | null;
}
export type ChuSquare = [number, number];
export type ChuAction = { from: ChuSquare; route: ChuSquare[]; promote: boolean };
export type ChuSnapshot = GameSnapshotBase & {
  game: "chu";
  chu: { root: unknown; declared_root_plies: string; earlier_history_unknown: boolean; historical_reachability_proven: boolean; selected: ChuHistory; live: ChuHistory; execution_supported: false };
  document: GameSnapshotBase["document"] & { settings: { game_kind: "chu"; settings: { orientation: ToriSide } } };
};
export type GameSnapshot = ChessSnapshot | ShogiSnapshot | XiangqiSnapshot | ToriSnapshot | ChuSnapshot;
export interface Observation {
  historical: boolean; application_error: string | null;
  observation: ExecutionObservation;
}
export type Operation =
  | { kind: "chu"; actor: ToriSide; command: { kind: "play"; action: ChuAction } | { kind: "resign" | "offer_draw" | "accept_draw" | "decline_draw" } | { kind: "stop"; reason: "cancelled" | "resource_limit" | "engine_failure" | "work_limit" } }
  | { kind: "chu_preferences"; settings: { orientation: ToriSide } }
  | { kind: "tori"; actor: ToriSide; command: { kind: "play"; action: ToriAction } | { kind: "resign" | "agree_draw" | "declare_impasse" } | { kind: "stop"; reason: ToriStop } }
  | { kind: "tori_preferences"; settings: { orientation: ToriSide } }
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

// Existing native saved-report projection, not a frontend proof validator or
// execution request. Exact source/provenance remains intact in About diagnostics.
export interface XiangqiExactEvidence {
  format: string; executor: string; source: unknown; purpose: string; goal: string;
  event_count: number; historical_target: boolean; position_json: string;
  native_session: number; native_generation: number;
  limits: { states: number; transitions: number; millis: number; memory_bytes: number; witness_plies: number; traversal: string };
  report: { verdict: { kind: "neither_can_win" } | { kind: "natural_win"; winner: "red" | "black"; reason: string; witness: string[] }
    | { kind: "unknown"; reason: "cancelled" | "states" | "transitions" | "memory" | "deadline" | "witness" }
    | { kind: "failure"; reason: string };
    work: { states: number; expanded: number; transitions: number; cache_hits: number; cache_probes: number; max_branching: number; memory_bytes: number; elapsed_micros: number } };
}
