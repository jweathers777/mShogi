//! Actual native Tenjiku facts and technical choices; playable notation/art belongs to MSH-44.
use super::*;

/// Native input targets remain distinct from the mover's actual route.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TenjikuChoiceShape {
    /// Ordered actual movement landings.
    Travel { route: Vec<String> },
    /// A capture target selected while the mover remains at its origin.
    StationaryCapture { target: String },
}
/// Original physical identity and current native movement face at one coordinate.
#[derive(Clone, Debug, Serialize)]
pub struct TenjikuPieceIdentity {
    pub square: String,
    pub piece: c::Piece,
    pub face: c::Face,
}
/// Exact rules-owned facts at one event prefix, without future history.
#[derive(Clone, Debug, Serialize)]
pub struct TenjikuHistory {
    pub event_index: usize,
    pub board_ply: usize,
    pub declared_total_plies: String,
    pub identities: Vec<TenjikuPieceIdentity>,
    pub royals: Vec<TenjikuPieceIdentity>,
    pub repetition_key: c::RepetitionKey,
    pub known_repetition_positions: usize,
    pub first_recurrence_forbidden: bool,
    pub pending_bare: Option<c::Side>,
    pub conclusion: Option<c::Conclusion>,
    pub draw_offer: Option<c::Side>,
    pub incomplete: Option<c::StopReason>,
    pub last_effects: Option<c::Effects>,
}
impl TenjikuHistory {
    fn new(game: &c::Game) -> Self {
        let identities: Vec<_> = game
            .position()
            .pieces()
            .map(|(at, piece)| TenjikuPieceIdentity {
                square: at.to_string(),
                face: piece.face(),
                piece,
            })
            .collect();
        Self {
            event_index: game.events().len(),
            board_ply: game.snapshot().board_ply,
            declared_total_plies: (game.position().root().declared_completed_plies()
                + game.position().accepted_plies())
            .to_string(),
            royals: identities
                .iter()
                .filter(|i| i.piece.is_royal())
                .cloned()
                .collect(),
            identities,
            repetition_key: game.repetition_key().clone(),
            known_repetition_positions: game.snapshot().board_ply + 1,
            first_recurrence_forbidden: true,
            pending_bare: game.snapshot().pending_bare,
            conclusion: game.conclusion(),
            draw_offer: game.snapshot().draw_offer,
            incomplete: game.snapshot().incomplete,
            last_effects: game.events().iter().rev().find_map(|e| e.effects.clone()),
        }
    }
}
/// Both profiles use the same shared owner while retaining native semantics.
#[derive(Clone, Debug, Serialize)]
pub struct TenjikuPresentation {
    pub profile: RulesProfile,
    pub edition: &'static str,
    pub root: c::RootMetadata,
    pub declared_root_plies: String,
    pub earlier_history_unknown: bool,
    pub historical_reachability_proven: bool,
    pub selected: TenjikuHistory,
    pub live: TenjikuHistory,
    pub execution_supported: bool,
}
impl TenjikuPresentation {
    pub(super) fn new(selected: &c::Game, live: &c::Game) -> Self {
        Self {
            profile: selected.profile().into(),
            edition: selected.profile().edition(),
            root: selected.position().root().clone(),
            declared_root_plies: selected
                .position()
                .root()
                .declared_completed_plies()
                .to_string(),
            earlier_history_unknown: selected.position().root().earlier_history_unknown(),
            historical_reachability_proven: selected.position().root().kind()
                == c::RootKind::Standard,
            selected: TenjikuHistory::new(selected),
            live: TenjikuHistory::new(live),
            execution_supported: true,
        }
    }
}
fn name(game: &c::Game, owner: c::Side) -> String {
    side(game.profile().into(), NativeSide::Tenjiku(owner)).label
}
fn face_name(face: c::Face) -> &'static str {
    c::notation::face_name(face)
}
fn piece_name(piece: c::Piece) -> String {
    if piece.promoted {
        format!(
            "{} (promoted {})",
            face_name(piece.face()),
            face_name(piece.base.face())
        )
    } else {
        face_name(piece.face()).into()
    }
}
pub(super) fn choice(action: c::Action, alternatives: &[c::Action]) -> PresentedChoice {
    let alternative = c::Action {
        promote: !action.promote,
        ..action.clone()
    };
    let stationary = action.kind == c::ActionKind::Igui;
    let completion = if stationary {
        Some(Completion {
            label: format!(
                "Capture {} without moving",
                action.target.expect("native target")
            ),
            glyph: None,
        })
    } else if action.promote || alternatives.contains(&alternative) {
        Some(Completion {
            label: if action.promote {
                "Promote"
            } else {
                "Do not promote"
            }
            .into(),
            glyph: None,
        })
    } else {
        None
    };
    let route: Vec<_> = action.route.iter().map(ToString::to_string).collect();
    let (steps, shape) = if stationary {
        let target = action.target.expect("native igui target").to_string();
        (
            vec![target.clone()],
            TenjikuChoiceShape::StationaryCapture { target },
        )
    } else {
        (route.clone(), TenjikuChoiceShape::Travel { route })
    };
    PresentedChoice {
        origin: ChoiceOrigin::Board {
            square: action.from.to_string(),
        },
        from: action.from.to_string(),
        steps,
        tenjiku: Some(shape),
        action: serde_json::to_string(&action).expect("native complete action"),
        completion,
    }
}
pub(super) fn inspect(game: &c::Game, origin: &str) -> Result<inspection::Inspected, String> {
    let square = c::Square::all()
        .find(|s| s.to_string() == origin)
        .ok_or("Unknown Tenjiku coordinate.")?;
    let piece = game
        .position()
        .piece(square)
        .ok_or("No native piece at this origin.")?;
    let actions: Vec<_> = game
        .local_position()
        .inspection(square)
        .into_iter()
        .map(|v| v.action)
        .collect();
    let choices = actions
        .iter()
        .cloned()
        .map(|a| choice(a, &actions))
        .collect();
    Ok((
        GameKind::Tenjiku,
        NativeSide::Tenjiku(piece.side),
        format!("{} {}", name(game, piece.side), piece_name(piece)),
        choices,
    ))
}
fn event_label(game: &c::Game, event: &c::GameEvent) -> String {
    let actor = name(game, event.actor);
    match &event.command {
        c::GameCommand::Play { action } => {
            let before = game
                .event_prefix(event.index)
                .expect("accepted event prefix");
            format!(
                "{actor} · {}",
                c::notation::format_history_action(&before, action.clone())
                    .expect("accepted action")
            )
        }
        c::GameCommand::Resign => format!("{actor} resigned"),
        c::GameCommand::OfferDraw => format!("{actor} offered a draw"),
        c::GameCommand::AcceptDraw => format!("{actor} accepted the draw"),
        c::GameCommand::DeclineDraw => format!("{actor} declined the draw"),
        c::GameCommand::Stop { reason } => format!("{reason:?} · unscored"),
    }
}
pub(super) fn status(game: &c::Game, has_board_actions: bool) -> PresentedStatus {
    let label = match game.conclusion() {
        Some(c::Conclusion::Agreement) => "Draw by agreement".into(),
        Some(c::Conclusion::BothBareRoyals) => "Draw: both sides bare royals".into(),
        Some(c::Conclusion::SimultaneousRoyalLoss) => "Draw: simultaneous final royal loss".into(),
        Some(outcome) => format!(
            "{} wins by {}",
            name(game, outcome.winner().expect("winning native conclusion")),
            match outcome {
                c::Conclusion::RoyalCapture { .. } => "final royal destruction",
                c::Conclusion::BareRoyalLoss { .. } => "bare royal loss",
                c::Conclusion::Resignation { .. } => "resignation",
                _ => unreachable!("draws handled above"),
            }
        ),
        None if game.snapshot().incomplete.is_some() => format!(
            "{:?} · unscored",
            game.snapshot().incomplete.expect("native annotation")
        ),
        None if game.snapshot().pending_bare.is_some() => format!(
            "{} to move · bare royal reply required",
            name(game, game.position().side_to_move())
        ),
        None if !has_board_actions => "No admitted board action · unconcluded and unscored".into(),
        None => format!("{} to move", name(game, game.position().side_to_move())),
    };
    PresentedStatus {
        concluded: game.conclusion().is_some(),
        label,
        score_half_points: game.conclusion().map(|c| match c.winner() {
            Some(c::Side::Sente) => [2, 0],
            Some(c::Side::Gote) => [0, 2],
            None => [1, 1],
        }),
    }
}
impl DesktopController {
    pub(crate) fn tenjiku_snapshot(&self) -> SharedSnapshot {
        let state = self.session.tenjiku().expect("native owner");
        let game = state.selected();
        let position = game.position();
        let profile: RulesProfile = game.profile().into();
        let sides: Vec<_> = [c::Side::Sente, c::Side::Gote]
            .map(|s| side(profile, NativeSide::Tenjiku(s)))
            .into();
        let files: Vec<_> = (1..=16).rev().map(|n| n.to_string()).collect();
        let ranks: Vec<_> = ('a'..='p').map(|n| n.to_string()).collect();
        let historical = state.viewed.is_some();
        let ended = game.conclusion().is_some();
        SharedSnapshot {
            moves: self.present_moves(game.events().len()),
            xiangqi: None,
            tori: None,
            chu: None,
            tenjiku: Some(state.presentation.clone()),
            open_token: self.shared_open_token(),
            document: self.shared_document(),
            io: self.io_state.clone(),
            capacity: self.session.capacity.clone(),
            token: self.shared_token(),
            game: GameKind::Tenjiku,
            profile,
            edition: game.profile().edition(),
            human: side(profile, NativeSide::Tenjiku(state.document.human())),
            turn: side(profile, NativeSide::Tenjiku(position.side_to_move())),
            sides: sides.clone(),
            board: BoardDescriptor {
                intersections: None,
                files: files.clone(),
                ranks: ranks.clone(),
                label: "Tenjiku Shogi 16×16 verification board".into(),
                initial_focus: "8i".into(),
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
            pieces: position
                .pieces()
                .map(|(square, p)| PresentedPiece {
                    square: square.to_string(),
                    owner: side(profile, NativeSide::Tenjiku(p.side)).key,
                    kind: p.base.code().into(),
                    promoted: p.promoted,
                    text: format!("{}{}", if p.promoted { "+" } else { "" }, p.face().code()),
                    label: format!("{square}, {} {}", name(game, p.side), piece_name(p)),
                })
                .collect(),
            hands: vec![],
            choices: state.choices.clone(),
            capabilities: CommandCapabilities {
                board_action: !historical
                    && !ended
                    && position.side_to_move() == state.document.human(),
                drops: false,
                promotion: true,
                take_back: !historical && state.take_back_target().is_some(),
                undo: !historical && !state.game.events().is_empty(),
                engine_turn: true,
                move_analysis: true,
                chess_full: false,
                claims: false,
                declaration: false,
                resign: !historical && !ended,
                hands: false,
                history: true,
                save: true,
            },
            status: state.status.clone(),
            events: state
                .game
                .events()
                .iter()
                .map(|e| PresentedEvent {
                    index: e.index + 1,
                    board_ply: e.board_ply,
                    label: event_label(&state.game, e),
                })
                .collect(),
            event_index: game.events().len(),
            board_ply: game.snapshot().board_ply as u64,
            play_limits: play_limits(),
            historical,
            paused: state.paused,
            close_requires_save: self.close_requires_save(),
            phase: state.phase,
            execution_purpose: state.pending.as_ref().map(|p| {
                if matches!(p.purpose, ExecutionPurpose::EngineTurn {}) {
                    "engine_turn"
                } else {
                    "move_analysis"
                }
            }),
            diagnostic: state.diagnostic.clone(),
            declaration_advice: vec![],
            context: CanonicalRecord::tenjiku(
                c::GameRecord::from_game(game).expect("admitted native record"),
            ),
            analysis: state.analysis.as_ref().map(|o| PresentedObservation {
                observation: o.clone(),
                historical: false,
                application_error: None,
            }),
            observations: state
                .document
                .observations()
                .iter()
                .map(|o| PresentedObservation {
                    observation: o.clone(),
                    historical: !state.current_observations.contains(&o.context.request_id),
                    application_error: None,
                })
                .collect(),
            search_observations: state
                .document
                .observations()
                .iter()
                .enumerate()
                .map(|(i, o)| display_observation(i, o, state))
                .collect(),
            score_meaning: "Transparent material evaluation in Pawns from the analyzed native side; native terminal wins use board-action distance. Operational failures and absence of board actions remain unscored.",
            chess: None,
            chess_match: None,
        }
    }
}
