//! Technical complete-action display only. Final Tori notation belongs MSH-42.
use super::*;
use mshogi_core::tori as t;
pub(super) fn project(game: &t::Game) -> MovesProjection {
    let root = game.position().root();
    let mut view = projection(
        "Starting position".into(),
        format!(
            "Technical native actions · not final Tori notation. Declared root counter {} · {}",
            root.declared_completed_plies(),
            if root.earlier_history_unknown() {
                "Earlier history is unknown"
            } else {
                "Known standard starting history"
            }
        ),
    );
    // Only board actions advance a pair, irrespective of intervening Stop events.
    let mut number = 1;
    for event in game.events() {
        let t::GameCommand::Play { action } = event.command else {
            continue;
        };
        let (origin, to, label) = match action {
            t::Action::Move {
                from,
                to,
                result_face,
            } => (
                ChoiceOrigin::Board {
                    square: from.to_string(),
                },
                to,
                format!("Technical: {from} → {to} ({})", result_face.code()),
            ),
            t::Action::Drop { face, to } => (
                ChoiceOrigin::Hand {
                    owner: shared::side(NativeSide::Tori(event.actor)).key,
                    piece_kind: face.code().into(),
                },
                to,
                format!("Technical: {} drop → {to}", face.code()),
            ),
        };
        let column = u8::from(event.actor == t::Side::Gote);
        push(
            &mut view,
            number,
            column,
            label,
            CompletedActionGeometry {
                action: serde_json::to_string(&action).expect("native Tori action"),
                event_index: event.index + 1,
                origin,
                steps: vec![to.to_string()],
                effects: vec![],
            },
            event.index,
            event.index + 1,
        );
        if column == 1 {
            number += 1;
        }
    }
    view
}
