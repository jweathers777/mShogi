//! Xiangqi in the sole desktop runtime: the generic native session (`native_session`)
//! with the Xiangqi presentation and exact-endgame evidence. Rules stay native-owned.
use super::native_session::*;
use super::*;
pub(crate) use crate::native_game::Xiangqi;
use crate::native_game::{NativeGame, xiangqi_work};
use mshogi_core::xiangqi as x;
use mshogi_engine::{XiangqiChoice, XiangqiResult, XiangqiStop};
#[cfg(test)]
#[path = "shared_xiangqi_label_tests.rs"]
mod label_tests;
#[cfg(feature = "desktop-verification")]
#[path = "shared_xiangqi_verification.rs"]
mod verification;

// Matches MSH-97's measured interactive default; requests also undergo native validation.
const PLAY_LIMITS: WorkLimits = mshogi_engine::XIANGQI_PLAY_LIMITS;

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
    /// Completed native exact reports retained only as historical evidence.
    pub exact_observations: Vec<XiangqiExactEvidence>,
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

/// Persisted native preferences; execution ownership is never a setting.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XiangqiSettings {
    /// Persisted ownership of the two native participants.
    #[serde(default)]
    pub play_mode: crate::PlayMode,
    /// Native side at the bottom, independent of human ownership.
    pub orientation: x::Side,
    /// Ordinary sound-fast-only analysis defaults.
    pub analysis_defaults: WorkLimits,
}
impl Default for XiangqiSettings {
    fn default() -> Self {
        Self {
            play_mode: crate::PlayMode::default(),
            orientation: x::Side::Red,
            analysis_defaults: PLAY_LIMITS,
        }
    }
}
fn valid_limits(limits: WorkLimits) -> Result<(), String> {
    if !(1..=mshogi_engine::XIANGQI_MAX_DEPTH).contains(&limits.depth)
        || !(1..=mshogi_engine::XIANGQI_MAX_NODES).contains(&limits.nodes)
        || limits.time_ms.is_some()
    {
        return Err(
            "Xiangqi analysis requires depth 1–64 and 1–5,000,000 nodes; no downgrade was applied."
                .into(),
        );
    }
    Ok(())
}

impl crate::native_game::DocumentSettings for XiangqiSettings {
    type Side = x::Side;
    fn new(orientation: x::Side, play_mode: crate::PlayMode) -> Self {
        Self {
            play_mode,
            orientation,
            ..Self::default()
        }
    }
    fn play_mode(&self) -> crate::PlayMode {
        self.play_mode
    }
    fn set_play_mode(&mut self, play_mode: crate::PlayMode) {
        self.play_mode = play_mode;
    }
    fn validate(&self) -> Result<(), String> {
        valid_limits(self.analysis_defaults)
    }
    fn with_analysis_defaults(&self, limits: WorkLimits) -> Option<Self> {
        Some(Self {
            analysis_defaults: limits,
            ..self.clone()
        })
    }
}
fn error(e: impl ToString) -> String {
    archive::bounded(e)
}

/// Immutable completed native exact work, never a resumable request or an outcome.
/// On Open this is historical reported evidence, not a newly verified graph proof.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XiangqiExactEvidence {
    format: String,
    executor: String,
    source: CanonicalRecord,
    purpose: String,
    goal: String,
    event_count: usize,
    historical_target: bool,
    position_json: String,
    native_session: u64,
    native_generation: u64,
    limits: x::ExactLimits,
    report: x::ExactReport,
}
impl XiangqiExactEvidence {
    /// Construct only from completed native work. This does not execute or accept work.
    pub fn from_native(observation: &x::ExactObservation) -> Self {
        let request = observation.request();
        let (native_session, native_generation) = request.identity();
        let (event_count, historical_target) = match request.target() {
            x::ExactTarget::Live => (request.source_record().event_count(), false),
            x::ExactTarget::History { event_count } => (event_count, true),
        };
        Self {
            format: "mshogi.xiangqi.exact-observation.v1".into(),
            executor: "native_in_process".into(),
            source: CanonicalRecord::xiangqi(request.source_record().clone()),
            purpose: "natural_board_win".into(),
            goal: x::NATURAL_WIN_GOAL.into(),
            event_count,
            historical_target,
            position_json: x::PositionRoot::analysis(request.position().clone()).to_json(),
            native_session,
            native_generation,
            limits: request.limits(),
            report: observation.report().clone(),
        }
    }
    /// Exact source history, separate from the analyzed event prefix.
    pub fn source(&self) -> &CanonicalRecord {
        &self.source
    }
    /// Actual reported proof, positive witness, Unknown/cancellation or failure.
    pub fn report(&self) -> &x::ExactReport {
        &self.report
    }
    /// Exact native event selection, not board ply.
    pub fn event_count(&self) -> usize {
        self.event_count
    }
    /// Validate against the game at its journal position; the native work used.
    pub(crate) fn validate_against(
        &self,
        game: &x::Game,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<u64, String> {
        if stopped() {
            return Err("Xiangqi evidence validation stopped".into());
        }
        if self.format != "mshogi.xiangqi.exact-observation.v1"
            || self.executor != "native_in_process"
            || self.purpose != "natural_board_win"
            || self.goal != x::NATURAL_WIN_GOAL
            || self.native_session == 0
            || self.native_generation == 0
            || self.native_session > MAX_SAFE_ID
            || self.native_generation > MAX_SAFE_ID
            || self.source
                != CanonicalRecord::xiangqi(x::GameRecord::from_game(game).map_err(error)?)
            || self.event_count > game.events().len()
            || !self.historical_target && self.event_count != game.events().len()
        {
            return Err("Corrupt exact historical evidence: source, purpose, identity or event target differs.".into());
        }
        let root = x::PositionRoot::from_json(&self.position_json).map_err(error)?;
        let target = game.event_prefix(self.event_count).map_err(error)?.position;
        if root != x::PositionRoot::analysis(target.clone()) {
            return Err("Corrupt exact evidence target position/provenance.".into());
        }
        let unsupported = !self.limits.is_supported();
        if matches!(
            self.report.verdict,
            x::ExactVerdict::Failure {
                reason: x::ExactFailure::UnsupportedLimits
            }
        ) != unsupported
        {
            return Err("Corrupt exact evidence limits/failure contract.".into());
        }
        let work = &self.report.work;
        if work.states > self.limits.states
            || work.expanded > work.states
            || work.transitions > self.limits.transitions
            || work.cache_hits > work.transitions
            || work.memory_bytes > self.limits.memory_bytes
            || matches!(self.report.verdict, x::ExactVerdict::NeitherCanWin)
                && (work.states == 0 || work.expanded != work.states)
        {
            return Err(
                "Corrupt exact evidence work/closure metadata; no graph was executed.".into(),
            );
        }
        if let x::ExactVerdict::NaturalWin {
            winner,
            reason,
            witness,
        } = &self.report.verdict
        {
            if witness.len() > 4096 || witness.len() > self.limits.witness_plies as usize {
                return Err(
                    "Application capacity: exact witness output exceeds its recorded bound.".into(),
                );
            }

            let p = &target;
            let mut p = x::Position::from_diagram(
                &p.pieces().collect::<Vec<_>>(),
                p.side_to_move(),
                x::Counters::unknown(),
            )
            .map_err(error)?;
            for &action in witness {
                if stopped() {
                    return Err("Xiangqi evidence validation stopped".into());
                }
                if !matches!(p.local_status(), Ok(x::LocalStatus::Ongoing { .. })) {
                    return Err("Corrupt exact witness: action after a natural terminal.".into());
                }
                p.apply(action).map_err(error)?;
            }
            if p.side_to_move().opponent() != *winner
                || p.local_status().map_err(error)?
                    != (x::LocalStatus::NoLegalAction { reason: *reason })
            {
                return Err(
                    "Corrupt exact natural-board witness; repetition penalties are not witnesses."
                        .into(),
                );
            }
        }
        // Negative graph closure is deliberately NOT recomputed here. Reports retain
        // their provenance as historical observations and cannot adjudicate the game.
        let witness = match &self.report.verdict {
            x::ExactVerdict::NaturalWin { witness, .. } => witness.len() as u64,
            _ => 0,
        };
        Ok(
            xiangqi_work::of(self.source.event_count(), self.source.known_board_actions())
                + xiangqi_work::VALIDATION_WORK
                + xiangqi_work::WITNESS_ACTION_WORK * witness,
        )
    }
}

impl SessionGame for Xiangqi {
    type Presentation = ();
    const PLAY_LIMITS: WorkLimits = PLAY_LIMITS;
    const MAX_DEPTH: u16 = mshogi_engine::XIANGQI_MAX_DEPTH;
    const MAX_NODES: u64 = mshogi_engine::XIANGQI_MAX_NODES;
    const BOARD_SIZE: u8 = 9;
    const BOARD_LABEL: &'static str = "Xiangqi intersection board";
    const INITIAL_FOCUS: &'static str = "e3";
    const SCORE_MEANING: &'static str = "Native Red/Black root-side hundredths of soldier. Terminal distances count command edges; heuristic evaluation and existential win witnesses never establish a forced win. Ordinary analysis uses sound_fast_only; Unknown is unscored evidence, not proof of win capability.";

    session_wiring!(Xiangqi, x, operation: XiangqiCommand, preferences: XiangqiPreferences,
    state: xiangqi, xiangqi_mut, local: {
        SessionOperation::Xiangqi { command } => command,
    });

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
        settings: &XiangqiSettings,
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
        let t = match result {
            XiangqiResult::Choice { telemetry, .. }
            | XiangqiResult::Finished { telemetry, .. }
            | XiangqiResult::Incomplete { telemetry }
            | XiangqiResult::Unresolved { telemetry, .. } => telemetry,
        };
        TelemetryView {
            score: t.score_label(),
            completed_depth: t.completed_depth,
            nodes: t.nodes,
            variation: t
                .principal_variation
                .iter()
                .map(|c| match c {
                    XiangqiChoice::Move { action } => action.to_string(),
                    XiangqiChoice::Claim { actor } => format!("{actor:?} count claim"),
                })
                .collect(),
            status: match t.status {
                XiangqiStop::Complete => "Complete",
                XiangqiStop::NodeLimit => "Node limit reached",
                XiangqiStop::Stopped => "Stopped",
                XiangqiStop::Unresolved => "Unresolved, unscored",
            },
        }
    }
    fn incomplete(result: &XiangqiResult) -> bool {
        matches!(
            result,
            XiangqiResult::Incomplete { .. } | XiangqiResult::Unresolved { .. }
        )
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
            exact_observations: state.document.evidence().to_vec(),
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
fn draw_offer_actor(game: &x::Game, settings: &XiangqiSettings, human: x::Side) -> x::Side {
    if settings.play_mode.is_two_humans() {
        game.position().side_to_move().opponent()
    } else {
        human
    }
}
fn draw_response_actor(
    game: &x::Game,
    settings: &XiangqiSettings,
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
    /// Attach already completed native exact work as historical evidence only.
    /// This issues no request, advertises no execution capability and applies no result.
    pub fn retain_xiangqi_exact_evidence(
        &mut self,
        expected: SessionToken,
        observation: &x::ExactObservation,
    ) -> Result<AdmissionReceipt, String> {
        self.retain_game_evidence::<Xiangqi>(
            expected,
            XiangqiExactEvidence::from_native(observation),
        )
    }
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
