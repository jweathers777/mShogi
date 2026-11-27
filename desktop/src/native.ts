// Presentation DTOs mirror mshogi-desktop-session and mshogi-engine serde output.
// Actions are opaque native identifiers; legality and adjudication stay in Rust.
import type { GameKind, GameSide, RulesProfile, TenjikuProfile } from "./games";

export type Color = GameSide<"chess">;
export type ShogiSide = GameSide<"shogi">;
export type NativeSide = { [K in GameKind]: { game_kind: K; side: GameSide<K> } }[GameKind];
export type SessionToken = { document: number; version: number; revision: number; selection: number };
export type WorkLimits = AnalysisOperation & { time_ms?: number };
export type Purpose = "engine_turn" | "move_analysis";
type CanonicalRecord = { envelope_version: number; game_kind: GameKind; payload: unknown };
type XiangqiPending = { reason: string; occurrences: number[] };
type XiangqiAssessment =
  | { kind: "proven_neither_can_win" | "unknown" | "failure" }
  | { kind: "proven_natural_win"; winner: GameSide<"xiangqi">; witness: string[] };
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
interface SearchTelemetry<Choice> {
  score: number | null;
  completed_depth: number;
  nodes: number;
  principal_variation: Choice[];
  status: "complete" | "node_limit" | "stopped" | "unresolved";
}
export interface Telemetry extends SearchTelemetry<EngineChoice> {
  selective_depth: number;
}
interface NativeTelemetry<K extends GameKind> extends SearchTelemetry<unknown> {
  evaluator_id: string;
  search_id: string;
  perspective: GameSide<K>;
}
export type FailureKind =
  | "invalid_request"
  | "search"
  | "start"
  | "io"
  | "unexpected_exit"
  | "malformed_response"
  | "missing_response"
  | "timeout"
  | "context_mismatch"
  | "cleanup";

type EngineFailure = { kind: FailureKind; message: string };

export interface AnalysisOperation {
  depth: number;
  nodes: number;
}
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
  game: CanonicalRecord;
  limits: WorkLimits;
  purpose:
    | { kind: "engine_turn" }
    | { kind: "move_analysis"; target: ExecutionTarget };
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
  | { kind: "failure"; failure: EngineFailure };
// Native choices and conclusions remain opaque where the UI uses the native
// search projection. Only Xiangqi has an unresolved result with a pending witness.
type NativeResult<K extends GameKind> =
  | { kind: "choice"; choice: unknown; telemetry: NativeTelemetry<K> }
  | { kind: "finished"; conclusion: unknown; telemetry: NativeTelemetry<K> }
  | { kind: "incomplete"; telemetry: NativeTelemetry<K> }
  | (K extends "xiangqi" ? { kind: "unresolved"; pending: XiangqiPending; telemetry: NativeTelemetry<K> } : never);
export type ExecutionResult =
  | { kind: "chess"; result: ChessResult }
  | { [K in Exclude<GameKind, "chess">]: { kind: K; result: NativeResult<K> } }[Exclude<GameKind, "chess">]
  | { kind: "failure"; failure: EngineFailure };
export interface ExecutionObservation {
  schema_version: number;
  context: ExecutionContext;
  result: ExecutionResult;
}

// Native admission facts. R4 consumes these in the shared application notices;
// callers must never derive a category or count from diagnostic text.
export interface ResourceFact {
  category: "native_root_bytes"
    | "identity_headroom"
    | "validation_budget" | "unknown";
  measurement: "accepted" | "candidate" | "unknown";
  count: number | null;
  reserved: number | null;
  limit: number | null;
  scope: "application" | GameKind | null;
  unit: "identities" | "milliseconds" | null;
}
export interface ResourceFailure {
  fact: ResourceFact;
  diagnostic: string;
}

export type NativeOrigin = { kind: "board"; square: string }
  | { kind: "intersection"; point: string }
  | { kind: "hand"; owner: string; piece_kind: string };
export interface BoardChoice {
  action: string;
  from: string;
  steps: string[];
  completion: { label: string; glyph: string | null } | null;
  // Input targets are not movement: igui selects a victim while route stays empty.
  tenjiku?: { kind: "travel"; route: string[] } | { kind: "stationary_capture"; target: string };
}
export interface NativeChoice extends BoardChoice { origin: NativeOrigin }
export interface NativePiece { square: string; owner: string; kind: string; promoted: boolean; text: string; label: string }
export interface NativeHand { origin: string; owner: string; kind: string; count: number; text: string; label: string }

// MSH-123 supplies this from exact native history. It is deliberately not input
// to the interaction controller. R5 does not derive, remember or draw last moves.
export interface CompletedActionGeometry {
  readonly action: string;
  readonly event_index: number;
  readonly origin: NativeOrigin;
  readonly steps: readonly string[];
  // Ordered native victims, including intermediate Lion captures. Never inferred
  // from the board currently displayed or from an action spelling.
  readonly captures?: readonly { readonly square: string; readonly owner: NativeSide; readonly original_kind: string; readonly movement_face: string; readonly promoted: boolean }[];
  readonly effects: readonly { readonly from: string; readonly to: string }[];
  readonly native_effects?: { readonly kind: "tenjiku"; readonly stationary_target: string | null; readonly phases: readonly TenjikuEffectPhase[] };
}

export type TenjikuEffectPhase = { readonly phase: "capture"; readonly victim: TenjikuPieceIdentity }
  | { readonly phase: "burn"; readonly victims: readonly TenjikuPieceIdentity[] }
  | { readonly phase: "mover_dies"; readonly mover: TenjikuPieceIdentity }
  | { readonly phase: "promotion"; readonly before: TenjikuPieceIdentity; readonly after: TenjikuPieceIdentity };

export type PlayMode = "human_engine" | "two_humans";
export interface Side { id: NativeSide; key: string; label: string; first_mover: boolean }
// Document settings have the same shape in every game.
export interface GameSettings { play_mode: PlayMode; orientation: NativeSide; analysis_defaults: AnalysisOperation }
export interface OpenToken { document: number; version: number; document_revision: number }
export interface SearchView {
  selected: boolean; observation_index: number; purpose: Purpose; event_index: number; board_ply: number;
  perspective: string; engine: string; score: string | null; completed_depth: number | null;
  selective_depth?: number;
  nodes: number | null; variation: string[]; status: string; historical: boolean;
}
export interface MovesPresentation {
  root: { label: string; numbering: string };
  entries: { event_index: number; board_ply: number; number: number; column: 0 | 1; notation: string; effects_description?: string; action: string; command_start: number; command_end: number }[];
  last_action: CompletedActionGeometry | null;
}
interface GameSnapshotBase<K extends GameKind = GameKind> {
  game: K;
  chess: K extends "chess" ? ChessPresentation : null;
  token: SessionToken;
  profile: RulesProfile;
  board: { files: string[]; ranks: string[]; label: string; initial_focus: string;
    intersections?: { points: string[]; river_between_ranks: [number, number]; palaces: { side: Side; points: string[]; diagonals: string[][] }[] } };
  pieces: NativePiece[];
  hands: NativeHand[];
  choices: NativeChoice[];
  edition: string;
  score_meaning: string;
  open_token: OpenToken;
  play_mode: PlayMode; local_actor: Side;
  human: Side; turn: Side; sides: Side[];
  orientations: { side: Side; files: string[]; ranks: string[] }[];
  document: { revision: number; dirty: boolean; path: string | null; settings: GameSettings };
  // Native close admission can exempt only its own unchanged stop journal.
  // The document's canonical dirty state remains independent and authoritative.
  close_requires_save: boolean;
  io: { id: number; phase: "idle" | "saving" | "opening" | "viewing" | "complete" | "failed"; message: string | null; resource: ResourceFailure | null };
  capacity: { id: number; status: "available" | "checking" | "paused"; message: string | null; resource: ResourceFailure | null };
  capabilities: { board_action: boolean; declaration: boolean; claims: boolean; resign: boolean; undo: boolean; take_back: boolean; move_analysis: boolean; save: boolean; drops: boolean; promotion: boolean; engine_turn: boolean; hands: boolean; history: boolean };
  status: { concluded: boolean; label: string; score_half_points: [number, number] | null };
  events: { index: number; board_ply: number; label: string }[];
  moves: MovesPresentation;
  event_index: number; board_ply: number; play_limits: WorkLimits; historical: boolean; paused: boolean;
  phase: "idle" | "thinking" | "cancelled" | "failed" | "incomplete" | "paused";
  diagnostic: string | null;
  execution_purpose: Purpose | null;
  declaration_advice: { actor: Side; eligible: boolean }[];
  context: CanonicalRecord;
  analysis: Observation | null;
  observations: Observation[];
  search_observations: SearchView[];
  chess_match: unknown | null;
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
}
export type ChessSnapshot = GameSnapshotBase<"chess">;
export type ShogiSnapshot = GameSnapshotBase<"shogi">;
export type XiangqiSnapshot = GameSnapshotBase<"xiangqi"> & {
  xiangqi: {
    terminal_actions: { offer_actor: Side; response_actor: Side; offer_draw: boolean; accept_draw: boolean; decline_draw: boolean };
    canonical_context_scope: "selected_committed" | "live_with_immutable_selection";
    selected_canonical: CanonicalRecord | null; selected_events: unknown[]; selected_position: string; root: string;
    claim_policy: "explicit" | "auto_valid_claims";
    count_advice: { claimant: GameSide<"xiangqi">; known_plies: number; claimant_checks: number; effective_plies: number; eligible: boolean }[];
    warnings: { side: GameSide<"xiangqi">; fault: "perpetual_check" | "perpetual_chase"; active: boolean; event_index: number; occurrences: number[] }[];
    offered_by: GameSide<"xiangqi"> | null;
    conclusion: unknown | null; pending: XiangqiPending | null;
    no_win_assessment: XiangqiAssessment;
  };
};
export type ToriFace = "phoenix" | "falcon" | "eagle" | "crane" | "pheasant" | "left_quail" | "right_quail" | "swallow" | "goose";
export type ToriAction = { kind: "move"; from: string; to: string; result_face: ToriFace } | { kind: "drop"; face: ToriFace; to: string };
export type ToriConclusion = { kind: "checkmate" | "resignation"; winner: ShogiSide } | { kind: "stalemate_loss" | "repetition_forfeit"; loser: ShogiSide };
export interface ToriHistory {
  event_index: number; board_ply: number; in_check: boolean; occurrences: number[];
  repetition: { side: ShogiSide; forbidden: ToriAction[] } | null;
  conclusion: ToriConclusion | null;
}
export type ToriSnapshot = GameSnapshotBase<"tori"> & {
  tori: { root_kind: "standard" | "diagram" | "imported"; root_source: string | null; declared_root_plies: string;
    earlier_history_unknown: boolean; historical_reachability_proven: boolean; selected: ToriHistory; live: ToriHistory };
};
// Real native Chu session used by the ordinary fifth-game product.
export interface ChuHistory {
  event_index: number; board_ply: number; declared_total_plies: string;
  identities: { square: string; piece: { side: ShogiSide; base: string; promoted: boolean }; face: string }[];
  royals: { identity: ChuHistory["identities"][number]; local_capture_threat: boolean }[];
  capture_all_royals_threat: [boolean | null, boolean | null]; threat_diagnostics: string[];
  lion_rights: unknown; occurrences: number[]; obligations: unknown[];
  conclusion: unknown | null; draw_offer: ShogiSide | null; last_effects: unknown | null;
}
export type ChuSnapshot = GameSnapshotBase<"chu"> & {
  chu: { root: unknown; declared_root_plies: string; earlier_history_unknown: boolean; historical_reachability_proven: boolean; selected: ChuHistory; live: ChuHistory; execution_supported: boolean };
};
export interface TenjikuPieceIdentity {
  readonly square: string; readonly face: string;
  readonly piece: { readonly id: number; readonly side: ShogiSide; readonly base: string; readonly promoted: boolean; readonly right: string };
}
export interface TenjikuHistory {
  event_index: number; board_ply: number; declared_total_plies: string;
  identities: TenjikuPieceIdentity[]; royals: TenjikuPieceIdentity[];
  repetition_key: unknown; known_repetition_positions: number; first_recurrence_forbidden: true;
  pending_bare: ShogiSide | null; conclusion: unknown | null; draw_offer: ShogiSide | null;
  last_effects: unknown | null;
}
export type TenjikuSnapshot = GameSnapshotBase<"tenjiku"> & {
  profile: TenjikuProfile;
  tenjiku: { profile: TenjikuProfile; edition: string; root: unknown; declared_root_plies: string;
    earlier_history_unknown: boolean; historical_reachability_proven: boolean;
    selected: TenjikuHistory; live: TenjikuHistory; execution_supported: boolean };
};
export type GameSnapshot = ChessSnapshot | ShogiSnapshot | XiangqiSnapshot | ToriSnapshot | ChuSnapshot | TenjikuSnapshot;
export interface Observation {
  historical: boolean; application_error: string | null;
  observation: ExecutionObservation;
}
// A native command of the active game, as its records spell it.
export type NativeCommand = { kind: "offer_draw" | "accept_draw" | "decline_draw" | "claim_natural_count" | "declare_entering_king" }
  | { claim: { reason: Claim["reason"]; intended: string | null } };
// Operation kinds name no game: game data is only in the payload.
export type Operation =
  | { kind: "play"; action: string }
  | { kind: "command"; command: NativeCommand }
  | { kind: "preferences"; settings: GameSettings }
  | { kind: "resume" | "retry" | "cancel" | "take_back" | "undo" | "return_live" | "resign" }
  | { kind: "inspect"; event_index: number }
  | { kind: "analyze"; operation: AnalysisOperation };

export interface DesktopCommand { expected: SessionToken; game: GameKind; operation: Operation }
export interface MovementInspectionQuery { expected: SessionToken; origin: string }

export interface InspectionContext {
  token: SessionToken;
  game: GameKind;
  profile?: RulesProfile;
}
export interface MovementInspection extends InspectionContext {
  origin: string;
  side: Side;
  label: string;
  choices: NativeChoice[];
  read_only: true;
}

export interface SyntheticSnapshot {
  profile: "mshogi.synthetic.strider16.v1";
  version: number;
  geometry: Pick<GameSnapshotBase["board"], "files" | "ranks" | "label" | "initial_focus">;
  pieces: BoardPiece[];
  choices: (Omit<BoardChoice, "completion" | "tenjiku"> & { completion: null })[];
  record: {
    schema_version: 1;
    rules_profile: "mshogi.synthetic.strider16.v1";
    initial_fixture: string;
    actions: string[];
  };
  complete: boolean;
  last_action: CompletedActionGeometry | null;
}
export type SyntheticCommand =
  | { kind: "play"; version: number; action: string }
  | { kind: "reset"; version: number };
