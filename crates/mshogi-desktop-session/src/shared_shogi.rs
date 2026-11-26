//! Shogi in the sole desktop runtime: the generic native session (`native_session`)
//! with the Shogi presentation. Rules and persistence stay native-owned.
use super::native_session::*;
use super::*;
pub(crate) use crate::native_game::Shogi;
use mshogi_core::shogi as s;

/// The Shogi session.
pub(crate) type ShogiSession = NativeSession<Shogi>;

impl SessionGame for Shogi {
    type Presentation = ();
    const HANDS: bool = true;
    const BOARD_SIZE: u8 = 9;
    const BOARD_LABEL: &'static str = "Shogi board";
    const INITIAL_FOCUS: &'static str = "5g";
    const SCORE_MEANING: &'static str = "Native shogi root-side hundredths of pawn; terminal win distance counts complete command edges, including declarations. Not a chess mate score.";

    session_wiring!(Shogi, state: shogi, shogi_mut);

    fn play(action: &str) -> Result<s::GameCommand, String> {
        action
            .parse()
            .map(s::GameCommand::Play)
            .map_err(|e: s::ActionParseError| e.to_string())
    }

    fn presentation(_selected: &s::Game, _live: &s::Game) {}
    fn status(game: &s::Game, _has_actions: bool) -> PresentedStatus {
        status(game)
    }
    fn choice(game: &s::Game, action: s::Action, all: &[s::Action]) -> PresentedChoice {
        let mover = shogi_side(game.position().side_to_move());
        let (from, to, completion) = match action {
            s::Action::Move { from, to, promote } => {
                let alternate = s::Action::Move {
                    from,
                    to,
                    promote: !promote,
                };
                (
                    from.to_string(),
                    to,
                    (promote || all.contains(&alternate)).then(|| Completion {
                        label: if promote { "Promote" } else { "Do not promote" }.into(),
                        glyph: None,
                    }),
                )
            }
            s::Action::Drop { kind, to } => {
                (format!("hand:{}:{}", mover.key, piece_text(kind)), to, None)
            }
        };
        PresentedChoice {
            tenjiku: None,
            origin: match action {
                s::Action::Move { from, .. } => ChoiceOrigin::Board {
                    square: from.to_string(),
                },
                s::Action::Drop { kind, .. } => ChoiceOrigin::Hand {
                    owner: mover.key,
                    piece_kind: format!("{kind:?}").to_lowercase(),
                },
            },
            action: action.to_string(),
            from,
            steps: vec![to.to_string()],
            completion,
        }
    }
    fn legal_actions(
        game: &s::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<s::Action>, String> {
        if stop() {
            return Err("Native operation stopped before acceptance.".into());
        }
        // A concluded game offers no choices.
        Ok(if game.conclusion().is_some() {
            vec![]
        } else {
            game.position().legal_actions()
        })
    }
    fn retire_context(_game: &mut s::Game) {
        // A Shogi game has no runtime identity; its context is its history.
    }
    fn event(game: &s::Game, index: usize) -> (s::Side, CommandKind) {
        let event = &game.events()[index];
        (event.actor, Self::command_kind(&event.command))
    }
    fn command_kind(command: &s::GameCommand) -> CommandKind {
        match command {
            s::GameCommand::Play(_) => CommandKind::Play,
            s::GameCommand::Resign | s::GameCommand::DeclareEnteringKing => CommandKind::Terminal,
        }
    }
    fn telemetry(result: &mshogi_engine::ShogiResult) -> TelemetryView {
        engine_telemetry(result, choice_label)
    }
    fn incomplete(result: &mshogi_engine::ShogiResult) -> bool {
        engine_incomplete(result)
    }
    fn declaration_advice(game: &s::Game) -> Vec<DeclarationAdvice> {
        [s::Side::Sente, s::Side::Gote]
            .into_iter()
            .map(|actor| {
                let e = game.declaration_eligibility(actor);
                DeclarationAdvice {
                    actor: shogi_side(actor),
                    eligible: e.eligible(),
                    own_turn: e.own_turn,
                    king_in_camp: e.king_in_camp,
                    camp_pieces: e.camp_pieces,
                    points: e.points,
                    required_points: e.required_points,
                    not_in_check: e.not_in_check,
                }
            })
            .collect()
    }
    fn can_declare(game: &s::Game, human: s::Side) -> bool {
        game.declaration_eligibility(human).eligible()
    }
    fn edition(_game: &s::Game) -> &'static str {
        RulesProfile::ShogiCsa36V1.edition()
    }
    fn pieces(game: &s::Game) -> Vec<PresentedPiece> {
        let position = game.position();
        s::Square::all()
            .filter_map(|square| {
                position.piece(square).map(|p| {
                    let owner = shogi_side(p.side);
                    let text = format!(
                        "{}{}",
                        if p.promoted { "+" } else { "" },
                        piece_text(p.kind)
                    );
                    PresentedPiece {
                        square: square.to_string(),
                        owner: owner.key,
                        kind: kind_name(p.kind),
                        promoted: p.promoted,
                        label: format!(
                            "{square}, {} {}{}",
                            owner.label,
                            if p.promoted { "promoted " } else { "" },
                            kind_name(p.kind)
                        ),
                        text,
                    }
                })
            })
            .collect()
    }
    fn hands(game: &s::Game) -> Vec<PresentedHand> {
        let position = game.position();
        let mut hands = vec![];
        for actor in [s::Side::Sente, s::Side::Gote] {
            let owner = shogi_side(actor);
            for kind in s::PieceKind::HAND_KINDS {
                let text = piece_text(kind).to_string();
                let count = position.hand(actor).count(kind);
                hands.push(PresentedHand {
                    origin: format!("hand:{}:{text}", owner.key),
                    owner: owner.key.clone(),
                    kind: kind_name(kind),
                    count,
                    label: format!("{} {} in hand, {count}", owner.label, kind_name(kind)),
                    text,
                });
            }
        }
        hands
    }
    fn presented_events(game: &s::Game) -> Vec<PresentedEvent> {
        let mut ply = 0;
        game.events()
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
                        shogi_side(e.actor).label,
                        command_label(e.command)
                    ),
                }
            })
            .collect()
    }
    fn board_ply(game: &s::Game) -> u64 {
        game.events()
            .iter()
            .filter(|e| matches!(e.command, s::GameCommand::Play(_)))
            .count() as u64
    }
}

fn shogi_side(side: s::Side) -> PresentedSide {
    shared::side(RulesProfile::ShogiCsa36V1, NativeSide::Shogi(side))
}
pub(super) fn piece_text(kind: s::PieceKind) -> &'static str {
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
fn choice_label(choice: &mshogi_engine::ShogiChoice) -> String {
    match choice {
        mshogi_engine::ShogiChoice::Move {
            action: s::Action::Move { from, to, promote },
        } => format!("{from} → {to}{}", if *promote { " · promote" } else { "" }),
        mshogi_engine::ShogiChoice::Move {
            action: s::Action::Drop { kind, to },
        } => format!("Drop {} at {to}", format!("{kind:?}").to_lowercase()),
        mshogi_engine::ShogiChoice::Declare { actor } => {
            format!("{} declares entering king", shogi_side(*actor).label)
        }
    }
}
fn status(game: &s::Game) -> PresentedStatus {
    let label = match game.conclusion() {
        None => format!(
            "{} to move{}",
            shogi_side(game.position().side_to_move()).label,
            if game.position().is_in_check(game.position().side_to_move()) {
                " · check"
            } else {
                ""
            }
        ),
        Some(s::Conclusion::Win { winner, reason }) => {
            let winner_label = shogi_side(winner).label;
            let loser_label = shogi_side(winner.opponent()).label;
            match reason {
                s::WinReason::NoLegalAction { in_check: true } => {
                    format!("{winner_label} wins by checkmate")
                }
                s::WinReason::NoLegalAction { in_check: false } => {
                    format!("{winner_label} wins: opponent has no legal action")
                }
                s::WinReason::ContinuousCheck { .. } => {
                    format!("{loser_label} loses by continuous check")
                }
                s::WinReason::Resignation => {
                    format!("{loser_label} resigned · {winner_label} wins")
                }
                s::WinReason::EnteringKing => {
                    format!("{winner_label} wins by entering-king declaration")
                }
                s::WinReason::FalseDeclaration => {
                    format!("{loser_label} loses by false entering-king declaration")
                }
            }
        }
        Some(s::Conclusion::Draw(reason)) => match reason {
            s::DrawReason::Fourfold { .. } => "Draw by fourfold repetition".into(),
            s::DrawReason::Length => "Draw at the 320-action game limit".into(),
        },
    };
    PresentedStatus {
        concluded: game.conclusion().is_some(),
        label,
        score_half_points: game.conclusion().map(|c| c.score_half_points()),
    }
}
/// Lowercase native piece name, as the presentation spells it.
fn kind_name(kind: s::PieceKind) -> String {
    format!("{kind:?}").to_lowercase()
}
