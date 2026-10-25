//! Authoritative untimed desktop sessions. Framework-free commands and snapshots.
use mshogi_core::chess::*;
use mshogi_engine::{
    EngineBridge, EngineClaim, EngineRequest, EngineResponse, EngineResult, WorkLimits,
};
use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf, time::Instant};

const MAX_SAFE_ID: u64 = 9_007_199_254_740_991;
/// Initial interactive work limits; recorded on every engine request.
pub const PLAY_LIMITS: WorkLimits = WorkLimits {
    depth: 2,
    nodes: 20_000,
};

/// One complete host command. A frontend version is mandatory for mutations.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DesktopCommand {
    /// Replace the live game, invalidating every previous search.
    NewGame {
        /// Expected current session version.
        version: u64,
        /// Player color in the new game.
        human: Color,
    },
    /// Commit one native complete action on the human's live turn.
    Play {
        /// Expected current session version.
        version: u64,
        /// Native complete action identifier.
        action: Action,
    },
    /// Exercise a native optional claim; its intended action is not played.
    Claim {
        /// Expected current session version.
        version: u64,
        /// Native claim choice.
        claim: EngineClaim,
    },
    /// Stop work without a fallback move or chess result.
    Cancel {
        /// Expected current session version.
        version: u64,
    },
    /// Explicitly retry a paused engine turn with a fresh identity/generation.
    Retry {
        /// Expected current session version.
        version: u64,
    },
}
impl DesktopCommand {
    fn version(&self) -> u64 {
        match self {
            Self::NewGame { version, .. }
            | Self::Play { version, .. }
            | Self::Claim { version, .. }
            | Self::Cancel { version }
            | Self::Retry { version } => *version,
        }
    }
}
/// Native piece presentation; coordinates are ordinary chess square names.
#[derive(Clone, Debug, Serialize)]
pub struct BoardPiece {
    /// Square occupied by the piece.
    pub square: String,
    /// Piece owner.
    pub color: Color,
    /// Lowercase orthodox piece name.
    pub kind: String,
    /// Accessible square and piece description.
    pub label: String,
}
/// One legal complete action, split only for UI selection.
#[derive(Clone, Debug, Serialize)]
pub struct LegalChoice {
    /// Canonical complete action identifier.
    pub action: Action,
    /// Source coordinate.
    pub from: String,
    /// Destination coordinate.
    pub to: String,
    /// Explicit promotion label when required.
    pub promotion: Option<String>,
}
/// Engine activity is separate from the game's conclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnginePhase {
    /// No work is running.
    Idle,
    /// A bounded request is outstanding.
    Thinking,
    /// User cancelled; explicit retry is needed.
    Cancelled,
    /// A process/search/protocol failure paused the engine turn.
    Failed,
    /// No full iteration was completed; no fallback is applied.
    Incomplete,
}
/// Last actual engine observations, never synthetic progress.
#[derive(Clone, Debug, Serialize)]
pub struct EngineState {
    /// Current process/search lifecycle.
    pub status: EnginePhase,
    /// Last final result, kept distinct from current position evaluation.
    pub last: Option<EngineResult>,
    /// Side whose perspective the last score describes.
    pub last_turn: Option<Color>,
    /// Session version searched by the last result.
    pub searched_version: Option<u64>,
    /// Plain diagnostic separate from any chess result.
    pub message: Option<String>,
}
impl Default for EngineState {
    fn default() -> Self {
        Self {
            status: EnginePhase::Idle,
            last: None,
            last_turn: None,
            searched_version: None,
            message: None,
        }
    }
}
/// Immutable native view. Revision protects async frontend responses.
#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    /// Monotonic observation revision, including engine lifecycle changes.
    pub revision: u64,
    /// Monotonic live-session command version, independent of ply count.
    pub version: u64,
    /// Color controlled by the human.
    pub human: Color,
    /// Current side to move.
    pub turn: Color,
    /// Canonical current position, for inspection only.
    pub fen: String,
    /// Native piece list.
    pub pieces: Vec<BoardPiece>,
    /// Complete legal actions, only actionable on the human live turn.
    pub legal_choices: Vec<LegalChoice>,
    /// Claim opportunities; these are not concluded outcomes.
    pub claims: Vec<EngineClaim>,
    /// Complete played actions, in order.
    pub history: Vec<String>,
    /// Actual match conclusion, possibly explicitly unresolved/unscored.
    pub conclusion: Option<MatchConclusion>,
    /// Rules-owned check state.
    pub in_check: bool,
    /// Process status and final search observations.
    pub engine: EngineState,
    /// Explicit supported interactive adjudication identity.
    pub rules_mode: &'static str,
    /// Deterministic interactive work limits.
    pub limits: WorkLimits,
}

/// Pure serialized rules/session state; the controller supplies process results.
pub struct DesktopSession {
    game: ChessMatch,
    human: Color,
    version: u64,
    revision: u64,
    next_request: u64,
    pending: Option<EngineRequest>,
    engine: EngineState,
}
impl Default for DesktopSession {
    fn default() -> Self {
        Self::new()
    }
}
impl DesktopSession {
    /// New untimed M2 game with optional claims and a human playing White.
    pub fn new() -> Self {
        Self {
            game: new_match(),
            human: Color::White,
            version: 1,
            revision: 1,
            next_request: 0,
            pending: None,
            engine: EngineState::default(),
        }
    }
    /// Restore supported live chess history without silently changing its mode.
    /// Persistence/UI file handling remains a later milestone deliverable.
    pub fn from_record(record: &GameRecord, human: Color) -> Result<Self, String> {
        if record.adjudication_mode() != AdjudicationMode::M2 {
            return Err("Full adjudication restoration is unsupported by this untimed play slice; its mode and limits were not changed.".into());
        }
        let live = AdjudicatedGame::replay_record(record).map_err(|e| e.to_string())?;
        let mut session = Self::new();
        session.game = ChessMatch::new(live, TimeControl::Untimed).map_err(|e| e.to_string())?;
        if session.game.conclusion().is_some() {
            session.human = human;
            return Ok(session);
        }
        let reply = session.game.submit(MatchRequest {
            at_ns: 0,
            version: 0,
            actor: session.game.game().position().side_to_move(),
            command: MatchCommand::Start,
        });
        if let Some(rejection) = reply.rejection {
            return Err(rejection.detail);
        }
        session.human = human;
        Ok(session)
    }
    fn changed(&mut self) {
        // Checked before every external mutation; exhaustion never wraps.
        self.version += 1;
        self.revision += 1;
    }
    fn ensure_capacity(&self) -> Result<(), String> {
        if self.version >= MAX_SAFE_ID
            || self.revision >= MAX_SAFE_ID
            || self.next_request >= MAX_SAFE_ID
        {
            Err("Session identity space exhausted; restart the application.".into())
        } else {
            Ok(())
        }
    }
    /// Apply an atomic human command. Errors leave all session state unchanged.
    pub fn command(&mut self, command: DesktopCommand, at_ns: u64) -> Result<(), String> {
        if command.version() != self.version {
            return Err("The game changed. Use the current position and try again.".into());
        }
        self.ensure_capacity()?;
        match command {
            DesktopCommand::NewGame { human, .. } => {
                self.game = new_match();
                self.human = human;
                self.pending = None;
                self.engine = EngineState::default();
                self.changed();
            }
            DesktopCommand::Play { action, .. } => {
                self.human_turn()?;
                self.apply_match(self.human, MatchCommand::Move(action), at_ns)?;
                self.pending = None;
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
            DesktopCommand::Claim { claim, .. } => {
                self.human_turn()?;
                if claim.actor != self.human {
                    return Err("The claim must belong to the player to move.".into());
                }
                self.apply_match(
                    claim.actor,
                    MatchCommand::Claim {
                        reason: claim.reason,
                        intended: claim.intended,
                    },
                    at_ns,
                )?;
                self.pending = None;
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
            DesktopCommand::Cancel { .. } => {
                self.pending = None;
                self.engine.status = EnginePhase::Cancelled;
                self.engine.message =
                    Some("Engine work cancelled. No move or chess result was applied.".into());
                self.changed();
            }
            DesktopCommand::Retry { .. } => {
                if self.game.conclusion().is_some()
                    || self.game.game().position().side_to_move() == self.human
                {
                    return Err("There is no engine turn to retry.".into());
                }
                self.pending = None;
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
        }
        Ok(())
    }
    fn human_turn(&self) -> Result<(), String> {
        if self.game.conclusion().is_some() {
            return Err("This game has concluded. Start a new game to play again.".into());
        }
        if self.game.game().position().side_to_move() != self.human {
            return Err("It is mShogi's turn.".into());
        }
        Ok(())
    }
    fn apply_match(
        &mut self,
        actor: Color,
        command: MatchCommand,
        at_ns: u64,
    ) -> Result<(), String> {
        // ChessMatch logs rejected requests. A UI rejection is atomic, so commit
        // this private copy only after acceptance.
        let mut candidate = self.game.clone();
        let reply = candidate.submit(MatchRequest {
            at_ns,
            version: candidate.version(),
            actor,
            command,
        });
        if let Some(rejection) = reply.rejection {
            return Err(rejection.detail);
        }
        self.game = candidate;
        Ok(())
    }
    /// Create one immutable exact engine envelope if the live engine turn is ready.
    pub fn engine_request(&mut self) -> Option<EngineRequest> {
        if self.pending.is_some()
            || self.engine.status != EnginePhase::Idle
            || self.game.conclusion().is_some()
            || self.game.game().position().side_to_move() == self.human
        {
            return None;
        }
        if let Err(error) = self.ensure_capacity() {
            self.fail(error);
            return None;
        }
        self.next_request += 1;
        let request = EngineRequest {
            protocol: 1,
            request_id: self.next_request,
            session_version: self.version,
            process_generation: self.next_request,
            game: self.game.game().record(),
            limits: PLAY_LIMITS,
        };
        self.pending = Some(request.clone());
        self.engine.status = EnginePhase::Thinking;
        self.revision += 1;
        Some(request)
    }
    /// Accept and apply only the exact outstanding live context. Stale responses do nothing.
    pub fn accept(&mut self, response: EngineResponse, at_ns: u64) -> bool {
        if self.pending.as_ref() != Some(&response.context)
            || response.context.session_version != self.version
            || response.context.game != self.game.game().record()
            || self.game.conclusion().is_some()
            || self.game.game().position().side_to_move() == self.human
        {
            return false;
        }
        if let Err(error) = self.ensure_capacity() {
            self.pending = None;
            self.fail(error);
            return false;
        }
        self.pending = None;
        self.engine.last_turn = Some(self.game.game().position().side_to_move());
        self.engine.searched_version = Some(self.version);
        self.engine.last = Some(response.result.clone());
        let applied = match response.result {
            EngineResult::Move { action, .. } => {
                self.apply_match(self.human.opponent(), MatchCommand::Move(action), at_ns)
            }
            EngineResult::Claim { claim, .. } => {
                if claim.actor != self.human.opponent() {
                    Err("Engine returned a claim for the wrong player.".into())
                } else {
                    self.apply_match(
                        claim.actor,
                        MatchCommand::Claim {
                            reason: claim.reason,
                            intended: claim.intended,
                        },
                        at_ns,
                    )
                }
            }
            EngineResult::Finished { outcome, .. } => {
                if self.game.game().outcome() != Some(outcome) {
                    Err("Engine reported a conclusion absent from the authoritative game.".into())
                } else {
                    Ok(())
                }
            }
            EngineResult::Incomplete { .. } => {
                self.engine.status = EnginePhase::Incomplete;
                self.engine.message = Some("Search ended before a complete iteration. No move was applied. Retry or start a new game.".into());
                self.changed();
                return true;
            }
            EngineResult::Failure { failure } => {
                self.fail(format!(
                    "Engine failure ({:?}): {}",
                    failure.kind, failure.message
                ));
                return true;
            }
        };
        match applied {
            Ok(()) => {
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
            Err(error) => self.fail(format!("Invalid engine choice: {error}")),
        }
        true
    }
    /// Pause for an application/process failure without assigning a chess score.
    pub fn fail(&mut self, message: String) {
        self.pending = None;
        self.engine.status = EnginePhase::Failed;
        self.engine.message = Some(message);
        if self.version < MAX_SAFE_ID && self.revision < MAX_SAFE_ID {
            self.changed();
        }
    }
    /// Build a rules-owned immutable view. This never starts search or changes the game.
    pub fn snapshot(&self) -> Snapshot {
        let position = self.game.game().position();
        let mut pieces = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::new(file, rank).expect("bounded coordinates");
                if let Some(piece) = position.piece_at(square) {
                    let kind = format!("{:?}", piece.kind).to_lowercase();
                    pieces.push(BoardPiece {
                        square: square.to_string(),
                        color: piece.color,
                        label: format!("{square}, {:?} {kind}", piece.color),
                        kind,
                    });
                }
            }
        }
        let legal_choices = if self.game.conclusion().is_some() {
            Vec::new()
        } else {
            position
                .legal_actions()
                .into_iter()
                .map(|action| LegalChoice {
                    from: action.from.to_string(),
                    to: action.to.to_string(),
                    promotion: action.promotion.map(|p| format!("{p:?}").to_lowercase()),
                    action,
                })
                .collect()
        };
        Snapshot {
            revision: self.revision,
            version: self.version,
            human: self.human,
            turn: position.side_to_move(),
            fen: position.to_fen(),
            pieces,
            legal_choices,
            claims: self
                .game
                .game()
                .available_claims()
                .into_iter()
                .map(EngineClaim::from)
                .collect(),
            history: self
                .game
                .game()
                .history()
                .actions()
                .map(|a| a.to_string())
                .collect(),
            conclusion: self.game.conclusion(),
            in_check: position.in_check(),
            engine: self.engine.clone(),
            rules_mode: "m2",
            limits: PLAY_LIMITS,
        }
    }
}
fn new_match() -> ChessMatch {
    let mut game = ChessMatch::new(
        AdjudicatedGame::new(ClaimPolicy::ExplicitOnly),
        TimeControl::Untimed,
    )
    .expect("valid untimed match");
    let reply = game.submit(MatchRequest {
        at_ns: 0,
        version: 0,
        actor: Color::White,
        command: MatchCommand::Start,
    });
    assert!(reply.rejection.is_none());
    game
}

/// Tauri-owned serialized controller. Process work runs on the bridge worker.
pub struct DesktopController {
    session: DesktopSession,
    bridge: EngineBridge,
    origin: Instant,
}
impl DesktopController {
    /// Launch the supervisor worker, without starting a child or doing pipe I/O.
    pub fn new(engine_path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            session: DesktopSession::new(),
            bridge: EngineBridge::new(engine_path)?,
            origin: Instant::now(),
        })
    }
    fn at_ns(&self) -> u64 {
        u64::try_from(self.origin.elapsed().as_nanos()).unwrap_or(u64::MAX)
    }
    /// Serialize an explicit command before polling completions; cancellation wins races.
    pub fn command(&mut self, command: DesktopCommand) -> Result<Snapshot, String> {
        let at_ns = self.at_ns();
        self.session.command(command, at_ns)?;
        self.bridge.cancel();
        Ok(self.snapshot())
    }
    /// Poll the bounded completion slot, accept it atomically, and start eligible work.
    pub fn snapshot(&mut self) -> Snapshot {
        if let Some(response) = self.bridge.poll() {
            let at_ns = self.at_ns();
            self.session.accept(response, at_ns);
        }
        if let Some(request) = self.session.engine_request()
            && let Err(error) = self.bridge.submit(request)
        {
            self.session
                .fail(format!("Cannot start engine work: {error:?}"));
        }
        self.session.snapshot()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_engine::{StopReason, Telemetry};
    fn telemetry() -> Telemetry {
        Telemetry {
            score: Some(0),
            completed_depth: 1,
            nodes: 21,
            principal_variation: vec![],
            status: StopReason::Complete,
        }
    }
    fn response(context: EngineRequest, action: &str) -> EngineResponse {
        EngineResponse {
            context,
            result: EngineResult::Move {
                action: action.parse().unwrap(),
                telemetry: telemetry(),
            },
        }
    }
    fn human_black() -> DesktopSession {
        let mut s = DesktopSession::new();
        s.command(
            DesktopCommand::NewGame {
                version: 1,
                human: Color::Black,
            },
            0,
        )
        .unwrap();
        s
    }
    #[test]
    fn native_snapshot_and_atomic_human_rejection() {
        let mut s = DesktopSession::new();
        assert_eq!(s.snapshot().pieces.len(), 32);
        assert_eq!(s.snapshot().legal_choices.len(), 20);
        let before = s.game.record().to_json();
        assert!(
            s.command(
                DesktopCommand::Play {
                    version: 1,
                    action: "e2e5".parse().unwrap()
                },
                1
            )
            .is_err()
        );
        assert_eq!(s.game.record().to_json(), before);
        assert_eq!(s.version, 1);
        s.command(
            DesktopCommand::Play {
                version: 1,
                action: "e2e4".parse().unwrap(),
            },
            1,
        )
        .unwrap();
        assert_eq!(s.snapshot().history, ["e2e4"]);
        assert!(
            s.command(
                DesktopCommand::Play {
                    version: 1,
                    action: "d2d4".parse().unwrap()
                },
                2
            )
            .is_err()
        );
        assert_eq!(s.snapshot().history, ["e2e4"]);
    }
    #[test]
    fn engine_applies_once_and_keeps_actual_observations() {
        let mut s = human_black();
        let request = s.engine_request().unwrap();
        let result = response(request, "e2e4");
        assert!(s.accept(result.clone(), 1));
        assert!(!s.accept(result, 2));
        assert_eq!(s.snapshot().history, ["e2e4"]);
        assert_eq!(s.engine.last_turn, Some(Color::White));
        assert_eq!(s.engine.searched_version, Some(2));
        assert!(s.engine_request().is_none());
    }
    #[test]
    fn new_game_aba_and_generation_changes_reject_old_results() {
        let mut s = human_black();
        let old = s.engine_request().unwrap();
        let mut wrong_generation = old.clone();
        wrong_generation.process_generation += 1;
        assert!(!s.accept(response(wrong_generation, "e2e4"), 1));
        s.command(
            DesktopCommand::NewGame {
                version: s.version,
                human: Color::Black,
            },
            2,
        )
        .unwrap();
        let current = s.engine_request().unwrap();
        assert_eq!(current.game, old.game);
        assert_ne!(current.session_version, old.session_version);
        assert!(current.process_generation > old.process_generation);
        assert!(!s.accept(response(old, "e2e4"), 3));
        assert!(s.accept(response(current, "d2d4"), 4));
        assert_eq!(s.snapshot().history, ["d2d4"]);
    }
    #[test]
    fn exact_same_fen_with_different_known_history_cannot_accept() {
        let mut live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        for action in ["g1f3", "g8f6", "f3g1", "f6g8"] {
            live.apply(action.parse().unwrap()).unwrap();
        }
        let fen = live.position().to_fen();
        let imported = AdjudicatedGame::from_fen(&fen, ClaimPolicy::ExplicitOnly).unwrap();
        assert_eq!(imported.position(), live.position());
        let mut s = DesktopSession::from_record(&live.record(), Color::Black).unwrap();
        let request = s.engine_request().unwrap();
        let mut incorrect = request.clone();
        incorrect.game = imported.record();
        assert_ne!(incorrect.game, request.game);
        assert!(!s.accept(response(incorrect, "e2e4"), 1));
        assert_eq!(s.snapshot().history.len(), 4);
        assert!(s.accept(response(request, "e2e4"), 2));
    }
    #[test]
    fn cancel_wins_even_if_a_complete_response_has_arrived() {
        let mut s = human_black();
        let old = s.engine_request().unwrap();
        s.command(DesktopCommand::Cancel { version: s.version }, 1)
            .unwrap();
        assert!(!s.accept(response(old.clone(), "e2e4"), 2));
        assert!(s.snapshot().history.is_empty());
        assert!(s.engine_request().is_none());
        s.command(DesktopCommand::Retry { version: s.version }, 3)
            .unwrap();
        let fresh = s.engine_request().unwrap();
        assert!(fresh.request_id > old.request_id);
        assert!(s.accept(response(fresh, "e2e4"), 4));
    }
    #[test]
    fn invalid_engine_choice_is_unscored_and_atomic() {
        let mut s = human_black();
        let request = s.engine_request().unwrap();
        let before = s.game.record().to_json();
        assert!(s.accept(response(request, "e2e5"), 1));
        assert_eq!(s.game.record().to_json(), before);
        assert_eq!(s.engine.status, EnginePhase::Failed);
        assert!(s.snapshot().conclusion.is_none());
    }
    #[test]
    fn incomplete_search_never_falls_back_to_a_move() {
        let mut s = human_black();
        let context = s.engine_request().unwrap();
        let mut observation = telemetry();
        observation.score = None;
        observation.completed_depth = 0;
        observation.nodes = 0;
        observation.status = StopReason::NodeLimit;
        assert!(s.accept(
            EngineResponse {
                context,
                result: EngineResult::Incomplete {
                    telemetry: observation
                }
            },
            1
        ));
        assert_eq!(s.engine.status, EnginePhase::Incomplete);
        assert!(s.snapshot().history.is_empty());
        assert!(s.snapshot().conclusion.is_none());
    }
    #[test]
    fn intended_claim_is_an_opportunity_then_conclusion_without_a_move() {
        let live =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 99 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut s = DesktopSession::from_record(&live.record(), Color::White).unwrap();
        let claim = s
            .snapshot()
            .claims
            .into_iter()
            .find(|c| c.reason == DrawClaim::FiftyMove && c.intended.is_some())
            .unwrap();
        assert!(s.snapshot().conclusion.is_none());
        s.command(
            DesktopCommand::Claim {
                version: s.version,
                claim,
            },
            1,
        )
        .unwrap();
        assert_eq!(
            s.snapshot().conclusion,
            Some(MatchConclusion::Chess(ChessOutcome::Draw(
                DrawReason::Claimed(DrawClaim::FiftyMove)
            )))
        );
        assert!(s.snapshot().history.is_empty());
        assert!(s.snapshot().legal_choices.is_empty());
    }
    #[test]
    fn engine_claim_remains_a_claim() {
        let live =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 100 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut s = DesktopSession::from_record(&live.record(), Color::Black).unwrap();
        let claim = s
            .snapshot()
            .claims
            .into_iter()
            .find(|c| c.intended.is_none())
            .unwrap();
        let context = s.engine_request().unwrap();
        assert!(s.accept(
            EngineResponse {
                context,
                result: EngineResult::Claim {
                    claim,
                    telemetry: telemetry()
                }
            },
            1
        ));
        assert!(s.snapshot().history.is_empty());
        assert!(s.snapshot().conclusion.is_some());
    }
    #[test]
    fn promotion_choices_and_automatic_checkmate_are_native() {
        let live =
            AdjudicatedGame::from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let s = DesktopSession::from_record(&live.record(), Color::White).unwrap();
        assert_eq!(
            s.snapshot()
                .legal_choices
                .iter()
                .filter(|c| c.from == "a7")
                .count(),
            4
        );
        let mut live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        for action in ["f2f3", "e7e5", "g2g4", "d8h4"] {
            live.apply(action.parse().unwrap()).unwrap();
        }
        let s = DesktopSession::from_record(&live.record(), Color::White).unwrap();
        assert_eq!(
            s.snapshot().conclusion,
            Some(MatchConclusion::Chess(ChessOutcome::Checkmate {
                winner: Color::Black
            }))
        );
    }
    #[test]
    fn full_mode_restoration_is_explicitly_rejected_before_solver_execution() {
        let live = AdjudicatedGame::from_fen_with_mode(
            "7k/8/8/8/8/8/R7/K7 w - - 0 1",
            ClaimPolicy::ExplicitOnly,
            AdjudicationMode::Full {
                limits: ReachabilityLimits {
                    max_positions: Some(0),
                    max_transitions: Some(0),
                },
            },
        )
        .unwrap();
        let error = DesktopSession::from_record(&live.record(), Color::White)
            .err()
            .unwrap();
        assert!(error.contains("unsupported"));
        assert!(error.contains("mode and limits were not changed"));
    }
}
