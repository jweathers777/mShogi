//! Human-readable mShogi Tori action notation, not a game-record format.
//!
//! Formatting describes a complete native action. Parsing selects an actual
//! native local choice; live admission (including T06 history) remains Game's job.
use super::{Action, Face, HistorySnapshot, LocalPosition, Position};
use std::{error::Error, fmt};

/// A human string cannot identify a complete native local choice at this position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotationError {
    /// The supplied move has no owned source at this position.
    Source,
    /// A royal or promoted face is not a hand identity.
    DropFace,
    /// The position cannot supply native local choices.
    Position,
    /// No complete local action has this exact spelling.
    NoLocalChoice,
}
impl fmt::Display for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori notation refused: {self:?}")
    }
}
impl Error for NotationError {}

/// Format an admitted event's action against its exact preceding native prefix.
/// This copies only structural state; it does not replay or re-admit the game.
/// The caller still owns the correspondence between prefix and complete action.
pub fn format_history_action(
    before: &HistorySnapshot,
    action: Action,
) -> Result<String, NotationError> {
    format_action(&Position::install(before.state().clone()), action)
}

/// Render source face, exact coordinates, capture, drop and changed result face.
///
/// This formatter does not grant legality or live-game permission. Consumers
/// should supply a native admitted action or re-admit it through Game.submit.
pub fn format_action(position: &Position, action: Action) -> Result<String, NotationError> {
    match action {
        Action::Move {
            from,
            to,
            result_face,
        } => {
            let source = position
                .piece(from)
                .filter(|p| p.side == position.side_to_move())
                .ok_or(NotationError::Source)?;
            let separator = if position.piece(to).is_some() {
                'x'
            } else {
                '-'
            };
            let suffix = if result_face == source.face {
                String::new()
            } else {
                format!("={}", result_face.code())
            };
            Ok(format!(
                "{}{from}{separator}{to}{suffix}",
                source.face.code()
            ))
        }
        Action::Drop { face, to } => {
            if !Face::HANDS.contains(&face) {
                return Err(NotationError::DropFace);
            }
            Ok(format!("{}*{to}", face.code()))
        }
    }
}

/// Select a complete action from rules-owned local choices using its exact label.
///
/// Case, source, capture marker and compulsory resulting face are significant.
/// Parsing cannot bypass a conclusion, stale context or repetition obligation:
/// the caller must still submit a fresh GameRequest to the authoritative Game.
pub fn parse_action(position: &Position, text: &str) -> Result<Action, NotationError> {
    // All supported labels are ASCII and at most eleven bytes. Bound malformed
    // input before generating legal choices; do not accept incidental whitespace.
    if text.is_empty() || text.len() > 11 || !text.is_ascii() {
        return Err(NotationError::NoLocalChoice);
    }
    let local = LocalPosition::admit(position.clone()).map_err(|_| NotationError::Position)?;
    local
        .legal_actions()
        .into_iter()
        .find(|&action| format_action(position, action).is_ok_and(|label| label == text))
        .ok_or(NotationError::NoLocalChoice)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tori::{Game, GameCommand, GameRequest, Hand, Piece, RootMetadata, Side};

    fn root() -> Position {
        let pieces = [
            ("4g", Side::Sente, Face::Phoenix),
            ("4a", Side::Gote, Face::Phoenix),
            ("7c", Side::Sente, Face::Falcon),
            ("7b", Side::Gote, Face::Swallow),
            ("6g", Side::Sente, Face::LeftQuail),
            ("1g", Side::Sente, Face::RightQuail),
        ];
        Position::from_diagram(
            &pieces.map(|(s, side, face)| (s.parse().unwrap(), Piece { side, face })),
            [
                Hand::from_counts(&[(Face::RightQuail, 1)]).unwrap(),
                Hand::empty(),
            ],
            Side::Sente,
            RootMetadata::diagram(31),
        )
        .unwrap()
    }

    #[test]
    fn independent_complete_examples_capture_compulsory_result_and_handed_drop() {
        // Manually established from T02/T03: Sente Falcon7c captures7b, enters
        // the two-rank zone and must become Eagle; RQ is not interchangeable LQ.
        let position = root();
        let capture = Action::Move {
            from: "7c".parse().unwrap(),
            to: "7b".parse().unwrap(),
            result_face: Face::Eagle,
        };
        assert_eq!(format_action(&position, capture).unwrap(), "Fa7cx7b=Ea");
        assert_eq!(parse_action(&position, "Fa7cx7b=Ea"), Ok(capture));
        assert!(parse_action(&position, "Fa7cx7b").is_err());
        assert!(parse_action(&position, "Fa7c-7b=Ea").is_err());
        let drop = Action::Drop {
            face: Face::RightQuail,
            to: "5e".parse().unwrap(),
        };
        assert_eq!(parse_action(&position, "RQ*5e"), Ok(drop));
        assert!(parse_action(&position, "LQ*5e").is_err());
        assert!(parse_action(&position, "LQ6g-6f").is_ok());
        assert!(parse_action(&position, "RQ1g-1f").is_ok());
        for file in 1..=7 {
            let label = format!("RQ*{file}e");
            let action = parse_action(&position, &label).unwrap();
            assert_eq!(format_action(&position, action).unwrap(), label);
        }
        for bad in [
            "Fa8c-7b=Ea",
            "Fa7h-7b=Ea",
            "fa7cx7b=ea",
            "Fa7cx7b+",
            "RQ*5e ",
            "Ea*5e",
            "Ph*5e",
            "Fa7c-1a=Ea",
        ] {
            assert!(parse_action(&position, bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn every_complete_choice_roundtrips_both_owners_and_all_seven_files() {
        let mut game = Game::standard();
        for _ in 0..12 {
            let choices = game.legal_actions();
            for action in &choices {
                let label = format_action(game.position(), *action).unwrap();
                assert_eq!(parse_action(game.position(), &label), Ok(*action));
            }
            let Some(action) = choices.first() else { break };
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: GameCommand::Play { action: *action },
            })
            .unwrap();
        }
        for file in 1..=7 {
            let square = super::super::Square::new(file, 4).unwrap();
            assert_eq!(square.to_string().parse(), Ok(square));
        }
    }

    #[test]
    fn notation_does_not_resurrect_stale_permission_or_replace_canonical_record() {
        let mut game = Game::from_root(root()).unwrap();
        let old = game.context();
        let action = parse_action(game.position(), "Fa7cx7b=Ea").unwrap();
        let mut undo = game
            .submit(GameRequest {
                expected: old.clone(),
                actor: Side::Sente,
                command: GameCommand::Play { action },
            })
            .unwrap();
        assert_eq!(
            format_history_action(&game.event_prefix(0).unwrap(), action).unwrap(),
            "Fa7cx7b=Ea"
        );
        game.undo(&mut undo).unwrap();
        assert!(
            game.submit(GameRequest {
                expected: old,
                actor: Side::Sente,
                command: GameCommand::Play { action }
            })
            .is_err()
        );
        let record = super::super::GameRecord::from_game(&game).unwrap();
        assert_eq!(
            super::super::GameRecord::from_json(record.to_json()).unwrap(),
            record
        );
        assert!(
            super::super::GameRecord::from_json(
                &record
                    .to_json()
                    .replace(super::super::PROFILE_ID, "unknown.tori.profile")
            )
            .is_err()
        );
    }
}
