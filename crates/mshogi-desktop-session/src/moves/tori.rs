//! Native complete-action Tori notation, subordinate to admitted event history.
use super::*;
use mshogi_core::tori as t;
pub(super) fn project(game: &t::Game) -> MovesProjection {
    let root = game.position().root();
    let mut view = projection(
        "Starting position".into(),
        format!(
            "Tori action notation · Declared root counter {} · {}",
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
        let label = t::notation::format_history_action(
            &game.event_prefix(event.index).expect("admitted prefix"),
            action,
        )
        .expect("admitted complete action");
        let (origin, to) = match action {
            t::Action::Move { from, to, .. } => (
                ChoiceOrigin::Board {
                    square: from.to_string(),
                },
                to,
            ),
            t::Action::Drop { face, to } => (
                ChoiceOrigin::Hand {
                    owner: shared::side(NativeSide::Tori(event.actor)).key,
                    piece_kind: face.code().into(),
                },
                to,
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
                captures: None,
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
