//! Concrete native desktop consumer boundary. Native games remain authoritative.
//! Native-specific capability data composes into the one application contract.
use super::*;
#[path = "shared_archive.rs"]
pub(super) mod archive;
pub use archive::{NativeDesktopSettings, SharedDocumentState, SharedOpenToken, ShogiSettings};
#[path = "shared_inspection.rs"]
mod inspection;
#[path = "shared_xiangqi.rs"]
pub(super) mod xiangqi;
pub use inspection::{MovementInspection, MovementInspectionQuery};
pub use mshogi_core::native::{GameKind, NativeSide};
pub use mshogi_core::shogi::Side as ShogiSide;
use mshogi_core::{
    native::{CanonicalRecord, RulesProfile},
    shogi as s,
};
use mshogi_engine::{
    ExecutionContext, ExecutionObservation, ExecutionPurpose, ExecutionRequest, ExecutionResponse,
    ExecutionResult, ExecutionTarget, OBSERVATION_SCHEMA_VERSION, ShogiChoice, ShogiResult,
};
pub use xiangqi::{
    IntersectionTopology, Palace, XiangqiExactEvidence, XiangqiPresentation, XiangqiSettings,
};

// The displayed play budget and submitted work use the same native value.
const SHOGI_PLAY_LIMITS: WorkLimits = WorkLimits {
    depth: 2,
    nodes: 5000,
};

/// Exact UI precondition; neither board ply nor semantic undo restores this token.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionToken {
    /// Monotonic desktop replacement epoch; never restored from a game record.
    pub document: u64,
    /// Monotonic session version, independent of board ply.
    pub version: u64,
    /// Monotonic snapshot revision including execution observations.
    pub revision: u64,
    /// Monotonic historical/live selection identity.
    pub selection: u64,
}
/// Native-qualified side with presentation-ready labels.
#[derive(Clone, Debug, Serialize)]
pub struct PresentedSide {
    /// Native game-qualified side identity.
    pub id: NativeSide,
    /// Profile-qualified stable presentation key.
    pub key: String,
    /// Native human-readable description.
    pub label: String,
    /// Whether this is the native first-moving side.
    pub first_mover: bool,
}
pub(super) fn side(id: NativeSide) -> PresentedSide {
    let (profile, name, first) = match id {
        NativeSide::Chess(Color::White) => (RulesProfile::ChessMatchV1, "White", true),
        NativeSide::Chess(Color::Black) => (RulesProfile::ChessMatchV1, "Black", false),
        NativeSide::Shogi(s::Side::Sente) => (RulesProfile::ShogiCsa36V1, "Sente", true),
        NativeSide::Shogi(s::Side::Gote) => (RulesProfile::ShogiCsa36V1, "Gote", false),
        NativeSide::Xiangqi(mshogi_core::xiangqi::Side::Red) => {
            (RulesProfile::XiangqiWxf2018V1, "Red", true)
        }
        NativeSide::Xiangqi(mshogi_core::xiangqi::Side::Black) => {
            (RulesProfile::XiangqiWxf2018V1, "Black", false)
        }
    };
    PresentedSide {
        id,
        key: format!("{}:{}", profile.id(), name.to_lowercase()),
        label: name.into(),
        first_mover: first,
    }
}
/// Semantic native piece presentation; glyphs are optional.
#[derive(Clone, Debug, Serialize)]
pub struct PresentedPiece {
    /// Native board coordinate, used only as an opaque presentation key.
    pub square: String,
    /// Profile-qualified piece/resource owner key.
    pub owner: String,
    /// Native semantic piece kind, or tagged operation category.
    pub kind: String,
    /// Native current promoted state; never inferred from a glyph.
    pub promoted: bool,
    /// Readable semantic piece abbreviation, including native promotion.
    pub text: String,
    /// Native human-readable description.
    pub label: String,
}
/// Native hand resource with actual count and opaque selection identity.
#[derive(Clone, Debug, Serialize)]
pub struct PresentedHand {
    /// Native board/hand origin category or opaque hand selection key.
    pub origin: String,
    /// Profile-qualified piece/resource owner key.
    pub owner: String,
    /// Native semantic piece kind, or tagged operation category.
    pub kind: String,
    /// Actual native hand inventory count, including zero.
    pub count: u8,
    /// Readable semantic piece abbreviation, including native promotion.
    pub text: String,
    /// Native human-readable description.
    pub label: String,
}
/// Complete actions have opaque identity, an origin (board or hand), ordered route
/// and explicit completion. Distinct actions can share every route coordinate.
#[derive(Clone, Debug, Serialize)]
pub struct PresentedChoice {
    /// Native board/hand origin category or opaque hand selection key.
    pub origin: ChoiceOrigin,
    /// Opaque complete native action identifier.
    pub action: String,
    /// Opaque board/hand selection key; not a partial command.
    pub from: String,
    /// Ordered route destinations; equal endpoints need not mean equal actions.
    pub steps: Vec<String>,
    /// Native explicit completion choice, including promotion or decline.
    pub completion: Option<Completion>,
}
/// Native origin category; opaque selection keys do not determine its meaning.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ChoiceOrigin {
    /// Native intersection occupied by the moving piece (not a square cell).
    Intersection {
        /// Native intersection coordinate.
        point: String,
    },
    /// Board square occupied by the moving piece.
    Board {
        /// Native board coordinate.
        square: String,
    },
    /// Native hand resource owned by a qualified side.
    Hand {
        /// Profile-qualified hand owner.
        owner: String,
        /// Unpromoted native hand piece kind.
        piece_kind: String,
    },
}
/// Explicit native action completion option.
#[derive(Clone, Debug, Serialize)]
pub struct Completion {
    /// Native human-readable description.
    pub label: String,
    /// Optional decorative glyph; the label is always available.
    pub glyph: Option<String>,
}
/// Native geometry metadata; no rules or coordinate parsing in the webview.
#[derive(Clone, Debug, Serialize)]
pub struct BoardDescriptor {
    /// Explicit native intersection geometry where applicable; absent for existing square games.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersections: Option<IntersectionTopology>,
    /// File coordinates in display order.
    pub files: Vec<String>,
    /// Rank coordinates in display order.
    pub ranks: Vec<String>,
    /// Native human-readable description.
    pub label: String,
    /// Native initial keyboard focus coordinate.
    pub initial_focus: String,
}
/// Native coordinate order for a named side at the bottom.
#[derive(Clone, Debug, Serialize)]
pub struct Orientation {
    /// Native side and presentation labels for this orientation.
    pub side: PresentedSide,
    /// File coordinates in display order.
    pub files: Vec<String>,
    /// Rank coordinates in display order.
    pub ranks: Vec<String>,
}
/// Native-owned operation availability; never inferred from board dimensions.
#[derive(Clone, Debug, Serialize)]
pub struct CommandCapabilities {
    /// Whether the current live human can submit a complete board action.
    pub board_action: bool,
    /// Whether native complete actions support hand drops.
    pub drops: bool,
    /// Whether native choices support promotion completion.
    pub promotion: bool,
    /// Whether an atomic playable human decision can be restored.
    pub take_back: bool,
    /// Whether semantic undo is currently supported.
    pub undo: bool,
    /// Whether live engine-turn execution is available.
    pub engine_turn: bool,
    /// Whether bounded read-only ordinary analysis is supported.
    pub move_analysis: bool,
    /// Whether the separately bounded chess Full operation is supported.
    pub chess_full: bool,
    /// Whether native optional draw claims are currently available.
    pub claims: bool,
    /// Whether a formal native declaration may be submitted; eligibility is advisory.
    pub declaration: bool,
    /// Whether a native resignation may currently be submitted.
    pub resign: bool,
    /// Native hand-resource presentation or capability.
    pub hands: bool,
    /// Whether exact event-prefix inspection is supported.
    pub history: bool,
    /// Whether this active game has desktop archive support.
    pub save: bool,
}
/// Native complete event boundary with independent board-action count.
#[derive(Clone, Debug, Serialize)]
pub struct PresentedEvent {
    /// Native event boundary, separate from board ply.
    pub index: usize,
    /// Known board-action count, excluding terminal command edges.
    pub board_ply: usize,
    /// Native human-readable description.
    pub label: String,
}
/// Advisory native CSA eligibility; formal commands remain distinct.
#[derive(Clone, Debug, Serialize)]
pub struct DeclarationAdvice {
    /// Native command actor; terminal commands retain their native turn semantics.
    pub actor: PresentedSide,
    /// Advisory declaration prerequisite result; does not commit a declaration.
    pub eligible: bool,
    /// Whether the declarer has the move.
    pub own_turn: bool,
    /// Whether the king occupies the native opposing camp.
    pub king_in_camp: bool,
    /// Other owned pieces in the opposing camp.
    pub camp_pieces: u8,
    /// Native camp and hand declaration points.
    pub points: u16,
    /// Native side-specific declaration threshold.
    pub required_points: u16,
    /// Whether the native king is unattacked.
    pub not_in_check: bool,
}
/// Native selected-event status, separate from process and incomplete work.
#[derive(Clone, Debug, Serialize)]
pub struct PresentedStatus {
    /// Whether native adjudication has concluded this selected event prefix.
    pub concluded: bool,
    /// Native human-readable description.
    pub label: String,
    /// Native first/second-side score in half points, absent for unscored work.
    pub score_half_points: Option<[u8; 2]>,
}
/// Actual observation retains the exact native execution context. A score is never
/// reinterpreted as chess mate; declaration distance counts native command edges.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresentedObservation {
    /// Exact submitted context and actual native result.
    pub observation: ExecutionObservation,
    /// Native command admission diagnostic, separate from a game outcome.
    #[serde(deserialize_with = "archive::required_option")]
    pub application_error: Option<String>,
    /// True after reopening; evidence never resumes execution.
    pub historical: bool,
}
/// Native-derived display of a retained observation. The exact original response
/// remains available separately for diagnostics and archival provenance.
#[derive(Clone, Debug, Serialize)]
pub struct PresentedSearchObservation {
    /// Index in the unchanged canonical observations collection.
    pub observation_index: usize,
    /// Explicit execution purpose: engine_turn or move_analysis.
    pub purpose: &'static str,
    /// Native complete event target, independent of board ply.
    pub event_index: usize,
    /// Native known board-action count at the target.
    pub board_ply: usize,
    /// Named native root-side evaluation perspective.
    pub perspective: String,
    /// Actual bundled engine identity in player-facing form.
    pub engine: &'static str,
    /// Native heuristic or terminal-command distance presentation.
    pub score: Option<String>,
    /// Actual last completed iteration, absent for process failure.
    pub completed_depth: Option<u16>,
    /// Actual admitted visits, absent for process failure.
    pub nodes: Option<u64>,
    /// Readable native complete continuation choices.
    pub variation: Vec<String>,
    /// Execution completion/resource stop/failure, separate from game outcome.
    pub status: &'static str,
    /// This observation came from a reopened archive and is historical evidence.
    pub historical: bool,
    /// Exact currently accepted read-only analysis for the selected prefix.
    pub selected: bool,
}
fn display_shogi_choice(choice: &ShogiChoice) -> String {
    match choice {
        ShogiChoice::Move {
            action: s::Action::Move { from, to, promote },
        } => format!("{from} → {to}{}", if *promote { " · promote" } else { "" }),
        ShogiChoice::Move {
            action: s::Action::Drop { kind, to },
        } => format!("Drop {} at {to}", format!("{kind:?}").to_lowercase()),
        ShogiChoice::Declare { actor } => format!(
            "{} declares entering king",
            side(NativeSide::Shogi(*actor)).label
        ),
    }
}
fn display_shogi_observation(
    index: usize,
    observation: &PresentedObservation,
    current: Option<&PresentedObservation>,
) -> PresentedSearchObservation {
    let context = &observation.observation.context;
    let (purpose, event_index, board_ply) = match &context.purpose {
        ExecutionPurpose::EngineTurn {} => (
            "engine_turn",
            context.game.event_count(),
            context.game.known_board_actions(),
        ),
        ExecutionPurpose::MoveAnalysis { target } => (
            "move_analysis",
            target.event_index as usize,
            target.board_ply as usize,
        ),
        ExecutionPurpose::ChessFullReachability { .. } => {
            unreachable!("native shogi admission rejects chess Full")
        }
    };
    let root_side = match &context.game {
        CanonicalRecord::Shogi(record) => record.side_to_move(),
        _ => unreachable!("shogi observation context"),
    };
    let telemetry = match &observation.observation.result {
        ExecutionResult::Shogi {
            result:
                ShogiResult::Choice { telemetry, .. }
                | ShogiResult::Finished { telemetry, .. }
                | ShogiResult::Incomplete { telemetry },
        } => Some(telemetry),
        ExecutionResult::Failure { .. } => None,
        ExecutionResult::Chess { .. } | ExecutionResult::Xiangqi { .. } => {
            unreachable!("native shogi response admission")
        }
    };
    PresentedSearchObservation {
        observation_index: index,
        purpose,
        event_index,
        board_ply,
        perspective: side(NativeSide::Shogi(root_side)).label,
        engine: "mShogi",
        score: telemetry.and_then(|t| t.score_label()),
        completed_depth: telemetry.map(|t| t.completed_depth),
        nodes: telemetry.map(|t| t.nodes),
        variation: telemetry.map_or_else(Vec::new, |t| {
            t.principal_variation
                .iter()
                .map(display_shogi_choice)
                .collect()
        }),
        status: if observation.application_error.is_some() {
            "Rejected engine result"
        } else {
            match telemetry.map(|t| t.status) {
                Some(mshogi_engine::StopReason::Complete) => "Complete",
                Some(mshogi_engine::StopReason::NodeLimit) => "Node limit reached",
                Some(mshogi_engine::StopReason::Stopped) => "Analysis stopped",
                None => "Engine failure",
            }
        },
        historical: observation.historical,
        selected: !observation.historical
            && current.is_some_and(|c| c.observation == observation.observation),
    }
}
fn display_chess_observation(
    index: usize,
    observation: &PresentedObservation,
    current: Option<&PresentedObservation>,
    root_side: Color,
) -> PresentedSearchObservation {
    let context = &observation.observation.context;
    let (purpose, event_index, board_ply) = match &context.purpose {
        ExecutionPurpose::EngineTurn {} => (
            "engine_turn",
            context.game.event_count(),
            context.game.known_board_actions(),
        ),
        ExecutionPurpose::MoveAnalysis { target } => (
            "move_analysis",
            target.event_index as usize,
            target.board_ply as usize,
        ),
        ExecutionPurpose::ChessFullReachability { target, .. } => (
            "chess_full_reachability",
            target.event_index as usize,
            target.board_ply as usize,
        ),
    };
    let result =
        chess_observation_result(&observation.observation).expect("admitted Chess observation");
    let telemetry = match &result {
        ChessResult::Move { telemetry, .. }
        | ChessResult::Claim { telemetry, .. }
        | ChessResult::Finished { telemetry, .. }
        | ChessResult::Incomplete { telemetry } => Some(telemetry),
        _ => None,
    };
    let status = if observation.application_error.is_some() {
        "Rejected engine result"
    } else {
        match &result {
            ChessResult::Reachability {
                result: MateReachability::Possible { .. },
            } => "Cooperative mate witness",
            ChessResult::Reachability {
                result: MateReachability::Impossible { .. },
            } => "Mating reachability disproved",
            ChessResult::Reachability {
                result: MateReachability::Unknown { .. },
            } => "Unknown mating reachability",
            ChessResult::Failure { .. } => "Engine failure",
            _ => match telemetry.expect("search telemetry").status {
                mshogi_engine::StopReason::Complete => "Complete",
                mshogi_engine::StopReason::NodeLimit => "Node limit reached",
                mshogi_engine::StopReason::Stopped => "Analysis stopped",
            },
        }
    };

    let score = telemetry.and_then(|t| t.score).map(|value| {
        if value.abs() > 900_000 {
            format!(
                "{}mate in {} plies",
                if value < 0 { "−" } else { "" },
                1_000_000 - value.abs()
            )
        } else {
            format!("{:+.2} pawns", f64::from(value) / 100.0)
        }
    });
    PresentedSearchObservation {
        observation_index: index,
        purpose,
        event_index,
        board_ply,
        perspective: side(NativeSide::Chess(root_side)).label,
        engine: "mShogi",
        score,
        completed_depth: telemetry.map(|t| t.completed_depth),
        nodes: telemetry.map(|t| t.nodes),
        variation: telemetry.map_or_else(Vec::new, |t| {
            t.principal_variation
                .iter()
                .map(|choice| match choice {
                    mshogi_engine::EngineChoice::Move { action } => action.to_string(),
                    mshogi_engine::EngineChoice::Claim { claim } => format!(
                        "Claim {:?}{}",
                        claim.reason,
                        claim
                            .intended
                            .map(|a| format!(" by intended {a}"))
                            .unwrap_or_default()
                    ),
                })
                .collect()
        }),
        status,
        historical: observation.historical,
        selected: !observation.historical
            && current.is_some_and(|c| c.observation == observation.observation),
    }
}
/// Typed shared desktop view with exact runtime identity and native context.
#[derive(Clone, Debug, Serialize)]
pub struct SharedSnapshot {
    /// Native standard notation, exact move/event mapping and selected cue.
    pub moves: MovesProjection,
    /// Native Xiangqi-only warnings, claims and assessment; no square-game inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xiangqi: Option<XiangqiPresentation>,
    /// Exact native replacement precondition for New Game and Open.
    pub open_token: SharedOpenToken,
    /// Authoritative document metadata and game-native settings.
    pub document: SharedDocumentState,
    /// Off-path document operation.
    pub io: IoState,
    /// Pre-commit persistence resource admission.
    pub capacity: CapacityState,
    /// Exact runtime snapshot and selection precondition.
    pub token: SessionToken,
    /// Explicit native game identity.
    pub game: GameKind,
    /// Immutable selected native rules profile identity.
    pub profile: RulesProfile,
    /// Immutable rules/source edition.
    pub edition: &'static str,
    /// Native side controlled by the human.
    pub human: PresentedSide,
    /// Native side to move at the selected event.
    pub turn: PresentedSide,
    /// Native side identities and labels in first-mover order.
    pub sides: Vec<PresentedSide>,
    /// Native geometry descriptor; explicit intersection topology where applicable.
    pub board: BoardDescriptor,
    /// Native display ordering for each side at the bottom.
    pub orientations: Vec<Orientation>,
    /// Native semantic piece presentation at the selected event.
    pub pieces: Vec<PresentedPiece>,
    /// Native hand-resource presentation or capability.
    pub hands: Vec<PresentedHand>,
    /// Complete native legal actions; capability gates live submission.
    pub choices: Vec<PresentedChoice>,
    /// Native-owned supported commands and features.
    pub capabilities: CommandCapabilities,
    /// Native selected-event conclusion or turn status.
    pub status: PresentedStatus,
    /// Native event timeline, including same-board-ply terminal commands.
    pub events: Vec<PresentedEvent>,
    /// Selected complete event prefix, not a board move number.
    pub event_index: usize,
    /// Known board actions at the selected prefix, distinct from terminal events.
    pub board_ply: u64,
    /// Actual native budget used for ordinary engine turns, not analysis defaults.
    pub play_limits: WorkLimits,
    /// Whether viewing a read-only earlier event prefix.
    pub historical: bool,
    /// Whether explicit Resume is required before live engine work.
    pub paused: bool,
    /// Execution lifecycle, independent of native game outcome.
    pub phase: EnginePhase,
    /// Actual currently owned request purpose, never inferred from turn or pause.
    pub execution_purpose: Option<&'static str>,
    /// Bounded execution/application diagnostic, never a chess or shogi result.
    pub diagnostic: Option<String>,
    /// Read-only eligibility evidence for each native side.
    pub declaration_advice: Vec<DeclarationAdvice>,
    /// Authoritative native canonical record. Xiangqi additionally distinguishes an immutable
    /// intra-command selection from a committed canonical prefix in its native presentation.
    pub context: CanonicalRecord,
    /// Read-only observation bound to its exact selected target.
    pub analysis: Option<PresentedObservation>,
    /// Accepted historical observations with their exact contexts.
    pub observations: Vec<PresentedObservation>,
    /// Readable actual search observations, with native score/continuation meaning.
    pub search_observations: Vec<PresentedSearchObservation>,
    /// Native evaluation perspective and terminal-distance interpretation.
    pub score_meaning: &'static str,
    /// Native Chess capability details; common application state lives above.
    pub chess: Option<ChessPresentation>,
    /// Complete authoritative Chess match record, including clock/policy/events.
    /// Native GameRecord context above remains separately authoritative for rules/search.
    pub chess_match: Option<MatchRecord>,
}
/// Native Chess capability information, without a second application snapshot.
#[derive(Clone, Debug, Serialize)]
pub struct ChessPresentation {
    /// Current optional claims. An intended action is not played by claiming.
    pub claims: Vec<EngineClaim>,
    /// Current authoritative live conclusion, including unresolved outcomes.
    pub live_conclusion: Option<MatchConclusion>,
    /// Conclusion at the selected exact event prefix.
    pub selected_conclusion: Option<MatchConclusion>,
    /// Native check status at the selected position.
    pub in_check: bool,
    /// Actual read-only analysis, including Full proof/Unknown and exact context.
    pub analysis: AnalysisState,
    /// Latest actual engine result, with native score meaning.
    pub last_engine_result: Option<ChessResult>,
    /// Side whose position was searched for the latest engine result.
    pub last_engine_turn: Option<Color>,
    /// Exact live version searched by that result.
    pub searched_version: Option<u64>,
    /// Explicit interactive adjudication mode.
    pub rules_mode: &'static str,
}
/// The request still carries a mandatory game tag, even for common operations.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SharedDesktopCommand {
    /// Complete current runtime precondition, required for every shared command.
    pub expected: SessionToken,
    /// Explicit native game identity.
    pub game: GameKind,
    /// Typed native/common operation; game-specific commands retain admission ownership.
    pub operation: SessionOperation,
}
impl SharedDesktopCommand {
    /// Parse the complete bounded command from raw JSON; retain duplicate native
    /// record fields for the authoritative strict canonical parser.
    pub fn from_json(json: &str) -> Result<Self, String> {
        if json.len() > mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES + 4096 {
            return Err("Native contract command exceeds its encoded capacity.".into());
        }
        serde_json::from_str(json).map_err(|e| {
            let mut message = e.to_string();
            let mut end = message
                .len()
                .min(mshogi_core::native::MAX_RECORD_DIAGNOSTIC_BYTES);
            while !message.is_char_boundary(end) {
                end -= 1;
            }
            message.truncate(end);
            message
        })
    }
}
/// Small concrete dispatch across three native games; product admission is separate.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SessionOperation {
    /// Explicit replacement approval is tied to the entire current token. Records
    /// use native strict parsing (including duplicate fields); not a desktop archive.
    Replace {
        /// Strict authoritative native canonical record; not a desktop archive.
        record: CanonicalRecord,
        /// Native side controlled by the human.
        human: NativeSide,
    },
    /// Complete native Chess board action, admitted under the outer token.
    ChessPlay {
        /// Complete authoritative action.
        action: Action,
    },
    /// Native optional claim; intended actions are claimed without being played.
    ChessClaim {
        /// Native claimant, reason and optional intended action.
        claim: EngineClaim,
    },
    /// Native Chess document preferences, retaining Full analysis defaults.
    ChessPreferences {
        /// Native orientation.
        orientation: Color,
        /// Purpose-specific analysis defaults.
        analysis_defaults: AnalysisOperation,
    },
    /// Native Xiangqi command; actor is the assigned human, not inferred from turn.
    Xiangqi {
        /// Authoritative native command including board actions, claims and agreement.
        command: mshogi_core::xiangqi::GameCommand,
    },
    /// Native document preferences; does not change human ownership or execution.
    XiangqiPreferences {
        /// Persisted orientation and ordinary analysis defaults.
        settings: XiangqiSettings,
    },
    /// Shogi complete action is admitted by Game::submit, never reconstructed in JS.
    Play {
        /// Opaque complete native action identifier.
        action: s::Action,
    },
    /// Save native shogi desktop preferences; no rules are computed by the client.
    ShogiPreferences {
        /// Supported native shogi settings.
        settings: ShogiSettings,
    },
    /// Submit formal native declaration, distinct from advisory eligibility.
    Declare {},
    /// Submit native human resignation.
    Resign {},
    /// Undo the last accepted native event while retiring all execution work.
    Undo {},
    /// Restore the preceding human decision while retiring all execution work.
    TakeBack {},
    /// Select a read-only native event prefix.
    Inspect {
        /// Selected complete event prefix, not a board move number.
        event_index: usize,
    },
    /// Explicitly return to live with execution still paused.
    ReturnLive {},
    /// Replace execution with bounded read-only analysis of the selected prefix.
    Analyze {
        /// Typed native/common operation; game-specific commands retain admission ownership.
        operation: AnalysisOperation,
    },
    /// Retire pending work without a fallback move or outcome.
    Cancel {},
    /// Explicitly resume live play after inspection or analysis.
    Resume {},
    /// Start fresh execution after incomplete or failed work.
    Retry {},
}
#[derive(Deserialize)]
#[serde(
    remote = "SessionOperation",
    tag = "kind",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum OperationWire {
    // Explicit replacement approval is tied to the entire current token. Records
    // use native strict parsing (including duplicate fields); not a desktop archive.
    Replace {
        // Strict authoritative native canonical record; not a desktop archive.
        record: CanonicalRecord,
        // Native side controlled by the human.
        human: NativeSide,
    },
    ChessPlay {
        action: Action,
    },
    ChessClaim {
        claim: EngineClaim,
    },
    ChessPreferences {
        orientation: Color,
        analysis_defaults: AnalysisOperation,
    },
    Xiangqi {
        command: mshogi_core::xiangqi::GameCommand,
    },
    XiangqiPreferences {
        settings: XiangqiSettings,
    },
    // Shogi complete action is admitted by Game::submit, never reconstructed in JS.
    Play {
        // Opaque complete native action identifier.
        action: s::Action,
    },
    // Submit a formal entering-king declaration, including its native false-declaration semantics.
    ShogiPreferences {
        settings: ShogiSettings,
    },
    Declare {},
    // Submit native human resignation.
    Resign {},
    // Undo the last accepted native event while retiring all execution work.
    Undo {},
    /// Restore the preceding human decision while retiring all execution work.
    TakeBack {},
    // Select a read-only native event prefix.
    Inspect {
        // Selected complete event prefix, not a board move number.
        event_index: usize,
    },
    // Explicitly return to live with execution still paused.
    ReturnLive {},
    // Replace execution with bounded read-only analysis of the selected prefix.
    Analyze {
        // Typed native/common operation; game-specific commands retain admission ownership.
        operation: AnalysisOperation,
    },
    // Retire pending work without a fallback move or outcome.
    Cancel {},
    // Explicitly resume live play after inspection or analysis.
    Resume {},
    // Start fresh execution after incomplete or failed work.
    Retry {},
}
impl<'de> Deserialize<'de> for SessionOperation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(deserializer)?;
        #[derive(Deserialize)]
        struct Header {
            kind: String,
        }
        let header: Header = serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        if header.kind == "replace" {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Replacement {
                kind: String,
                record: CanonicalRecord,
                human: NativeSide,
            }
            let wire: Replacement =
                serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
            debug_assert_eq!(wire.kind, "replace");
            Ok(Self::Replace {
                record: wire.record,
                human: wire.human,
            })
        } else {
            OperationWire::deserialize(&mut serde_json::Deserializer::from_str(raw.get()))
                .map_err(serde::de::Error::custom)
        }
    }
}

#[derive(Clone)]
pub(super) struct ShogiSession {
    initial_record: s::GameRecord,
    initial_settings: ShogiSettings,
    settings: ShogiSettings,
    controls: Vec<archive::ShogiControl>,
    analysis_intent: Option<(CanonicalRecord, WorkLimits)>,
    durable_needs_stop: bool,
    execution_floor: u64,
    game: s::Game,
    human: s::Side,
    viewed: Option<s::Game>,
    pub(crate) paused: bool,
    pending: Option<ExecutionContext>,
    submitted: bool,
    phase: EnginePhase,
    diagnostic: Option<String>,
    analysis: Option<PresentedObservation>,
    observations: Vec<PresentedObservation>,
}
type ShogiRollback = (
    bool,
    EnginePhase,
    Option<String>,
    Option<PresentedObservation>,
    bool,
);
#[derive(Default)]
pub(super) struct SharedState {
    pub moves_cache: std::cell::RefCell<crate::moves::Cache>,
    pub admission: Option<(u64, bool)>,
    pub admission_consumes_response: bool,
    pub rollback: Option<ShogiRollback>,
    #[cfg(test)]
    pub manual_engine: bool,
}
impl SharedState {
    fn auto_submit(&self) -> bool {
        #[cfg(test)]
        if self.manual_engine {
            return false;
        }
        true
    }
}
impl ShogiSession {
    pub(crate) fn reopened(&mut self, floor: u64) {
        self.retire();
        self.paused = true;
        self.execution_floor = floor;
        for observation in &mut self.observations {
            observation.historical = true;
        }
        self.diagnostic=Some("Document reopened with work stopped. Saved observations are historical; Resume explicitly.".into());
    }
    fn take_back_target(&self) -> Option<usize> {
        let events = self.game.events();
        if let Some(last) = events.last()
            && last.actor == self.human
            && matches!(
                last.command,
                s::GameCommand::Resign | s::GameCommand::DeclareEnteringKing
            )
            && last.conclusion.is_some()
            && self.game.position().side_to_move() == self.human
            && !self.game.position().legal_actions().is_empty()
        {
            return Some(last.index);
        }
        events.iter().rposition(|event| {
            event.actor == self.human && matches!(event.command, s::GameCommand::Play(_))
        })
    }
    fn selected(&self) -> &s::Game {
        self.viewed.as_ref().unwrap_or(&self.game)
    }
    pub(crate) fn retire(&mut self) {
        self.pending = None;
        self.submitted = false;
        self.analysis_intent = None;
        self.analysis = None;
        self.phase = EnginePhase::Paused;
        self.diagnostic = None;
    }
    fn record(&self) -> CanonicalRecord {
        CanonicalRecord::shogi(s::GameRecord::from_game(self.selected()))
    }
}
fn piece_text(kind: s::PieceKind) -> &'static str {
    match kind {
        s::PieceKind::King => "K",
        s::PieceKind::Rook => "R",
        s::PieceKind::Bishop => "B",
        s::PieceKind::Gold => "G",
        s::PieceKind::Silver => "S",
        s::PieceKind::Knight => "N",
        s::PieceKind::Lance => "L",
        s::PieceKind::Pawn => "P",
    }
}
fn command_label(command: s::GameCommand) -> String {
    match command {
        s::GameCommand::Play(action) => action.to_string(),
        s::GameCommand::Resign => "Resignation".into(),
        s::GameCommand::DeclareEnteringKing => "Formal entering-king declaration".into(),
    }
}
fn shogi_status(game: &s::Game) -> PresentedStatus {
    let label = match game.conclusion() {
        None => format!(
            "{} to move{}",
            side(NativeSide::Shogi(game.position().side_to_move())).label,
            if game.position().is_in_check(game.position().side_to_move()) {
                " · check"
            } else {
                ""
            }
        ),
        Some(s::Conclusion::Win { winner, reason }) => {
            let winner_label = side(NativeSide::Shogi(winner)).label;
            let loser_label = side(NativeSide::Shogi(winner.opponent())).label;
            match reason {
                s::WinReason::NoLegalAction { in_check: true } => {
                    format!("{winner_label} wins by checkmate")
                }
                s::WinReason::NoLegalAction { in_check: false } => {
                    format!("{winner_label} wins: opponent has no legal action")
                }
                s::WinReason::ContinuousCheck { .. } => {
                    format!("{loser_label} loses by continuous check")
                }
                s::WinReason::Resignation => {
                    format!("{loser_label} resigned · {winner_label} wins")
                }
                s::WinReason::EnteringKing => {
                    format!("{winner_label} wins by entering-king declaration")
                }
                s::WinReason::FalseDeclaration => {
                    format!("{loser_label} loses by false entering-king declaration")
                }
            }
        }
        Some(s::Conclusion::Draw(reason)) => match reason {
            s::DrawReason::Fourfold { .. } => "Draw by fourfold repetition".into(),
            s::DrawReason::Length => "Draw at the 320-action game limit".into(),
        },
    };
    PresentedStatus {
        concluded: game.conclusion().is_some(),
        label,
        score_half_points: game.conclusion().map(|c| c.score_half_points()),
    }
}
impl DesktopController {
    /// Inspect the explicit active game without starting work or polling the wrong
    /// wire family. The enclosing Tauri mutex serializes all these entry points.
    pub fn shared_token(&self) -> SessionToken {
        SessionToken {
            document: self.session.document_epoch,
            version: self.session.version,
            revision: self.session.revision,
            selection: self.session.selection_id,
        }
    }
    /// Start a standard game through persistence admission. Native-qualified human
    /// assignment is recorded; orientation initially places that player at bottom.
    /// The exact replacement token is checked before retiring any accepted state.
    pub fn shared_new_game(
        &mut self,
        expected: SharedOpenToken,
        game: GameKind,
        human: NativeSide,
    ) -> Result<(SharedSnapshot, AdmissionReceipt), String> {
        if self.lifecycle.close_requested() {
            return Err(
                "Close is pending. Finish or cancel close before replacing the document.".into(),
            );
        }
        if expected != self.shared_open_token() {
            return Err("The document changed. Review unsaved changes again.".into());
        }
        if human.game() != game {
            return Err("The selected player side does not belong to this game.".into());
        }
        self.ensure_runtime_capacity()?;
        if self.session.document_epoch >= MAX_SAFE_ID - 64 {
            return Err("Application capacity: document identity exhausted.".into());
        }
        let receipt = match human {
            NativeSide::Xiangqi(human) => {
                use mshogi_core::xiangqi as x;
                let game =
                    x::Game::standard(x::ClaimPolicy::Explicit).map_err(|e| e.to_string())?;
                let record = x::GameRecord::from_game(&game).map_err(|e| e.to_string())?;
                let candidate = xiangqi::archive::XiangqiDraft::replacement(&record, human);
                self.queue_xiangqi_candidate(
                    candidate,
                    Some(SessionOperation::Resume {}),
                    None,
                    None,
                    true,
                )?
            }
            NativeSide::Shogi(human) => {
                let record = s::GameRecord::from_game(&s::Game::new());
                self.queue_shogi_seed(
                    archive::ShogiSeed::Record {
                        record: Box::new(record),
                        human,
                        orient_human: true,
                    },
                    Some(SessionOperation::Resume {}),
                    None,
                    true,
                )?
            }
            NativeSide::Chess(human) => {
                let record = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record();
                self.queue_chess_replacement(record, human, false)?
            }
        };
        self.last_command_receipt = Some(receipt.clone());
        Ok((self.shared_snapshot(), receipt))
    }
    fn queue_chess_replacement(
        &mut self,
        record: GameRecord,
        human: Color,
        paused: bool,
    ) -> Result<AdmissionReceipt, String> {
        self.retire_admission();
        self.bridge.cancel();
        self.retire_native_files()?;
        self.session.retire_execution();
        let mut candidate = DesktopSession::new();
        candidate.runtime = std::ops::Deref::deref(&self.session).clone();
        self.queue_chess_candidate(
            candidate,
            Some(persistence::ChessMutation::Replace {
                record,
                human,
                paused,
            }),
        )?;
        self.shared.admission = Some((self.session.document_epoch, false));
        Ok(self
            .admission_receipt
            .clone()
            .expect("queued replacement receipt"))
    }
    fn shared_changed(&mut self) {
        self.session.version += 1;
        self.session.revision += 1;
        self.session.selection_id += 1;
        self.session.document_revision += 1;
        self.session.archive_revision = self.session.revision.min(MAX_SAFE_ID - 65);
    }
    pub(super) fn require_chess(&self) -> Result<(), String> {
        if self.session.shogi().is_some() || self.session.xiangqi().is_some() {
            Err("Wrong game: a Chess mutation cannot act on another active native game.".into())
        } else {
            Ok(())
        }
    }
    /// Apply a native-qualified operation. Cancellation/replacement is processed
    /// before polling, so an already completed retired response cannot win a race.
    pub fn shared_command(
        &mut self,
        command: SharedDesktopCommand,
    ) -> Result<SharedSnapshot, String> {
        if self.lifecycle.close_requested() {
            return Err(
                "Close is pending. Finish or cancel close before changing the document.".into(),
            );
        }
        if command.expected != self.shared_token() {
            return Err(
                "The session or selected event changed; use its current native snapshot.".into(),
            );
        }
        let active = if self.session.xiangqi().is_some() {
            GameKind::Xiangqi
        } else if self.session.shogi().is_some() {
            GameKind::Shogi
        } else {
            GameKind::Chess
        };
        if command.game != active {
            return Err("Command game does not match the active native session.".into());
        }
        self.ensure_runtime_capacity()?;
        if self.session.document_epoch >= MAX_SAFE_ID - 64 {
            return Err("Application capacity: document identity exhausted.".into());
        }
        if let SessionOperation::Replace { record, human } = command.operation {
            if record.game_kind() != human.game() {
                return Err("Replacement side and game disagree.".into());
            }
            // Validate before retiring any current state or work.
            if let CanonicalRecord::Xiangqi(record) = &record {
                let NativeSide::Xiangqi(human) = human else {
                    unreachable!()
                };
                return self.replace_xiangqi(record, human);
            }
            if let CanonicalRecord::Shogi(record) = record {
                let NativeSide::Shogi(human) = human else {
                    unreachable!()
                };
                let receipt = self.queue_shogi_seed(
                    archive::ShogiSeed::Record {
                        record,
                        human,
                        orient_human: false,
                    },
                    None,
                    None,
                    true,
                )?;
                self.last_command_receipt = Some(receipt);
            } else if let CanonicalRecord::Chess(record) = record {
                let NativeSide::Chess(human) = human else {
                    unreachable!()
                };
                let receipt = self.queue_chess_replacement(*record, human, true)?;
                self.last_command_receipt = Some(receipt);
            } else {
                return Err("Unsupported desktop game; the current document was preserved.".into());
            }
            return Ok(self.shared_snapshot());
        }
        if self.session.xiangqi().is_some() {
            if self.admission_token.is_some() {
                if !matches!(command.operation, SessionOperation::Cancel {}) {
                    return Err(
                        "Wait for native replacement admission before another command.".into(),
                    );
                }
                self.retire_admission();
            }
            self.last_command_receipt = None;
            return self.xiangqi_command(command.operation);
        }
        if self.session.shogi().is_none() {
            let native = match command.operation {
                SessionOperation::ChessPlay { action } => ChessMutationCommand::Play {
                    version: self.session.version,
                    action,
                },
                SessionOperation::ChessClaim { claim } => ChessMutationCommand::Claim {
                    version: self.session.version,
                    claim,
                },
                SessionOperation::ChessPreferences {
                    orientation,
                    analysis_defaults,
                } => ChessMutationCommand::Preferences {
                    orientation,
                    analysis_defaults,
                },
                SessionOperation::Analyze { operation } => ChessMutationCommand::Analyze {
                    version: self.session.version,
                    operation,
                },
                SessionOperation::Inspect { event_index } => ChessMutationCommand::Inspect {
                    version: self.session.version,
                    event_index,
                },
                SessionOperation::ReturnLive {} => ChessMutationCommand::ReturnLive {
                    version: self.session.version,
                },
                SessionOperation::Cancel {} => {
                    if self.session.chess_mut().analysis_pending.is_some() {
                        ChessMutationCommand::CancelAnalysis {
                            version: self.session.version,
                        }
                    } else {
                        ChessMutationCommand::Cancel {
                            version: self.session.version,
                        }
                    }
                }
                SessionOperation::Resume {} => ChessMutationCommand::Resume {
                    version: self.session.version,
                },
                SessionOperation::Retry {} => ChessMutationCommand::Retry {
                    version: self.session.version,
                },
                SessionOperation::TakeBack {} => ChessMutationCommand::TakeBack {
                    version: self.session.version,
                },
                SessionOperation::Resign {} => ChessMutationCommand::Resign {
                    version: self.session.version,
                },
                _ => {
                    return Err(
                        "Unsupported chess operation; use its native complete command.".into(),
                    );
                }
            };
            self.command(native)?;
            return Ok(self.shared_snapshot());
        }
        if self.admission_token.is_some()
            && !matches!(command.operation, SessionOperation::Cancel {})
        {
            return Err("Wait for native persistence admission before another command.".into());
        }
        let state = self.session.shogi().unwrap();
        if matches!(command.operation, SessionOperation::TakeBack {})
            && (state.viewed.is_some() || state.take_back_target().is_none())
        {
            return Err("No live playable human decision is available for Take back.".into());
        }
        let candidate = state.clone();
        let receipt =
            self.queue_native_candidate(candidate, Some(command.operation), None, false)?;
        self.last_command_receipt = Some(receipt);
        Ok(self.shared_snapshot())
    }
    fn start_shogi(&mut self, limits: WorkLimits, analysis: bool) -> Result<(), OperationFailure> {
        self.ensure_runtime_capacity()?;
        self.session.next_request += 1;
        let state = self.session.shogi().unwrap();
        if state.observations.len() >= 256 {
            return Err(resource_refusal(
                vec![
                    resource_fact(
                        ResourceCategory::OrdinaryObservations,
                        state.observations.len(),
                        MAX_ARCHIVE_OBSERVATIONS,
                        0,
                        ResourceScope::Shogi,
                        ResourceUnit::Observations,
                    )
                    .with_measurement(ResourceMeasurement::Accepted),
                ],
                "Application capacity: all256 retained observation slots are occupied. Current document remains saveable; start a new document to continue.",
            ));
        }
        let game = state.record();
        let purpose = if analysis {
            ExecutionPurpose::MoveAnalysis {
                target: ExecutionTarget {
                    selection_id: self.session.selection_id,
                    event_index: game.event_count() as u64,
                    board_ply: game.known_board_actions() as u64,
                    session_version: self.session.version,
                    label: format!(
                        "Shogi event {}{}",
                        game.event_count(),
                        if state.viewed.is_some() {
                            " (history)"
                        } else {
                            " (live)"
                        }
                    ),
                    turn: NativeSide::Shogi(state.selected().position().side_to_move()),
                },
            }
        } else {
            ExecutionPurpose::EngineTurn {}
        };
        let request = ExecutionContext {
            purpose,
            request_id: self.session.next_request,
            session_version: self.session.version,
            process_generation: self.session.next_request,
            game,
            limits,
        };
        let selection_id = self.session.selection_id;
        let state = self.session.shogi_mut().unwrap();
        state.execution_floor = state
            .execution_floor
            .max(request.request_id)
            .max(request.session_version)
            .max(selection_id);
        state.pending = Some(request);
        state.submitted = false;
        state.phase = EnginePhase::Thinking;
        Ok(())
    }
    /// Exact acceptance, separate read-only observation path and native command admission.
    pub fn accept_shared(&mut self, response: ExecutionResponse) -> bool {
        if response.protocol != mshogi_engine::PROTOCOL_VERSION {
            return false;
        }
        self.accept_observation(response.observation)
    }
    pub(crate) fn accept_observation(&mut self, response: ExecutionObservation) -> bool {
        if self.session.xiangqi().is_some() {
            return self.accept_xiangqi_observation(response);
        }
        let Some(state) = self.session.shogi() else {
            return false;
        };
        if self.session.ensure_capacity().is_err() || state.observations.len() >= 256 {
            return false;
        }
        let Some(pending) = &state.pending else {
            return false;
        };
        if !response.matches(pending)
            || pending.session_version != self.session.version
            || pending.game != state.record()
        {
            return false;
        }
        let candidate = state.clone();
        self.queue_native_candidate(candidate, None, Some(response), false)
            .is_ok()
    }
    /// Poll only the active game and return its native presentation and provenance.
    pub fn shared_snapshot(&mut self) -> SharedSnapshot {
        self.poll_native_admission();
        self.poll_files();
        if self.session.xiangqi().is_some() {
            return if self.shared.admission.is_some() {
                self.xiangqi_snapshot()
            } else {
                self.poll_xiangqi()
            };
        }
        if self.session.shogi().is_some() {
            self.poll_files();
            if self.session.shogi().is_none() {
                return self.shared_snapshot();
            }
            if self.admission_token.is_none()
                && let Some(response) = self.bridge.poll()
            {
                self.accept_shared(response);
            }
            let state = self.session.shogi().unwrap();
            if self.admission_token.is_none()
                && state.pending.is_none()
                && !state.paused
                && state.viewed.is_none()
                && state.game.conclusion().is_none()
                && state.game.position().side_to_move() != state.human
                && let Err(e) = self.start_shogi(SHOGI_PLAY_LIMITS, false)
            {
                self.pause_native_failure(e);
            }
            let auto_submit = self.shared.auto_submit();
            let state = self.session.shogi_mut().unwrap();
            if self.admission_token.is_none()
                && auto_submit
                && let Some(request) = state.pending.clone()
                && !state.submitted
            {
                state.submitted = true;
                if let Err(e) = self.bridge.submit(execution_request(request.clone())) {
                    self.accept_observation(ExecutionObservation {
                        schema_version: OBSERVATION_SCHEMA_VERSION,
                        context: request,
                        result: ExecutionResult::Failure {
                            failure: mshogi_engine::EngineFailure {
                                kind: mshogi_engine::FailureKind::Start,
                                message: format!("Cannot start native work: {e:?}"),
                            },
                        },
                    });
                }
            }
            return self.shogi_snapshot();
        }
        self.poll_chess();
        // The native poll may commit a cross-game Open. Never publish its Chess
        // projection with the replacement Shogi epoch: equal-token freshness
        // rejection would then hide the correct Shogi view indefinitely.
        if self.session.shogi().is_some() || self.session.xiangqi().is_some() {
            return self.shared_snapshot();
        }
        let mut snapshot = self.session.snapshot();
        snapshot.io = self.io_state.clone();
        self.chess_snapshot(snapshot)
    }
    pub(super) fn pause_shared_for_close(&mut self) {
        self.retire_admission();
        if let Some(state) = self.session.xiangqi_mut()
            && state.close_pause()
        {
            self.bridge.cancel();
            self.shared_changed();
        }
        if let Some(state) = self.session.shogi_mut()
            && (state.pending.is_some() || !state.paused || state.analysis_intent.is_some())
        {
            state.retire();
            state.paused = true;
            state.control(None, None, true);
            self.bridge.cancel();
            self.shared_changed();
        }
    }
    fn shogi_snapshot(&self) -> SharedSnapshot {
        let state = self.session.shogi().unwrap();
        let game = state.selected();
        let position = game.position();
        let sides = vec![
            side(NativeSide::Shogi(s::Side::Sente)),
            side(NativeSide::Shogi(s::Side::Gote)),
        ];
        let files: Vec<String> = (1..=9).rev().map(|n| n.to_string()).collect();
        let ranks: Vec<String> = (b'a'..=b'i').map(|c| char::from(c).to_string()).collect();
        let pieces = s::Square::all()
            .filter_map(|square| {
                position.piece(square).map(|p| {
                    let owner = side(NativeSide::Shogi(p.side));
                    let text = format!(
                        "{}{}",
                        if p.promoted { "+" } else { "" },
                        piece_text(p.kind)
                    );
                    PresentedPiece {
                        square: square.to_string(),
                        owner: owner.key,
                        kind: format!("{:?}", p.kind).to_lowercase(),
                        promoted: p.promoted,
                        label: format!("{square}, {} {text}", owner.label),
                        text,
                    }
                })
            })
            .collect();
        let mut hands = vec![];
        for actor in [s::Side::Sente, s::Side::Gote] {
            for kind in s::PieceKind::HAND_KINDS {
                let owner = side(NativeSide::Shogi(actor));
                let text = piece_text(kind).to_string();
                let count = position.hand(actor).count(kind);
                hands.push(PresentedHand {
                    origin: format!("hand:{}:{text}", owner.key),
                    owner: owner.key,
                    kind: format!("{kind:?}").to_lowercase(),
                    count,
                    label: format!("{} hand {text}, {count}", owner.label),
                    text,
                });
            }
        }
        let legal = position.legal_actions();
        let choices = if game.conclusion().is_none() {
            legal
                .iter()
                .copied()
                .map(|action| {
                    let (from, to, completion) = match action {
                        s::Action::Move { from, to, promote } => {
                            let alternate = s::Action::Move {
                                from,
                                to,
                                promote: !promote,
                            };
                            let optional = legal.contains(&alternate);
                            (
                                from.to_string(),
                                to,
                                if promote || optional {
                                    Some(Completion {
                                        label: if promote { "Promote" } else { "Do not promote" }
                                            .into(),
                                        glyph: None,
                                    })
                                } else {
                                    None
                                },
                            )
                        }
                        s::Action::Drop { kind, to } => (
                            format!(
                                "hand:{}:{}",
                                side(NativeSide::Shogi(position.side_to_move())).key,
                                piece_text(kind)
                            ),
                            to,
                            None,
                        ),
                    };
                    PresentedChoice {
                        origin: match action {
                            s::Action::Move { from, .. } => ChoiceOrigin::Board {
                                square: from.to_string(),
                            },
                            s::Action::Drop { kind, .. } => ChoiceOrigin::Hand {
                                owner: side(NativeSide::Shogi(position.side_to_move())).key,
                                piece_kind: format!("{kind:?}").to_lowercase(),
                            },
                        },
                        action: action.to_string(),
                        from,
                        steps: vec![to.to_string()],
                        completion,
                    }
                })
                .collect()
        } else {
            vec![]
        };
        let mut ply = 0;
        let events = state
            .game
            .events()
            .iter()
            .map(|e| {
                if matches!(e.command, s::GameCommand::Play(_)) {
                    ply += 1;
                }
                PresentedEvent {
                    index: e.index + 1,
                    board_ply: ply,
                    label: format!(
                        "{}: {}",
                        side(NativeSide::Shogi(e.actor)).label,
                        command_label(e.command)
                    ),
                }
            })
            .collect();
        let historical = state.viewed.is_some();
        let ended = state.game.conclusion().is_some();
        let human_turn = state.game.position().side_to_move() == state.human;
        let declaration_advice = [s::Side::Sente, s::Side::Gote]
            .into_iter()
            .map(|actor| {
                let e = game.declaration_eligibility(actor);
                DeclarationAdvice {
                    actor: side(NativeSide::Shogi(actor)),
                    eligible: e.eligible(),
                    own_turn: e.own_turn,
                    king_in_camp: e.king_in_camp,
                    camp_pieces: e.camp_pieces,
                    points: e.points,
                    required_points: e.required_points,
                    not_in_check: e.not_in_check,
                }
            })
            .collect();
        let record = state.record();
        SharedSnapshot {
            moves: self.present_moves(game.events().len()),
            xiangqi: None,
            open_token: self.shared_open_token(),
            document: self.shared_document(),
            io: self.io_state.clone(),
            capacity: self.session.capacity.clone(),
            token: self.shared_token(),
            game: GameKind::Shogi,
            profile: RulesProfile::ShogiCsa36V1,
            edition: RulesProfile::ShogiCsa36V1.edition(),
            human: side(NativeSide::Shogi(state.human)),
            turn: side(NativeSide::Shogi(position.side_to_move())),
            sides: sides.clone(),
            board: BoardDescriptor {
                intersections: None,
                files: files.clone(),
                ranks: ranks.clone(),
                label: "Shogi board".into(),
                initial_focus: "5g".into(),
            },
            orientations: vec![
                Orientation {
                    side: sides[0].clone(),
                    files: files.clone(),
                    ranks: ranks.clone(),
                },
                Orientation {
                    side: sides[1].clone(),
                    files: files.into_iter().rev().collect(),
                    ranks: ranks.into_iter().rev().collect(),
                },
            ],
            pieces,
            hands,
            choices,
            capabilities: CommandCapabilities {
                board_action: !historical && !ended && !state.paused && human_turn,
                drops: true,
                promotion: true,
                take_back: !historical && state.take_back_target().is_some(),
                undo: !historical && !state.game.events().is_empty(),
                engine_turn: !historical && !ended && !human_turn,
                move_analysis: true,
                chess_full: false,
                claims: false,
                declaration: !historical
                    && !ended
                    && state.game.declaration_eligibility(state.human).eligible(),
                resign: !historical && !ended,
                hands: true,
                history: true,
                save: true,
            },
            status: shogi_status(game),
            events,
            event_index: game.events().len(),
            board_ply: record.known_board_actions() as u64,
            play_limits: SHOGI_PLAY_LIMITS,
            historical,
            paused: state.paused,
            phase: state.phase,
            execution_purpose: state.pending.as_ref().map(|request| match request.purpose {
                ExecutionPurpose::EngineTurn {} => "engine_turn",
                ExecutionPurpose::MoveAnalysis { .. } => "move_analysis",
                ExecutionPurpose::ChessFullReachability { .. } => "chess_full_reachability",
            }),
            diagnostic: state.diagnostic.clone(),
            declaration_advice,
            context: record,
            analysis: state.analysis.clone(),
            observations: state.observations.clone(),
            search_observations: state
                .observations
                .iter()
                .enumerate()
                .map(|(index, observation)| {
                    display_shogi_observation(index, observation, state.analysis.as_ref())
                })
                .collect(),
            score_meaning: "Native shogi root-side hundredths of pawn; terminal win distance counts complete command edges, including declarations. Not a chess mate score.",
            chess: None,
            chess_match: None,
        }
    }
    fn chess_snapshot(&self, snapshot: ChessSnapshot) -> SharedSnapshot {
        let sides = vec![
            side(NativeSide::Chess(Color::White)),
            side(NativeSide::Chess(Color::Black)),
        ];
        let files: Vec<String> = (b'a'..=b'h').map(|c| char::from(c).to_string()).collect();
        let ranks: Vec<String> = (1..=8).rev().map(|n| n.to_string()).collect();
        let pieces = snapshot
            .view
            .pieces
            .iter()
            .map(|p| PresentedPiece {
                square: p.square.clone(),
                owner: side(NativeSide::Chess(p.color)).key,
                kind: p.kind.clone(),
                promoted: false,
                text: match p.kind.as_str() {
                    "king" => "K",
                    "queen" => "Q",
                    "rook" => "R",
                    "bishop" => "B",
                    "knight" => "N",
                    _ => "P",
                }
                .into(),
                label: p.label.clone(),
            })
            .collect();
        let choices = snapshot
            .legal_choices
            .iter()
            .map(|c| PresentedChoice {
                origin: ChoiceOrigin::Board {
                    square: c.from.clone(),
                },
                action: c.action.to_string(),
                from: c.from.clone(),
                steps: vec![c.to.clone()],
                completion: c.promotion.as_ref().map(|p| Completion {
                    label: p.clone(),
                    glyph: None,
                }),
            })
            .collect();
        let events = snapshot
            .view
            .timeline
            .iter()
            .map(|e| PresentedEvent {
                index: e.index,
                board_ply: e.ply as usize,
                label: e.label.clone(),
            })
            .collect();
        let concluded = snapshot.view.conclusion.is_some();
        let status = PresentedStatus {
            concluded,
            label: if concluded {
                format!("{:?}", snapshot.view.conclusion.unwrap())
            } else {
                format!(
                    "{} to move",
                    side(NativeSide::Chess(snapshot.view.turn)).label
                )
            },
            score_half_points: snapshot
                .view
                .conclusion
                .and_then(|c| c.white_score_twice())
                .map(|n| [n, 2 - n]),
        };
        let active = !snapshot.view.historical && snapshot.conclusion.is_none();
        let observations: Vec<_> = snapshot
            .observations
            .iter()
            .map(|entry| PresentedObservation {
                observation: entry.observation.clone(),
                application_error: entry.application_error.clone(),
                historical: entry.historical,
            })
            .collect();
        let analysis = observations
            .iter()
            .find(|entry| {
                snapshot.analysis.context.as_ref() == Some(&entry.observation.context)
                    && chess_observation_result(&entry.observation).as_ref()
                        == snapshot.analysis.result.as_ref()
            })
            .cloned();
        let search_observations = observations
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                display_chess_observation(
                    index,
                    entry,
                    analysis.as_ref(),
                    snapshot.observations[index].analyzed_turn,
                )
            })
            .collect();
        SharedSnapshot {
            moves: self.present_moves(snapshot.view.event_index),
            xiangqi: None,
            open_token: self.shared_open_token(),
            document: self.shared_document(),
            io: self.io_state.clone(),
            capacity: self.session.capacity.clone(),
            token: self.shared_token(),
            game: GameKind::Chess,
            profile: RulesProfile::ChessMatchV1,
            edition: RulesProfile::ChessMatchV1.edition(),
            human: side(NativeSide::Chess(snapshot.human)),
            turn: side(NativeSide::Chess(snapshot.view.turn)),
            sides: sides.clone(),
            board: BoardDescriptor {
                intersections: None,
                files: files.clone(),
                ranks: ranks.clone(),
                label: "Chess board".into(),
                initial_focus: "e2".into(),
            },
            orientations: vec![
                Orientation {
                    side: sides[0].clone(),
                    files: files.clone(),
                    ranks: ranks.clone(),
                },
                Orientation {
                    side: sides[1].clone(),
                    files: files.into_iter().rev().collect(),
                    ranks: ranks.into_iter().rev().collect(),
                },
            ],
            pieces,
            hands: vec![],
            choices,
            capabilities: CommandCapabilities {
                board_action: active && !snapshot.play_paused && snapshot.turn == snapshot.human,
                drops: false,
                promotion: true,
                take_back: snapshot.can_take_back,
                undo: false,
                engine_turn: active && snapshot.turn != snapshot.human,
                move_analysis: true,
                chess_full: true,
                claims: active && !snapshot.claims.is_empty(),
                declaration: false,
                resign: active,
                hands: false,
                history: true,
                save: true,
            },
            status,
            events,
            event_index: snapshot.view.event_index,
            board_ply: snapshot.view.ply,
            play_limits: snapshot.limits,
            historical: snapshot.view.historical,
            paused: snapshot.play_paused,
            phase: snapshot.engine.status,
            execution_purpose: self
                .session
                .chess()
                .analysis_pending
                .as_ref()
                .or(self.session.chess().pending.as_ref())
                .map(|request| match request.purpose {
                    mshogi_engine::ExecutionPurpose::EngineTurn {} => "engine_turn",
                    mshogi_engine::ExecutionPurpose::MoveAnalysis { .. } => "move_analysis",
                    mshogi_engine::ExecutionPurpose::ChessFullReachability { .. } => {
                        "chess_full_reachability"
                    }
                }),
            diagnostic: snapshot.engine.message.clone(),
            declaration_advice: vec![],
            context: CanonicalRecord::chess(self.session.selected_game().record()),
            analysis,
            observations,
            search_observations,
            score_meaning: "Chess root-side centipawns; native mate distance and bounded cooperative Full proof remain distinct (see chess analysis).",
            chess_match: Some(self.session.chess().game.record().clone()),
            chess: Some(ChessPresentation {
                claims: snapshot.claims,
                live_conclusion: snapshot.conclusion,
                selected_conclusion: snapshot.view.conclusion,
                in_check: self.session.selected_game().position().in_check(),
                analysis: snapshot.analysis,
                last_engine_result: snapshot.engine.last,
                last_engine_turn: snapshot.engine.last_turn,
                searched_version: snapshot.engine.searched_version,
                rules_mode: snapshot.rules_mode,
            }),
        }
    }
}

/// Difficult states for the separately labeled verification application only.
#[cfg(feature = "desktop-verification")]
impl DesktopController {
    pub(super) fn verification_shogi(&mut self, fixture: &str) -> Result<(), String> {
        fn imported(sfen: &str) -> Result<s::Game, String> {
            let position = s::Position::from_sfen(sfen).map_err(|e| e.to_string())?;
            let root = s::GameRoot::import(
                position,
                s::RootCategory::ReducedMaterialDiagram,
                s::CounterSource::Supplied,
            )
            .map_err(|e| format!("Fixture root: {e:?}"))?;
            Ok(s::Game::from_root(root))
        }
        let mut game = match fixture {
            "shogi-standard"
            | "shogi-gote"
            | "shogi-failure"
            | "shogi-capacity"
            | "shogi-control-capacity" => s::Game::new(),
            "shogi-hands" => {
                let mut game = imported("8k/9/4+p4/9/4Rr3/9/5P3/9/K8 b - 17")?;
                for action in ["5e5c+", "4e4g+"] {
                    game.submit(s::GameRequest {
                        expected: game.context(),
                        actor: game.position().side_to_move(),
                        command: s::GameCommand::Play(
                            action
                                .parse()
                                .map_err(|e: s::ActionParseError| e.to_string())?,
                        ),
                    })
                    .map_err(|e| format!("Fixture move: {e:?}"))?;
                }
                game
            }
            "shogi-promotion" => imported("8k/9/9/4P4/9/9/9/9/K8 b - 1")?,
            "shogi-forced" => imported("8k/4P4/9/9/9/9/9/9/K8 b - 1")?,
            "shogi-declaration" => imported("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1")?,
            "shogi-concluded" => s::Game::new(),
            _ => return Err("Unknown shogi verification fixture.".into()),
        };
        if fixture == "shogi-concluded" {
            game.submit(s::GameRequest {
                expected: game.context(),
                actor: s::Side::Sente,
                command: s::GameCommand::Resign,
            })
            .map_err(|e| format!("Fixture resignation: {e:?}"))?;
        }
        let human = if matches!(fixture, "shogi-gote" | "shogi-failure") {
            s::Side::Gote
        } else {
            s::Side::Sente
        };
        let mut candidate = ShogiSession::restored(&s::GameRecord::from_game(&game), human)?;
        candidate.settings.orientation = human;
        candidate.initial_settings = candidate.settings.clone();
        let operation = game
            .conclusion()
            .is_none()
            .then_some(SessionOperation::Resume {});
        let receipt = self.queue_native_candidate(candidate, operation, None, true)?;
        while receipt.completion().is_none() {
            self.poll_native_admission();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        receipt.completion().expect("completed fixture admission")?;
        if fixture == "shogi-control-capacity" {
            self.session
                .shogi_mut()
                .unwrap()
                .verification_control_capacity()?;
        }
        if fixture == "shogi-capacity" {
            for _ in 0..MAX_ARCHIVE_OBSERVATIONS - 1 {
                let candidate = self.session.shogi().expect("shogi fixture").clone();
                let receipt = self.queue_native_candidate(
                    candidate,
                    Some(SessionOperation::Analyze {
                        operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
                    }),
                    None,
                    false,
                )?;
                while receipt.completion().is_none() {
                    self.poll_native_admission();
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                receipt.completion().expect("completed fixture analysis")?;
                let request = self
                    .session
                    .shogi()
                    .and_then(|s| s.pending.clone())
                    .expect("fixture request");
                let mut bytes = serde_json::to_vec(&execution_request(request.clone()))
                    .map_err(|e| e.to_string())?;
                bytes.push(b'\n');
                let mut response = vec![];
                mshogi_engine::serve_once(std::io::Cursor::new(bytes), &mut response)
                    .map_err(|e| e.to_string())?;
                let response = serde_json::from_slice::<ExecutionResponse>(&response)
                    .map_err(|e| e.to_string())?
                    .observation;
                let candidate = self.session.shogi().expect("shogi fixture").clone();
                let receipt =
                    self.queue_native_candidate(candidate, None, Some(response), false)?;
                while receipt.completion().is_none() {
                    self.poll_native_admission();
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                receipt
                    .completion()
                    .expect("completed fixture observation")?;
            }
        }
        Ok(())
    }
}
#[cfg(test)]
#[path = "shared_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "shared_takeback_tests.rs"]
mod takeback_tests;

fn execution_request(context: ExecutionContext) -> ExecutionRequest {
    ExecutionRequest {
        protocol: mshogi_engine::PROTOCOL_VERSION,
        context,
    }
}
#[cfg(test)]
fn decode_execution_observation(bytes: &[u8]) -> ExecutionObservation {
    let response: ExecutionResponse = serde_json::from_slice(bytes).unwrap();
    assert_eq!(response.protocol, mshogi_engine::PROTOCOL_VERSION);
    response.observation
}

impl DesktopController {
    pub(super) fn present_moves(&self, selected: usize) -> MovesProjection {
        // Native identity is monotonic across undo/reopen; this transient cache
        // owns no live/canonical state. Polling never replays game adjudication.
        let key = (self.session.document_epoch, self.session.version);
        self.shared
            .moves_cache
            .borrow_mut()
            .get(key, selected, || match &self.session {
                ActiveDocument::Chess(state) => crate::moves::chess_projection(state),
                ActiveDocument::Shogi { state, .. } => crate::moves::shogi_projection(&state.game),
                ActiveDocument::Xiangqi { state, .. } => {
                    crate::moves::xiangqi_projection(&state.game)
                }
            })
    }
}

#[cfg(test)]
#[path = "shared_moves_tests.rs"]
mod moves_tests;
