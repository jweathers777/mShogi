//! Untimed rules-owned game history. A `Position` by itself is not a history.

use super::{Action, ActionError, CounterSource, PieceKind, Position, RepetitionIdentity, Side};

/// Root category is retained, rather than presenting an imported diagram as a recovered game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootCategory {
    /// Exact even-game initial state, followed by the recorded actions.
    Standard,
    /// All forty pieces are present; historical reachability is not established.
    FullMaterialDiagram,
    /// Some material is absent; historical reachability is not established.
    ReducedMaterialDiagram,
}

/// An explicitly admitted start of known game history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRoot {
    position: Position,
    category: RootCategory,
    counter_source: Option<CounterSource>,
}

/// Imported-root admission failed. Structural analysis remains available on `Position`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootError {
    /// A diagram cannot assert complete initial history.
    StandardRequiresInitialRoot,
    /// Full/reduced material label disagrees with the actual inventory.
    CategoryMismatch,
    /// Defaulting means one, never a silently reset supplied counter.
    CounterProvenance,
    /// Necessary local live-game condition; not a reachability test.
    NonmovingKingAttacked,
}

impl GameRoot {
    /// Complete history begins at the exact initial state, not at a moved `Position`.
    pub fn standard() -> Self {
        Self {
            position: Position::initial(),
            category: RootCategory::Standard,
            counter_source: None,
        }
    }

    /// Begin a new known segment. Earlier checks and repetitions remain unknown.
    /// The moving king may be checked. The nonmoving king must be unattacked.
    pub fn import(
        position: Position,
        category: RootCategory,
        source: CounterSource,
    ) -> Result<Self, RootError> {
        if category == RootCategory::Standard {
            return Err(RootError::StandardRequiresInitialRoot);
        }
        if source == CounterSource::Defaulted && position.next_ply() != 1 {
            return Err(RootError::CounterProvenance);
        }
        let material: usize = position.board().iter().filter(|p| p.is_some()).count()
            + [Side::Sente, Side::Gote]
                .into_iter()
                .map(|side| {
                    PieceKind::HAND_KINDS
                        .into_iter()
                        .map(|kind| usize::from(position.hand(side).count(kind)))
                        .sum::<usize>()
                })
                .sum::<usize>();
        if (material == 40) != (category == RootCategory::FullMaterialDiagram) {
            return Err(RootError::CategoryMismatch);
        }
        if position.is_in_check(position.side_to_move().opponent()) {
            return Err(RootError::NonmovingKingAttacked);
        }
        // Deliberately discard a bare Position's old origin. It is not an event log.
        let position = Position::from_diagram(
            *position.board(),
            [position.hand(Side::Sente), position.hand(Side::Gote)],
            position.side_to_move(),
            (source == CounterSource::Supplied).then_some(position.next_ply()),
        )
        .expect("rebuilding a validated position with the same counter");
        Ok(Self {
            position,
            category,
            counter_source: Some(source),
        })
    }
    /// Admitted position, immutable through this interface.
    pub fn position(&self) -> &Position {
        &self.position
    }
    /// Declared material/history category.
    pub fn category(&self) -> RootCategory {
        self.category
    }
    /// Imported counter provenance; standard roots have a fixed initial counter.
    pub fn counter_source(&self) -> Option<CounterSource> {
        self.counter_source
    }
    /// Only the exact initial root and subsequent ledger establish complete initial history.
    pub fn has_complete_initial_history(&self) -> bool {
        self.category == RootCategory::Standard
    }
}

/// All supported commands are untimed. Queries do not create events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameCommand {
    /// A complete, locally legal board action.
    Play(Action),
    /// The named actor resigns; own turn is not required.
    Resign,
    /// Formal CSA declaration. Failing a prerequisite loses, including wrong turn.
    DeclareEnteringKing,
}

/// Advisory evidence; querying this never declares or changes a game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationEligibility {
    /// Declarer has the move.
    pub own_turn: bool,
    /// Declarer's king is in the opposing camp.
    pub king_in_camp: bool,
    /// Other owned camp pieces, excluding the king.
    pub camp_pieces: u8,
    /// Camp and hand points, ignoring promotion.
    pub points: u16,
    /// Sente requires 28, gote 27.
    pub required_points: u16,
    /// Declarer's king is unattacked.
    pub not_in_check: bool,
    /// This untimed profile has no exhausted clock.
    pub time_available: bool,
}
impl DeclarationEligibility {
    /// Whether the declaration prerequisites hold for this actor. This advisory
    /// evidence does not grant admission: an already concluded game rejects commands.
    pub fn eligible(self) -> bool {
        self.own_turn
            && self.king_in_camp
            && self.camp_pieces >= 10
            && self.points >= self.required_points
            && self.not_in_check
            && self.time_available
    }
}

/// Selected reason for a win, distinct from board legality or API rejection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WinReason {
    /// Opponent has no legal action; retains mate/non-check distinction.
    NoLegalAction {
        /// Whether the immobile side's king is checked.
        in_check: bool,
    },
    /// All loser's actions within the first-to-fourth known occurrence give check.
    ContinuousCheck {
        /// Known position index at the first occurrence.
        first: usize,
        /// Known position index at the fourth occurrence.
        fourth: usize,
    },
    /// Opponent explicitly resigned.
    Resignation,
    /// Actor made a valid formal declaration.
    EnteringKing,
    /// Opponent made an admitted formal declaration failing at least one prerequisite.
    FalseDeclaration,
}
/// Selected final draws; neither requests a replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrawReason {
    /// Four known occurrences without a continuous checker.
    Fourfold {
        /// First known occurrence's position index.
        first: usize,
        /// Fourth occurrence's position index.
        fourth: usize,
    },
    /// Declared completed-action count reached 320, absent a higher-priority conclusion.
    Length,
}
/// A concluded chess-independent shogi game. Rejected requests are never outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Conclusion {
    /// One winner under a recorded reason.
    Win {
        /// Winning side.
        winner: Side,
        /// Governing reason.
        reason: WinReason,
    },
    /// Final draw under CSA convention.
    Draw(DrawReason),
}
impl Conclusion {
    /// CSA competition points in half-point units, ordered sente then gote.
    pub fn score_half_points(self) -> [u8; 2] {
        match self {
            Self::Win {
                winner: Side::Sente,
                ..
            } => [2, 0],
            Self::Win {
                winner: Side::Gote, ..
            } => [0, 2],
            Self::Draw(_) => [1, 1],
        }
    }
}

/// Accepted command and reconstructed effects at a specific event prefix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameEvent {
    /// Zero-based event index, independent of the board-action counter.
    pub index: usize,
    /// Named command actor.
    pub actor: Side,
    /// Accepted command.
    pub command: GameCommand,
    /// Derived check observation for board actions only.
    pub gave_check: Option<bool>,
    /// Evidence retained for formal declarations only.
    pub declaration: Option<DeclarationEligibility>,
    /// Derived exact SFEN after the event.
    pub after_sfen: String,
    /// Derived conclusion after this prefix.
    pub conclusion: Option<Conclusion>,
}

/// Exact rules context, including root provenance and event order, not merely board identity.
/// This is not a future engine request/session/process identity. A host must add those identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameContext {
    root: GameRoot,
    commands: Vec<(Side, GameCommand)>,
}
impl GameContext {
    /// Exact admitted root, including material category and counter/history provenance.
    pub fn root(&self) -> &GameRoot {
        &self.root
    }
    /// Ordered actors and commands, including any candidate action in an invariant diagnostic.
    pub fn commands(&self) -> &[(Side, GameCommand)] {
        &self.commands
    }
}

/// Evidence for an internal violation of the reviewed mutual-check impossibility invariant.
/// This is a defect/corruption diagnostic, never a game outcome or tournament penalty.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MutualCheckDiagnostic {
    /// Complete root and command history including the uncommitted candidate action.
    pub candidate_context: GameContext,
    /// Exact candidate board, hands, turn and declared counter.
    pub candidate_sfen: String,
    /// All known indices of the candidate repetition identity, root indexed zero.
    pub occurrences: Vec<usize>,
    /// Actor and stored check observation for each action after first through fourth occurrence.
    pub interval_checks: Vec<(Side, bool)>,
}
/// Admission request bound to the entire known rules context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRequest {
    /// Context captured when choosing this command.
    pub expected: GameContext,
    /// Side making this command.
    pub actor: Side,
    /// Complete action or explicit terminal command.
    pub command: GameCommand,
}
/// Rejected admission changes no game state or ledger.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameError {
    /// Request targets another known event prefix or root.
    StaleContext,
    /// No further play or result-changing command is accepted.
    Concluded,
    /// A board action was addressed by the nonmoving side.
    WrongActor,
    /// Locally illegal board action, not a chess outcome or tournament forfeit.
    IllegalAction(ActionError),
    /// Internal state/implementation violates the reviewed impossibility invariant.
    /// A candidate is not committed and no win, draw or penalty is fabricated.
    MutualCheckInvariant(Box<MutualCheckDiagnostic>),
}

/// Prefix reconstruction failure, retaining internal diagnostics rather than erasing them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GamePrefixError {
    /// Requested event count exceeds the accepted ledger.
    OutOfRange,
    /// An accepted ledger unexpectedly failed authoritative reconstruction.
    Replay {
        /// Index of the event that could not be reconstructed.
        event_index: usize,
        /// Original native diagnostic.
        error: GameError,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CheckStep {
    actor: Side,
    gave_check: bool,
}

/// Complete known game state. Position queries and game conclusions remain separate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    root: GameRoot,
    position: Position,
    events: Vec<GameEvent>,
    identities: Vec<RepetitionIdentity>,
    checks: Vec<CheckStep>,
    conclusion: Option<Conclusion>,
}

/// Exact semantic reversal token, checked against the complete resulting state.
#[derive(Debug)]
pub struct GameUndo {
    before: Game,
    after: Game,
}
/// A reversal token belongs to a different current state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GameUndoError;

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
impl Game {
    /// Even-game initial root with complete known history.
    pub fn new() -> Self {
        Self::from_root(GameRoot::standard())
    }
    /// Start from an admitted root. Root no-action loss precedes declared length.
    pub fn from_root(root: GameRoot) -> Self {
        let position = root.position.clone();
        let mut game = Self {
            identities: vec![position.repetition_identity()],
            root,
            position,
            events: vec![],
            checks: vec![],
            conclusion: None,
        };
        game.conclusion = game.local_or_length_conclusion();
        game
    }
    /// Authoritative current position.
    pub fn position(&self) -> &Position {
        &self.position
    }
    /// Admitted beginning of known history.
    pub fn root(&self) -> &GameRoot {
        &self.root
    }
    /// Ordered accepted ledger, including non-board terminal commands.
    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }
    /// Current final outcome, if any.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.conclusion
    }
    /// Captures full rules context; equal boards can have different contexts.
    pub fn context(&self) -> GameContext {
        GameContext {
            root: self.root.clone(),
            commands: self.events.iter().map(|e| (e.actor, e.command)).collect(),
        }
    }
    /// Known position indices for this board/hand/turn identity. Counters are excluded.
    pub fn occurrences(&self) -> Vec<usize> {
        let current = self.identities.last().expect("root occurrence");
        self.identities
            .iter()
            .enumerate()
            .filter_map(|(i, id)| (id == current).then_some(i))
            .collect()
    }
    /// Read-only declaration evidence for either side, regardless of turn or conclusion.
    pub fn declaration_eligibility(&self, actor: Side) -> DeclarationEligibility {
        let mut camp_pieces = 0;
        let mut points = 0;
        let mut king_in_camp = false;
        for square in super::Square::all() {
            if let Some(piece) = self
                .position
                .piece(square)
                .filter(|p| p.side == actor && actor.in_camp(square.rank()))
            {
                if piece.kind == PieceKind::King {
                    king_in_camp = true;
                } else {
                    camp_pieces += 1;
                    points += piece_points(piece.kind);
                }
            }
        }
        for kind in PieceKind::HAND_KINDS {
            points += u16::from(self.position.hand(actor).count(kind)) * piece_points(kind);
        }
        DeclarationEligibility {
            own_turn: self.position.side_to_move() == actor,
            king_in_camp,
            camp_pieces,
            points,
            required_points: if actor == Side::Sente { 28 } else { 27 },
            not_in_check: !self.position.is_in_check(actor),
            time_available: true,
        }
    }
    /// Atomically accept a context-bound command. A legal losing repetition action is retained.
    pub fn submit(&mut self, request: GameRequest) -> Result<GameUndo, GameError> {
        if request.expected != self.context() {
            return Err(GameError::StaleContext);
        }
        if self.conclusion.is_some() {
            return Err(GameError::Concluded);
        }
        let before = self.clone();
        let mut candidate = self.clone();
        let (gave_check, declaration) = match request.command {
            GameCommand::Play(action) => {
                if request.actor != candidate.position.side_to_move() {
                    return Err(GameError::WrongActor);
                }
                candidate
                    .position
                    .apply(action)
                    .map_err(GameError::IllegalAction)?;
                let check = candidate
                    .position
                    .is_in_check(candidate.position.side_to_move());
                candidate.checks.push(CheckStep {
                    actor: request.actor,
                    gave_check: check,
                });
                candidate
                    .identities
                    .push(candidate.position.repetition_identity());
                candidate.conclusion = candidate.after_action_conclusion(request.actor, action)?;
                (Some(check), None)
            }
            GameCommand::Resign => {
                candidate.conclusion = Some(Conclusion::Win {
                    winner: request.actor.opponent(),
                    reason: WinReason::Resignation,
                });
                (None, None)
            }
            GameCommand::DeclareEnteringKing => {
                let evidence = candidate.declaration_eligibility(request.actor);
                candidate.conclusion = Some(Conclusion::Win {
                    winner: if evidence.eligible() {
                        request.actor
                    } else {
                        request.actor.opponent()
                    },
                    reason: if evidence.eligible() {
                        WinReason::EnteringKing
                    } else {
                        WinReason::FalseDeclaration
                    },
                });
                (None, Some(evidence))
            }
        };
        candidate.events.push(GameEvent {
            index: candidate.events.len(),
            actor: request.actor,
            command: request.command,
            gave_check,
            declaration,
            after_sfen: candidate.position.to_sfen(),
            conclusion: candidate.conclusion,
        });
        *self = candidate;
        Ok(GameUndo {
            before,
            after: self.clone(),
        })
    }
    /// Restore board, commands, counters, occurrences and checks exactly. Runtime hosts must
    /// separately invalidate pending execution identities when changing their session.
    pub fn undo(&mut self, undo: GameUndo) -> Result<(), GameUndoError> {
        if *self != undo.after {
            return Err(GameUndoError);
        }
        *self = undo.before;
        Ok(())
    }
    /// Authoritatively reconstruct an accepted event prefix. Same-ply terminal commands count.
    pub fn prefix(&self, event_count: usize) -> Result<Self, GamePrefixError> {
        if event_count > self.events.len() {
            return Err(GamePrefixError::OutOfRange);
        }
        let mut prefix = Self::from_root(self.root.clone());
        for event in &self.events[..event_count] {
            prefix
                .submit(GameRequest {
                    expected: prefix.context(),
                    actor: event.actor,
                    command: event.command,
                })
                .map_err(|error| GamePrefixError::Replay {
                    event_index: event.index,
                    error,
                })?;
        }
        Ok(prefix)
    }
    fn no_action_conclusion(&self) -> Option<Conclusion> {
        self.position
            .legal_actions()
            .is_empty()
            .then(|| Conclusion::Win {
                winner: self.position.side_to_move().opponent(),
                reason: WinReason::NoLegalAction {
                    in_check: self.position.is_in_check(self.position.side_to_move()),
                },
            })
    }
    fn local_or_length_conclusion(&self) -> Option<Conclusion> {
        self.no_action_conclusion().or_else(|| {
            (self.position.completed_actions() >= 320)
                .then_some(Conclusion::Draw(DrawReason::Length))
        })
    }
    fn after_action_conclusion(
        &self,
        actor: Side,
        action: Action,
    ) -> Result<Option<Conclusion>, GameError> {
        if let Some(conclusion) = self.no_action_conclusion() {
            return Ok(Some(conclusion));
        }
        let occurrences = self.occurrences();
        if occurrences.len() >= 4 {
            let first = occurrences[occurrences.len() - 4];
            let fourth = *occurrences.last().expect("four occurrences");
            let continuous = |side| {
                self.checks[first..fourth]
                    .iter()
                    .filter(|s| s.actor == side)
                    .all(|s| s.gave_check)
            };
            let sente = continuous(Side::Sente);
            let gote = continuous(Side::Gote);
            if sente && gote {
                // The reviewed chain in docs/m4/mutual-check-chain-review.md excludes
                // this for every admitted root and checked local continuation. Preserve
                // the complete candidate for diagnosing corrupted caches or a defect.
                let mut candidate_context = self.context();
                candidate_context
                    .commands
                    .push((actor, GameCommand::Play(action)));
                return Err(GameError::MutualCheckInvariant(Box::new(
                    MutualCheckDiagnostic {
                        candidate_context,
                        candidate_sfen: self.position.to_sfen(),
                        occurrences,
                        interval_checks: self.checks[first..fourth]
                            .iter()
                            .map(|step| (step.actor, step.gave_check))
                            .collect(),
                    },
                )));
            }
            return Ok(Some(if sente || gote {
                Conclusion::Win {
                    winner: if sente { Side::Gote } else { Side::Sente },
                    reason: WinReason::ContinuousCheck { first, fourth },
                }
            } else {
                Conclusion::Draw(DrawReason::Fourfold { first, fourth })
            }));
        }
        Ok((self.position.completed_actions() >= 320)
            .then_some(Conclusion::Draw(DrawReason::Length)))
    }
}

fn piece_points(kind: PieceKind) -> u16 {
    match kind {
        PieceKind::King => 0,
        PieceKind::Rook | PieceKind::Bishop => 5,
        _ => 1,
    }
}

#[cfg(test)]
mod invariant_tests {
    use super::*;

    #[test]
    fn corrupted_check_cache_retains_candidate_evidence_without_committing_or_scoring() {
        // Test-only corruption, NOT a legal mutual-check example. The real sequence
        // is the independently established checked-endpoint Sente checking loss.
        let root = GameRoot::import(
            Position::from_sfen("4k4/9/4R4/9/9/9/9/9/K8 w - 1").unwrap(),
            RootCategory::ReducedMaterialDiagram,
            CounterSource::Supplied,
        )
        .unwrap();
        let mut game = Game::from_root(root);
        let cycle = ["5a4a", "5c4c", "4a5a", "4c5c"];
        for text in cycle.iter().cycle().take(11) {
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position.side_to_move(),
                command: GameCommand::Play(text.parse().unwrap()),
            })
            .unwrap();
        }
        // The public API and canonical replay cannot inject these cache values.
        for step in &mut game.checks {
            step.gave_check = true;
        }
        let before = game.clone();
        let action = cycle[3].parse().unwrap();
        let actor = game.position.side_to_move();
        let error = game
            .submit(GameRequest {
                expected: game.context(),
                actor,
                command: GameCommand::Play(action),
            })
            .unwrap_err();
        let GameError::MutualCheckInvariant(diagnostic) = error else {
            panic!("expected internal invariant diagnostic");
        };
        assert_eq!(game, before);
        assert_eq!(game.conclusion(), None);
        assert_eq!(diagnostic.occurrences, vec![0, 4, 8, 12]);
        assert_eq!(diagnostic.interval_checks.len(), 12);
        assert!(diagnostic.interval_checks.iter().all(|(_, check)| *check));
        assert_eq!(diagnostic.candidate_context.root(), game.root());
        assert_eq!(diagnostic.candidate_context.commands().len(), 12);
        assert_eq!(
            diagnostic.candidate_context.commands().last(),
            Some(&(actor, GameCommand::Play(action)))
        );
        let mut expected_position = game.position.clone();
        expected_position.apply(action).unwrap();
        assert_eq!(diagnostic.candidate_sfen, expected_position.to_sfen());
        assert_eq!(
            diagnostic.candidate_context.commands()[..11],
            game.context().commands()[..]
        );
        // Authoritative replay recomputes the actual checks; this is an ordinary
        // one-sided checking loss, demonstrating that the injected data is corrupt.
        let mut replay = Game::from_root(diagnostic.candidate_context.root().clone());
        for &(actor, command) in diagnostic.candidate_context.commands() {
            replay
                .submit(GameRequest {
                    expected: replay.context(),
                    actor,
                    command,
                })
                .unwrap();
        }
        assert_eq!(replay.position.to_sfen(), diagnostic.candidate_sfen);
        assert_eq!(
            replay.conclusion,
            Some(Conclusion::Win {
                winner: Side::Gote,
                reason: WinReason::ContinuousCheck {
                    first: 0,
                    fourth: 12
                },
            })
        );
    }

    #[test]
    fn corrupted_command_prefix_preserves_replay_error_and_original_game() {
        let mut game = Game::new();
        game.submit(GameRequest {
            expected: game.context(),
            actor: Side::Sente,
            command: GameCommand::Play("7g7f".parse().unwrap()),
        })
        .unwrap();
        game.events[0].actor = Side::Gote;
        let before = game.clone();
        assert_eq!(
            game.prefix(1),
            Err(GamePrefixError::Replay {
                event_index: 0,
                error: GameError::WrongActor
            })
        );
        assert_eq!(game.prefix(2), Err(GamePrefixError::OutOfRange));
        assert_eq!(game, before);
    }
}
