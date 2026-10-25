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
export type EngineResult =
  | { kind: "move"; action: string; telemetry: Telemetry }
  | { kind: "claim"; claim: Claim; telemetry: Telemetry }
  | { kind: "finished"; outcome: ChessOutcome; telemetry: Telemetry }
  | { kind: "incomplete"; telemetry: Telemetry }
  | { kind: "failure"; failure: { kind: FailureKind; message: string } };
export interface Snapshot {
  revision: number;
  version: number;
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
    status: "idle" | "thinking" | "cancelled" | "failed" | "incomplete";
    last: EngineResult | null;
    last_turn: Color | null;
    searched_version: number | null;
    message: string | null;
  };
  rules_mode: "m2";
  limits: { depth: number; nodes: number };
}
export type DesktopCommand =
  | { kind: "new_game"; version: number; human: Color }
  | { kind: "play"; version: number; action: string }
  | { kind: "claim"; version: number; claim: Claim }
  | { kind: "cancel" | "retry"; version: number };

export function newestSnapshot(
  current: Snapshot | null,
  incoming: Snapshot,
): Snapshot {
  return current === null || incoming.revision >= current.revision
    ? incoming
    : current;
}
