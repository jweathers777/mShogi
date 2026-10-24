//! Rules-owned commands and injected monotonic clocks; no process/engine I/O.
use super::*;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// One-sided mating feasibility, independent of force-mate strength.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatingFeasibility {
    /// A verified cooperative legal continuation delivers mate.
    Possible,
    /// A sound impossibility proof applies.
    Impossible,
    /// Neither result is established; never score it as a win or draw.
    Unknown,
}
/// Validate a positive Article-3 legal-move witness, ignoring hypothetical
/// claims/50/75 counters. Counter storage limits do not change mating geometry.
/// No witness means only bare-king/global-dead proofs are considered in M2.
pub fn mating_feasibility(
    position: &Position,
    side: Color,
    witness: Option<&[Action]>,
) -> Result<MatingFeasibility, &'static str> {
    let bare = !(0..8).any(|r| {
        (0..8).any(|f| {
            position
                .piece_at(Square::new(f, r).unwrap())
                .is_some_and(|p| p.color == side && p.kind != PieceKind::King)
        })
    });
    if bare || position.dead_recognition() == DeadRecognition::ProvenDead {
        return Ok(MatingFeasibility::Impossible);
    }
    let Some(witness) = witness else {
        return Ok(MatingFeasibility::Unknown);
    };
    let mut p = position.clone();
    for &a in witness {
        p.halfmove_clock = 0;
        p.fullmove_number = 1;
        p.apply(a).map_err(|_| "illegal mating witness")?;
    }
    if p.classify() == Classification::Checkmate && p.side_to_move().opponent() == side {
        Ok(MatingFeasibility::Possible)
    } else {
        Err("witness does not establish the requested side's mate")
    }
}

/// Software clock control, without delay/stages/lag compensation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum TimeControl {
    /// No chess-clock deadline; protocol watchdogs remain a runner concern.
    Untimed,
    /// Single-stage Fischer increment, White then Black initial balances.
    Increment {
        /// Positive initial nanoseconds for each side.
        initial_ns: [u64; 2],
        /// Added once after each accepted nonterminal move.
        increment_ns: u64,
    },
}
/// Trigger requiring side-specific possible-mate adjudication.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminationTrigger {
    /// Explicit resignation.
    Resignation,
    /// Chess-clock expiry, distinct from process watchdog expiry.
    FlagFall,
}
/// Match conclusion, including explicitly unscored uncertainty.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum MatchConclusion {
    /// Rules-owned automatic outcome or exercised claim.
    Chess(ChessOutcome),
    /// Opponent's mating possibility was established.
    Win {
        /// Winning color.
        winner: Color,
        /// Why play stopped.
        trigger: TerminationTrigger,
    },
    /// Opponent cannot mate, or both players agreed a draw.
    Draw {
        /// Cause; None denotes agreement.
        trigger: Option<TerminationTrigger>,
    },
    /// No exact chess score can be justified.
    Unresolved {
        /// Resigning/flagged player.
        actor: Color,
        /// Why play stopped.
        trigger: TerminationTrigger,
    },
}
impl MatchConclusion {
    /// Conventional White score doubled (0/1/2), or None for unscored uncertainty.
    pub fn white_score_twice(self) -> Option<u8> {
        match self {
            Self::Unresolved { .. } => None,
            Self::Chess(ChessOutcome::Draw(_)) | Self::Draw { .. } => Some(1),
            Self::Chess(ChessOutcome::Checkmate { winner }) | Self::Win { winner, .. } => {
                Some(if winner == Color::White { 2 } else { 0 })
            }
        }
    }
}
/// Complete match command, separate from board effects.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum MatchCommand {
    /// Start the active clock immediately before its first move request.
    Start,
    /// Complete player action resolved by the live rules boundary.
    Move(Action),
    /// Exercise an optional draw claim.
    Claim {
        /// Named rule.
        reason: DrawClaim,
        /// Unplayed intention, if any.
        intended: Option<Action>,
    },
    /// Resign, optionally supplying a cooperative opponent-mating certificate.
    Resign {
        /// Legal continuation proving opponent mate, not an evaluation.
        witness: Option<Vec<Action>>,
    },
    /// Create an offer which persists until opponent action/rejection/conclusion.
    OfferDraw,
    /// Accept an opponent offer after both recorded first actions.
    AcceptDraw,
    /// Explicitly reject the opponent's offer.
    DeclineDraw,
    /// Observe elapsed clock time. Witness may justify the opponent's ability
    /// to mate on expiry; it never changes the board.
    Observe {
        /// Optional positive mating certificate for the nonmoving side.
        witness: Option<Vec<Action>>,
    },
}
/// Versioned actor request with observed integer timestamp.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchRequest {
    /// Nanoseconds since the recorded monotonic origin.
    pub at_ns: u64,
    /// Version observed by the caller before submitting this event.
    pub version: u64,
    /// Command's actor; controller events use the current moving side.
    pub actor: Color,
    /// Requested operation.
    pub command: MatchCommand,
}
/// Stable machine rejection category; detail is diagnostic only.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchRejectCode {
    /// A previous conclusion is final.
    GameOver,
    /// Request based on an old context.
    WrongVersion,
    /// Player does not own this command.
    WrongActor,
    /// Clock not started yet.
    NotStarted,
    /// Start already occurred.
    AlreadyStarted,
    /// Invalid claim.
    InvalidClaim,
    /// Invalid or unrepresentable board action.
    InvalidMove,
    /// Offer unavailable.
    NoOffer,
    /// Existing offer cannot be withdrawn by a new offer.
    OfferPending,
    /// Both recorded first actions have not occurred.
    OfferTooEarly,
    /// Invalid positive mating certificate.
    InvalidWitness,
    /// Timestamp went backwards.
    NonmonotonicTime,
    /// Increment arithmetic cannot be represented.
    ClockOverflow,
}
/// Rejection plus a human diagnostic.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchRejection {
    /// Stable category.
    pub code: MatchRejectCode,
    /// Human explanation, not a parsed rule identity.
    pub detail: String,
}
/// Observed result after one serialized request (including rejected requests).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchReply {
    /// New event version; ordered observations, not just board plies, advance it.
    pub version: u64,
    /// White/Black nanoseconds, None when untimed.
    pub remaining_ns: Option<[u64; 2]>,
    /// Active side, None when stopped.
    pub active: Option<Color>,
    /// Final result if established.
    pub conclusion: Option<MatchConclusion>,
    /// Rejection does not imply a chess outcome.
    pub rejection: Option<MatchRejection>,
    /// Generated full-solver proof/witness or unfinished work, when requested.
    /// Legacy M2 replies omit this field and preserve their original encoding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feasibility_evidence: Option<MateReachability>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MatchEvent {
    request: MatchRequest,
    reply: MatchReply,
}
/// Replayable match envelope. Initial canonical history is embedded as a string
/// so its own strict decoder retains duplicate-field rejection.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchRecord {
    schema_version: u32,
    rules_profile: String,
    initial_record_json: String,
    time_control: TimeControl,
    events: Vec<MatchEvent>,
}
/// Invalid match configuration or inconsistent replay evidence.
#[derive(Debug)]
pub struct MatchRecordError(pub String);
impl fmt::Display for MatchRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl Error for MatchRecordError {}

/// Rules-owned serialized match state. All board changes go through
/// AdjudicatedGame; command rejection still charges elapsed active time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChessMatch {
    game: AdjudicatedGame,
    record: MatchRecord,
    remaining: Option<[u64; 2]>,
    active: Option<Color>,
    last_at: Option<u64>,
    last_observed: Option<u64>,
    started: bool,
    offer: Option<Color>,
    conclusion: Option<MatchConclusion>,
    feasibility_evidence: Option<MateReachability>,
}
impl ChessMatch {
    /// Create a match. Handshake/setup time is not charged until Start.
    pub fn new(game: AdjudicatedGame, control: TimeControl) -> Result<Self, MatchRecordError> {
        let remaining = match control {
            TimeControl::Untimed => None,
            TimeControl::Increment { initial_ns, .. } => {
                if initial_ns.contains(&0) {
                    return Err(MatchRecordError(
                        "initial clock balances must be positive".into(),
                    ));
                }
                Some(initial_ns)
            }
        };
        let record = MatchRecord {
            schema_version: if game.adjudication_mode() == AdjudicationMode::M2 {
                1
            } else {
                2
            },
            rules_profile: crate::CHESS_MATCH_PROFILE_ID.into(),
            initial_record_json: game.record().to_json(),
            time_control: control,
            events: Vec::new(),
        };
        let conclusion = game.outcome().map(MatchConclusion::Chess);
        Ok(Self {
            game,
            record,
            remaining,
            active: None,
            last_at: None,
            last_observed: None,
            started: false,
            offer: None,
            conclusion,
            feasibility_evidence: None,
        })
    }
    /// Immutable live game context for adapters/search.
    pub const fn game(&self) -> &AdjudicatedGame {
        &self.game
    }
    /// Event version required by the next request.
    pub fn version(&self) -> u64 {
        self.record.events.len() as u64
    }
    /// Final result, if any.
    pub const fn conclusion(&self) -> Option<MatchConclusion> {
        self.conclusion
    }
    /// Remaining clock balances, if timed.
    pub const fn remaining_ns(&self) -> Option<[u64; 2]> {
        self.remaining
    }
    /// Current running side, if started and not concluded.
    pub const fn active(&self) -> Option<Color> {
        self.active
    }
    /// Full deterministic input/observation record.
    pub const fn record(&self) -> &MatchRecord {
        &self.record
    }
    fn reject(code: MatchRejectCode, detail: impl Into<String>) -> MatchRejection {
        MatchRejection {
            code,
            detail: detail.into(),
        }
    }
    fn end(&mut self, c: MatchConclusion) {
        self.conclusion = Some(c);
        self.active = None;
        self.offer = None;
    }
    fn feasibility_result(
        &mut self,
        actor: Color,
        trigger: TerminationTrigger,
        witness: Option<&[Action]>,
    ) -> Result<MatchConclusion, MatchRejection> {
        // Existing sound proofs remain usable even when full work has zero
        // budget. A full operational cap cannot erase an established result.
        let baseline = mating_feasibility(self.game.position(), actor.opponent(), witness)
            .map_err(|e| Self::reject(MatchRejectCode::InvalidWitness, e))?;
        let feasibility = if witness.is_none() && baseline == MatingFeasibility::Unknown {
            if let AdjudicationMode::Full { limits } = self.game.adjudication_mode() {
                let analysis = self.game.position().mate_reachability(
                    MateTarget::Side(actor.opponent()),
                    limits,
                    || false,
                );
                let result = match analysis {
                    MateReachability::Possible { .. } => MatingFeasibility::Possible,
                    MateReachability::Impossible { .. } => MatingFeasibility::Impossible,
                    MateReachability::Unknown { .. } => MatingFeasibility::Unknown,
                };
                self.feasibility_evidence = Some(analysis);
                result
            } else {
                baseline
            }
        } else {
            baseline
        };
        Ok(match feasibility {
            MatingFeasibility::Possible => MatchConclusion::Win {
                winner: actor.opponent(),
                trigger,
            },
            MatingFeasibility::Impossible => MatchConclusion::Draw {
                trigger: Some(trigger),
            },
            MatingFeasibility::Unknown => MatchConclusion::Unresolved { actor, trigger },
        })
    }
    /// Serialize one observed request. Flag fall at/before arrival is processed
    /// before the requested move/command. Terminal moves receive no increment.
    pub fn submit(&mut self, request: MatchRequest) -> MatchReply {
        self.feasibility_evidence = None;
        let rejection = self.process(&request).err();
        let reply = MatchReply {
            version: self.version() + 1,
            remaining_ns: self.remaining,
            active: self.active,
            conclusion: self.conclusion,
            rejection,
            feasibility_evidence: self.feasibility_evidence.clone(),
        };
        self.record.events.push(MatchEvent {
            request,
            reply: reply.clone(),
        });
        reply
    }
    fn process(&mut self, r: &MatchRequest) -> Result<(), MatchRejection> {
        use MatchRejectCode::*;
        if self.conclusion.is_some() {
            return Err(Self::reject(GameOver, "prior conclusion remains final"));
        }
        if self.last_observed.is_some_and(|last| r.at_ns < last) {
            return Err(Self::reject(NonmonotonicTime, "timestamp decreased"));
        }
        self.last_observed = Some(r.at_ns);
        if self.started {
            if let (Some(active), Some(remaining), Some(last)) =
                (self.active, self.remaining.as_mut(), self.last_at)
            {
                let i = if active == Color::White { 0 } else { 1 };
                let elapsed = r.at_ns - last;
                if elapsed >= remaining[i] {
                    remaining[i] = 0;
                    self.last_at = Some(r.at_ns);
                    let witness = match &r.command {
                        MatchCommand::Observe { witness } | MatchCommand::Resign { witness } => {
                            witness.as_deref()
                        }
                        _ => None,
                    };
                    // Invalid supplied evidence cannot postpone an already
                    // observed flag: retain an explicit unresolved conclusion.
                    let result =
                        self.feasibility_result(active, TerminationTrigger::FlagFall, witness);
                    match result {
                        Ok(c) => {
                            self.end(c);
                            return Ok(());
                        }
                        Err(e) => {
                            self.end(MatchConclusion::Unresolved {
                                actor: active,
                                trigger: TerminationTrigger::FlagFall,
                            });
                            return Err(e);
                        }
                    }
                }
                remaining[i] -= elapsed;
            }
            self.last_at = Some(r.at_ns);
        }
        if r.version != self.version() {
            return Err(Self::reject(WrongVersion, "request version is stale"));
        }
        if matches!(r.command, MatchCommand::Start) {
            if self.started {
                return Err(Self::reject(AlreadyStarted, "clock already started"));
            }
            if r.actor != self.game.position().side_to_move() {
                return Err(Self::reject(
                    WrongActor,
                    "start actor must be the active player",
                ));
            }
            self.started = true;
            self.active = Some(r.actor);
            self.last_at = Some(r.at_ns);
            return Ok(());
        }
        if !self.started {
            return Err(Self::reject(NotStarted, "start the match before commands"));
        }
        match &r.command {
            MatchCommand::Move(a) => {
                if r.actor != self.game.position().side_to_move() {
                    return Err(Self::reject(
                        WrongActor,
                        "move actor is not the side to move",
                    ));
                }
                let mut next = self.game.clone();
                next.apply(*a)
                    .map_err(|e| Self::reject(InvalidMove, e.to_string()))?;
                let outcome = next.outcome();
                if outcome.is_none()
                    && let (Some(remaining), TimeControl::Increment { increment_ns, .. }) =
                        (self.remaining.as_mut(), self.record.time_control)
                {
                    let i = if r.actor == Color::White { 0 } else { 1 };
                    remaining[i] = remaining[i]
                        .checked_add(increment_ns)
                        .ok_or_else(|| Self::reject(ClockOverflow, "increment exceeds u64"))?;
                }
                self.game = next;
                if self.offer == Some(r.actor.opponent()) {
                    self.offer = None;
                }
                if let Some(c) = outcome {
                    self.end(MatchConclusion::Chess(c));
                } else {
                    self.active = Some(r.actor.opponent());
                }
            }
            MatchCommand::Claim { reason, intended } => {
                self.game
                    .claim(ClaimRequest {
                        actor: r.actor,
                        reason: *reason,
                        intended: *intended,
                    })
                    .map_err(|e| Self::reject(InvalidClaim, e.to_string()))?;
                self.end(MatchConclusion::Chess(
                    self.game.outcome().expect("accepted claim concludes"),
                ));
            }
            MatchCommand::Resign { witness } => {
                let result = self.feasibility_result(
                    r.actor,
                    TerminationTrigger::Resignation,
                    witness.as_deref(),
                );
                match result {
                    Ok(c) => self.end(c),
                    Err(e) => {
                        self.end(MatchConclusion::Unresolved {
                            actor: r.actor,
                            trigger: TerminationTrigger::Resignation,
                        });
                        return Err(e);
                    }
                }
            }
            MatchCommand::OfferDraw => {
                if self.offer.is_some() {
                    return Err(Self::reject(OfferPending, "an offer is already pending"));
                }
                self.offer = Some(r.actor);
            }
            MatchCommand::AcceptDraw => {
                if self.offer != Some(r.actor.opponent()) {
                    return Err(Self::reject(NoOffer, "no opponent offer to accept"));
                }
                if self.game.history().ply_count() < 2 {
                    return Err(Self::reject(
                        OfferTooEarly,
                        "both players must have a recorded move",
                    ));
                }
                self.end(MatchConclusion::Draw { trigger: None });
            }
            MatchCommand::DeclineDraw => {
                if self.offer != Some(r.actor.opponent()) {
                    return Err(Self::reject(NoOffer, "no opponent offer to decline"));
                }
                self.offer = None;
            }
            MatchCommand::Observe { .. } => {}
            MatchCommand::Start => unreachable!(),
        }
        Ok(())
    }
}
impl MatchRecord {
    /// Canonical compact JSON, including requests and computed observations.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("finite record primitives")
    }
    /// Decode the strict envelope; replay verifies every observation.
    pub fn from_json(text: &str) -> Result<Self, MatchRecordError> {
        let r: Self = serde_json::from_str(text).map_err(|e| MatchRecordError(e.to_string()))?;
        if !matches!(r.schema_version, 1 | 2) || r.rules_profile != crate::CHESS_MATCH_PROFILE_ID {
            return Err(MatchRecordError("unsupported match record identity".into()));
        }
        Ok(r)
    }
    /// Recompute actions, commands, clocks, rejection reasons and conclusions
    /// from ordered recorded inputs; reject any inconsistent observation.
    pub fn replay(&self) -> Result<ChessMatch, MatchRecordError> {
        if !matches!(self.schema_version, 1 | 2)
            || self.rules_profile != crate::CHESS_MATCH_PROFILE_ID
        {
            return Err(MatchRecordError("unsupported match record identity".into()));
        }
        let initial = GameRecord::from_json(&self.initial_record_json)
            .map_err(|e| MatchRecordError(e.to_string()))?;
        if (self.schema_version == 1) != (initial.adjudication_mode() == AdjudicationMode::M2) {
            return Err(MatchRecordError(
                "match/history completeness schema mismatch".into(),
            ));
        }
        if self.schema_version == 1
            && self
                .events
                .iter()
                .any(|e| e.reply.feasibility_evidence.is_some())
        {
            return Err(MatchRecordError(
                "legacy match cannot contain full-solver evidence".into(),
            ));
        }
        let game = AdjudicatedGame::replay_record(&initial)
            .map_err(|e| MatchRecordError(e.to_string()))?;
        let mut m = ChessMatch::new(game, self.time_control)?;
        for (i, e) in self.events.iter().enumerate() {
            if m.submit(e.request.clone()) != e.reply {
                return Err(MatchRecordError(format!(
                    "match event {} observation mismatch",
                    i + 1
                )));
            }
        }
        Ok(m)
    }
}
