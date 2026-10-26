//! Concrete chess/shogi desktop boundary. Native games remain authoritative.
//! The stable chess APIs/archive are a compatibility adapter, not a common rules model.
use super::*;
#[path = "shared_archive.rs"]
pub(super) mod archive;
pub use archive::{NativeDesktopSettings, SharedDocumentState, SharedOpenToken, ShogiSettings};
use mshogi_core::{
    native::{CanonicalRecord, GameKind, NativeSide, RulesProfile},
    shogi as s,
};
use mshogi_engine::{
    SHARED_PROTOCOL_VERSION, SharedAnalysisTarget, SharedPurpose, SharedRequest, SharedResponse,
    SharedResult, ShogiChoice, ShogiResult,
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
fn side(id: NativeSide) -> PresentedSide {
    let (profile, name, first) = match id {
        NativeSide::Chess(Color::White) => (RulesProfile::ChessMatchV1, "White", true),
        NativeSide::Chess(Color::Black) => (RulesProfile::ChessMatchV1, "Black", false),
        NativeSide::Shogi(s::Side::Sente) => (RulesProfile::ShogiCsa36V1, "Sente", true),
        NativeSide::Shogi(s::Side::Gote) => (RulesProfile::ShogiCsa36V1, "Gote", false),
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
/// Native square geometry; no rules or coordinate parsing in the webview.
#[derive(Clone, Debug, Serialize)]
pub struct BoardDescriptor {
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
    pub response: SharedResponse,
    /// Native command admission diagnostic, separate from a game outcome.
    #[serde(deserialize_with = "archive::required_option")]
    pub application_error: Option<String>,
    /// True after reopening; evidence never resumes execution.
    pub historical: bool,
}
/// Typed shared desktop view with exact runtime identity and native context.
#[derive(Clone, Debug, Serialize)]
pub struct SharedSnapshot {
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
    /// Native square geometry descriptor.
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
    /// Whether viewing a read-only earlier event prefix.
    pub historical: bool,
    /// Whether explicit Resume is required before live engine work.
    pub paused: bool,
    /// Execution lifecycle, independent of native game outcome.
    pub phase: EnginePhase,
    /// Bounded execution/application diagnostic, never a chess or shogi result.
    pub diagnostic: Option<String>,
    /// Read-only eligibility evidence for each native side.
    pub declaration_advice: Vec<DeclarationAdvice>,
    /// Authoritative native selected-prefix canonical record.
    pub context: CanonicalRecord,
    /// Read-only observation bound to its exact selected target.
    pub analysis: Option<PresentedObservation>,
    /// Accepted historical observations with their exact contexts.
    pub observations: Vec<PresentedObservation>,
    /// Native evaluation perspective and terminal-distance interpretation.
    pub score_meaning: &'static str,
    /// Existing chess view/archive/claims/observations remain natively typed and unchanged.
    pub chess: Option<Snapshot>,
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
/// Small concrete dispatch across two native desktop games.
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
    /// Existing chess operation, with its original additional document preconditions.
    Chess {
        /// Existing native game-specific command.
        command: DesktopCommand,
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
    // Existing chess operation, with its original additional document preconditions.
    Chess {
        // Existing native game-specific command.
        command: DesktopCommand,
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
    paused: bool,
    pending: Option<SharedRequest>,
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
    pub epoch: u64,
    pub shogi: Option<ShogiSession>,
    pub admission: Option<(u64, bool)>,
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
    fn selected(&self) -> &s::Game {
        self.viewed.as_ref().unwrap_or(&self.game)
    }
    fn retire(&mut self) {
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
        Some(s::Conclusion::Win { winner, reason }) => format!(
            "{} wins — {}",
            side(NativeSide::Shogi(winner)).label,
            match reason {
                s::WinReason::NoLegalAction { in_check: true } => "checkmate",
                s::WinReason::NoLegalAction { in_check: false } => "no legal action",
                s::WinReason::ContinuousCheck { .. } => "continuous-check repetition",
                s::WinReason::Resignation => "resignation",
                s::WinReason::EnteringKing => "entering-king declaration",
                s::WinReason::FalseDeclaration => "false declaration",
            }
        ),
        Some(s::Conclusion::Draw(reason)) => format!(
            "Draw — {}",
            match reason {
                s::DrawReason::Fourfold { .. } => "fourfold repetition",
                s::DrawReason::Length => "320-action limit",
            }
        ),
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
            document: self.shared.epoch,
            version: self.session.version,
            revision: self.session.revision,
            selection: self.session.selection_id,
        }
    }
    fn shared_changed(&mut self) {
        self.session.version += 1;
        self.session.revision += 1;
        self.session.selection_id += 1;
        self.session.document_revision += 1;
        self.session.archive_revision = self.session.revision.min(MAX_SAFE_ID - 65);
    }
    pub(super) fn require_chess(&self) -> Result<(), String> {
        if self.shared.shogi.is_some() {
            Err("Wrong game: this legacy chess endpoint cannot act on a shogi document; use the shared native endpoint.".into())
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
        if command.expected != self.shared_token() {
            return Err(
                "The session or selected event changed; use its current native snapshot.".into(),
            );
        }
        let active = if self.shared.shogi.is_some() {
            GameKind::Shogi
        } else {
            GameKind::Chess
        };
        if command.game != active {
            return Err("Command game does not match the active native session.".into());
        }
        self.session.ensure_capacity()?;
        if self.shared.epoch >= MAX_SAFE_ID - 64 {
            return Err("Application capacity: document identity exhausted.".into());
        }
        if let SessionOperation::Replace { record, human } = command.operation {
            if record.game_kind() != human.game() {
                return Err("Replacement side and game disagree.".into());
            }
            // Validate before retiring any current state or work.
            if let CanonicalRecord::Shogi(record) = record {
                let NativeSide::Shogi(human) = human else {
                    unreachable!()
                };
                let candidate = ShogiSession::restored(&record, human)?;
                let receipt = self.queue_native_candidate(candidate, None, None, true)?;
                self.last_command_receipt = Some(receipt);
            } else if let CanonicalRecord::Chess(record) = record {
                let NativeSide::Chess(human) = human else {
                    unreachable!()
                };
                let mut candidate = DesktopSession::from_record(&record, human)?;
                candidate.version = self.session.version + 1;
                candidate.revision = self.session.revision + 1;
                candidate.next_request = self.session.next_request;
                candidate.selection_id = self.session.selection_id + 1;
                candidate.document_identity = self.session.document_identity + 1;
                candidate.document_revision = self.session.document_revision + 1;
                candidate.play_paused = true;
                self.retire_admission();
                self.bridge.cancel();
                self.retire_native_files()?;
                if let Some(state) = &mut self.shared.shogi {
                    state.retire();
                    state.paused = true;
                }
                self.queue_admission(candidate)?;
                self.shared.admission = Some((self.shared.epoch, false));
                self.last_command_receipt = self.admission_receipt.clone();
            }
            return Ok(self.shared_snapshot());
        }
        if self.shared.shogi.is_none() {
            let legacy = match command.operation {
                SessionOperation::Chess { command } => command,
                SessionOperation::Analyze { operation } => DesktopCommand::Analyze {
                    version: self.session.version,
                    operation,
                },
                SessionOperation::Inspect { event_index } => DesktopCommand::Inspect {
                    version: self.session.version,
                    event_index,
                },
                SessionOperation::ReturnLive {} => DesktopCommand::ReturnLive {
                    version: self.session.version,
                },
                SessionOperation::Cancel {} => {
                    if self.session.analysis_pending.is_some() {
                        DesktopCommand::CancelAnalysis {
                            version: self.session.version,
                        }
                    } else {
                        DesktopCommand::Cancel {
                            version: self.session.version,
                        }
                    }
                }
                SessionOperation::Resume {} => DesktopCommand::Resume {
                    version: self.session.version,
                },
                SessionOperation::Retry {} => DesktopCommand::Retry {
                    version: self.session.version,
                },
                SessionOperation::Resign {} => DesktopCommand::Resign {
                    version: self.session.version,
                },
                _ => {
                    return Err(
                        "Unsupported chess operation; use its native complete command.".into(),
                    );
                }
            };
            self.command(legacy)?;
            return Ok(self.shared_snapshot());
        }
        if self.admission_token.is_some()
            && !matches!(command.operation, SessionOperation::Cancel {})
        {
            return Err("Wait for native persistence admission before another command.".into());
        }
        let candidate = self.shared.shogi.as_ref().unwrap().clone();
        let receipt =
            self.queue_native_candidate(candidate, Some(command.operation), None, false)?;
        self.last_command_receipt = Some(receipt);
        Ok(self.shared_snapshot())
    }
    fn start_shogi(&mut self, limits: WorkLimits, analysis: bool) -> Result<(), String> {
        let state = self.shared.shogi.as_ref().unwrap();
        if state.observations.len() >= 256 {
            return Err("Application capacity: all256 observations are retained. Save this complete document, then start a new document to continue.".into());
        }
        self.session.ensure_capacity()?;
        self.session.next_request += 1;
        let game = state.record();
        let purpose = if analysis {
            SharedPurpose::MoveAnalysis {
                target: SharedAnalysisTarget {
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
            SharedPurpose::EngineTurn {}
        };
        let request = SharedRequest {
            protocol: SHARED_PROTOCOL_VERSION,
            purpose,
            request_id: self.session.next_request,
            session_version: self.session.version,
            process_generation: self.session.next_request,
            game,
            limits,
        };
        let state = self.shared.shogi.as_mut().unwrap();
        state.execution_floor = state
            .execution_floor
            .max(request.request_id)
            .max(request.session_version)
            .max(self.session.selection_id);
        state.pending = Some(request);
        state.submitted = false;
        state.phase = EnginePhase::Thinking;
        Ok(())
    }
    /// Exact acceptance, separate read-only observation path and native command admission.
    pub fn accept_shared(&mut self, response: SharedResponse) -> bool {
        let Some(state) = self.shared.shogi.as_ref() else {
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
        if self.shared.shogi.is_some() {
            self.poll_files();
            if self.shared.shogi.is_none() {
                return self.shared_snapshot();
            }
            if self.admission_token.is_none()
                && let Some(response) = self.bridge.poll_shared()
            {
                self.accept_shared(response);
            }
            let state = self.shared.shogi.as_ref().unwrap();
            if self.admission_token.is_none()
                && state.pending.is_none()
                && !state.paused
                && state.viewed.is_none()
                && state.game.conclusion().is_none()
                && state.game.position().side_to_move() != state.human
                && let Err(e) = self.start_shogi(
                    WorkLimits {
                        depth: 2,
                        nodes: 5000,
                    },
                    false,
                )
            {
                self.pause_native_capacity(e);
            }
            let auto_submit = self.shared.auto_submit();
            let state = self.shared.shogi.as_mut().unwrap();
            if self.admission_token.is_none()
                && auto_submit
                && let Some(request) = state.pending.clone()
                && !state.submitted
            {
                state.submitted = true;
                if let Err(e) = self.bridge.submit_shared(request.clone()) {
                    self.accept_shared(SharedResponse {
                        context: request,
                        result: SharedResult::Failure {
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
        let snapshot = self.snapshot();
        self.chess_snapshot(snapshot)
    }
    pub(super) fn pause_shared_for_close(&mut self) {
        self.retire_admission();
        if let Some(state) = self.shared.shogi.as_mut()
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
        let state = self.shared.shogi.as_ref().unwrap();
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
        SharedSnapshot {
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
                files: files.clone(),
                ranks: ranks.clone(),
                label: "Standard shogi contract board (MSH-72; application delivery MSH-74)".into(),
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
                undo: !historical && !state.game.events().is_empty(),
                engine_turn: !historical && !ended && !human_turn,
                move_analysis: true,
                chess_full: false,
                claims: false,
                declaration: !historical && !ended,
                resign: !historical && !ended,
                hands: true,
                history: true,
                save: true,
            },
            status: shogi_status(game),
            events,
            event_index: game.events().len(),
            historical,
            paused: state.paused,
            phase: state.phase,
            diagnostic: state.diagnostic.clone(),
            declaration_advice,
            context: state.record(),
            analysis: state.analysis.clone(),
            observations: state.observations.clone(),
            score_meaning: "Native shogi root-side hundredths of pawn; terminal win distance counts complete command edges, including declarations. Not a chess mate score.",
            chess: None,
        }
    }
    fn chess_snapshot(&self, snapshot: Snapshot) -> SharedSnapshot {
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
        SharedSnapshot {
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
            historical: snapshot.view.historical,
            paused: snapshot.play_paused,
            phase: snapshot.engine.status,
            diagnostic: snapshot.engine.message.clone(),
            declaration_advice: vec![],
            context: CanonicalRecord::chess(self.session.selected_game().record()),
            analysis: None,
            observations: vec![],
            score_meaning: "Chess root-side centipawns; native mate distance and bounded cooperative Full proof remain distinct (see chess analysis).",
            chess: Some(snapshot),
        }
    }
}

#[cfg(test)]
#[path = "shared_tests.rs"]
mod tests;
