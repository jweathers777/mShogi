//! Compact mShogi Tenjiku action notation, not historical notation or a record.
//!
//! Original code plus optional `+`, origin, each actual landing prefixed by `-`
//! or `x`, and optional final `^` for the requested flip. Stationary igui uses
//! `!x` and its exact target. Native derived burn/death phases remain separate.
use super::{Action, ActionKind, HistorySnapshot, LocalPosition, Position};
use std::{error::Error, fmt};

/// A human label cannot describe or identify a complete native action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotationError {
    /// No current-owner original occupies the source.
    Source,
    /// Travel/igui fields do not form a complete native action shape.
    Shape,
    /// No complete native local choice has the exact supplied spelling.
    NoLocalChoice,
}
impl fmt::Display for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku notation refused: {self:?}")
    }
}
impl Error for NotationError {}

/// Describe an admitted action against its exact preceding native prefix.
pub fn format_history_action(
    before: &HistorySnapshot,
    action: Action,
) -> Result<String, NotationError> {
    // Formatting an already admitted immutable snapshot requires its board,
    // not another LocalPosition admission or generation of legal continuations.
    let position = Position::install(before.state().clone()).map_err(|_| NotationError::Source)?;
    format_action(&position, action)
}

/// Describe the complete choice without granting native legality or permission.
pub fn format_action(position: &Position, action: Action) -> Result<String, NotationError> {
    let source = position
        .piece(action.from)
        .filter(|p| p.side == position.side_to_move())
        .ok_or(NotationError::Source)?;
    let mut label = format!(
        "{}{}{}",
        source.base.code(),
        if source.promoted { "+" } else { "" },
        action.from
    );
    match action.kind {
        ActionKind::Igui => {
            if !action.route.is_empty() || action.promote {
                return Err(NotationError::Shape);
            }
            let target = action.target.ok_or(NotationError::Shape)?;
            label.push_str(&format!("!x{target}"));
        }
        ActionKind::Travel => {
            if !(1..=3).contains(&action.route.len()) || action.target.is_some() {
                return Err(NotationError::Shape);
            }
            let mut removed = vec![action.from];
            for landing in action.route {
                let capture = !removed.contains(&landing) && position.piece(landing).is_some();
                label.push(if capture { 'x' } else { '-' });
                label.push_str(&landing.to_string());
                removed.push(landing);
            }
            if action.promote {
                label.push('^');
            }
        }
    }
    Ok(label)
}

/// Select a real complete native local choice; Game still owns history admission.
pub fn parse_local_action(local: &LocalPosition, text: &str) -> Result<Action, NotationError> {
    if text.is_empty() || text.len() > 32 || !text.is_ascii() {
        return Err(NotationError::NoLocalChoice);
    }
    local
        .legal_actions()
        .into_iter()
        .find(|a| format_action(local.position(), a.clone()).is_ok_and(|label| label == text))
        .ok_or(NotationError::NoLocalChoice)
}

/// Select a structural root's local choice, without inventing earlier game history.
pub fn parse_action(position: &Position, text: &str) -> Result<Action, NotationError> {
    let local = LocalPosition::admit(position.clone()).map_err(|_| NotationError::Source)?;
    parse_local_action(&local, text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tenjiku::{
        Base, Game, GameCommand, GameRequest, Piece, Profile, RootMetadata, Side, Square,
    };
    fn sq(f: u8, r: u8) -> Square {
        Square::new(f, r).unwrap()
    }
    fn local(profile: Profile, entries: &[(u8, u8, Side, Base, bool)]) -> LocalPosition {
        LocalPosition::admit(
            Position::from_diagram(
                profile,
                &entries
                    .iter()
                    .enumerate()
                    .map(|(id, &(f, r, side, base, promoted))| {
                        (
                            sq(f, r),
                            Piece::new(id as u16, side, base, promoted).unwrap(),
                        )
                    })
                    .collect::<Vec<_>>(),
                Side::Sente,
                RootMetadata::diagram(0),
            )
            .unwrap(),
        )
        .unwrap()
    }
    #[test]
    fn independently_specified_lion_routes_captures_and_returns_are_distinct() {
        for profile in Profile::ALL {
            let local = local(
                profile,
                &[
                    (8, 8, Side::Sente, Base::Ln, false),
                    (8, 7, Side::Gote, Base::S, false),
                    (8, 6, Side::Gote, Base::R, false),
                ],
            );
            for (label, route) in [
                ("Ln8hx8f", vec![sq(8, 6)]),
                ("Ln8hx8gx8f", vec![sq(8, 7), sq(8, 6)]),
                ("Ln8hx8g-8h", vec![sq(8, 7), sq(8, 8)]),
                ("Ln8h-7g-8h", vec![sq(7, 7), sq(8, 8)]),
            ] {
                let expected = Action {
                    kind: ActionKind::Travel,
                    from: sq(8, 8),
                    route,
                    promote: false,
                    target: None,
                };
                assert_eq!(
                    parse_local_action(&local, label),
                    Ok(expected.clone()),
                    "{profile:?} {label}"
                );
                assert_eq!(format_action(local.position(), expected).unwrap(), label);
            }
            for bad in [
                "Ln8h-8f",
                "Ln8h-8g-8h",
                "Ln8hx8gx8h",
                "ln8hx8f",
                "Ln8hx8f ",
                "Ln8h",
                "Ln8hx17f",
                "Ln8h!x8g",
            ] {
                assert!(parse_local_action(&local, bad).is_err(), "{bad}");
            }
        }
    }
    #[test]
    fn original_promoted_identity_igui_three_landings_and_flip_remain_explicit() {
        for profile in Profile::ALL {
            let local = local(
                profile,
                &[
                    (8, 8, Side::Sente, Base::ChS, true),
                    (9, 8, Side::Gote, Base::P, false),
                    (4, 8, Side::Sente, Base::VGn, false),
                    (12, 6, Side::Sente, Base::P, false),
                ],
            );
            let igui = Action::igui(sq(8, 8), sq(9, 8));
            assert_eq!(parse_local_action(&local, "ChS+8h!x9h"), Ok(igui.clone()));
            assert_eq!(format_action(local.position(), igui).unwrap(), "ChS+8h!x9h");
            let three = Action {
                kind: ActionKind::Travel,
                from: sq(4, 8),
                route: vec![sq(4, 7), sq(5, 7), sq(5, 6)],
                promote: false,
                target: None,
            };
            assert_eq!(parse_local_action(&local, "VGn4h-4g-5g-5f"), Ok(three));
            for label in ["P12f-12e", "P12f-12e^"] {
                let a = parse_local_action(&local, label).unwrap();
                assert_eq!(format_action(local.position(), a).unwrap(), label);
            }
            for bad in [
                "HT8h!x9h",
                "ChS8h!x9h",
                "ChS+8hx9h",
                "ChS+8h!x9h^",
                "VGn4hx4g-5g-5f",
                "P12f-12e+",
                "P12f-12e^^",
            ] {
                assert!(parse_local_action(&local, bad).is_err(), "{bad}");
            }
        }
    }
    #[test]
    fn admitted_history_preserves_capture_label_undo_and_fresh_permission() {
        for profile in Profile::ALL {
            let local = local(
                profile,
                &[
                    (16, 16, Side::Sente, Base::K, false),
                    (1, 1, Side::Gote, Base::K, false),
                    (8, 8, Side::Sente, Base::Ln, false),
                    (8, 7, Side::Gote, Base::S, false),
                    (3, 3, Side::Gote, Base::G, false),
                ],
            );
            let mut game = Game::from_root(local.position().clone()).unwrap();
            let before = game.context();
            let action = parse_local_action(game.local_position(), "Ln8hx8g-8h").unwrap();
            let mut undo = game
                .submit(GameRequest {
                    expected: before.clone(),
                    actor: Side::Sente,
                    command: GameCommand::Play {
                        action: action.clone(),
                    },
                })
                .unwrap();
            assert_eq!(
                format_history_action(&game.event_prefix(0).unwrap(), action.clone()).unwrap(),
                "Ln8hx8g-8h"
            );
            game.undo(&mut undo).unwrap();
            assert!(
                game.submit(GameRequest {
                    expected: before,
                    actor: Side::Sente,
                    command: GameCommand::Play { action }
                })
                .is_err()
            );
        }
    }
    #[test]
    fn malformed_action_shapes_reject_and_all_standard_choices_roundtrip_both_owners() {
        for profile in Profile::ALL {
            let mut local = LocalPosition::standard(profile);
            for _ in 0..2 {
                let actions = local.legal_actions();
                for action in &actions {
                    let label = format_action(local.position(), action.clone()).unwrap();
                    assert_eq!(parse_local_action(&local, &label), Ok(action.clone()));
                }
                local.apply(&actions[0]).unwrap();
            }
            let action = local.legal_actions()[0].clone();
            let mut malformed = action.clone();
            malformed.route.clear();
            assert_eq!(
                format_action(local.position(), malformed),
                Err(NotationError::Shape)
            );
            let mut malformed = action.clone();
            malformed.target = Some(sq(1, 1));
            assert_eq!(
                format_action(local.position(), malformed),
                Err(NotationError::Shape)
            );
            let mut malformed = Action::igui(action.from, sq(1, 1));
            malformed.route = vec![sq(1, 2)];
            assert_eq!(
                format_action(local.position(), malformed),
                Err(NotationError::Shape)
            );
            assert!(parse_local_action(&local, &"x".repeat(33)).is_err());
        }
    }
}
