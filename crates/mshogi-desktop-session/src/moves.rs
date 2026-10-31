//! Presentation only: local native positions/actions, never game adjudication,
//! exact reachability, notation input, canonical records or frontend replay.
use crate::*;
use mshogi_core::{shogi as s, xiangqi as x};
mod chess;
mod shogi;
mod xiangqi;

/// Additional movement derived from native before/after effects (e.g. rook).
#[derive(Clone, Debug, Serialize)]
pub struct MoveEffect {
    /// Native source coordinate.
    pub from: String,
    /// Native destination coordinate.
    pub to: String,
}
/// Completed history, independent of actionable selection/inspection/focus.
#[derive(Clone, Debug, Serialize)]
pub struct CompletedActionGeometry {
    /// Opaque complete native action identity.
    pub action: String,
    /// Exact post-action event-prefix count, not board ply.
    pub event_index: usize,
    /// Native board, intersection or hand origin.
    pub origin: ChoiceOrigin,
    /// Ordered complete route, including repeated destinations.
    pub steps: Vec<String>,
    /// Native secondary movement; empty for ordinary actions.
    pub effects: Vec<MoveEffect>,
}
/// One actual native board action, never a warning or terminal command.
#[derive(Clone, Debug, Serialize)]
pub struct PresentedMove {
    /// Exact prefix immediately after the action, before later derived events.
    pub event_index: usize,
    /// Known board-action count, independent of event count.
    pub board_ply: usize,
    /// Native display pair number; root provenance explains its scope.
    pub number: u64,
    /// Native first/second mover column, zero or one.
    pub column: u8,
    /// Source-based display notation; not a canonical action parser.
    pub notation: String,
    /// Exact native complete-action identity.
    pub action: String,
    /// Prefix before this command: associated prefix IDs are (start,end].
    pub command_start: usize,
    /// Committed end of this command, distinct from the action prefix.
    pub command_end: usize,
    #[serde(skip)]
    geometry: Option<CompletedActionGeometry>,
}
/// Counters describe the imported root; they never fabricate omitted history.
#[derive(Clone, Debug, Serialize)]
pub struct MovesRoot {
    /// Accessible starting-position target label.
    pub label: String,
    /// Plain-language known-history/counter provenance.
    pub numbering: String,
}
/// Cached native board-action projection and selected-prefix decoration.
#[derive(Clone, Debug, Serialize)]
pub struct MovesProjection {
    /// Explicit known root and numbering provenance.
    pub root: MovesRoot,
    /// Only actual completed native board actions.
    pub entries: Vec<PresentedMove>,
    /// Latest completed action at the exact selected prefix, or none.
    pub last_action: Option<CompletedActionGeometry>,
}
#[derive(Default)]
pub(super) struct Cache {
    key: Option<(u64, u64)>,
    projection: Option<MovesProjection>,
}
impl Cache {
    pub(super) fn get(
        &mut self,
        key: (u64, u64),
        selected: usize,
        build: impl FnOnce() -> MovesProjection,
    ) -> MovesProjection {
        if self.key != Some(key) {
            self.projection = Some(build());
            self.key = Some(key);
        }
        let mut view = self.projection.as_ref().expect("native projection").clone();
        view.last_action = view
            .entries
            .iter()
            .rev()
            .find(|m| m.event_index <= selected)
            .and_then(|m| m.geometry.clone());
        view
    }
}
fn projection(label: String, numbering: String) -> MovesProjection {
    MovesProjection {
        root: MovesRoot { label, numbering },
        entries: vec![],
        last_action: None,
    }
}
fn push(
    view: &mut MovesProjection,
    number: u64,
    column: u8,
    notation: String,
    geometry: CompletedActionGeometry,
    start: usize,
    end: usize,
) {
    view.entries.push(PresentedMove {
        event_index: geometry.event_index,
        board_ply: view.entries.len() + 1,
        number,
        column,
        notation,
        action: geometry.action.clone(),
        command_start: start,
        command_end: end,
        geometry: Some(geometry),
    });
}
pub(super) fn chess_projection(state: &DesktopSession) -> MovesProjection {
    let record = &state.initial_record;
    let mut position = record.initial_position().clone();
    let mut view = projection(
        "Starting position".into(),
        if record.history_origin() == HistoryOrigin::StandardStart {
            "Moves from the starting position".into()
        } else {
            format!(
                "Move numbering starts at {} · Earlier history is unknown",
                position.fullmove_number()
            )
        },
    );
    let mut add = |action: Action, index: usize| {
        let notation = chess::notation(&position, action);
        let number = u64::from(position.fullmove_number());
        let column = u8::from(position.side_to_move() == Color::Black);
        let before = position.clone();
        position.apply(action).expect("admitted Chess action");
        let mut effects = vec![];
        // Native transition effects identify any secondary moving piece; no
        // frontend castling/file/rank assumption is needed.
        for from in (0..64).map(|n| Square::new(n % 8, n / 8).unwrap()) {
            if from == action.from {
                continue;
            }
            if let Some(piece) = before.piece_at(from)
                && piece.color == before.side_to_move()
                && position.piece_at(from) != Some(piece)
            {
                for to in (0..64).map(|n| Square::new(n % 8, n / 8).unwrap()) {
                    if to != action.to
                        && position.piece_at(to) == Some(piece)
                        && before.piece_at(to) != Some(piece)
                    {
                        effects.push(MoveEffect {
                            from: from.to_string(),
                            to: to.to_string(),
                        });
                    }
                }
            }
        }
        let geometry = CompletedActionGeometry {
            action: action.to_string(),
            event_index: index,
            origin: ChoiceOrigin::Board {
                square: action.from.to_string(),
            },
            steps: vec![action.to.to_string()],
            effects,
        };
        push(
            &mut view,
            number,
            column,
            notation,
            geometry,
            index - 1,
            index,
        );
    };
    for (i, event) in record.events().iter().enumerate() {
        if let RecordEventKind::Move(action) = event.kind {
            add(action, i + 1);
        }
    }
    for (i, (request, reply)) in state.game.record().events().enumerate() {
        if reply.rejection.is_none()
            && let MatchCommand::Move(action) = request.command
        {
            add(action, record.events().len() + i + 1);
        }
    }
    view
}
pub(super) fn shogi_projection(game: &s::Game) -> MovesProjection {
    let mut position = game.root().position().clone();
    let mut view = projection(
        "Starting position".into(),
        if game.root().has_complete_initial_history() {
            "Move pairs from the starting position".into()
        } else {
            format!(
                "Known move pairs · SFEN declares move {}; earlier history is unknown",
                position.next_ply()
            )
        },
    );
    let mut number = 1u64;
    let mut previous = None;
    for event in game.events() {
        if let s::GameCommand::Play(action) = event.command {
            let notation = shogi::notation(&position, action, previous);
            let column = u8::from(event.actor == s::Side::Gote);
            let (origin, to) = match action {
                s::Action::Move { from, to, .. } => (
                    ChoiceOrigin::Board {
                        square: from.to_string(),
                    },
                    to,
                ),
                s::Action::Drop { kind, to } => (
                    ChoiceOrigin::Hand {
                        owner: shared::side(NativeSide::Shogi(event.actor)).key,
                        piece_kind: format!("{kind:?}").to_lowercase(),
                    },
                    to,
                ),
            };
            let geometry = CompletedActionGeometry {
                action: action.to_string(),
                event_index: event.index + 1,
                origin,
                steps: vec![to.to_string()],
                effects: vec![],
            };
            push(
                &mut view,
                number,
                column,
                notation,
                geometry,
                event.index,
                event.index + 1,
            );
            if column == 1 {
                number += 1;
            }
            position.apply(action).expect("admitted Shogi action");
            previous = Some(to);
        }
    }
    view
}
pub(super) fn xiangqi_projection(game: &x::Game) -> MovesProjection {
    let mut view = projection(
        "Starting position".into(),
        format!(
            "{}; only known actions are shown",
            match game.root().position().counters().fullmove_number() {
                Some(n) => format!("FEN fullmove {n}"),
                None => "Known-root move pairs; earlier counter unknown".into(),
            }
        ),
    );
    let mut number = u64::from(
        game.root()
            .position()
            .counters()
            .fullmove_number()
            .unwrap_or(1),
    );
    let ranges: Vec<_> = game.command_event_ranges().collect();
    for event in game.events() {
        if let x::EventKind::BoardAction { action, .. } = event.event {
            let pre = game
                .event_prefix(event.index)
                .expect("admitted native event prefix");
            let column = u8::from(event.actor == Some(x::Side::Black));
            let notation = xiangqi::notation(&pre.position, action);
            let geometry = CompletedActionGeometry {
                action: action.to_string(),
                event_index: event.index + 1,
                origin: ChoiceOrigin::Intersection {
                    point: action.from.to_string(),
                },
                steps: vec![action.to.to_string()],
                effects: vec![],
            };
            let (start, end) = ranges
                .iter()
                .copied()
                .find(|(start, end)| *start <= event.index && event.index < *end)
                .expect("board action owns a command range");
            push(&mut view, number, column, notation, geometry, start, end);
            if column == 1 {
                number += 1;
            }
        }
    }
    view
}

#[cfg(test)]
mod tests;
