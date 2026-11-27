//! Read-only projection. No replay, transition or inferred adjudication.
use super::*;
/// Tori selected or live history facts.
#[derive(Clone, Debug, Serialize)]
pub struct ToriHistoryPresentation {
    /// Exact game event count, including administrative commands.
    pub event_index: usize,
    /// Known board actions, independent of imported counters.
    pub board_ply: usize,
    /// Actual royal-safety query.
    pub in_check: bool,
    /// Root-indexed known occurrences at this prefix only.
    pub occurrences: Vec<usize>,
    /// Actions the side to move may not play: each would create a third
    /// occurrence of a position (T06, MSH-208). Not a conclusion by itself.
    pub repetition: Option<t::Repetition>,
    /// Concluded outcome only.
    pub conclusion: Option<t::Conclusion>,
}
impl ToriHistoryPresentation {
    fn new(game: &t::Game) -> Self {
        let snapshot = game.snapshot();
        Self {
            event_index: snapshot.event_count,
            board_ply: snapshot.board_ply,
            in_check: game
                .local_position()
                .is_in_check(game.position().side_to_move()),
            occurrences: snapshot.occurrences.clone(),
            repetition: snapshot.repetition.clone(),
            conclusion: snapshot.conclusion,
        }
    }
}
/// Tori provenance and exact selected/live semantic distinction.
#[derive(Clone, Debug, Serialize)]
pub struct ToriPresentation {
    /// Game root category; no historical reachability inferred from a diagram.
    pub root_kind: t::RootKind,
    /// Supplied source provenance when present.
    pub root_source: Option<String>,
    /// Declared root counter, not known board history.
    pub declared_root_plies: String,
    /// Unknown earlier history is retained explicitly.
    pub earlier_history_unknown: bool,
    /// Provenance guarantee only.
    pub historical_reachability_proven: bool,
    /// Facts at the exact selected prefix, without future knowledge.
    pub selected: ToriHistoryPresentation,
    /// Live facts, kept visibly distinct while inspecting history.
    pub live: ToriHistoryPresentation,
}
impl ToriPresentation {
    pub(super) fn new(selected: &t::Game, live: &t::Game) -> Self {
        let root = selected.position().root();
        Self {
            root_kind: root.kind(),
            root_source: root.source().map(str::to_owned),
            declared_root_plies: root.declared_completed_plies().to_string(),
            earlier_history_unknown: root.earlier_history_unknown(),
            historical_reachability_proven: root.historical_reachability_proven(),
            selected: ToriHistoryPresentation::new(selected),
            live: ToriHistoryPresentation::new(live),
        }
    }
}
pub(super) fn face_name(face: t::Face) -> &'static str {
    match face {
        t::Face::Phoenix => "Phoenix",
        t::Face::Falcon => "Falcon",
        t::Face::Eagle => "Eagle (promoted Falcon)",
        t::Face::Crane => "Crane",
        t::Face::Pheasant => "Pheasant",
        t::Face::LeftQuail => "Left Quail",
        t::Face::RightQuail => "Right Quail",
        t::Face::Swallow => "Swallow",
        t::Face::Goose => "Goose (promoted Swallow)",
    }
}
pub(super) fn status_label(game: &t::Game) -> String {
    let name = |s| {
        side(
            mshogi_core::game::RulesProfile::ToriModernV1,
            TaggedSide::Tori(s),
        )
        .label
    };
    if let Some(conclusion) = game.conclusion() {
        return match conclusion {
            t::Conclusion::Checkmate { winner } => format!("{} wins by checkmate", name(winner)),
            t::Conclusion::StalemateLoss { loser } => {
                format!("{} loses with no legal board action", name(loser))
            }
            t::Conclusion::RepetitionForfeit { loser } => {
                format!("{} loses by repetition forfeit", name(loser))
            }
            t::Conclusion::Resignation { winner } => {
                format!("{} wins by resignation", name(winner))
            }
        };
    }
    format!(
        "{} to move{}",
        name(game.position().side_to_move()),
        if game
            .local_position()
            .is_in_check(game.position().side_to_move())
        {
            " · Check"
        } else {
            ""
        }
    )
}
pub(super) fn event_label(event: &t::GameEvent) -> String {
    let actor = side(
        mshogi_core::game::RulesProfile::ToriModernV1,
        TaggedSide::Tori(event.actor),
    )
    .label;
    match event.command {
        t::GameCommand::Play { action } => format!(
            "{actor} · Technical action {}",
            serde_json::to_string(&action).expect("action")
        ),
        t::GameCommand::Resign => format!("{actor} resigned"),
        // Rejected game commands can never occur in an admitted history.
        t::GameCommand::AgreeDraw | t::GameCommand::DeclareImpasse => {
            unreachable!("admitted Tori history")
        }
    }
}
