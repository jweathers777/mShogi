//! Native facts and technical complete actions, not the finished MSH-43 renderer.
use super::*;

/// Exact Chu identity at a native coordinate. Promotion does not erase its origin.
#[derive(Clone, Debug, Serialize)]
pub struct ChuPieceIdentity {
    /// Opaque native coordinate.
    pub square: String,
    /// Original native piece, including native owner and one-flip state.
    pub piece: c::Piece,
    /// Exact current movement face, supplied by the native piece.
    pub face: c::Face,
}
/// King/Prince identity and local capture threat; this is not Chess self-check.
#[derive(Clone, Debug, Serialize)]
pub struct ChuRoyal {
    /// Exact native original/promoted royal identity.
    pub identity: ChuPieceIdentity,
    /// Hypothetical opposing local capture influence, not a legal-response permit.
    pub local_capture_threat: bool,
}
/// Authoritative facts at an exact prefix, separate from execution observations.
#[derive(Clone, Debug, Serialize)]
pub struct ChuHistory {
    /// Complete event prefix, independent of board ply.
    pub event_index: usize,
    /// Known complete board actions.
    pub board_ply: usize,
    /// Exact declared counter at this prefix, distinct from known board history.
    pub declared_total_plies: String,
    /// Every original/promoted native identity.
    pub identities: Vec<ChuPieceIdentity>,
    /// All surviving King/Prince pieces.
    pub royals: Vec<ChuRoyal>,
    /// Per first/second defending side, native has_capture_all: whether an opposing
    /// complete action can capture all its royals. Native history admission applies
    /// when that attacker is the actual mover; otherwise this is advisory local
    /// geometry. This is never permission to move or a forced-mate proof. None
    /// means the bounded native query could not establish it.
    pub capture_all_royals_threat: [Option<bool>; 2],
    /// Bounded native threat-query refusals, never converted into false or mate.
    pub threat_diagnostics: Vec<String>,
    /// Actual Lion-law rights at this prefix.
    pub lion_rights: c::LionProtection,
    /// Actual known occurrences, excluding unknown earlier history.
    pub occurrences: Vec<usize>,
    /// Full must-vary witnesses and progress.
    pub obligations: Vec<c::Obligation>,
    /// Exact native conclusion, independent of evaluation and process state.
    pub conclusion: Option<c::Conclusion>,
    /// Actual current opposing draw offer, not a concluded draw.
    pub draw_offer: Option<c::Side>,
    /// Unscored native operational annotation.
    pub incomplete: Option<c::StopReason>,
    /// Actual last complete board action and all ordered derived effects.
    pub last_effects: Option<c::Effects>,
}
impl ChuHistory {
    fn new(game: &c::Game) -> Self {
        let local = game.local_position();
        let influence = [
            local.capture_influence(c::Side::Gote),
            local.capture_influence(c::Side::Sente),
        ];
        let identities: Vec<_> = game
            .position()
            .pieces()
            .map(|(at, piece)| ChuPieceIdentity {
                square: at.to_string(),
                face: piece.face(),
                piece,
            })
            .collect();
        let royals = identities
            .iter()
            .filter(|i| matches!(i.face, c::Face::King | c::Face::Prince))
            .map(|i| {
                let at: c::Square = i.square.parse().expect("native coordinate");
                let index = usize::from(i.piece.side == c::Side::Gote);
                ChuRoyal {
                    identity: i.clone(),
                    local_capture_threat: influence[index].iter().any(|(_, victim)| *victim == at),
                }
            })
            .collect();
        let mut threat_diagnostics = vec![];
        let capture_all_royals_threat =
            [c::Side::Sente, c::Side::Gote].map(|s| match game.has_capture_all(s) {
                Ok(v) => Some(v),
                Err(e) => {
                    threat_diagnostics.push(e.to_string());
                    None
                }
            });
        Self {
            event_index: game.events().len(),
            board_ply: game.snapshot().board_ply,
            declared_total_plies: game.position().declared_total_plies().to_string(),
            identities,
            royals,
            capture_all_royals_threat,
            threat_diagnostics,
            lion_rights: local.protection().clone(),
            occurrences: game.occurrences().to_vec(),
            obligations: game.obligations().to_vec(),
            conclusion: game.conclusion(),
            draw_offer: game.snapshot().draw_offer,
            incomplete: game.snapshot().incomplete,
            last_effects: game.events().iter().rev().find_map(|e| e.effects.clone()),
        }
    }
}
/// Native Chu root provenance and visibly separate selected/live facts.
#[derive(Clone, Debug, Serialize)]
pub struct ChuPresentation {
    /// Native root category, declared counters and known-history provenance.
    pub root: c::RootMetadata,
    /// Exact declared root counter as decimal text for JavaScript consumers.
    /// This is not a claim of known moves before the imported root.
    pub declared_root_plies: String,
    /// Native provenance fact; consumers need not infer history from root labels.
    pub earlier_history_unknown: bool,
    /// Only native established reachability, never diagram admissibility.
    pub historical_reachability_proven: bool,
    /// Exact viewed history with no future knowledge.
    pub selected: ChuHistory,
    /// Current authoritative live history.
    pub live: ChuHistory,
    /// Functional native execution exists; normal product activation remains separately gated.
    pub execution_supported: bool,
}
impl ChuPresentation {
    pub(super) fn new(selected: &c::Game, live: &c::Game) -> Self {
        Self {
            root: selected.position().root().clone(),
            declared_root_plies: selected
                .position()
                .root()
                .declared_completed_plies()
                .to_string(),
            earlier_history_unknown: selected.position().root().earlier_history_unknown(),
            historical_reachability_proven: selected
                .position()
                .root()
                .historical_reachability_proven(),
            selected: ChuHistory::new(selected),
            live: ChuHistory::new(live),
            execution_supported: true,
        }
    }
}
fn piece_name(piece: c::Piece) -> String {
    if piece.promoted {
        format!("{:?} (promoted {:?})", piece.face(), piece.base)
    } else {
        format!("{:?}", piece.base)
    }
}
pub(super) fn choice(action: c::Action, alternatives: &[c::Action]) -> PresentedChoice {
    let alternative = c::Action {
        promote: !action.promote,
        ..action.clone()
    };
    let completion = (action.promote || alternatives.contains(&alternative)).then(|| Completion {
        label: if action.promote {
            "Promote"
        } else {
            "Do not promote"
        }
        .into(),
        glyph: None,
    });
    PresentedChoice {
        origin: ChoiceOrigin::Board {
            square: action.from.to_string(),
        },
        from: action.from.to_string(),
        steps: action.route.iter().map(ToString::to_string).collect(),
        action: serde_json::to_string(&action).expect("native action"),
        completion,
    }
}
pub(super) fn inspect(game: &c::Game, origin: &str) -> Result<inspection::Inspected, String> {
    let square: c::Square = origin.parse().map_err(|e: c::SquareError| e.to_string())?;
    let piece = game
        .position()
        .piece(square)
        .ok_or("No native piece at this origin.")?;
    let choices = game
        .local_position()
        .inspection(square)
        .into_iter()
        .map(|v| {
            let action = v.action;
            PresentedChoice {
                origin: ChoiceOrigin::Board {
                    square: origin.into(),
                },
                from: origin.into(),
                steps: action.route.iter().map(ToString::to_string).collect(),
                action: serde_json::to_string(&action).expect("native action"),
                completion: Some(Completion {
                    label: if action.promote {
                        "Promote"
                    } else {
                        "Complete native route"
                    }
                    .into(),
                    glyph: None,
                }),
            }
        })
        .collect();
    Ok((
        GameKind::Chu,
        NativeSide::Chu(piece.side),
        format!(
            "{} {}",
            side(NativeSide::Chu(piece.side)).label,
            piece_name(piece)
        ),
        choices,
    ))
}
fn event_label(event: &c::GameEvent) -> String {
    let actor = side(NativeSide::Chu(event.actor)).label;
    match &event.command {
        c::GameCommand::Play { action } => format!(
            "{actor} · Technical action {}",
            serde_json::to_string(action).unwrap()
        ),
        c::GameCommand::Resign => format!("{actor} resigned"),
        c::GameCommand::OfferDraw => format!("{actor} offered a draw"),
        c::GameCommand::AcceptDraw => format!("{actor} accepted the draw"),
        c::GameCommand::DeclineDraw => format!("{actor} declined the draw"),
        c::GameCommand::Stop { reason } => format!("{reason:?} · unscored"),
    }
}
pub(super) fn status(game: &c::Game) -> PresentedStatus {
    let label = match game.conclusion() {
        Some(c::Conclusion::AgreedDraw) => "Draw by agreement".into(),
        Some(outcome) => {
            let reason = match outcome {
                c::Conclusion::RoyalCapture { .. } => "final royal capture",
                c::Conclusion::Mate { .. } => "mate",
                c::Conclusion::Material { .. } => "material exhaustion",
                c::Conclusion::Resignation { .. } => "resignation",
                c::Conclusion::AgreedDraw => unreachable!(),
            };
            format!(
                "{} wins by {reason}",
                side(NativeSide::Chu(outcome.winner().unwrap())).label
            )
        }
        None if game.snapshot().incomplete.is_some() => {
            format!("{:?} · unscored", game.snapshot().incomplete.unwrap())
        }
        None if game.status() == c::GameStatus::NoBoardActions => {
            "No admitted board action · unresolved and unscored".into()
        }
        None => format!(
            "{} to move",
            side(NativeSide::Chu(game.position().side_to_move())).label
        ),
    };
    PresentedStatus {
        concluded: game.conclusion().is_some(),
        label,
        score_half_points: game.conclusion().map(|v| match v.winner() {
            Some(c::Side::Sente) => [2, 0],
            Some(c::Side::Gote) => [0, 2],
            None => [1, 1],
        }),
    }
}
impl DesktopController {
    pub(crate) fn chu_snapshot(&self) -> SharedSnapshot {
        let state = self.session.chu().unwrap();
        let game = state.selected();
        let position = game.position();
        let sides: Vec<_> = [c::Side::Sente, c::Side::Gote]
            .map(|s| side(NativeSide::Chu(s)))
            .into();
        let files: Vec<_> = (1..=12).rev().map(|n| n.to_string()).collect();
        let ranks: Vec<_> = ('a'..='l').map(|n| n.to_string()).collect();
        let historical = state.viewed.is_some();
        let ended = game.conclusion().is_some();
        SharedSnapshot {
            moves: self.present_moves(game.events().len()),
            xiangqi: None,
            tori: None,
            chu: Some(state.presentation.clone()),
            open_token: self.shared_open_token(),
            document: self.shared_document(),
            io: self.io_state.clone(),
            capacity: self.session.capacity.clone(),
            token: self.shared_token(),
            game: GameKind::Chu,
            profile: RulesProfile::ChuJcsa2019V1,
            edition: c::PROFILE_EDITION,
            human: side(NativeSide::Chu(state.document.human())),
            turn: side(NativeSide::Chu(position.side_to_move())),
            sides: sides.clone(),
            board: BoardDescriptor {
                intersections: None,
                files: files.clone(),
                ranks: ranks.clone(),
                label: "Chu Shogi 12×12 board".into(),
                initial_focus: "6g".into(),
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
                    owner: side(NativeSide::Chu(p.side)).key,
                    kind: p.base.code().into(),
                    promoted: p.promoted,
                    text: p.code(),
                    label: format!(
                        "{square}, {} {}",
                        side(NativeSide::Chu(p.side)).label,
                        piece_name(p)
                    ),
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
                    label: event_label(e),
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
            context: CanonicalRecord::chu(
                c::GameRecord::from_game(game).expect("admitted native game"),
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
                .map(|(index, o)| display_observation(index, o, state))
                .collect(),
            score_meaning: "Native root-side evaluation in hundredths of a Pawn. Win/loss distances count complete board actions, not necessarily mate. Failures and incomplete work are unscored.",
            chess: None,
            chess_match: None,
        }
    }
}
