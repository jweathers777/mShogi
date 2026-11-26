//! Known native commands and events. A position and a declared counter are not history.
use super::conduct::{self, FaultKind, Responsibility, Step};
use super::{
    Action, ActionError, LocalPositionError, LocalStatus, NoActionReason, NoWinAssessment,
    Position, PositionRoot, Side,
};
pub use crate::native::GameContext;
use crate::native::history::{AppendGame, Authority, GameUndo as Binding};
use crate::repetition::RepetitionIndex;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};
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
    /// Submit an Article 8 claim for this actor, regardless of turn.
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
    /// Valid claimant-specific Article 8 request.
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
    /// Whether the independently known count reaches 100.
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
/// Context-bound participant request, separate from derived warning/conclusion events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRequest {
    /// Request token of the game when the command was chosen.
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
    /// The request token is not the current one.
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
#[derive(Debug)]
pub(crate) struct Checkpoint {
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
/// Consumed local reversal, bound to the game instance and the history node after
/// its command. Undo retires requests instead of restoring their token.
#[derive(Debug)]
pub struct GameUndo {
    before: Checkpoint,
    binding: Binding,
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
#[derive(Clone, Debug)]
pub struct Game {
    pub(super) root: PositionRoot,
    pub(super) policy: ClaimPolicy,
    pub(super) position: Position,
    ids: [Option<u8>; 90],
    pub(super) commands: Vec<CommandEntry>,
    pub(super) events: Vec<GameEvent>,
    snapshots: Vec<HistorySnapshot>,
    steps: Vec<Step>,
    identities: RepetitionIndex<BoardIdentity>,
    warnings: Vec<Warning>,
    offered_by: Option<Side>,
    pub(super) conclusion: Option<Conclusion>,
    pending: Option<PendingAdjudication>,
    assessment: NoWinAssessment,
    authority: Authority,
    last_offer_ply: [Option<usize>; 2],
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
            identities: RepetitionIndex::new(BoardIdentity::from(&position)),
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
            authority: Authority::default(),
            last_offer_ply: [None; 2],
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
    /// Exact before/after event-prefix counts for each admitted command.
    /// Event array indices occupy [start, end); their prefix IDs occupy (start, end].
    /// A board-action prefix inside this range is independently viewable, but
    /// is not thereby a committed execution context. No replay is performed.
    pub fn command_event_ranges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.commands.iter().scan(0, |start, command| {
            let range = (*start, command.event_end);
            *start = command.event_end;
            Some(range)
        })
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
    /// Sound fast no-win assessment of the current position; Unknown is not a result.
    pub fn no_win_assessment(&self) -> NoWinAssessment {
        self.assessment.clone()
    }
    /// Current request token; it changes at each accepted command and undo.
    pub fn context(&self) -> GameContext {
        self.authority.context()
    }
    /// Invalidate outstanding requests without changing the command history.
    pub fn retire_context(&mut self) {
        self.authority.retire();
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
    /// the formal request still requires the current request token.
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
        let occurrences = self.identities.current_occurrences();
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
        let (result, before, binding) =
            self.append_transaction(|game| game.apply_command(request))?;
        Ok(Submission {
            result,
            undo: GameUndo { before, binding },
        })
    }
    fn apply_command(&mut self, request: GameRequest) -> Result<CommandResult, GameError> {
        let result = match request.command {
            GameCommand::Play { action } => {
                if request.actor != self.position.side_to_move() {
                    return Err(GameError::WrongActor);
                }
                let before_position = self.position.clone();
                let before_ids = self.ids;
                let moving_id = self.ids[action.from.index()]
                    .ok_or(GameError::Action(ActionError::EmptySource))?;
                let captured_id = self.ids[action.to.index()];
                self.position.apply(action).map_err(GameError::Action)?;
                self.assessment = super::assess_natural_win(&self.position);
                self.ids[action.from.index()] = None;
                self.ids[action.to.index()] = Some(moving_id);
                self.steps.push(Step {
                    before: before_position,
                    after: self.position.clone(),
                    action,
                    actor: request.actor,
                    ids_before: before_ids,
                    ids_after: self.ids,
                });
                self.identities.push(BoardIdentity::from(&self.position));
                self.offered_by = None;
                let gave_check = self.position.is_in_check(request.actor.opponent());
                self.emit(
                    Some(request.actor),
                    EventKind::BoardAction {
                        action,
                        moving_id,
                        captured_id,
                        gave_check,
                    },
                );
                self.resolve_board_terminal()?;
                if self.conclusion.is_none() {
                    self.resolve_repetition()?;
                }
                if self.conclusion.is_none() && self.pending.is_none() {
                    self.resolve_fast_no_win();
                }
                if self.conclusion.is_none()
                    && self.pending.is_none()
                    && self.policy == ClaimPolicy::AutoValidClaims
                {
                    for claimant in [Side::Red, Side::Black] {
                        if self.natural_count_eligibility(claimant).eligible {
                            self.claim(claimant, true);
                            break;
                        }
                    }
                }
                CommandResult::Accepted
            }
            GameCommand::Resign => {
                self.emit(Some(request.actor), EventKind::Resigned);
                self.conclude(Conclusion::Win {
                    winner: request.actor.opponent(),
                    reason: WinReason::Resignation,
                });
                CommandResult::Accepted
            }
            GameCommand::OfferDraw => {
                if !self.can_offer_draw(request.actor) {
                    return Err(GameError::DrawOffer);
                }
                self.last_offer_ply[request.actor.index()] = Some(self.steps.len());
                self.offered_by = Some(request.actor);
                self.emit(
                    Some(request.actor),
                    EventKind::DrawOffer {
                        offered_by: self.offered_by,
                    },
                );
                CommandResult::Accepted
            }
            GameCommand::DeclineDraw => {
                if !self.can_respond_to_draw(request.actor) {
                    return Err(GameError::DrawOffer);
                }
                self.offered_by = None;
                self.emit(
                    Some(request.actor),
                    EventKind::DrawOffer { offered_by: None },
                );
                CommandResult::Accepted
            }
            GameCommand::AcceptDraw => {
                if !self.can_respond_to_draw(request.actor) {
                    return Err(GameError::DrawOffer);
                }
                self.offered_by = None;
                self.emit(Some(request.actor), EventKind::AgreementAccepted);
                self.conclude(Conclusion::Draw {
                    reason: DrawReason::Agreement,
                });
                CommandResult::Accepted
            }
            GameCommand::ClaimNaturalCount => self.claim(request.actor, false),
        };
        self.commands.push(CommandEntry {
            actor: request.actor,
            command: request.command,
            result,
            event_end: self.events.len(),
        });
        Ok(result)
    }
    /// Exact semantic reversal with a fresh monotonic runtime revision.
    pub fn undo(&mut self, undo: GameUndo) -> Result<(), GameError> {
        if !self.authority.undo(&undo.binding) {
            return Err(GameError::StaleContext);
        }
        self.restore(undo.before);
        Ok(())
    }
    /// Exact copy of the complete committed game with a fresh runtime identity.
    ///
    /// The copy has the same root, policy, commands, events and derived state,
    /// without replay. Reversal tokens of this game do not apply to the copy.
    /// Use it for private analysis copies; it admits nothing new.
    pub fn fork(&self) -> Self {
        self.clone()
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

impl AppendGame for Game {
    type Checkpoint = Checkpoint;
    fn authority(&mut self) -> &mut Authority {
        &mut self.authority
    }
    fn checkpoint(&mut self) -> Checkpoint {
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
    fn restore(&mut self, b: Checkpoint) {
        self.position = b.position;
        self.ids = b.ids;
        self.commands.truncate(b.command_len);
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
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::*;

    #[test]
    fn late_refusal_restores_the_board_and_all_appended_events() {
        let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
        let first = GameRequest {
            expected: game.context(),
            actor: Side::Red,
            command: GameCommand::Play {
                action: game.position().legal_actions().unwrap()[0],
            },
        };
        let undo = game.submit(first).unwrap().undo;
        let before = game.clone();
        let expected = game.context();
        let play = GameRequest {
            expected,
            actor: Side::Black,
            command: GameCommand::Play {
                action: game.position().legal_actions().unwrap()[0],
            },
        };
        // Model a late invariant error after a board event and a multi-event
        // control. This exercises the same append transaction used by submit.
        let refused = game.append_transaction(|game| {
            game.apply_command(play.clone())?;
            game.apply_command(GameRequest {
                expected,
                actor: Side::Red,
                command: GameCommand::Resign,
            })?;
            Err::<(), _>(GameError::Invariant)
        });
        assert!(matches!(refused, Err(GameError::Invariant)));
        assert_eq!(game.context(), expected);
        assert_eq!(game.snapshot(), before.snapshot());
        assert_eq!(game.commands, before.commands);
        assert_eq!(game.events, before.events);
        assert_eq!(game.snapshots, before.snapshots);
        assert_eq!(game.steps, before.steps);
        assert_eq!(game.ids, before.ids);
        assert_eq!(game.identities, before.identities);
        let next = game.submit(play).unwrap().undo;
        game.undo(next).unwrap();
        game.undo(undo).unwrap();
        assert!(game.events().is_empty());
    }
}
