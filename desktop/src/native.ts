import type { NativeSide } from "./product/native";
// Presentation DTOs mirror mshogi-desktop-session and mshogi-engine serde output.
// Actions are opaque native identifiers; legality and adjudication stay in Rust.
export type Color = "white" | "black";
export type DrawClaim = "threefold" | "fifty_move";
export interface Claim {
  actor: Color;
  reason: DrawClaim;
  intended: string | null;
}
export interface BoardPiece {
  square: string;
  color: Color;
  kind: string;
  label: string;
}
export type DrawReason =
  | "stalemate"
  | "dead_position"
  | "fivefold"
  | "seventy_five_move"
  | { claimed: DrawClaim };
export type ChessOutcome =
  { checkmate: { winner: Color } } | { draw: DrawReason };
export type TerminationTrigger = "resignation" | "flag_fall";
export type MatchConclusion =
  | { chess: ChessOutcome }
  | { win: { winner: Color; trigger: TerminationTrigger } }
  | { draw: { trigger: TerminationTrigger | null } }
  | { unresolved: { actor: Color; trigger: TerminationTrigger } };
export type EngineChoice =
  { kind: "move"; action: string } | { kind: "claim"; claim: Claim };
export interface Telemetry {
  score: number | null;
  completed_depth: number;
  nodes: number;
  principal_variation: EngineChoice[];
  status: "complete" | "node_limit" | "stopped";
}
export type FailureKind =
  | "unsupported_mode"
  | "invalid_request"
  | "search"
  | "start"
  | "io"
  | "unexpected_exit"
  | "malformed_response"
  | "oversized_response"
  | "missing_response"
  | "timeout"
  | "context_mismatch"
  | "cleanup";

export type ReachabilityTarget = "either" | Color;
export interface ReachabilityWork {
  positions: number;
  expanded: number;
  transitions: number;
  static_leaves: number;
}
export type MateReachability =
  | { possible: { witness: string[]; work: ReachabilityWork } }
  | { impossible: { work: ReachabilityWork } }
  | {
      unknown: {
        reason:
          | "cancelled"
          | "position_limit"
          | "transition_limit"
          | "resource_limit";
        work: ReachabilityWork;
      };
    };
export type AnalysisOperation =
  | { kind: "moves"; depth: number; nodes: number }
  | {
      kind: "full_reachability";
      target: ReachabilityTarget;
      max_positions: number;
      max_transitions: number;
    };
export interface ExecutionTarget {
  selection_id: number;
  board_ply: number;
  event_index: number;
  session_version: number;
  label: string;
  turn: NativeSide;
}
export interface ExecutionContext {
  request_id: number;
  session_version: number;
  process_generation: number;
  game: unknown;
  limits: { depth: number; nodes: number };
  purpose:
    | { kind: "engine_turn" }
    | { kind: "move_analysis"; target: ExecutionTarget }
    | {
        kind: "chess_full_reachability";
        target: ExecutionTarget;
        mate: ReachabilityTarget;
        limits: { max_positions: number; max_transitions: number };
      };
}
export interface AnalysisState {
  status:
    "idle" | "thinking" | "complete" | "incomplete" | "cancelled" | "failed";
  target: ExecutionTarget | null;
  operation: AnalysisOperation | null;
  context: ExecutionContext | null;
  result: ChessResult | null;
  message: string | null;
}

export type ChessResult =
  | { kind: "move"; action: string; telemetry: Telemetry }
  | { kind: "claim"; claim: Claim; telemetry: Telemetry }
  | { kind: "finished"; outcome: ChessOutcome; telemetry: Telemetry }
  | { kind: "incomplete"; telemetry: Telemetry }
  | { kind: "failure"; failure: { kind: FailureKind; message: string } }
  | { kind: "reachability"; result: MateReachability };
export type ExecutionResult =
  | { kind: "chess"; result: ChessResult }
  | { kind: "shogi" | "xiangqi"; result: unknown }
  | { kind: "failure"; failure: { kind: FailureKind; message: string } };
export interface ExecutionObservation {
  schema_version: number;
  context: ExecutionContext;
  result: ExecutionResult;
}
export interface ArchivedObservation {
  sequence: number;
  label: string;
  observation: ExecutionObservation;
  historical: boolean;
  analyzed_turn: Color;
  application_error: string | null;
}
