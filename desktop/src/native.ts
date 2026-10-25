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
export interface LegalChoice {
  action: string;
  from: string;
  to: string;
  promotion: string | null;
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
export interface AnalysisTarget {
  selection_id: number;
  ply: number;
  session_version: number;
  label: string;
  turn: Color;
}
export interface EngineContext {
  protocol: number;
  request_id: number;
  session_version: number;
  process_generation: number;
  game: unknown;
  limits: { depth: number; nodes: number };
  purpose:
    | { kind: "engine_turn" }
    | { kind: "move_analysis"; target: AnalysisTarget }
    | {
        kind: "full_reachability";
        target: AnalysisTarget;
        mate: ReachabilityTarget;
        limits: { max_positions: number; max_transitions: number };
      };
}
export interface AnalysisState {
  status:
    "idle" | "thinking" | "complete" | "incomplete" | "cancelled" | "failed";
  target: AnalysisTarget | null;
  operation: AnalysisOperation | null;
  context: EngineContext | null;
  result: EngineResult | null;
  message: string | null;
}

export type EngineResult =
  | { kind: "move"; action: string; telemetry: Telemetry }
  | { kind: "claim"; claim: Claim; telemetry: Telemetry }
  | { kind: "finished"; outcome: ChessOutcome; telemetry: Telemetry }
  | { kind: "incomplete"; telemetry: Telemetry }
  | { kind: "failure"; failure: { kind: FailureKind; message: string } }
  | { kind: "reachability"; result: MateReachability };
export interface EngineResponse {
  context: EngineContext;
  result: EngineResult;
}
export interface ArchivedObservation {
  sequence: number;
  label: string;
  response: EngineResponse;
  historical: boolean;
  analyzed_turn: Color;
  application_error: string | null;
}
export interface DocumentState {
  revision: number;
  dirty: boolean;
  path: string | null;
  settings: { orientation: Color; analysis_defaults: AnalysisOperation };
}
export interface DocumentIo {
  id: number;
  phase: "idle" | "saving" | "opening" | "viewing" | "complete" | "failed";
  message: string | null;
}
export interface HistoricalView {
  historical: boolean;
  event_index: number;
  event_count: number;
  label: string;
  fen: string;
  pieces: BoardPiece[];
  turn: Color;
  conclusion: MatchConclusion | null;
  ply: number;
  selection_id: number;
  timeline: { index: number; label: string; ply: number }[];
}
export interface Snapshot {
  revision: number;
  version: number;
  document: DocumentState;
  io: DocumentIo;
  view: HistoricalView;
  observations: ArchivedObservation[];
  human: Color;
  turn: Color;
  fen: string;
  pieces: BoardPiece[];
  legal_choices: LegalChoice[];
  claims: Claim[];
  history: string[];
  conclusion: MatchConclusion | null;
  in_check: boolean;
  engine: {
    status:
      "idle" | "thinking" | "cancelled" | "failed" | "incomplete" | "paused";
    last: EngineResult | null;
    last_turn: Color | null;
    searched_version: number | null;
    message: string | null;
  };
  play_paused: boolean;
  analysis: AnalysisState;
  rules_mode: "m2";
  limits: { depth: number; nodes: number };
}
export type DesktopCommand =
  | {
      kind: "new_game";
      version: number;
      document_revision: number;
      human: Color;
    }
  | { kind: "play"; version: number; action: string }
  | { kind: "claim"; version: number; claim: Claim }
  | {
      kind: "cancel" | "retry" | "resign" | "cancel_analysis" | "resume";
      version: number;
    }
  | { kind: "analyze"; version: number; operation: AnalysisOperation }
  | {
      kind: "preferences";
      orientation: Color;
      analysis_defaults: AnalysisOperation;
    }
  | { kind: "inspect"; version: number; event_index: number }
  | { kind: "return_live"; version: number };

export function newestSnapshot(
  current: Snapshot | null,
  incoming: Snapshot,
): Snapshot {
  return current === null || incoming.revision >= current.revision
    ? incoming
    : current;
}
