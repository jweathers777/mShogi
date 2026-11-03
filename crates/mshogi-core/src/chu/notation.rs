//! Human mShogi Chu action notation, not an authoritative record or new rules.
//!
//! Source original identity (a trailing `+` means already promoted), source
//! square, then each actual landing preceded by `-` (quiet) or `x` (capture).
//! A final `^` means this complete action promotes. Absence of `^` means no
//! flip; it does not assert that promotion was available. Thus jump `Ln6fx6d`,
//! walk `Ln6fx6ex6d`, capture-return `Ln6fx6e-6f` and quiet-return
//! `Ln6f-5e-6f` remain distinct. Coordinates are file1..12/rank a..l.
use super::{Action, HistorySnapshot, LocalPosition, Position};
use std::{error::Error, fmt};

/// Invalid human spelling or an action that cannot be described at this root.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotationError {
    /// No owned source exists.
    Source,
    /// Complete native actions have one or two actual landings.
    Route,
    /// No native local choice has this exact spelling.
    NoLocalChoice,
}
impl fmt::Display for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chu notation refused: {self:?}")
    }
}
impl Error for NotationError {}

/// Describe an admitted event at its exact preceding native prefix.
pub fn format_history_action(
    before: &HistorySnapshot,
    action: Action,
) -> Result<String, NotationError> {
    format_action(before.local_position().position(), action)
}

/// Describe a complete action without granting admission or reconstructing history.
pub fn format_action(position: &Position, action: Action) -> Result<String, NotationError> {
    let source = position
        .piece(action.from)
        .filter(|p| p.side == position.side_to_move())
        .ok_or(NotationError::Source)?;
    if !(1..=2).contains(&action.route.len()) {
        return Err(NotationError::Route);
    }
    let mut result = format!("{}{from}", source.code(), from = action.from);
    let mut removed = vec![action.from];
    for landing in action.route {
        let capture = !removed.contains(&landing) && position.piece(landing).is_some();
        result.push(if capture { 'x' } else { '-' });
        result.push_str(&landing.to_string());
        removed.push(landing);
    }
    if action.promote {
        result.push('^');
    }
    Ok(result)
}

/// Select a real local choice. Native Game must still enforce exact live history.
/// This structural-root convenience has no imported Lion protection rights.
pub fn parse_action(position: &Position, text: &str) -> Result<Action, NotationError> {
    parse_local_action(&LocalPosition::admit(position.clone()), text)
}

/// Select a real local choice with the caller's native Lion protection rights.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chu::{Piece, RootMetadata, Side, Square};
    fn local(entries: &[(&str, Side, &str)]) -> LocalPosition {
        LocalPosition::admit(
            Position::from_diagram(
                &entries
                    .iter()
                    .map(|(s, side, p)| (s.parse().unwrap(), Piece::from_code(*side, p).unwrap()))
                    .collect::<Vec<_>>(),
                Side::Sente,
                RootMetadata::diagram(0),
            )
            .unwrap(),
        )
    }
    #[test]
    fn independent_lion_jump_ordered_captures_and_returns_are_distinct() {
        let local = local(&[
            ("6f", Side::Sente, "Ln"),
            ("6e", Side::Gote, "Si"),
            ("6d", Side::Gote, "R"),
        ]);
        for (label, landings) in [
            ("Ln6fx6d", vec!["6d"]),
            ("Ln6fx6ex6d", vec!["6e", "6d"]),
            ("Ln6fx6e-6f", vec!["6e", "6f"]),
            ("Ln6f-5e-6f", vec!["5e", "6f"]),
        ] {
            let expected = Action {
                from: "6f".parse().unwrap(),
                route: landings.iter().map(|s| s.parse().unwrap()).collect(),
                promote: false,
            };
            assert_eq!(parse_local_action(&local, label), Ok(expected.clone()));
            assert_eq!(format_action(local.position(), expected).unwrap(), label);
        }
        for bad in [
            "Ln6f-6d",
            "Ln6f-6e-6f",
            "Ln6fx6ex6f",
            "ln6fx6d",
            "Ln6fx6d ",
            "Ln6f",
            "Ln6fx13d",
        ] {
            assert!(parse_local_action(&local, bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn source_original_identity_and_optional_flip_remain_explicit() {
        let local = local(&[
            ("6e", Side::Sente, "P"),
            ("2f", Side::Sente, "DE+"),
            ("9f", Side::Sente, "Ki+"),
        ]);
        for label in ["P6e-6d", "P6e-6d^", "DE+2f-2e", "Ki+9f-8e-9f"] {
            let action = parse_local_action(&local, label).unwrap();
            assert_eq!(format_action(local.position(), action).unwrap(), label);
        }
        assert!(parse_local_action(&local, "Pr2f-2e").is_err());
        assert!(parse_local_action(&local, "Ln9f-8e-9f").is_err());
        for square in Square::all() {
            assert_eq!(square.to_string().parse(), Ok(square));
        }
    }
    #[test]
    fn native_choice_roundtrip_both_sides_and_malformed_route_bounds() {
        let mut local = LocalPosition::standard();
        for _ in 0..4 {
            let choices = local.legal_actions();
            for action in &choices {
                let label = format_action(local.position(), action.clone()).unwrap();
                assert_eq!(parse_local_action(&local, &label), Ok(action.clone()));
            }
            local.apply(&choices[0]).unwrap();
        }
        assert_eq!(
            format_action(
                local.position(),
                Action {
                    from: local.legal_actions()[0].from,
                    route: vec![],
                    promote: false
                }
            ),
            Err(NotationError::Route)
        );
    }
}
