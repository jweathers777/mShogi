//! Known native commands and events. A position and a declared counter are not history.
use super::conduct::{self, FaultKind, Responsibility, Step};
use super::{
    Action, ActionError, LocalPositionError, LocalStatus, NoActionReason, NoWinAssessment,
    Position, PositionRoot, Side,
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

/// Native bound includes commands whose claims were semantically rejected.
pub const MAX_GAME_COMMANDS: usize = 512;
/// Separate board, warning, request, rejection and conclusion events are counted.
pub const MAX_GAME_EVENTS: usize = 2048;
static NEXT_INSTANCE: AtomicU64 = AtomicU64::new(1);
/// Initial unattended software policy, independent of native claim eligibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPolicy {
    /// A participant explicitly submits a claim.
    Explicit,
    /// Exercise valid claims after automatic board adjudication, Red first.
    AutoValidClaims,
}
/// Complete participant command; warnings and conclusions cannot be submitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GameCommand {
    /// Complete ordinary native action.
    Play {
        /// Native origin and destination.
        action: Action,
    },
    /// Resigning actor need not have the move.
    Resign,
    /// Offer agreement; board actions retire outstanding offers.
    OfferDraw,
    /// Accept an opponent's outstanding offer.
    AcceptDraw,
    /// Reject the opponent's offer without an action or outcome.
    DeclineDraw,
    /// Submit an Art8 claim for this actor, regardless of turn.
    ClaimNaturalCount,
}
/// Native score reasons. No external engine failure is a reason for a win.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum WinReason {
    /// Opponent lacks a local board action.
    NoLegalAction {
        /// Checkmate versus stalemate-as-loss.
        reason: NoActionReason,
    },
    /// Continued one-sided prohibited conduct.
    Repetition {
        /// Native conduct class.
        fault: FaultKind,
    },
    /// Explicit resignation.
    Resignation,
}
/// Native draw reasons, separate from opportunities and unknown assessments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrawReason {
    /// Fourth known occurrence without continued unilateral fault.
    Repetition,
    /// Valid claimant-specific Art8 request.
    NaturalCount,
    /// Opponent accepted a draw offer.
    Agreement,
    /// A sound proof establishes neither side can win.
    NeitherCanWin,
}
/// Authoritative concluded chess-family result, never inferred from stored turn.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Conclusion {
    /// A named native side won.
    Win {
        /// Explicit winner, including after a non-board event.
        winner: Side,
        /// Native reason.
        reason: WinReason,
    },
    /// A proven or requested native draw.
    Draw {
        /// Native reason.
        reason: DrawReason,
    },
}
/// Exact known-window arithmetic; supplied pre-root counters never add evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NaturalCountEligibility {
    /// Actor whose excess checks are deducted.
    pub claimant: Side,
    /// Observed capture-free plies since known root or last capture.
    pub known_plies: usize,
    /// Claimant's actual checks in that window.
    pub claimant_checks: usize,
    /// Only checks above ten are deducted.
    pub effective_plies: usize,
    /// Whether the independently known count reaches100.
    pub eligible: bool,
}
/// Semantic warning witness uses board indices; event identities remain separate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Warning {
    /// Side required to vary.
    pub side: Side,
    /// Source-resolved prohibited conduct.
    pub fault: FaultKind,
    /// Root-indexed first/second/third known occurrences.
    pub occurrences: Vec<usize>,
    /// Event that committed this warning.
    pub event_index: usize,
    /// A broken obligation remains in history, marked inactive.
    pub active: bool,
}
/// Explicit pause when classification could change the result. No fallback draw.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingAdjudication {
    /// Exact occurrences whose interval needs classification.
    pub occurrences: Vec<usize>,
    /// Bounded diagnostic, not an engine score or outcome.
    pub reason: String,
}
/// Exact committed native event. Derived records are replay-validated, never trusted input.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EventKind {
    /// A locally legal action, even when its later history consequence is a loss.
    BoardAction {
        /// Complete player choice.
        action: Action,
        /// Root-assigned persistent mover identity.
        moving_id: u8,
        /// Removed identity, if any.
        captured_id: Option<u8>,
        /// Authoritative check observation.
        gave_check: bool,
    },
    /// Native requirement to vary; does not consume board ply.
    Warning {
        /// Exact source-resolved witness.
        warning: Warning,
    },
    /// A proven variation retires an obligation without erasing its original event.
    WarningRetired {
        /// Identity of the original warning event.
        warning_event_index: usize,
    },
    /// Claim submitted explicitly or under the recorded unattended policy.
    ClaimSubmitted {
        /// Authoritative claimant-specific evidence.
        evidence: NaturalCountEligibility,
        /// Whether the named policy generated it.
        automatic: bool,
    },
    /// Invalid claim audit, separate from accepted actions and semantic penalties.
    ClaimRejected {
        /// Exact evidence at this prefix.
        evidence: NaturalCountEligibility,
    },
    /// Accepted valid claim, before its separate conclusion event.
    ClaimAccepted {
        /// Exact count witness.
        evidence: NaturalCountEligibility,
    },
    /// An offer was created or withdrawn; no board action or chess result.
    DrawOffer {
        /// Current offer owner, or none after withdrawal.
        offered_by: Option<Side>,
    },
    /// Native actor resigned; winner is assigned explicitly afterward.
    Resigned,
    /// Agreement was accepted by the opponent of the offeror.
    AgreementAccepted,
    /// Automatic classification did not establish the required outcome.
    AdjudicationPending {
        /// Exact unscored pending state.
        pending: PendingAdjudication,
    },
    /// Final native result, never a request or an engine observation.
    Concluded {
        /// Exact outcome and native reason.
        conclusion: Conclusion,
    },
}
/// Event index and board ply are deliberately different counters.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEvent {
    /// Zero-based canonical event identity.
    pub index: usize,
    /// Number of accepted board actions at this event.
    pub board_ply: usize,
    /// Actor, absent only for automatic native conclusions.
    pub actor: Option<Side>,
    /// Native event payload.
    pub event: EventKind,
}
/// Snapshot for immutable historical viewing, including inside an atomic command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistorySnapshot {
    /// Exact position after the requested event prefix.
    pub position: Position,
    /// Committed warnings at that prefix.
    pub warnings: Vec<Warning>,
    /// Current agreement offer, if any.
    pub offered_by: Option<Side>,
    /// Conclusion at this exact prefix.
    pub conclusion: Option<Conclusion>,
    /// Explicit unresolved automatic adjudication.
    pub pending: Option<PendingAdjudication>,
    /// Exact event prefix length.
    pub event_count: usize,
    /// Known board prefix length.
    pub board_ply: usize,
    /// Assessment tied to this exact historical position.
    pub no_win_assessment: NoWinAssessment,
}
/// Actual runtime acceptance identity; never serialized as canonical history.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeToken {
    instance: u64,
    revision: u64,
}
/// A request matches full root, policy, commands and a never-rewound runtime token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameContext {
    root: PositionRoot,
    policy: ClaimPolicy,
    commands: Vec<CommandEntry>,
    token: RuntimeToken,
}
/// Context-bound participant request, separate from derived warning/conclusion events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRequest {
    /// Complete submitted context.
    pub expected: GameContext,
    /// Native actor.
    pub actor: Side,
    /// Complete command.
    pub command: GameCommand,
}
/// Command result distinguishes rejection audit from accepted chess actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandResult {
    /// Accepted native command.
    Accepted,
    /// Invalid admitted claim; semantic game state is unchanged.
    RejectedNaturalCount,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CommandEntry {
    pub actor: Side,
    pub command: GameCommand,
    pub result: CommandResult,
    pub event_end: usize,
}
/// Admission refusal leaves both game and audit untouched.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameError {
    /// Request has retired runtime work or a different full history.
    StaleContext,
    /// Already concluded; no overwrite is permitted.
    Concluded,
    /// Required automatic adjudication is unresolved and unscored.
    AdjudicationPending,
    /// Nonmoving participant tried a board action.
    WrongActor,
    /// Local rules or declared representation rejected the action.
    Action(ActionError),
    /// Root fails the native local-rule domain.
    Root(LocalPositionError),
    /// No opponent offer is available to accept/withdraw.
    DrawOffer,
    /// A pre-commit native record/event/command/token limit was reached.
    Capacity,
    /// Historical selection is not the end of a committed native command.
    /// Automatic derived events remain viewable through `event_prefix`.
    NotCommandBoundary,
    /// A native invariant failed; no result is invented.
    Invariant,
}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xiangqi game request rejected: {self:?}")
    }
}
impl Error for GameError {}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Checkpoint {
    position: Position,
    ids: [Option<u8>; 90],
    command_len: usize,
    event_len: usize,
    step_len: usize,
    warnings: Vec<Warning>,
    offered_by: Option<Side>,
    conclusion: Option<Conclusion>,
    pending: Option<PendingAdjudication>,
    assessment: NoWinAssessment,
    last_offer_ply: [Option<usize>; 2],
}
/// Consumed local reversal token. Undo retires execution work instead of restoring its token.
#[derive(Debug)]
pub struct GameUndo {
    before: Checkpoint,
    after: Checkpoint,
    after_commands: Vec<CommandEntry>,
    instance: u64,
    after_lineage: Vec<u64>,
}
/// Admitted result with exact reversal, including rejected-claim audit entries.
#[derive(Debug)]
pub struct Submission {
    /// Accepted command versus semantically rejected claim.
    pub result: CommandResult,
    /// Exact native reversal token.
    pub undo: GameUndo,
}
/// Bounded native known game. Each concrete consumer advertises its own capabilities.
#[derive(Debug)]
pub struct Game {
    pub(super) root: PositionRoot,
    pub(super) policy: ClaimPolicy,
    pub(super) position: Position,
    ids: [Option<u8>; 90],
    pub(super) commands: Vec<CommandEntry>,
    pub(super) events: Vec<GameEvent>,
    snapshots: Vec<HistorySnapshot>,
    steps: Vec<Step>,
    identities: Vec<BoardIdentity>,
    warnings: Vec<Warning>,
    offered_by: Option<Side>,
    pub(super) conclusion: Option<Conclusion>,
    pending: Option<PendingAdjudication>,
    assessment: NoWinAssessment,
    token: RuntimeToken,
    lineage: Vec<u64>,
    last_offer_ply: [Option<usize>; 2],
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct BoardIdentity {
    board: [Option<super::Piece>; 90],
    side: Side,
}
impl From<&Position> for BoardIdentity {
    fn from(p: &Position) -> Self {
        Self {
            board: p.board,
            side: p.side_to_move(),
        }
    }
}
impl Game {
    /// Admit a known root without inventing pre-root repetitions or checks.
    pub fn from_root(root: PositionRoot, policy: ClaimPolicy) -> Result<Self, GameError> {
        root.position().validate_local().map_err(GameError::Root)?;
        let instance = NEXT_INSTANCE
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .map_err(|_| GameError::Capacity)?;
        let position = root.position().clone();
        let mut ids = [None; 90];
        for (id, (at, _)) in position.pieces().enumerate() {
            ids[at.index()] = Some(id as u8);
        }
        let assessment = super::assess_natural_win(&position);
        let mut game = Self {
            assessment,
            root,
            policy,
            identities: vec![BoardIdentity::from(&position)],
            position,
            ids,
            commands: vec![],
            events: vec![],
            snapshots: vec![],
            steps: vec![],
            warnings: vec![],
            offered_by: None,
            conclusion: None,
            pending: None,
            lineage: vec![],
            last_offer_ply: [None; 2],
            token: RuntimeToken {
                instance,
                revision: 0,
            },
        };
        game.resolve_board_terminal()?;
        if game.conclusion.is_none() {
            game.resolve_fast_no_win();
        }
        Ok(game)
    }
    /// Standard root, preserving the selected unattended claim policy.
    pub fn standard(policy: ClaimPolicy) -> Result<Self, GameError> {
        Self::from_root(PositionRoot::standard(), policy)
    }
    /// Recorded optional/automatic native claim policy, independent of turn.
    pub const fn claim_policy(&self) -> ClaimPolicy {
        self.policy
    }
    /// Exact authoritative current position.
    pub fn position(&self) -> &Position {
        &self.position
    }
    /// Immutable beginning of known history.
    pub fn root(&self) -> &PositionRoot {
        &self.root
    }
    /// Committed prefix before the participant's latest playable decision.
    /// A terminal command on their own playable turn can be reversed alone;
    /// otherwise the latest ordinary board action and all following commands
    /// are reversed together. Derived events never split an atomic command.
    pub fn preceding_playable_decision(&self, actor: Side) -> Option<usize> {
        let terminal = self.commands.last().filter(|entry| {
            entry.actor == actor
                && self.conclusion.is_some()
                && self.position.side_to_move() == actor
                && matches!(
                    entry.command,
                    GameCommand::Resign | GameCommand::AcceptDraw | GameCommand::ClaimNaturalCount
                )
                && self
                    .position
                    .legal_actions()
                    .is_ok_and(|actions| !actions.is_empty())
        });
        let target = if terminal.is_some() {
            self.commands.len().checked_sub(1)?
        } else {
            self.commands.iter().rposition(|entry| {
                entry.actor == actor && matches!(entry.command, GameCommand::Play { .. })
            })?
        };
        // Root admission emits events only when it concludes the game, which
        // forbids every command. An accepted command therefore starts at zero;
        // this advisory query must not reconstruct a game or allocate identities.
        Some(if target == 0 {
            0
        } else {
            self.commands[target - 1].event_end
        })
    }
    /// Canonical event history, distinct from board ply.
    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }
    /// Current committed warning state; historical inactive warnings are retained.
    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }
    /// Explicit native outcome, if concluded.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.conclusion
    }
    /// Classification that still prevents honest automatic adjudication.
    pub fn pending_adjudication(&self) -> Option<&PendingAdjudication> {
        self.pending.as_ref()
    }
    /// No exact capability is claimed before the selected question/backend exists.
    pub fn no_win_assessment(&self) -> NoWinAssessment {
        self.assessment.clone()
    }
    /// Exact native request identity, not a board/FEN digest.
    pub fn context(&self) -> GameContext {
        GameContext {
            root: self.root.clone(),
            policy: self.policy,
            commands: self.commands.clone(),
            token: self.token,
        }
    }
    /// Query claimant-specific known count without submitting or accepting a claim.
    pub fn natural_count_eligibility(&self, claimant: Side) -> NaturalCountEligibility {
        let since = self
            .steps
            .iter()
            .rposition(|step| step.before.piece(step.action.to).is_some())
            .map_or(0, |i| i + 1);
        let window = &self.steps[since..];
        let claimant_checks = window
            .iter()
            .filter(|step| step.actor == claimant && step.after.is_in_check(claimant.opponent()))
            .count();
        let effective_plies = window
            .len()
            .saturating_sub(claimant_checks.saturating_sub(10));
        NaturalCountEligibility {
            claimant,
            known_plies: window.len(),
            claimant_checks,
            effective_plies,
            eligible: effective_plies >= 100,
        }
    }
    /// Advisory offer admission for this actor. Querying never commits an offer;
    /// the formal request still requires the complete current game context.
    pub fn can_offer_draw(&self, actor: Side) -> bool {
        self.conclusion.is_none()
            && self.pending.is_none()
            && self.steps.last().is_some_and(|step| step.actor == actor)
            && self.offered_by.is_none()
            && self.last_offer_ply[actor.index()] != Some(self.steps.len())
    }
    /// Whether this actor may accept or decline the opponent's current offer.
    /// This is independent of board-action turn ownership.
    pub fn can_respond_to_draw(&self, actor: Side) -> bool {
        self.conclusion.is_none()
            && self.pending.is_none()
            && self.offered_by == Some(actor.opponent())
    }
    fn transaction_copy(&self) -> Self {
        Self {
            root: self.root.clone(),
            policy: self.policy,
            position: self.position.clone(),
            ids: self.ids,
            commands: self.commands.clone(),
            events: self.events.clone(),
            snapshots: self.snapshots.clone(),
            steps: self.steps.clone(),
            identities: self.identities.clone(),
            warnings: self.warnings.clone(),
            offered_by: self.offered_by,
            conclusion: self.conclusion,
            pending: self.pending.clone(),
            assessment: self.assessment.clone(),
            token: self.token,
            lineage: self.lineage.clone(),
            last_offer_ply: self.last_offer_ply,
        }
    }
    fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            position: self.position.clone(),
            ids: self.ids,
            command_len: self.commands.len(),
            event_len: self.events.len(),
            step_len: self.steps.len(),
            warnings: self.warnings.clone(),
            offered_by: self.offered_by,
            conclusion: self.conclusion,
            pending: self.pending.clone(),
            assessment: self.assessment.clone(),
            last_offer_ply: self.last_offer_ply,
        }
    }
    fn snapshot(&self) -> HistorySnapshot {
        HistorySnapshot {
            position: self.position.clone(),
            warnings: self.warnings.clone(),
            offered_by: self.offered_by,
            conclusion: self.conclusion,
            pending: self.pending.clone(),
            event_count: self.events.len(),
            board_ply: self.steps.len(),
            no_win_assessment: self.assessment.clone(),
        }
    }
    fn emit(&mut self, actor: Option<Side>, event: EventKind) {
        self.events.push(GameEvent {
            index: self.events.len(),
            board_ply: self.steps.len(),
            actor,
            event,
        });
        self.snapshots.push(self.snapshot());
    }
    fn conclude(&mut self, conclusion: Conclusion) {
        self.conclusion = Some(conclusion);
        self.emit(None, EventKind::Concluded { conclusion });
    }
    fn resolve_board_terminal(&mut self) -> Result<(), GameError> {
        if let LocalStatus::NoLegalAction { reason } =
            self.position.local_status().map_err(GameError::Root)?
        {
            self.conclude(Conclusion::Win {
                winner: self.position.side_to_move().opponent(),
                reason: WinReason::NoLegalAction { reason },
            });
        }
        Ok(())
    }
    fn resolve_fast_no_win(&mut self) {
        if matches!(self.assessment, NoWinAssessment::ProvenNeitherCanWin) {
            self.conclude(Conclusion::Draw {
                reason: DrawReason::NeitherCanWin,
            });
        }
    }
    fn occurrences(&self) -> Vec<usize> {
        let key = self.identities.last().expect("known root");
        self.identities
            .iter()
            .enumerate()
            .filter_map(|(i, k)| (k == key).then_some(i))
            .collect()
    }
    fn resolve_repetition(&mut self) -> Result<(), GameError> {
        // An unanswered response is not proof of variation. Conversely a known
        // non-chase/check change need not wait for the next anchor return.
        let latest = self.steps.last().expect("accepted board action");
        let captured = latest.before.piece(latest.action.to).is_some();
        let mut retired = Vec::new();
        for warning in &mut self.warnings {
            if !warning.active {
                continue;
            }
            let start = warning.occurrences[0];
            let varied = captured
                || match warning.fault {
                    FaultKind::PerpetualCheck => {
                        latest.actor == warning.side
                            && !latest.after.is_in_check(warning.side.opponent())
                    }
                    FaultKind::PerpetualChase => {
                        match conduct::continues_chase_prefix(&self.steps[start..], warning.side) {
                            Ok(not_yet_disproved) => !not_yet_disproved,
                            Err(conduct::ConductError::Invariant(_)) => {
                                return Err(GameError::Invariant);
                            }
                            Err(conduct::ConductError::Unresolved(_)) => false,
                        }
                    }
                };
            if varied {
                warning.active = false;
                retired.push(warning.event_index);
            }
        }
        for warning_event_index in retired {
            self.emit(
                None,
                EventKind::WarningRetired {
                    warning_event_index,
                },
            );
        }
        let occurrences = self.occurrences();
        if occurrences.len() < 3 {
            return Ok(());
        }
        let start = occurrences[0];
        let responsibility = match conduct::responsibility(&self.steps[start..]) {
            Ok(value) => value,
            Err(conduct::ConductError::Invariant(_)) => return Err(GameError::Invariant),
            Err(error) => {
                let pending = PendingAdjudication {
                    occurrences,
                    reason: error.to_string().chars().take(256).collect(),
                };
                self.pending = Some(pending.clone());
                self.emit(None, EventKind::AdjudicationPending { pending });
                return Ok(());
            }
        };
        if occurrences.len() == 3 {
            if let Responsibility::OneSided { side, kind } = responsibility
                && !self.warnings.iter().any(|w| {
                    w.active && w.side == side && w.fault == kind && w.occurrences[0] == start
                })
            {
                let warning = Warning {
                    side,
                    fault: kind,
                    occurrences,
                    event_index: self.events.len(),
                    active: true,
                };
                self.warnings.push(warning.clone());
                self.emit(None, EventKind::Warning { warning });
            }
        } else {
            let continuing = if let Responsibility::OneSided { side, kind } = responsibility {
                self.warnings
                    .iter()
                    .any(|w| {
                        w.active && w.side == side && w.fault == kind && w.occurrences[0] == start
                    })
                    .then_some(Conclusion::Win {
                        winner: side.opponent(),
                        reason: WinReason::Repetition { fault: kind },
                    })
            } else {
                None
            };
            self.conclude(continuing.unwrap_or(Conclusion::Draw {
                reason: DrawReason::Repetition,
            }));
        }
        Ok(())
    }
    fn claim(&mut self, actor: Side, automatic: bool) -> CommandResult {
        let evidence = self.natural_count_eligibility(actor);
        self.emit(
            Some(actor),
            EventKind::ClaimSubmitted {
                evidence,
                automatic,
            },
        );
        if evidence.eligible {
            self.emit(Some(actor), EventKind::ClaimAccepted { evidence });
            self.conclude(Conclusion::Draw {
                reason: DrawReason::NaturalCount,
            });
            CommandResult::Accepted
        } else {
            self.emit(Some(actor), EventKind::ClaimRejected { evidence });
            CommandResult::RejectedNaturalCount
        }
    }
    /// Serialize admission, actions, automatic events and claims atomically. Invalid
    /// admitted claims retain only their bounded audit; no board state or result changes.
    pub fn submit(&mut self, request: GameRequest) -> Result<Submission, GameError> {
        if request.expected != self.context() {
            return Err(GameError::StaleContext);
        }
        if self.conclusion.is_some() {
            return Err(GameError::Concluded);
        }
        if self.pending.is_some() {
            return Err(GameError::AdjudicationPending);
        }
        let revision = self
            .token
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity)?;
        let before = self.checkpoint();
        let mut candidate = self.transaction_copy();
        let result = match request.command {
            GameCommand::Play { action } => {
                if request.actor != self.position.side_to_move() {
                    return Err(GameError::WrongActor);
                }
                let before_position = candidate.position.clone();
                let before_ids = candidate.ids;
                let moving_id = candidate.ids[action.from.index()]
                    .ok_or(GameError::Action(ActionError::EmptySource))?;
                let captured_id = candidate.ids[action.to.index()];
                candidate
                    .position
                    .apply(action)
                    .map_err(GameError::Action)?;
                candidate.assessment = super::assess_natural_win(&candidate.position);
                candidate.ids[action.from.index()] = None;
                candidate.ids[action.to.index()] = Some(moving_id);
                candidate.steps.push(Step {
                    before: before_position,
                    after: candidate.position.clone(),
                    action,
                    actor: request.actor,
                    ids_before: before_ids,
                    ids_after: candidate.ids,
                });
                candidate
                    .identities
                    .push(BoardIdentity::from(&candidate.position));
                candidate.offered_by = None;
                let gave_check = candidate.position.is_in_check(request.actor.opponent());
                candidate.emit(
                    Some(request.actor),
                    EventKind::BoardAction {
                        action,
                        moving_id,
                        captured_id,
                        gave_check,
                    },
                );
                candidate.resolve_board_terminal()?;
                if candidate.conclusion.is_none() {
                    candidate.resolve_repetition()?;
                }
                if candidate.conclusion.is_none() && candidate.pending.is_none() {
                    candidate.resolve_fast_no_win();
                }
                if candidate.conclusion.is_none()
                    && candidate.pending.is_none()
                    && candidate.policy == ClaimPolicy::AutoValidClaims
                {
                    for claimant in [Side::Red, Side::Black] {
                        if candidate.natural_count_eligibility(claimant).eligible {
                            candidate.claim(claimant, true);
                            break;
                        }
                    }
                }
                CommandResult::Accepted
            }
            GameCommand::Resign => {
                candidate.emit(Some(request.actor), EventKind::Resigned);
                candidate.conclude(Conclusion::Win {
                    winner: request.actor.opponent(),
                    reason: WinReason::Resignation,
                });
                CommandResult::Accepted
            }
            GameCommand::OfferDraw => {
                if !candidate.can_offer_draw(request.actor) {
                    return Err(GameError::DrawOffer);
                }
                candidate.last_offer_ply[request.actor.index()] = Some(candidate.steps.len());
                candidate.offered_by = Some(request.actor);
                candidate.emit(
                    Some(request.actor),
                    EventKind::DrawOffer {
                        offered_by: candidate.offered_by,
                    },
                );
                CommandResult::Accepted
            }
            GameCommand::DeclineDraw => {
                if !candidate.can_respond_to_draw(request.actor) {
                    return Err(GameError::DrawOffer);
                }
                candidate.offered_by = None;
                candidate.emit(
                    Some(request.actor),
                    EventKind::DrawOffer { offered_by: None },
                );
                CommandResult::Accepted
            }
            GameCommand::AcceptDraw => {
                if !candidate.can_respond_to_draw(request.actor) {
                    return Err(GameError::DrawOffer);
                }
                candidate.offered_by = None;
                candidate.emit(Some(request.actor), EventKind::AgreementAccepted);
                candidate.conclude(Conclusion::Draw {
                    reason: DrawReason::Agreement,
                });
                CommandResult::Accepted
            }
            GameCommand::ClaimNaturalCount => candidate.claim(request.actor, false),
        };
        candidate.commands.push(CommandEntry {
            actor: request.actor,
            command: request.command,
            result,
            event_end: candidate.events.len(),
        });
        let terminal = candidate.conclusion.is_some();
        let command_limit = MAX_GAME_COMMANDS - usize::from(!terminal);
        let event_limit = MAX_GAME_EVENTS - if terminal { 0 } else { 4 };
        if candidate.commands.len() > command_limit || candidate.events.len() > event_limit {
            return Err(GameError::Capacity);
        }
        let encoded = super::record::GameRecord::encoded_candidate(&candidate)
            .map_err(|_| GameError::Capacity)?;
        if !terminal && encoded.len() > super::record::MAX_GAME_RECORD_BYTES - 4096 {
            return Err(GameError::Capacity);
        }
        candidate.token.revision = revision;
        candidate.lineage.push(revision);
        let after_lineage = candidate.lineage.clone();
        let after = candidate.checkpoint();
        let after_commands = candidate.commands.clone();
        let instance = candidate.token.instance;
        *self = candidate;
        Ok(Submission {
            result,
            undo: GameUndo {
                before,
                after,
                after_commands,
                instance,
                after_lineage,
            },
        })
    }
    /// Exact semantic reversal with a fresh monotonic runtime revision.
    pub fn undo(&mut self, undo: GameUndo) -> Result<(), GameError> {
        if self.token.instance != undo.instance
            || self.checkpoint() != undo.after
            || self.commands != undo.after_commands
            || self.lineage != undo.after_lineage
        {
            return Err(GameError::StaleContext);
        }
        let revision = self
            .token
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity)?;
        let b = undo.before;
        self.position = b.position;
        self.ids = b.ids;
        self.commands.truncate(b.command_len);
        self.lineage.truncate(b.command_len);
        self.events.truncate(b.event_len);
        self.snapshots.truncate(b.event_len);
        self.steps.truncate(b.step_len);
        self.identities.truncate(b.step_len + 1);
        self.warnings = b.warnings;
        self.offered_by = b.offered_by;
        self.conclusion = b.conclusion;
        self.pending = b.pending;
        self.assessment = b.assessment;
        self.last_offer_ply = b.last_offer_ply;
        self.token.revision = revision;
        Ok(())
    }
    /// Reconstruct a complete committed command prefix with fresh runtime identity.
    ///
    /// Event selections inside an atomic command's derived events cannot be
    /// resumed as a game and return `NotCommandBoundary`. Root admission may itself
    /// emit automatic events; those are indivisible too. Use `event_prefix` for
    /// arbitrary immutable historical views. Reconstruction preserves the root,
    /// declared counters, known history and claim policy without future events,
    /// and performs only the normal bounded native admission/replay path.
    pub fn committed_prefix(&self, event_count: usize) -> Result<Self, GameError> {
        if event_count > self.events.len() {
            return Err(GameError::NotCommandBoundary);
        }
        let mut prefix = Self::from_root(self.root.clone(), self.policy)?;
        if prefix.events.len() == event_count {
            return Ok(prefix);
        }
        if !self
            .commands
            .iter()
            .any(|entry| entry.event_end == event_count)
        {
            return Err(GameError::NotCommandBoundary);
        }
        for entry in &self.commands {
            if entry.event_end > event_count {
                break;
            }
            let submitted = prefix.submit(GameRequest {
                expected: prefix.context(),
                actor: entry.actor,
                command: entry.command,
            })?;
            if submitted.result != entry.result || prefix.events.len() != entry.event_end {
                return Err(GameError::Invariant);
            }
            if prefix.events.len() == event_count {
                if prefix.events != self.events[..event_count] {
                    return Err(GameError::Invariant);
                }
                return Ok(prefix);
            }
        }
        Err(GameError::NotCommandBoundary)
    }
    /// Immutable exact event-prefix view. It cannot submit actions or revive work.
    /// Automatic warning/conclusion boundaries can share a board ply.
    pub fn event_prefix(&self, count: usize) -> Result<HistorySnapshot, GameError> {
        if count > self.events.len() {
            return Err(GameError::Invariant);
        }
        if count == 0 {
            return Ok(HistorySnapshot {
                position: self.root.position().clone(),
                warnings: vec![],
                offered_by: None,
                conclusion: None,
                pending: None,
                event_count: 0,
                board_ply: 0,
                no_win_assessment: super::assess_natural_win(self.root.position()),
            });
        }
        Ok(self.snapshots[count - 1].clone())
    }
}
