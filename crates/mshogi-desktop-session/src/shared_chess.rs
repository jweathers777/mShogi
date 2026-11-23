//! Chess presentation and commands on the generic native session.
use super::native_session::*;
use super::*;
use crate::native_game::{Chess, NativeGame};
pub(crate) type ChessSession = NativeSession<Chess>;
fn player(color: Color) -> PresentedSide {
    side(RulesProfile::ChessMatchV1, NativeSide::Chess(color))
}
fn player_events(game: &ChessMatch) -> impl Iterator<Item = &MatchRequest> {
    game.record()
        .events()
        .filter(|(r, _)| {
            !matches!(
                r.command,
                MatchCommand::Start | MatchCommand::Observe { .. }
            )
        })
        .map(|(r, _)| r)
}
impl SessionGame for Chess {
    type Presentation = ChessPresentation;
    const PLAY_LIMITS: WorkLimits = crate::PLAY_LIMITS;
    const MAX_DEPTH: u16 = 128;
    const MAX_NODES: u64 = crate::PLAY_NODE_CEILING;
    const FULL_ANALYSIS: bool = true;
    const FINISH_CONCLUDED_ANALYSIS: bool = true;
    fn cancellation_phase(analysis: bool) -> EnginePhase {
        if analysis {
            EnginePhase::Paused
        } else {
            EnginePhase::Cancelled
        }
    }
    fn requires_running(command: &MatchCommand) -> bool {
        matches!(command, MatchCommand::Move(_) | MatchCommand::Claim { .. })
    }
    const UNDO: bool = false;
    fn validate_command(
        _game: &ChessMatch,
        human: Color,
        settings: &DesktopSettings,
        actor: Color,
        _command: &MatchCommand,
    ) -> Result<(), String> {
        if settings.play_mode.owns(human, actor) {
            Ok(())
        } else {
            Err("This command belongs to the other player.".into())
        }
    }
    const BOARD_SIZE: u8 = 8;
    const BOARD_LABEL: &'static str = "Chess board";
    const INITIAL_FOCUS: &'static str = "e2";
    const SCORE_MEANING: &'static str = "Chess root-side centipawns; native mate distance and bounded cooperative Full proof remain distinct (see chess analysis).";
    fn presentation(selected: &ChessMatch, live: &ChessMatch) -> ChessPresentation {
        ChessPresentation {
            claims: if live.conclusion().is_none() {
                live.game()
                    .available_claims()
                    .into_iter()
                    .map(EngineClaim::from)
                    .collect()
            } else {
                vec![]
            },
            live_conclusion: live.conclusion(),
            selected_conclusion: selected.conclusion(),
            in_check: selected.game().position().in_check(),
            analysis: AnalysisState::default(),
            last_engine_result: None,
            last_engine_turn: None,
            searched_version: None,
            rules_mode: "m2",
        }
    }
    fn status(game: &ChessMatch, _has_actions: bool) -> PresentedStatus {
        PresentedStatus {
            concluded: game.conclusion().is_some(),
            label: game.conclusion().map_or_else(
                || format!("{} to move", player(Self::side_to_move(game)).label),
                |c| format!("{c:?}"),
            ),
            score_half_points: game
                .conclusion()
                .and_then(MatchConclusion::white_score_twice)
                .map(|n| [n, 2 - n]),
        }
    }
    fn choice(_game: &ChessMatch, action: Action, _all: &[Action]) -> PresentedChoice {
        PresentedChoice {
            tenjiku: None,
            origin: ChoiceOrigin::Board {
                square: action.from.to_string(),
            },
            action: action.to_string(),
            from: action.from.to_string(),
            steps: vec![action.to.to_string()],
            completion: action.promotion.map(|p| Completion {
                label: format!("{p:?}").to_lowercase(),
                glyph: None,
            }),
        }
    }
    fn legal_actions(
        game: &ChessMatch,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Action>, String> {
        persistence::check(stop)?;
        Ok(if game.conclusion().is_some() {
            vec![]
        } else {
            game.game().position().legal_actions()
        })
    }
    fn retire_context(_game: &mut ChessMatch) {}
    fn event(game: &ChessMatch, index: usize) -> (Color, CommandKind) {
        let root = game.record().initial_record().expect("admitted root");
        if index < root.events().len() {
            let prefix = root.prefix(index).expect("admitted prefix");
            let actor = Self::context_side(&CanonicalRecord::chess(prefix));
            return (
                actor,
                if matches!(root.events()[index].kind, RecordEventKind::Move(_)) {
                    CommandKind::Play
                } else {
                    CommandKind::Terminal
                },
            );
        }
        let request = player_events(game)
            .nth(index - root.events().len())
            .expect("admitted command");
        (request.actor, Self::command_kind(&request.command))
    }
    fn command_kind(command: &MatchCommand) -> CommandKind {
        match command {
            MatchCommand::Move(_) => CommandKind::Play,
            MatchCommand::Claim { .. } | MatchCommand::Resign { .. } | MatchCommand::AcceptDraw => {
                CommandKind::Terminal
            }
            _ => CommandKind::Other,
        }
    }
    fn telemetry(result: &ChessResult) -> TelemetryView {
        let telemetry = match result {
            ChessResult::Move { telemetry, .. }
            | ChessResult::Claim { telemetry, .. }
            | ChessResult::Finished { telemetry, .. }
            | ChessResult::Incomplete { telemetry } => Some(telemetry),
            _ => None,
        };
        TelemetryView {
            score: telemetry.and_then(|t| t.score).map(|s| s.to_string()),
            completed_depth: telemetry.map_or(0, |t| t.completed_depth),
            nodes: telemetry.map_or(0, |t| t.nodes),
            variation: vec![],
            status: "Complete",
        }
    }
    fn incomplete(result: &ChessResult) -> bool {
        matches!(
            result,
            ChessResult::Incomplete { .. }
                | ChessResult::Reachability {
                    result: MateReachability::Unknown { .. }
                }
        )
    }
    fn operation(operation: SessionOperation) -> Result<GameOperation<Self>, SessionOperation> {
        match operation {
            SessionOperation::ChessCommand { actor, command } => {
                Ok(GameOperation::Command { actor, command })
            }
            SessionOperation::ChessPlay { action } => Ok(GameOperation::Local {
                command: MatchCommand::Move(action),
            }),
            SessionOperation::ChessClaim { claim } => Ok(GameOperation::Command {
                actor: claim.actor,
                command: MatchCommand::Claim {
                    reason: claim.reason,
                    intended: claim.intended,
                },
            }),
            SessionOperation::ChessPreferences {
                orientation,
                analysis_defaults,
            } => Ok(GameOperation::Preferences {
                settings: DesktopSettings {
                    orientation,
                    analysis_defaults,
                    ..DesktopSettings::default()
                },
            }),
            operation => Err(operation),
        }
    }
    fn command_operation(actor: Color, command: MatchCommand) -> SessionOperation {
        SessionOperation::ChessCommand { actor, command }
    }
    fn opened(state: Box<ChessSession>) -> archive::OpenedNative {
        archive::OpenedNative::Chess(state)
    }
    fn state(session: &ActiveDocument) -> Option<&ChessSession> {
        if let ActiveDocument::Chess { state, .. } = session {
            Some(state)
        } else {
            None
        }
    }
    fn state_mut(session: &mut ActiveDocument) -> Option<&mut ChessSession> {
        if let ActiveDocument::Chess { state, .. } = session {
            Some(state)
        } else {
            None
        }
    }
    fn take_back_target(game: &ChessMatch, human: Color, two_humans: bool) -> Option<usize> {
        let target = crate::takeback::target_mode(
            game,
            human,
            if two_humans {
                PlayMode::TwoHumans
            } else {
                PlayMode::HumanEngine
            },
        )?;
        let root = game.record().initial_record().ok()?.events().len();
        Some(
            root + game
                .record()
                .events()
                .take(target)
                .filter(|(r, _)| {
                    !matches!(
                        r.command,
                        MatchCommand::Start | MatchCommand::Observe { .. }
                    )
                })
                .count(),
        )
    }
    fn edition(_game: &ChessMatch) -> &'static str {
        RulesProfile::ChessMatchV1.edition()
    }
    fn pieces(game: &ChessMatch) -> Vec<PresentedPiece> {
        (0..64)
            .filter_map(|n| {
                let square = Square::new(n % 8, n / 8).unwrap();
                game.game().position().piece_at(square).map(|piece| {
                    let kind = format!("{:?}", piece.kind).to_lowercase();
                    let text = match piece.kind {
                        PieceKind::King => "K",
                        PieceKind::Queen => "Q",
                        PieceKind::Rook => "R",
                        PieceKind::Bishop => "B",
                        PieceKind::Knight => "N",
                        PieceKind::Pawn => "P",
                    };
                    PresentedPiece {
                        square: square.to_string(),
                        owner: player(piece.color).key,
                        label: format!("{square}, {:?} {kind}", piece.color),
                        kind,
                        promoted: false,
                        text: text.into(),
                    }
                })
            })
            .collect()
    }
    fn presented_events(game: &ChessMatch) -> Vec<PresentedEvent> {
        let root = game.record().initial_record().expect("admitted root");
        let mut ply = 0;
        let mut events = vec![];
        for event in root.events() {
            if matches!(event.kind, RecordEventKind::Move(_)) {
                ply += 1;
            }
            events.push(PresentedEvent {
                index: events.len() + 1,
                board_ply: ply,
                label: format!("{:?}", event.kind),
            });
        }
        for request in player_events(game) {
            if matches!(request.command, MatchCommand::Move(_)) {
                ply += 1;
            }
            events.push(PresentedEvent {
                index: events.len() + 1,
                board_ply: ply,
                label: format!("{}: {:?}", player(request.actor).label, request.command),
            });
        }
        events
    }
    fn board_ply(game: &ChessMatch) -> u64 {
        game.game().history().ply_count() as u64
    }
    fn present_in(snapshot: &mut SharedSnapshot, presentation: ChessPresentation) {
        snapshot.chess = Some(presentation);
    }
    fn finish_snapshot(snapshot: &mut SharedSnapshot, state: &ChessSession) {
        let files: Vec<_> = (b'a'..=b'h').map(|c| char::from(c).to_string()).collect();
        let ranks: Vec<_> = (1..=8).rev().map(|n| n.to_string()).collect();
        snapshot.board.files = files.clone();
        snapshot.board.ranks = ranks.clone();
        snapshot.orientations[0].files = files.clone();
        snapshot.orientations[0].ranks = ranks.clone();
        snapshot.orientations[1].files = files.into_iter().rev().collect();
        snapshot.orientations[1].ranks = ranks.into_iter().rev().collect();
        snapshot.capabilities.chess_full = true;
        snapshot.capabilities.undo = false;
        snapshot.capabilities.board_action &= !state.paused;
        snapshot.capabilities.claims = !state.historical()
            && snapshot
                .chess
                .as_ref()
                .is_some_and(|c| !c.claims.is_empty());
        snapshot.chess_match = Some(state.game.record().clone());
        snapshot.search_observations = snapshot
            .observations
            .iter()
            .enumerate()
            .map(|(i, o)| {
                display_chess_observation(
                    i,
                    o,
                    snapshot.analysis.as_ref(),
                    Self::context_side(&o.observation.context.game),
                )
            })
            .collect();
        let chess = snapshot.chess.as_mut().expect("Chess presentation");
        if let Some(o) = state
            .observations
            .iter()
            .rev()
            .find(|o| matches!(o.context.purpose, ExecutionPurpose::EngineTurn {}))
        {
            chess.last_engine_result = chess_observation_result(o);
            chess.last_engine_turn = Some(Self::context_side(&o.context.game));
            chess.searched_version = Some(o.context.session_version);
        }
        let context = state
            .pending
            .as_ref()
            .filter(|p| !matches!(p.purpose, ExecutionPurpose::EngineTurn {}))
            .or_else(|| state.analysis.as_ref().map(|o| &o.context))
            .or(state.cancelled_analysis.as_ref());
        if let Some(context) = context {
            let (target, operation) = match &context.purpose {
                ExecutionPurpose::MoveAnalysis { target } => (
                    target,
                    AnalysisOperation::Moves {
                        depth: context.limits.depth,
                        nodes: context.limits.nodes,
                    },
                ),
                ExecutionPurpose::ChessFullReachability {
                    target,
                    mate,
                    limits,
                } => (
                    target,
                    AnalysisOperation::FullReachability {
                        target: *mate,
                        max_positions: limits.max_positions,
                        max_transitions: limits.max_transitions,
                    },
                ),
                _ => unreachable!(),
            };
            chess.analysis = AnalysisState {
                status: if state.cancelled_analysis.is_some() {
                    AnalysisPhase::Cancelled
                } else if state.pending.is_some() {
                    AnalysisPhase::Thinking
                } else if state
                    .analysis
                    .as_ref()
                    .is_some_and(|o| Self::failure(&o.result).is_some())
                {
                    AnalysisPhase::Failed
                } else if state
                    .analysis
                    .as_ref()
                    .and_then(|o| Self::engine_result(&o.result))
                    .is_some_and(Self::incomplete)
                {
                    AnalysisPhase::Incomplete
                } else {
                    AnalysisPhase::Complete
                },
                target: Some(target.clone()),
                operation: Some(operation),
                context: Some(context.clone()),
                result: state.analysis.as_ref().and_then(chess_observation_result),
                message: state.diagnostic.clone(),
            };
        }
    }
}

#[cfg(feature = "desktop-verification")]
impl DesktopController {
    pub(crate) fn verification_chess(&mut self, fixture: &str) -> Result<(), String> {
        if ["shogi-", "xiangqi-", "tori-", "chu-", "tenjiku-"]
            .iter()
            .any(|prefix| fixture.starts_with(prefix))
        {
            return Ok(());
        }
        let fen = match fixture {
            "standard" | "shared-contract" | "capacity" | "incomplete" => STANDARD_START_FEN,
            "promotion" => "7k/P7/8/8/8/8/8/7K w - - 0 1",
            "claim" => "7k/8/8/8/8/8/R7/K7 w - - 99 1",
            "current-claim" => "7k/8/8/8/8/8/R7/K7 w - - 100 1",
            "concluded" => "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1",
            _ => return Err("Unknown desktop verification fixture.".into()),
        };
        let game = if fen == STANDARD_START_FEN {
            AdjudicatedGame::new(ClaimPolicy::ExplicitOnly)
        } else {
            AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).map_err(|e| e.to_string())?
        };
        let human = if matches!(fixture, "incomplete" | "concluded") {
            Color::Black
        } else {
            Color::White
        };
        let receipt = self.queue_game_candidate::<Chess>(
            native_session::RuntimeSeed::Record {
                record: Box::new(crate::chess_record::ChessRecord::new(&game.record())),
                human,
                play_mode: PlayMode::HumanEngine,
            },
            (game.outcome().is_none() && fixture != "incomplete")
                .then_some(SessionOperation::Resume {}),
            None,
            true,
        )?;
        while receipt.completion().is_none() {
            self.poll_native_admission();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        receipt.completion().unwrap()?;
        if fixture == "capacity" {
            for _ in 0..MAX_ARCHIVE_OBSERVATIONS - 1 {
                self.start_game_work::<Chess>(
                    WorkLimits {
                        depth: 1,
                        nodes: 0,
                        time_ms: None,
                    },
                    true,
                )
                .map_err(|e| e.diagnostic)?;
                let context = self
                    .session
                    .chess()
                    .expect("Chess fixture")
                    .pending
                    .clone()
                    .expect("started analysis");
                let mut input =
                    serde_json::to_vec(&execution_request(context)).map_err(|e| e.to_string())?;
                input.push(b'\n');
                let mut output = vec![];
                mshogi_engine::serve_once(std::io::Cursor::new(input), &mut output)
                    .map_err(|e| e.to_string())?;
                let response = serde_json::from_slice(&output).map_err(|e| e.to_string())?;
                if !self.accept_shared(response) {
                    return Err("Chess fixture observation refused".into());
                }
                let receipt = self.admission_receipt.clone().expect("fixture admission");
                while receipt.completion().is_none() {
                    self.poll_native_admission();
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                receipt.completion().unwrap()?;
            }
        }
        if fixture == "incomplete" {
            self.verify_game_execution::<Chess>(
                WorkLimits {
                    nodes: 0,
                    ..crate::PLAY_LIMITS
                },
                false,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess_record::ChessRecord;
    fn settle(c: &mut DesktopController) {
        while c.admission_token.is_some() {
            c.poll_native_admission();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    fn controller(human: Color) -> DesktopController {
        let mut c = DesktopController::new(PathBuf::from("/unused/chess-engine")).unwrap();
        c.shared.manual_engine = true;
        c.replace_game::<Chess>(
            ChessRecord::new(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record()),
            human,
        )
        .unwrap();
        settle(&mut c);
        c
    }
    fn command(c: &mut DesktopController, operation: SessionOperation) {
        c.shared_command(SharedDesktopCommand {
            expected: c.shared_token(),
            game: GameKind::Chess,
            operation,
        })
        .unwrap();
        let receipt = c.last_command_receipt.clone().expect("queued command");
        settle(c);
        receipt.completion().unwrap().unwrap();
    }
    #[test]
    fn chess_full_analysis_stays_read_only_and_remembers_its_kind() {
        let mut c = controller(Color::White);
        let record = Chess::from_game(&c.session.chess().unwrap().game).unwrap();
        let operation = AnalysisOperation::FullReachability {
            target: mshogi_engine::ReachabilityTarget::Either,
            max_positions: 0,
            max_transitions: 0,
        };
        command(&mut c, SessionOperation::Analyze { operation });
        let snapshot = c.shared_snapshot();
        assert_eq!(snapshot.execution_purpose, Some("chess_full_reachability"));
        let pending = c.session.chess().unwrap().pending.clone().unwrap();
        let mut bytes = serde_json::to_vec(&execution_request(pending.clone())).unwrap();
        bytes.push(b'\n');
        let mut output = vec![];
        mshogi_engine::serve_once(std::io::Cursor::new(bytes), &mut output).unwrap();
        let response = ExecutionResponse::decode(&output, &execution_request(pending)).unwrap();
        assert!(c.accept_shared(response));
        settle(&mut c);
        let snapshot = c.shared_snapshot();
        let chess = snapshot.chess.unwrap();
        assert_eq!(chess.analysis.status, AnalysisPhase::Incomplete);
        let target = chess.analysis.target.as_ref().unwrap();
        assert_eq!(target.session_version, snapshot.token.version);
        assert_eq!(target.selection_id, snapshot.token.selection);
        assert_eq!(chess.analysis.operation, Some(operation));
        assert_eq!(
            Chess::from_game(&c.session.chess().unwrap().game).unwrap(),
            record
        );
        assert!(snapshot.paused);
        assert_eq!(
            c.session
                .chess()
                .unwrap()
                .document
                .settings()
                .analysis_defaults,
            operation
        );
    }
    #[test]
    fn wrapped_chess_failure_stops_play_without_automatic_retry() {
        let mut c = controller(Color::Black);
        command(&mut c, SessionOperation::Resume {});
        c.shared_snapshot();
        let pending = c.session.chess().unwrap().pending.clone().unwrap();
        assert!(c.accept_observation(ExecutionObservation::new(
            pending,
            ExecutionResult::Chess {
                result: ChessResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Search,
                        message: "test failure".into()
                    }
                }
            }
        )));
        settle(&mut c);
        for _ in 0..2 {
            let snapshot = c.shared_snapshot();
            assert_eq!(snapshot.phase, EnginePhase::Failed);
            assert!(snapshot.paused);
            assert!(snapshot.diagnostic.unwrap().contains("test failure"));
            assert!(c.session.chess().unwrap().pending.is_none());
        }
    }
    #[test]
    fn chess_play_takeback_and_browse_keep_imported_history() {
        let mut c = controller(Color::White);
        let mut initial = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        initial.apply("e2e4".parse().unwrap()).unwrap();
        c.replace_game::<Chess>(ChessRecord::new(&initial.record()), Color::Black)
            .unwrap();
        settle(&mut c);
        command(&mut c, SessionOperation::Resume {});
        command(
            &mut c,
            SessionOperation::ChessPlay {
                action: "e7e5".parse().unwrap(),
            },
        );
        let moved = c.shared_snapshot();
        assert_eq!(moved.event_index, 2);
        command(&mut c, SessionOperation::Inspect { event_index: 0 });
        assert_eq!(c.shared_snapshot().board_ply, 0);
        command(&mut c, SessionOperation::ReturnLive {});
        command(&mut c, SessionOperation::TakeBack {});
        let state = c.session.chess().unwrap();
        assert_eq!(state.game.game().record(), initial.record());
        assert!(state.take_back_target().is_none());
    }
}
