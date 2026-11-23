//! Tori in the sole desktop runtime: the generic native session (`native_session`)
//! with the Tori presentation. Rules and persistence stay native-owned.
use super::native_session::*;
use super::*;
pub(crate) use crate::native_game::Tori;
use mshogi_core::tori as t;

/// The Tori session.
pub(crate) type ToriSession = NativeSession<Tori>;

impl SessionGame for Tori {
    type Presentation = ToriPresentation;
    const PLAY_LIMITS: WorkLimits = mshogi_engine::TORI_PLAY_LIMITS;
    const MAX_DEPTH: u16 = mshogi_engine::TORI_MAX_DEPTH;
    const MAX_NODES: u64 = mshogi_engine::TORI_MAX_NODES;
    const HANDS: bool = true;
    const BOARD_SIZE: u8 = 7;
    const BOARD_LABEL: &'static str = "Tori Shogi 7×7 board";
    const INITIAL_FOCUS: &'static str = "4e";
    const SCORE_MEANING: &'static str = "Native root-side evaluation in hundredths of a Swallow. Win/loss distances count board actions; failures and incomplete work are unscored.";

    session_wiring!(Tori, t, operation: Tori, preferences: ToriPreferences, state: tori, tori_mut, field: tori);

    fn presentation(selected: &t::Game, live: &t::Game) -> ToriPresentation {
        ToriPresentation::new(selected, live)
    }
    fn status(game: &t::Game, _has_actions: bool) -> PresentedStatus {
        PresentedStatus {
            concluded: game.conclusion().is_some(),
            label: status_label(game),
            score_half_points: game.conclusion().map(|c| {
                let winner = match c {
                    t::Conclusion::Checkmate { winner } | t::Conclusion::Resignation { winner } => {
                        winner
                    }
                    t::Conclusion::StalemateLoss { loser }
                    | t::Conclusion::RepetitionForfeit { loser } => loser.opponent(),
                };
                if winner == t::Side::Sente {
                    [2, 0]
                } else {
                    [0, 2]
                }
            }),
        }
    }
    fn choice(game: &t::Game, action: t::Action, _all: &[t::Action]) -> PresentedChoice {
        choice(action, game.position().side_to_move(), game.position())
    }
    fn legal_actions(
        game: &t::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<t::Action>, String> {
        if stop() {
            return Err("Native operation stopped before acceptance.".into());
        }
        Ok(game.legal_actions())
    }
    fn retire_context(game: &mut t::Game) {
        game.retire_context()
            .expect("bounded native runtime identity");
    }
    fn event(game: &t::Game, index: usize) -> (t::Side, CommandKind) {
        let event = &game.events()[index];
        (event.actor, Self::command_kind(&event.command))
    }
    fn command_kind(command: &t::GameCommand) -> CommandKind {
        match command {
            t::GameCommand::Play { .. } => CommandKind::Play,
            t::GameCommand::Resign => CommandKind::Terminal,
            _ => CommandKind::Other,
        }
    }
    fn telemetry(result: &mshogi_engine::ToriResult) -> TelemetryView {
        let t = result.telemetry();
        TelemetryView {
            score: t.score_label(),
            completed_depth: t.completed_depth,
            nodes: t.nodes,
            // Complete native encodings remain honest technical continuation
            // data; readable action notation is projected by the Moves adapter.
            variation: t
                .principal_variation
                .iter()
                .map(|a| serde_json::to_string(a).expect("native action"))
                .collect(),
            status: stop_label(t.status),
        }
    }
    fn incomplete(result: &mshogi_engine::ToriResult) -> bool {
        matches!(result, mshogi_engine::ToriResult::Incomplete { .. })
    }
    fn edition(_game: &t::Game) -> &'static str {
        t::PROFILE_EDITION
    }
    fn pieces(game: &t::Game) -> Vec<PresentedPiece> {
        let position = game.position();
        t::Square::all()
            .filter_map(|square| {
                position.piece(square).map(|p| PresentedPiece {
                    square: square.to_string(),
                    owner: tori_side(p.side).key,
                    kind: p.face.code().into(),
                    promoted: p.face.is_promoted(),
                    text: p.face.code().into(),
                    label: format!(
                        "{square}, {} {}",
                        tori_side(p.side).label,
                        face_name(p.face)
                    ),
                })
            })
            .collect()
    }
    fn hands(game: &t::Game) -> Vec<PresentedHand> {
        let position = game.position();
        [t::Side::Sente, t::Side::Gote]
            .into_iter()
            .flat_map(|actor| {
                t::Face::ALL
                    .into_iter()
                    .filter(|f| !f.is_promoted() && *f != t::Face::Phoenix)
                    .map(move |face| PresentedHand {
                        origin: hand_origin(actor, face),
                        owner: tori_side(actor).key,
                        kind: face.code().into(),
                        count: position.hand(actor).count(face),
                        text: face.code().into(),
                        label: format!(
                            "{} {} in hand, {}",
                            tori_side(actor).label,
                            face_name(face),
                            position.hand(actor).count(face)
                        ),
                    })
            })
            .collect()
    }
    fn presented_events(game: &t::Game) -> Vec<PresentedEvent> {
        game.events()
            .iter()
            .map(|e| PresentedEvent {
                index: e.index + 1,
                board_ply: e.board_ply,
                label: event_label(e),
            })
            .collect()
    }
    fn board_ply(game: &t::Game) -> u64 {
        game.snapshot().board_ply as u64
    }
}

fn tori_side(side: t::Side) -> PresentedSide {
    shared::side(
        mshogi_core::native::RulesProfile::ToriModernV1,
        NativeSide::Tori(side),
    )
}

impl DesktopController {
    /// Verification entry into the real native Tori request path.
    #[cfg(any(test, feature = "desktop-verification"))]
    pub fn verify_tori_execution(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<ExecutionContext, String> {
        self.verify_game_execution::<Tori>(limits, analysis)
    }
    /// Hold technical fixture play; excluded from ordinary distributables.
    #[cfg(feature = "desktop-verification")]
    pub fn verify_tori_pause_play(&mut self) -> Result<(), String> {
        self.verify_game_pause_play::<Tori>()
    }
    /// Project the actual submitted pending state without polling completion.
    #[cfg(feature = "desktop-verification")]
    pub fn verify_tori_pending_snapshot(&self) -> Result<SharedSnapshot, String> {
        self.verify_game_pending_snapshot::<Tori>()
    }
    /// Verification-only elapsed-clock injection; canonical state is unchanged.
    #[cfg(feature = "desktop-verification")]
    pub fn verify_tori_validation_deadline(
        &mut self,
        file: bool,
    ) -> Result<SharedSnapshot, String> {
        self.verify_game_validation_deadline::<Tori>(file)
    }
}

pub(super) fn choice(action: t::Action, actor: t::Side, position: &t::Position) -> PresentedChoice {
    let (origin, from, to, completion) = match action {
        t::Action::Move {
            from,
            to,
            result_face,
        } => (
            ChoiceOrigin::Board {
                square: from.to_string(),
            },
            from.to_string(),
            to,
            position
                .piece(from)
                .filter(|p| p.face != result_face)
                .map(|_| Completion {
                    label: format!("Compulsory promotion to {}", face_name(result_face)),
                    glyph: None,
                }),
        ),
        t::Action::Drop { face, to } => (
            ChoiceOrigin::Hand {
                owner: tori_side(actor).key,
                piece_kind: face.code().into(),
            },
            hand_origin(actor, face),
            to,
            None,
        ),
    };
    PresentedChoice {
        tenjiku: None,
        origin,
        action: serde_json::to_string(&action).expect("native action"),
        from,
        steps: vec![to.to_string()],
        completion,
    }
}
fn hand_origin(actor: t::Side, face: t::Face) -> String {
    format!("hand:{}:{}", tori_side(actor).key, face.code())
}
pub(super) fn inspect(game: &t::Game, origin: &str) -> Result<inspection::Inspected, String> {
    let (native, actor, face) = if let Ok(square) = origin.parse::<t::Square>() {
        let piece = game
            .position()
            .piece(square)
            .ok_or("No native piece at this origin.")?;
        (t::InspectionOrigin::Board(square), piece.side, piece.face)
    } else {
        let (actor, face) = [t::Side::Sente, t::Side::Gote]
            .into_iter()
            .find_map(|actor| {
                t::Face::ALL.into_iter().find_map(|face| {
                    (origin == hand_origin(actor, face)
                        && game.position().hand(actor).count(face) > 0)
                        .then_some((actor, face))
                })
            })
            .ok_or("Unknown or empty Tori hand origin.")?;
        (t::InspectionOrigin::Hand { side: actor, face }, actor, face)
    };
    let choices = game
        .local_position()
        .inspect(native)
        .into_iter()
        .map(|c| choice(c.action, actor, game.position()))
        .collect();
    Ok((
        GameKind::Tori,
        NativeSide::Tori(actor),
        format!(
            "{} {}{}",
            tori_side(actor).label,
            face_name(face),
            if matches!(native, t::InspectionOrigin::Hand { .. }) {
                " in hand"
            } else {
                ""
            }
        ),
        choices,
    ))
}

#[cfg(test)]
#[path = "shared_tori_tests.rs"]
mod tests;

#[path = "shared_tori_presentation.rs"]
mod presentation;
pub use presentation::ToriPresentation;
use presentation::{event_label, face_name, status_label};
