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
pub struct Presentation {
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
impl Presentation {
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
pub(super) fn profile(game: &c::Game) -> RulesProfile {
    game.profile().into()
}
pub(super) fn edition(game: &c::Game) -> &'static str {
    game.profile().edition()
}
pub(super) fn pieces(game: &c::Game) -> Vec<PresentedPiece> {
    let profile = profile(game);
    game.position()
        .pieces()
        .map(|(square, p)| PresentedPiece {
            square: square.to_string(),
            owner: side(profile, NativeSide::Tenjiku(p.side)).key,
            kind: p.base.code().into(),
            promoted: p.promoted,
            text: format!("{}{}", if p.promoted { "+" } else { "" }, p.face().code()),
            label: format!("{square}, {} {}", name(game, p.side), piece_name(p)),
        })
        .collect()
}
pub(super) fn events(game: &c::Game) -> Vec<PresentedEvent> {
    game.events()
        .iter()
        .map(|e| PresentedEvent {
            index: e.index + 1,
            board_ply: e.board_ply,
            label: event_label(game, e),
        })
        .collect()
}
