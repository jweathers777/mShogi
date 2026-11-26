//! Xiangqi in the sole desktop runtime: the generic native session (`native_session`)
//! with the Xiangqi presentation. Rules stay native-owned.
use super::native_session::*;
use super::*;
use crate::native_game::NativeGame;
pub(crate) use crate::native_game::Xiangqi;
use mshogi_core::xiangqi as x;
use mshogi_engine::{XiangqiChoice, XiangqiResult};
#[cfg(test)]
#[path = "shared_xiangqi_label_tests.rs"]
mod label_tests;
#[cfg(feature = "desktop-verification")]
#[path = "shared_xiangqi_verification.rs"]
mod verification;

/// The Xiangqi session.
pub(crate) type XiangqiSession = NativeSession<Xiangqi>;

/// Xiangqi intersection topology; coordinates are native a0–i9, never square cells.
#[derive(Clone, Debug, Serialize)]
pub struct IntersectionTopology {
    /// All ninety native intersections.
    pub points: Vec<String>,
    /// Native river separation in zero-based ranks.
    pub river_between_ranks: (u8, u8),
    /// Separate native Red and Black palace areas and diagonal geometry.
    pub palaces: Vec<Palace>,
}
/// Native palace ownership and lines, independent of shell layout.
#[derive(Clone, Debug, Serialize)]
pub struct Palace {
    /// Native owner with profile-qualified identity.
    pub side: PresentedSide,
    /// All nine palace intersections.
    pub points: Vec<String>,
    /// Two palace diagonal lines, including their center.
    pub diagonals: Vec<Vec<String>>,
}
/// Native selected-prefix facts, not inferred from heuristic evaluation.
#[derive(Clone, Debug, Serialize)]
pub struct XiangqiPresentation {
    /// Native human-actor command advice, disabled while viewing or paused.
    pub terminal_actions: XiangqiTerminalActions,
    /// Whether SharedSnapshot.context describes this committed selection or the live
    /// game alongside an immutable selection inside an atomic native command.
    pub canonical_context_scope: &'static str,
    /// Exact selected canonical prefix, absent inside an atomic command.
    pub selected_canonical: Option<CanonicalRecord>,
    /// Exact selected native event prefix; never includes later warnings/outcomes.
    pub selected_events: Vec<x::GameEvent>,
    /// Native selected position document; diagram only, not invented game history.
    pub selected_position: String,
    /// Original native root and counter provenance, unchanged by historical selection.
    pub root: String,
    /// Current native claim policy, distinct from available claims.
    pub claim_policy: x::ClaimPolicy,
    /// Claimant-specific known-window evidence, including ineligible queries.
    pub count_advice: Vec<x::NaturalCountEligibility>,
    /// Exact native selected-prefix warnings.
    pub warnings: Vec<x::Warning>,
    /// Native current offer, separate from a draw conclusion.
    pub offered_by: Option<x::Side>,
    /// Exact native committed conclusion, never an engine guess.
    pub conclusion: Option<x::Conclusion>,
    /// Required unresolved classification, unscored.
    pub pending: Option<x::PendingAdjudication>,
    /// Sound-fast-only provenance; Unknown never means positive win capability.
    pub no_win_assessment: x::NoWinAssessment,
}
/// Native terminal-command eligibility, independent of board-action turn ownership.
#[derive(Clone, Debug, Serialize)]
pub struct XiangqiTerminalActions {
    /// Native actor eligible to offer a draw.
    pub offer_actor: PresentedSide,
    /// Native actor for a pending draw response.
    pub response_actor: PresentedSide,
    /// Offer a draw after the human's own move under the native profile.
    pub offer_draw: bool,
    /// Accept the opponent's outstanding offer.
    pub accept_draw: bool,
    /// Decline the opponent's outstanding offer.
    pub decline_draw: bool,
}

fn error(e: impl ToString) -> String {
    archive::bounded(e)
}

impl SessionGame for Xiangqi {
    type Presentation = ();
    const BOARD_SIZE: u8 = 9;
    const BOARD_LABEL: &'static str = "Xiangqi intersection board";
    const INITIAL_FOCUS: &'static str = "e3";
    const SCORE_MEANING: &'static str = "Native Red/Black root-side hundredths of soldier. Terminal distances count command edges; heuristic evaluation and existential win witnesses never establish a forced win. Ordinary analysis uses sound_fast_only; Unknown is unscored evidence, not proof of win capability.";

    session_wiring!(Xiangqi, state: xiangqi, xiangqi_mut);

    fn play(action: &str) -> Result<x::GameCommand, String> {
        let action = action.parse().map_err(error)?;
        Ok(x::GameCommand::Play { action })
    }

    fn presentation(_selected: &x::Game, _live: &x::Game) {}
    fn status(game: &x::Game, _has_actions: bool) -> PresentedStatus {
        let concluded = game.conclusion();
        let position = game.position();
        PresentedStatus {
            concluded: concluded.is_some(),
            score_half_points: concluded.map(|c| match c {
                x::Conclusion::Win {
                    winner: x::Side::Red,
                    ..
                } => [2, 0],
                x::Conclusion::Win { .. } => [0, 2],
                x::Conclusion::Draw { .. } => [1, 1],
            }),
            label: if let Some(c) = concluded {
                conclusion_label(&c)
            } else if game.pending_adjudication().is_some() {
                "Adjudication unresolved · unscored".into()
            } else {
                format!(
                    "{} to move{}",
                    xiangqi_side(position.side_to_move()).label,
                    if position.is_in_check(position.side_to_move()) {
                        " · check"
                    } else {
                        ""
                    }
                )
            },
        }
    }
    fn choice(_game: &x::Game, action: x::Action, _all: &[x::Action]) -> PresentedChoice {
        choice(action)
    }
    fn legal_actions(
        game: &x::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<x::Action>, String> {
        if stop() {
            return Err("Native operation stopped before acceptance.".into());
        }
        if game.conclusion().is_some() || game.pending_adjudication().is_some() {
            return Ok(vec![]);
        }
        game.position().legal_actions().map_err(error)
    }
    fn retire_context(_game: &mut x::Game) {
        // Execution identities are retired by the session version.
    }
    fn event(game: &x::Game, index: usize) -> (x::Side, CommandKind) {
        let event = &game.events()[index];
        let actor = event.actor.unwrap_or(game.position().side_to_move());
        let kind = match event.event {
            x::EventKind::BoardAction { .. } => CommandKind::Play,
            x::EventKind::Resigned => CommandKind::Terminal,
            _ => CommandKind::Other,
        };
        (actor, kind)
    }
    fn command_kind(command: &x::GameCommand) -> CommandKind {
        match command {
            x::GameCommand::Play { .. } => CommandKind::Play,
            x::GameCommand::Resign => CommandKind::Terminal,
            _ => CommandKind::Other,
        }
    }
    fn take_back_target(game: &x::Game, human: x::Side, two_humans: bool) -> Option<usize> {
        if two_humans {
            Self::last_command_start(game)
        } else {
            game.preceding_playable_decision(human)
        }
    }
    fn command_actor(
        game: &x::Game,
        settings: &crate::game_document::SettingsOf<Xiangqi>,
        human: x::Side,
        command: &x::GameCommand,
        local: x::Side,
    ) -> x::Side {
        match command {
            x::GameCommand::OfferDraw => draw_offer_actor(game, settings, human),
            x::GameCommand::AcceptDraw | x::GameCommand::DeclineDraw => {
                draw_response_actor(game, settings, human, local)
            }
            _ => local,
        }
    }
    fn telemetry(result: &XiangqiResult) -> TelemetryView {
        engine_telemetry(result, |c| match c {
            XiangqiChoice::Move { action } => action.to_string(),
            XiangqiChoice::Claim { actor } => format!("{actor:?} count claim"),
        })
    }
    fn incomplete(result: &XiangqiResult) -> bool {
        engine_incomplete(result)
    }
    fn edition(_game: &x::Game) -> &'static str {
        RulesProfile::XiangqiWxf2018V1.edition()
    }
    fn pieces(game: &x::Game) -> Vec<PresentedPiece> {
        let position = game.position();
        x::Intersection::all()
            .filter_map(|point| {
                position.piece(point).map(|p| {
                    let owner = xiangqi_side(p.side);
                    PresentedPiece {
                        square: point.to_string(),
                        owner: owner.key,
                        kind: format!("{:?}", p.kind).to_lowercase(),
                        promoted: false,
                        text: format!("{:?}", p.kind),
                        label: format!("{point}, {} {:?}", owner.label, p.kind),
                    }
                })
            })
            .collect()
    }
    fn presented_events(game: &x::Game) -> Vec<PresentedEvent> {
        // One entry per command: history stops only at command boundaries.
        let events = game.events();
        let ranges: Vec<_> = game.command_event_ranges().collect();
        // Events of the start position itself (a root conclusion) come first.
        let first = ranges.first().map_or(events.len(), |r| r.0);
        let root = (first > 0).then_some((0, first));
        root.into_iter()
            .chain(ranges)
            .map(|(start, end)| {
                let shown = events[start..end]
                    .iter()
                    .find(|e| matches!(e.event, x::EventKind::BoardAction { .. }))
                    .unwrap_or(&events[start]);
                let consequences: Vec<String> = events[start..end]
                    .iter()
                    .filter(|e| !std::ptr::eq(*e, shown))
                    .map(event_label)
                    .collect();
                PresentedEvent {
                    index: end,
                    board_ply: events[end - 1].board_ply,
                    label: if consequences.is_empty() {
                        event_label(shown)
                    } else {
                        format!("{} · {}", event_label(shown), consequences.join(" · "))
                    },
                }
            })
            .collect()
    }
    fn board_ply(game: &x::Game) -> u64 {
        game.events().last().map_or(0, |e| e.board_ply as u64)
    }
    fn finish_snapshot(snapshot: &mut SharedSnapshot, state: &XiangqiSession) {
        let game = state.selected();
        let historical = state.historical();
        let settings = state.document.settings();
        let human = state.document.human();
        let live = &state.game;
        let active =
            !historical && live.conclusion().is_none() && live.pending_adjudication().is_none();
        let history = game
            .event_prefix(game.events().len())
            .expect("native current prefix");
        let count_advice: Vec<_> = [x::Side::Red, x::Side::Black]
            .into_iter()
            .map(|s| game.natural_count_eligibility(s))
            .collect();
        let local = settings
            .play_mode
            .actor(human, live.position().side_to_move());
        let offer_actor = draw_offer_actor(live, settings, human);
        let response_actor = draw_response_actor(live, settings, human, local);
        snapshot.xiangqi = Some(XiangqiPresentation {
            terminal_actions: XiangqiTerminalActions {
                offer_actor: xiangqi_side(offer_actor),
                response_actor: xiangqi_side(response_actor),
                offer_draw: active && live.can_offer_draw(offer_actor),
                accept_draw: active && live.can_respond_to_draw(response_actor),
                decline_draw: active && live.can_respond_to_draw(response_actor),
            },
            canonical_context_scope: "selected_committed",
            selected_canonical: Some(snapshot.context.clone()),
            selected_events: game.events().to_vec(),
            selected_position: x::PositionRoot::analysis(game.position().clone()).to_json(),
            root: live.root().to_json(),
            claim_policy: live.claim_policy(),
            count_advice: count_advice.clone(),
            warnings: history.warnings.clone(),
            offered_by: history.offered_by,
            conclusion: history.conclusion,
            pending: history.pending.clone(),
            no_win_assessment: history.no_win_assessment.clone(),
        });
        let files: Vec<_> = (b'a'..=b'i').map(|f| char::from(f).to_string()).collect();
        let ranks: Vec<_> = (0..10).rev().map(|r| r.to_string()).collect();
        snapshot.board.files = files.clone();
        snapshot.board.ranks = ranks.clone();
        snapshot.board.intersections = Some(topology());
        snapshot.orientations[0].files = files.clone();
        snapshot.orientations[0].ranks = ranks.clone();
        snapshot.orientations[1].files = files.into_iter().rev().collect();
        snapshot.orientations[1].ranks = ranks.into_iter().rev().collect();
        let caps = &mut snapshot.capabilities;
        caps.promotion = false;
        caps.board_action = caps.board_action && active;
        caps.resign = caps.resign && active;
        caps.claims = active
            && count_advice
                .iter()
                .any(|e| e.claimant == local && e.eligible);
    }
}

fn xiangqi_side(side: x::Side) -> PresentedSide {
    shared::side(RulesProfile::XiangqiWxf2018V1, NativeSide::Xiangqi(side))
}
fn draw_offer_actor(
    game: &x::Game,
    settings: &crate::game_document::SettingsOf<Xiangqi>,
    human: x::Side,
) -> x::Side {
    if settings.play_mode.is_two_humans() {
        game.position().side_to_move().opponent()
    } else {
        human
    }
}
fn draw_response_actor(
    game: &x::Game,
    settings: &crate::game_document::SettingsOf<Xiangqi>,
    human: x::Side,
    local: x::Side,
) -> x::Side {
    if settings.play_mode.is_two_humans() {
        [x::Side::Red, x::Side::Black]
            .into_iter()
            .find(|actor| game.can_respond_to_draw(*actor))
            .unwrap_or(local)
    } else {
        human
    }
}
fn topology() -> IntersectionTopology {
    IntersectionTopology {
        points: x::Intersection::all().map(|p| p.to_string()).collect(),
        river_between_ranks: x::RIVER_BETWEEN_RANKS,
        palaces: [x::Side::Red, x::Side::Black]
            .into_iter()
            .map(|actor| {
                let base = if actor == x::Side::Red { 0 } else { 7 };
                Palace {
                    side: xiangqi_side(actor),
                    points: x::Intersection::all()
                        .filter(|p| actor.in_palace(*p))
                        .map(|p| p.to_string())
                        .collect(),
                    diagonals: vec![
                        vec![
                            format!("d{base}"),
                            format!("e{}", base + 1),
                            format!("f{}", base + 2),
                        ],
                        vec![
                            format!("f{base}"),
                            format!("e{}", base + 1),
                            format!("d{}", base + 2),
                        ],
                    ],
                }
            })
            .collect(),
    }
}

impl DesktopController {
    /// Verification entry into the real native Xiangqi request path.
    #[cfg(any(test, feature = "desktop-verification"))]
    pub fn verify_xiangqi_execution(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<ExecutionContext, String> {
        self.verify_game_execution::<Xiangqi>(limits, analysis)
    }
}

fn choice(a: x::Action) -> PresentedChoice {
    PresentedChoice {
        tenjiku: None,
        origin: ChoiceOrigin::Intersection {
            point: a.from.to_string(),
        },
        action: a.to_string(),
        from: a.from.to_string(),
        steps: vec![a.to.to_string()],
        completion: None,
    }
}
// Presentation only. Native events, actor identity, indices and replay payloads
// remain authoritative; no reconstruction or adjudication runs to make a label.
fn event_label(event: &x::GameEvent) -> String {
    let name = |s| {
        side(
            mshogi_core::native::RulesProfile::XiangqiWxf2018V1,
            NativeSide::Xiangqi(s),
        )
        .label
    };
    let actor = event.actor.map(name).unwrap_or_else(|| "Game".into());
    let fault = |f| match f {
        x::FaultKind::PerpetualCheck => "perpetual check",
        x::FaultKind::PerpetualChase => "perpetual chase",
    };
    match &event.event {
        x::EventKind::BoardAction {
            action,
            captured_id,
            gave_check,
            ..
        } => format!(
            "{actor}: {}{}{}{}",
            action.from,
            if captured_id.is_some() { "×" } else { "–" },
            action.to,
            if *gave_check { " · check" } else { "" }
        ),
        x::EventKind::Warning { warning } => format!(
            "{}: repetition warning · {}",
            name(warning.side),
            fault(warning.fault)
        ),
        x::EventKind::WarningRetired {
            warning_event_index,
        } => format!(
            "Repetition warning cleared · event {}",
            warning_event_index + 1
        ),
        x::EventKind::ClaimSubmitted {
            evidence,
            automatic,
        } => format!(
            "{}: natural-count claim submitted{}",
            name(evidence.claimant),
            if *automatic { " (automatic)" } else { "" }
        ),
        x::EventKind::ClaimRejected { evidence } => {
            format!("{}: natural-count claim rejected", name(evidence.claimant))
        }
        x::EventKind::ClaimAccepted { evidence } => {
            format!("{}: natural-count claim accepted", name(evidence.claimant))
        }
        x::EventKind::DrawOffer {
            offered_by: Some(s),
        } => format!("{}: draw offered", name(*s)),
        x::EventKind::DrawOffer { offered_by: None } => "Draw offer withdrawn".into(),
        x::EventKind::Resigned => format!("{actor}: resigned"),
        x::EventKind::AgreementAccepted => format!("{actor}: draw agreement accepted"),
        x::EventKind::AdjudicationPending { .. } => "Adjudication unresolved · unscored".into(),
        x::EventKind::Concluded { conclusion } => conclusion_label(conclusion),
    }
}
fn conclusion_label(conclusion: &x::Conclusion) -> String {
    let name = |s| {
        side(
            mshogi_core::native::RulesProfile::XiangqiWxf2018V1,
            NativeSide::Xiangqi(s),
        )
        .label
    };
    let fault = |f| match f {
        x::FaultKind::PerpetualCheck => "perpetual check",
        x::FaultKind::PerpetualChase => "perpetual chase",
    };
    match conclusion {
        x::Conclusion::Win { winner, reason } => format!(
            "{} wins · {}",
            name(*winner),
            match reason {
                x::WinReason::NoLegalAction {
                    reason: x::NoActionReason::Checkmate,
                } => "checkmate",
                x::WinReason::NoLegalAction {
                    reason: x::NoActionReason::Stalemate,
                } => "no legal move (stalemate)",
                x::WinReason::Repetition { fault: f } => fault(*f),
                x::WinReason::Resignation => "resignation",
            }
        ),
        x::Conclusion::Draw { reason } => format!(
            "Draw · {}",
            match reason {
                x::DrawReason::Repetition => "repetition",
                x::DrawReason::NaturalCount => "natural-count claim",
                x::DrawReason::Agreement => "agreement",
                x::DrawReason::NeitherCanWin => "neither side can win",
            }
        ),
    }
}

pub(super) fn inspect(
    position: &x::Position,
    origin: &str,
) -> Result<inspection::Inspected, String> {
    let point = origin
        .parse::<x::Intersection>()
        .map_err(|_| "Unknown inspection intersection.")?;
    let piece = position
        .piece(point)
        .ok_or("No piece at this intersection.")?;
    let owner = NativeSide::Xiangqi(piece.side);
    Ok((
        GameKind::Xiangqi,
        owner,
        format!(
            "{} {}",
            side(RulesProfile::XiangqiWxf2018V1, owner).label,
            format!("{:?}", piece.kind).to_lowercase()
        ),
        position
            .inspection_actions(piece.side)
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter(|a| a.from == point)
            .map(choice)
            .collect(),
    ))
}

#[cfg(test)]
#[path = "shared_xiangqi_tests.rs"]
mod tests;
