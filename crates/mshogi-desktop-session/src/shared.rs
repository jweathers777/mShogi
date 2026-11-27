//! Concrete native desktop consumer boundary. Native games remain authoritative.
//! Native-specific capability data composes into the one application contract.
use super::*;
#[path = "shared_archive.rs"]
pub(super) mod archive;
pub use archive::{SharedDocumentState, SharedOpenToken};
#[path = "native_session.rs"]
#[macro_use]
pub(super) mod native_session;
#[path = "shared_chess.rs"]
pub(super) mod chess;
#[path = "shared_chu.rs"]
pub(super) mod chu;
#[path = "shared_inspection.rs"]
mod inspection;
#[path = "shared_shogi.rs"]
pub(super) mod shogi;
#[path = "shared_tenjiku.rs"]
pub(super) mod tenjiku;
#[path = "shared_tori.rs"]
pub(super) mod tori;
#[path = "shared_transaction.rs"]
mod transaction;
#[path = "shared_xiangqi.rs"]
pub(super) mod xiangqi;
use crate::native_game::NativeGame;
pub use inspection::{MovementInspection, MovementInspectionQuery};
pub use mshogi_core::native::{GameKind, NativeSide, RulesProfile};
pub use mshogi_core::shogi::Side as ShogiSide;
use mshogi_core::{native::CanonicalRecord, shogi as s};
#[cfg(test)]
use mshogi_engine::OBSERVATION_SCHEMA_VERSION;
use mshogi_engine::{
    ExecutionContext, ExecutionObservation, ExecutionPurpose, ExecutionRequest, ExecutionResponse,
    ExecutionResult, ExecutionTarget,
};
use native_session::SessionGame;
pub use xiangqi::{IntersectionTopology, Palace, XiangqiPresentation};

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
pub(super) fn side(profile: RulesProfile, id: NativeSide) -> PresentedSide {
    debug_assert_eq!(profile.game(), id.game());
    let (name, first) = match id {
        NativeSide::Chess(Color::White) => ("White", true),
        NativeSide::Chess(Color::Black) => ("Black", false),
        NativeSide::Xiangqi(mshogi_core::xiangqi::Side::Red) => ("Red", true),
        NativeSide::Xiangqi(mshogi_core::xiangqi::Side::Black) => ("Black", false),
        NativeSide::Shogi(s::Side::Sente)
        | NativeSide::Tori(mshogi_core::tori::Side::Sente)
        | NativeSide::Chu(mshogi_core::chu::Side::Sente)
        | NativeSide::Tenjiku(mshogi_core::tenjiku::Side::Sente) => ("Sente", true),
        NativeSide::Shogi(s::Side::Gote)
        | NativeSide::Tori(mshogi_core::tori::Side::Gote)
        | NativeSide::Chu(mshogi_core::chu::Side::Gote)
        | NativeSide::Tenjiku(mshogi_core::tenjiku::Side::Gote) => ("Gote", false),
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
    /// Tenjiku selection shape keeps stationary targets distinct from movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenjiku: Option<tenjiku::TenjikuChoiceShape>,
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
    /// Actual deepest visited board ply for conventional Chess; absent in old
    /// observations and profiles without selective-depth telemetry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_depth: Option<u16>,
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
            ChessResult::Failure { .. } => "Engine failure",
            _ => match telemetry.expect("search telemetry").status {
                mshogi_engine::StopReason::Complete => "Complete",
                mshogi_engine::StopReason::NodeLimit => "Node limit reached",
                mshogi_engine::StopReason::Stopped if context.limits.time_ms.is_some() => {
                    "Thinking time reached"
                }
                mshogi_engine::StopReason::Stopped => "Analysis stopped",
                mshogi_engine::StopReason::Unresolved => "Unresolved, unscored",
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
        selective_depth: telemetry.map(|t| t.selective_depth),
        observation_index: index,
        purpose,
        event_index,
        board_ply,
        perspective: side(
            mshogi_core::native::RulesProfile::ChessMatchV1,
            NativeSide::Chess(root_side),
        )
        .label,
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
    /// Tori-native exact prefix, outcome and provenance; no cross-game inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tori: Option<tori::ToriPresentation>,
    /// Native Chu royal, Lion-right, repetition and complete-effect metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chu: Option<chu::ChuPresentation>,
    /// Native two-profile Tenjiku facts and ordered effect phases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenjiku: Option<tenjiku::TenjikuPresentation>,
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
    /// Durable participant ownership.
    pub play_mode: PlayMode,
    /// Local actor for ordinary turn-based controls.
    pub local_actor: PresentedSide,
    /// Fixed human/orientation anchor.
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
    /// Native close guard distinguishes an unsaved document from its stop-only journal.
    pub close_requires_save: bool,
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
    /// Actual read-only analysis and its exact context.
    pub analysis: AnalysisState,
    /// Latest actual engine result, with native score meaning.
    pub last_engine_result: Option<ChessResult>,
    /// Side whose position was searched for the latest engine result.
    pub last_engine_turn: Option<Color>,
    /// Exact live version searched by that result.
    pub searched_version: Option<u64>,
}
/// The request still carries a mandatory game tag, even for common operations.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SharedDesktopCommand {
    /// Complete current runtime precondition, required for every shared command.
    pub expected: SessionToken,
    /// Explicit native game identity.
    pub game: GameKind,
    /// Game-neutral operation; the active game admits its payload.
    pub operation: SessionOperation,
}
impl SharedDesktopCommand {
    /// Parse the complete command from raw JSON. Duplicate fields are refused
    /// anywhere, also in the game payloads.
    pub fn from_json(json: &str) -> Result<Self, String> {
        let parsed = document::reject_duplicates(json.as_bytes())
            .and_then(|()| serde_json::from_str(json).map_err(|e| e.to_string()));
        parsed.map_err(|mut message| {
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
/// A session operation. The kind names no game: game data is only in the
/// action, command or settings payload, which the active game parses.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SessionOperation {
    /// Load a native record as a new unsaved document. Verification fixtures
    /// and tests only; the application opens documents through files.
    #[serde(skip_deserializing)]
    Replace {
        /// Strict authoritative native canonical record; not a desktop archive.
        record: CanonicalRecord,
        /// Native side controlled by the human.
        human: NativeSide,
    },
    /// Play the complete native action of a presented choice for the local player.
    Play {
        /// Opaque action identifier of a presented choice.
        action: String,
    },
    /// Submit a complete native command of the active game for the local player.
    Command {
        /// The native command, as the game's records spell it.
        command: serde_json::Value,
    },
    /// Save the orientation and analysis defaults; the play mode stays the document's.
    Preferences {
        /// Settings of the active game.
        settings: crate::game_document::GameSettings<NativeSide>,
    },
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
    /// Return to live, restoring only native browse-suspended running intent.
    ReturnLive {},
    /// Replace execution with bounded read-only analysis of the selected prefix.
    Analyze {
        /// Requested analysis limits.
        operation: AnalysisOperation,
    },
    /// Retire pending work without a fallback move or outcome.
    Cancel {},
    /// Explicitly resume live play after inspection or analysis.
    Resume {},
    /// Start fresh execution after incomplete or failed work.
    Retry {},
}
impl SessionOperation {
    /// A native command operation.
    pub fn command(command: &impl Serialize) -> Self {
        Self::Command {
            command: serde_json::to_value(command).expect("native command serializes"),
        }
    }
    /// A play operation for the presented choice identifier `action`.
    pub fn play(action: impl ToString) -> Self {
        Self::Play {
            action: action.to_string(),
        }
    }
}

#[derive(Default)]
pub(super) struct SharedState {
    pub moves_cache: std::cell::RefCell<crate::moves::Cache>,
    pub admission: Option<(u64, bool)>,
    pub admission_consumes_response: bool,
    pub transactional_admission: bool,
    #[cfg(test)]
    pub manual_engine: bool,
}
impl SharedState {
    pub(super) fn auto_submit(&self) -> bool {
        #[cfg(test)]
        if self.manual_engine {
            return false;
        }
        true
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
        profile: RulesProfile,
        human: NativeSide,
    ) -> Result<(SharedSnapshot, AdmissionReceipt), String> {
        self.shared_new_game_with_mode(expected, profile, human, PlayMode::HumanEngine)
    }
    /// Create a native document with durable local participant ownership.
    pub fn shared_new_game_with_mode(
        &mut self,
        expected: SharedOpenToken,
        profile: RulesProfile,
        human: NativeSide,
        play_mode: PlayMode,
    ) -> Result<(SharedSnapshot, AdmissionReceipt), String> {
        let game = profile.game();
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
        let receipt = with_game!(game, G => {
            let human = G::side_of(human).expect("side of this game");
            let record = G::new_record(profile).map_err(|e| e.to_string())?;
            self.queue_game_candidate::<G>(
                native_session::RuntimeSeed::Record {
                    record: Box::new(record),
                    human,
                    play_mode,
                },
                Some(SessionOperation::Resume {}),
                None,
                true,
            )?
        });
        Ok((self.shared_snapshot(), receipt))
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
        let active = self.session.kind();
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
            return with_game!(record.game_kind(), G => {
                let record = G::record_of(record).expect("record of this game");
                let human = G::side_of(human).expect("side of this game");
                self.replace_game::<G>(record, human)
            });
        }
        with_game!(active, G => self.game_command::<G>(command.operation))
    }
    /// Exact acceptance, separate read-only observation path and native command admission.
    pub fn accept_shared(&mut self, response: ExecutionResponse) -> bool {
        if response.protocol != mshogi_engine::PROTOCOL_VERSION {
            return false;
        }
        self.accept_observation(response.observation)
    }
    pub(crate) fn accept_observation(&mut self, response: ExecutionObservation) -> bool {
        with_game!(self.session.kind(), G => self.accept_game_observation::<G>(response))
    }
    /// Poll only the active game and return its native presentation and provenance.
    pub fn shared_snapshot(&mut self) -> SharedSnapshot {
        self.poll_native_admission();
        self.poll_files();
        with_game!(self.session.kind(), G => self.poll_game::<G>())
    }
    pub(super) fn pause_shared_for_close(&mut self) {
        self.retire_admission();
        with_game!(self.session.kind(), G => self.close_game::<G>())
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
        let wait = |c: &mut Self, receipt: AdmissionReceipt| -> Result<(), String> {
            while receipt.completion().is_none() {
                c.poll_native_admission();
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            receipt.completion().expect("completed fixture admission")
        };
        let record = s::GameRecord::from_game(&game);
        // There is no control journal any more; the control-capacity fixture
        // is the standard game (MSH-228).
        let seed = native_session::RuntimeSeed::Record {
            record: Box::new(record),
            human,
            play_mode: PlayMode::HumanEngine,
        };
        let operation = game
            .conclusion()
            .is_none()
            .then_some(SessionOperation::Resume {});
        let receipt = self.queue_game_candidate::<Shogi>(seed, operation, None, true)?;
        wait(self, receipt)?;
        if fixture == "shogi-capacity" {
            let limits = WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 1,
            };
            for _ in 0..native_session::KEPT_RESULTS - 1 {
                self.start_game_work::<Shogi>(limits, true)
                    .map_err(|e| e.diagnostic)?;
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
                let response = ExecutionResponse::decode(&response, &execution_request(request))
                    .map_err(|e| e.to_string())?
                    .observation;
                if !self.accept_game_observation::<Shogi>(response) {
                    return Err("Fixture observation refused.".into());
                }
                let receipt = self.admission_receipt.clone().expect("fixture admission");
                wait(self, receipt)?;
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
fn decode_execution_observation(bytes: &[u8], context: &ExecutionContext) -> ExecutionObservation {
    let response = ExecutionResponse::decode(bytes, &execution_request(context.clone())).unwrap();
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
                ActiveDocument::Tori { state, .. } => crate::moves::tori_projection(&state.game),
                ActiveDocument::Chu { state, .. } => crate::moves::chu_projection(&state.game),
                ActiveDocument::Tenjiku { state, .. } => {
                    crate::moves::tenjiku_projection(&state.game)
                }
                ActiveDocument::Chess { state, .. } => {
                    crate::moves::chess_match_projection(&state.game)
                }
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

#[cfg(test)]
#[path = "workflow_tests.rs"]
mod workflow_tests;

#[cfg(test)]
#[path = "shared_chu_tests.rs"]
mod chu_tests;

#[cfg(test)]
#[path = "shared_tenjiku_tests.rs"]
mod tenjiku_tests;

#[cfg(test)]
#[path = "six_game_integration_tests.rs"]
mod six_game_integration_tests;

#[cfg(test)]
#[path = "two_human_tests.rs"]
mod two_human_tests;
