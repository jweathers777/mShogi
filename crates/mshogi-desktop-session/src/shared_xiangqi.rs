//! Concrete native Xiangqi consumer; product UI remains separately owned.
use super::*;
use mshogi_core::xiangqi as x;
use mshogi_engine::{XiangqiChoice, XiangqiResult, XiangqiStop};
#[path = "shared_xiangqi_archive.rs"]
pub(super) mod archive;
pub use archive::{XiangqiExactEvidence, XiangqiSettings};
#[cfg(test)]
#[path = "shared_xiangqi_label_tests.rs"]
mod label_tests;
#[cfg(feature = "desktop-verification")]
#[path = "shared_xiangqi_verification.rs"]
mod verification;

// Matches MSH-97's measured interactive default; requests also undergo native validation.
const PLAY_LIMITS: WorkLimits = mshogi_engine::XIANGQI_PLAY_LIMITS;
const MAX_OBSERVATION_BYTES: usize = 16 * 1024 * 1024;
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
    /// Offer a draw after the human's own move under the native profile.
    pub offer_draw: bool,
    /// Accept the opponent's outstanding offer.
    pub accept_draw: bool,
    /// Decline the opponent's outstanding offer.
    pub decline_draw: bool,
}
pub(crate) struct XiangqiSession {
    pub(super) initial_record: x::GameRecord,
    pub(super) initial_settings: XiangqiSettings,
    pub(super) settings: XiangqiSettings,
    pub(super) controls: Vec<archive::XiangqiControl>,
    pub(super) analysis_intent: Option<(CanonicalRecord, WorkLimits)>,
    pub(super) durable_needs_stop: bool,
    pub(super) execution_floor: u64,
    pub(super) exact_observations: Vec<XiangqiExactEvidence>,
    pub(super) game: x::Game,
    pub(super) undo: Vec<x::GameUndo>,
    pub(super) human: x::Side,
    pub(super) viewed: Option<x::HistorySnapshot>,
    pub(super) viewed_game: Option<x::Game>,
    pub(crate) paused: bool,
    pub(super) pending: Option<ExecutionContext>,
    pub(super) submitted: bool,
    pub(super) phase: EnginePhase,
    pub(super) diagnostic: Option<String>,
    pub(super) analysis: Option<PresentedObservation>,
    pub(super) observations: Vec<PresentedObservation>,
    pub(super) observation_bytes: usize,
}
impl XiangqiSession {
    pub(crate) fn reopened(&mut self) {
        self.retire();
        self.paused = true;
        for observation in &mut self.observations {
            observation.historical = true;
        }
        self.diagnostic=Some("Document reopened with work stopped. Saved ordinary/exact observations are historical; Resume explicitly.".into());
    }
    pub(super) fn selected_position(&self) -> &x::Position {
        self.viewed
            .as_ref()
            .map_or(self.game.position(), |v| &v.position)
    }
    fn record(&self) -> CanonicalRecord {
        CanonicalRecord::xiangqi(
            x::GameRecord::from_game(self.viewed_game.as_ref().unwrap_or(&self.game))
                .expect("admitted native record capacity"),
        )
    }
    pub(crate) fn retire(&mut self) {
        self.pending = None;
        self.submitted = false;
        self.analysis = None;
        self.analysis_intent = None;
        self.phase = EnginePhase::Paused;
        self.diagnostic = None;
    }
    #[cfg(all(test, feature = "desktop-verification"))]
    fn observation_capacity(&self) -> bool {
        self.ensure_observation_capacity().is_ok()
    }
    pub(super) fn ensure_observation_capacity(&self) -> Result<(), OperationFailure> {
        let ordinary = self.observations.len();
        let exact = self.exact_observations.len();
        let mut facts = Vec::new();
        if ordinary.saturating_add(exact) >= MAX_ARCHIVE_OBSERVATIONS {
            facts.push(
                resource_fact(
                    ResourceCategory::CombinedObservations,
                    ordinary.saturating_add(exact),
                    MAX_ARCHIVE_OBSERVATIONS,
                    1,
                    ResourceScope::Xiangqi,
                    ResourceUnit::Observations,
                )
                .with_observations(ordinary as u64, exact as u64)
                .with_measurement(ResourceMeasurement::Accepted),
            );
        }
        if self
            .observation_bytes
            .saturating_add(mshogi_engine::MAX_RESPONSE_BYTES)
            > MAX_OBSERVATION_BYTES
        {
            facts.push(
                resource_fact(
                    ResourceCategory::EncodedObservationBytes,
                    self.observation_bytes,
                    MAX_OBSERVATION_BYTES,
                    mshogi_engine::MAX_RESPONSE_BYTES,
                    ResourceScope::Xiangqi,
                    ResourceUnit::Bytes,
                )
                .with_measurement(ResourceMeasurement::Accepted),
            );
        }
        if facts.is_empty() {
            Ok(())
        } else {
            let diagnostic = if facts.len() == 1
                && facts[0].category == ResourceCategory::CombinedObservations
            {
                "Application capacity: all256 observations are retained. Ordinary and exact observations share this limit. The complete accepted document remains saveable; saving or reopening cannot enlarge its structural limits."
            } else {
                "Application capacity: no remaining Xiangqi observation admission capacity. The complete accepted document remains saveable; saving or reopening cannot enlarge its structural limits."
            };
            Err(resource_refusal(facts, diagnostic))
        }
    }
}
fn choice(a: x::Action) -> PresentedChoice {
    PresentedChoice {
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
    let name = |s| side(NativeSide::Xiangqi(s)).label;
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
    let name = |s| side(NativeSide::Xiangqi(s)).label;
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
            side(owner).label,
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
impl DesktopController {
    pub(super) fn replace_xiangqi(
        &mut self,
        record: &x::GameRecord,
        human: x::Side,
    ) -> Result<SharedSnapshot, String> {
        let seed = archive::XiangqiDraft::replacement(record, human);
        let receipt = self.queue_xiangqi_candidate(seed, None, None, None, true)?;
        self.last_command_receipt = Some(receipt);
        Ok(self.shared_snapshot())
    }
    fn start_xiangqi(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<(), OperationFailure> {
        self.ensure_runtime_capacity()?;
        let state = self.session.xiangqi().unwrap();
        state.ensure_observation_capacity()?;
        let game = state.record();
        let request = ExecutionContext {
            purpose: if analysis {
                ExecutionPurpose::MoveAnalysis {
                    target: ExecutionTarget {
                        selection_id: self.session.selection_id,
                        event_index: game.event_count() as u64,
                        board_ply: game.known_board_actions() as u64,
                        session_version: self.session.version,
                        turn: NativeSide::Xiangqi(state.selected_position().side_to_move()),
                        label: format!(
                            "Xiangqi event {} ({})",
                            game.event_count(),
                            if state.viewed.is_some() {
                                "history"
                            } else {
                                "live"
                            }
                        ),
                    },
                }
            } else {
                ExecutionPurpose::EngineTurn {}
            },
            request_id: self.session.next_request + 1,
            session_version: self.session.version,
            process_generation: self.session.next_request + 1,
            game,
            limits,
        };
        if !request.valid_limits() {
            return Err("Unsupported Xiangqi operation or work limits; maximum depth3 and512 visits, ordinary sound_fast_only analysis only.".into());
        }
        let encoded =
            serde_json::to_vec(&execution_request(request.clone())).map_err(|e| e.to_string())?;
        if encoded.len() > mshogi_engine::MAX_REQUEST_BYTES {
            return Err(
                "Application capacity: complete encoded engine request exceeds transport capacity."
                    .into(),
            );
        }
        self.session.next_request += 1;
        let selection_id = self.session.selection_id;
        let state = self.session.xiangqi_mut().unwrap();
        state.execution_floor = state
            .execution_floor
            .max(request.request_id)
            .max(request.session_version)
            .max(selection_id);
        state.pending = Some(request);
        state.submitted = false;
        state.phase = EnginePhase::Thinking;
        Ok(())
    }
    pub(super) fn xiangqi_command(
        &mut self,
        op: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        // Cheap native queries reject impossible requests before even exposing
        // admission progress. All transitions still run on the private worker.
        let state = self.session.xiangqi().unwrap();
        match &op {
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves { depth, nodes },
            } => {
                if state.viewed.is_some() && state.viewed_game.is_none() {
                    return Err("This immutable event is inside an atomic native command. Analysis requires a committed command boundary; view and movement inspection remain available.".into());
                }
                if !(1..=3).contains(depth) || !(1..=512).contains(nodes) {
                    return Err("Xiangqi analysis requires depth1–3 and1–512 visits.".into());
                }
                if let Err(failure) = state.ensure_observation_capacity() {
                    let diagnostic = failure.diagnostic.clone();
                    self.pause_native_failure(failure);
                    return Err(diagnostic);
                }
            }
            SessionOperation::Analyze { .. } => {
                return Err(
                    "Chess Full is not Xiangqi analysis. Native exact execution remains separate."
                        .into(),
                );
            }
            SessionOperation::Xiangqi { .. } | SessionOperation::Resign {} => {
                if state.viewed.is_some() {
                    return Err(
                        "Historical viewing is read-only. Return to live explicitly.".into(),
                    );
                }
                if state.paused {
                    return Err("Resume live play before submitting a native command.".into());
                }
                if let SessionOperation::Xiangqi {
                    command: x::GameCommand::Play { action },
                } = &op
                {
                    if state.game.position().side_to_move() != state.human {
                        return Err("Native board action requires its actor turn.".into());
                    }
                    state
                        .game
                        .position()
                        .resolve_action(*action)
                        .map_err(|e| e.to_string())?;
                }
            }
            SessionOperation::TakeBack {}
                if state.viewed.is_some()
                    || state
                        .game
                        .preceding_playable_decision(state.human)
                        .is_none() =>
            {
                return Err("No live playable human decision is available for Take back.".into());
            }
            SessionOperation::Undo {} if state.viewed.is_some() || state.undo.is_empty() => {
                return Err("No live native command to undo.".into());
            }
            SessionOperation::Inspect { event_index }
                if *event_index > state.game.events().len() =>
            {
                return Err("Unknown native event prefix.".into());
            }
            SessionOperation::Resume {} | SessionOperation::Retry {} if state.viewed.is_some() => {
                return Err("Return to live before resuming play.".into());
            }
            SessionOperation::Cancel {}
            | SessionOperation::ReturnLive {}
            | SessionOperation::TakeBack {}
            | SessionOperation::Undo {}
            | SessionOperation::Inspect { .. }
            | SessionOperation::Resume {}
            | SessionOperation::Retry {}
            | SessionOperation::XiangqiPreferences { .. } => {}
            _ => return Err("Unsupported operation for the native Xiangqi session.".into()),
        }
        let seed = archive::XiangqiDraft::capture(self.session.xiangqi().unwrap());
        let receipt = self.queue_xiangqi_candidate(seed, Some(op), None, None, false)?;
        self.last_command_receipt = Some(receipt);
        Ok(self.shared_snapshot())
    }
    pub(super) fn accept_xiangqi_observation(&mut self, response: ExecutionObservation) -> bool {
        let state = self.session.xiangqi().unwrap();
        let Some(pending) = &state.pending else {
            return false;
        };
        if self.admission_token.is_some()
            || self.session.ensure_capacity().is_err()
            || !response.matches(pending)
            || pending.session_version != self.session.version
            || pending.game != state.record()
        {
            return false;
        }
        self.queue_xiangqi_candidate(
            archive::XiangqiDraft::capture(state),
            None,
            Some(response),
            None,
            false,
        )
        .is_ok()
    }
    pub(super) fn poll_xiangqi(&mut self) -> SharedSnapshot {
        if let Some(response) = self.bridge.poll() {
            self.accept_shared(response);
        }
        let state = self.session.xiangqi().unwrap();
        if state.pending.is_none()
            && !state.paused
            && state.viewed.is_none()
            && state.game.conclusion().is_none()
            && state.game.pending_adjudication().is_none()
            && state.game.position().side_to_move() != state.human
            && let Err(e) = self.start_xiangqi(PLAY_LIMITS, false)
        {
            self.pause_native_failure(e);
        }
        let auto_submit = self.shared.auto_submit();
        let state = self.session.xiangqi_mut().unwrap();
        if auto_submit
            && !state.submitted
            && let Some(request) = state.pending.clone()
        {
            state.submitted = true;
            if let Err(e) = self.bridge.submit(execution_request(request.clone())) {
                self.accept_xiangqi_observation(ExecutionObservation {
                    schema_version: OBSERVATION_SCHEMA_VERSION,
                    context: request,
                    result: ExecutionResult::Failure {
                        failure: mshogi_engine::EngineFailure {
                            kind: mshogi_engine::FailureKind::Start,
                            message: format!("Cannot start native work: {e:?}"),
                        },
                    },
                });
            }
        }
        self.xiangqi_snapshot()
    }
    pub(super) fn xiangqi_snapshot(&self) -> SharedSnapshot {
        let state = self.session.xiangqi().unwrap();
        let position = state.selected_position();
        let record = state.record();
        let history = state.viewed.clone().unwrap_or_else(|| {
            state
                .game
                .event_prefix(state.game.events().len())
                .expect("native current prefix")
        });
        let concluded = history.conclusion;
        let pending = history.pending.as_ref();
        let historical = state.viewed.is_some();
        let active = !historical && concluded.is_none() && pending.is_none();
        let sides = vec![
            side(NativeSide::Xiangqi(x::Side::Red)),
            side(NativeSide::Xiangqi(x::Side::Black)),
        ];
        let files: Vec<_> = (b'a'..=b'i').map(|f| char::from(f).to_string()).collect();
        let ranks: Vec<_> = (0..10).rev().map(|r| r.to_string()).collect();
        let count_advice: Vec<_> = if historical && state.viewed_game.is_none() {
            vec![]
        } else {
            [x::Side::Red, x::Side::Black]
                .into_iter()
                .map(|s| {
                    state
                        .viewed_game
                        .as_ref()
                        .unwrap_or(&state.game)
                        .natural_count_eligibility(s)
                })
                .collect()
        };
        let status = PresentedStatus {
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
            } else if pending.is_some() {
                "Adjudication unresolved · unscored".into()
            } else {
                format!(
                    "{} to move{}",
                    side(NativeSide::Xiangqi(position.side_to_move())).label,
                    if position.is_in_check(position.side_to_move()) {
                        " · check"
                    } else {
                        ""
                    }
                )
            },
        };
        SharedSnapshot {
            xiangqi: Some(XiangqiPresentation {
                terminal_actions: XiangqiTerminalActions {
                    offer_draw: active && !state.paused && state.game.can_offer_draw(state.human),
                    accept_draw: active
                        && !state.paused
                        && state.game.can_respond_to_draw(state.human),
                    decline_draw: active
                        && !state.paused
                        && state.game.can_respond_to_draw(state.human),
                },
                canonical_context_scope: if historical && state.viewed_game.is_none() {
                    "live_with_immutable_selection"
                } else {
                    "selected_committed"
                },
                selected_canonical: if historical && state.viewed_game.is_none() {
                    None
                } else {
                    Some(record.clone())
                },
                selected_events: state.game.events()[..history.event_count].to_vec(),
                selected_position: x::PositionRoot::analysis(position.clone()).to_json(),
                root: state.game.root().to_json(),
                claim_policy: state.game.claim_policy(),
                count_advice: count_advice.clone(),
                warnings: history.warnings.clone(),
                offered_by: history.offered_by,
                conclusion: concluded,
                pending: pending.cloned(),
                no_win_assessment: history.no_win_assessment.clone(),
                exact_observations: state.exact_observations.clone(),
            }),
            open_token: self.shared_open_token(),
            document: self.shared_document(),
            io: self.io_state.clone(),
            capacity: self.session.capacity.clone(),
            token: self.shared_token(),
            game: GameKind::Xiangqi,
            profile: RulesProfile::XiangqiWxf2018V1,
            edition: RulesProfile::XiangqiWxf2018V1.edition(),
            human: side(NativeSide::Xiangqi(state.human)),
            turn: side(NativeSide::Xiangqi(position.side_to_move())),
            sides: sides.clone(),
            board: BoardDescriptor {
                files: files.clone(),
                ranks: ranks.clone(),
                label: "Xiangqi intersection board".into(),
                initial_focus: "e3".into(),
                intersections: Some(IntersectionTopology {
                    points: x::Intersection::all().map(|p| p.to_string()).collect(),
                    river_between_ranks: x::RIVER_BETWEEN_RANKS,
                    palaces: [x::Side::Red, x::Side::Black]
                        .into_iter()
                        .map(|actor| {
                            let base = if actor == x::Side::Red { 0 } else { 7 };
                            Palace {
                                side: side(NativeSide::Xiangqi(actor)),
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
                }),
            },
            orientations: vec![
                Orientation {
                    side: sides[0].clone(),
                    files: files.clone(),
                    ranks: ranks.clone(),
                },
                Orientation {
                    side: sides[1].clone(),
                    files: files.into_iter().rev().collect(),
                    ranks: ranks.into_iter().rev().collect(),
                },
            ],
            pieces: x::Intersection::all()
                .filter_map(|point| {
                    position.piece(point).map(|p| {
                        let owner = side(NativeSide::Xiangqi(p.side));
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
                .collect(),
            hands: vec![],
            choices: if concluded.is_none() && pending.is_none() {
                position
                    .legal_actions()
                    .expect("admitted native position")
                    .into_iter()
                    .map(choice)
                    .collect()
            } else {
                vec![]
            },
            capabilities: CommandCapabilities {
                board_action: active && !state.paused && position.side_to_move() == state.human,
                drops: false,
                promotion: false,
                take_back: !historical
                    && state
                        .game
                        .preceding_playable_decision(state.human)
                        .is_some(),
                undo: !historical && !state.undo.is_empty(),
                engine_turn: active && position.side_to_move() != state.human,
                move_analysis: !historical || state.viewed_game.is_some(),
                chess_full: false,
                claims: active
                    && !state.paused
                    && count_advice
                        .iter()
                        .any(|e| e.claimant == state.human && e.eligible),
                declaration: false,
                resign: active && !state.paused,
                hands: false,
                history: true,
                save: true,
            },
            status,
            events: state
                .game
                .events()
                .iter()
                .map(|e| PresentedEvent {
                    index: e.index + 1,
                    board_ply: e.board_ply,
                    label: event_label(e),
                })
                .collect(),
            event_index: history.event_count,
            board_ply: history.board_ply as u64,
            play_limits: PLAY_LIMITS,
            historical,
            paused: state.paused,
            phase: state.phase,
            execution_purpose: state.pending.as_ref().map(|p| {
                if matches!(p.purpose, ExecutionPurpose::EngineTurn {}) {
                    "engine_turn"
                } else {
                    "move_analysis"
                }
            }),
            diagnostic: state.diagnostic.clone(),
            declaration_advice: vec![],
            context: record,
            analysis: state.analysis.clone(),
            observations: state.observations.clone(),
            search_observations: state
                .observations
                .iter()
                .enumerate()
                .map(|(index, o)| display_observation(index, o, state.analysis.as_ref()))
                .collect(),
            score_meaning: "Native Red/Black root-side hundredths of soldier. Terminal distances count command edges; heuristic evaluation and existential win witnesses never establish a forced win. Ordinary analysis uses sound_fast_only; Unknown is unscored evidence, not proof of win capability.",
            chess: None,
            chess_match: None,
        }
    }
}
fn display_observation(
    index: usize,
    o: &PresentedObservation,
    current: Option<&PresentedObservation>,
) -> PresentedSearchObservation {
    let t = match &o.observation.result {
        ExecutionResult::Xiangqi {
            result:
                XiangqiResult::Choice { telemetry, .. }
                | XiangqiResult::Finished { telemetry, .. }
                | XiangqiResult::Incomplete { telemetry }
                | XiangqiResult::Unresolved { telemetry, .. },
        } => Some(telemetry),
        _ => None,
    };
    let (purpose, event_index, board_ply) = match &o.observation.context.purpose {
        ExecutionPurpose::MoveAnalysis { target } => (
            "move_analysis",
            target.event_index as usize,
            target.board_ply as usize,
        ),
        _ => (
            "engine_turn",
            o.observation.context.game.event_count(),
            o.observation.context.game.known_board_actions(),
        ),
    };
    let perspective = match &o.observation.context.game {
        CanonicalRecord::Xiangqi(r) => side(NativeSide::Xiangqi(r.side_to_move())).label,
        _ => unreachable!(),
    };
    PresentedSearchObservation {
        observation_index: index,
        purpose,
        event_index,
        board_ply,
        perspective,
        engine: "mShogi",
        score: t.filter(|_| o.application_error.is_none()).and_then(|t| {
            t.score.map(|s| {
                if s.abs() >= 999_996 {
                    format!(
                        "{} in {} native command edges",
                        if s >= 0 { "Win" } else { "Loss" },
                        1_000_000 - s.abs()
                    )
                } else {
                    format!("{:+.2} soldiers", f64::from(s) / 100.0)
                }
            })
        }),
        completed_depth: t.map(|t| t.completed_depth),
        nodes: t.map(|t| t.nodes),
        variation: t.map_or_else(Vec::new, |t| {
            t.principal_variation
                .iter()
                .map(|c| match c {
                    XiangqiChoice::Move { action } => action.to_string(),
                    XiangqiChoice::Claim { actor } => format!("{actor:?} count claim"),
                })
                .collect()
        }),
        status: if o.application_error.is_some() {
            "Rejected engine result"
        } else {
            match t.map(|t| t.status) {
                Some(XiangqiStop::Complete) => "Complete",
                Some(XiangqiStop::NodeLimit) => "Node limit reached",
                Some(XiangqiStop::Stopped) => "Stopped",
                Some(XiangqiStop::Unresolved) => "Unresolved, unscored",
                None => "Engine failure",
            }
        },
        historical: o.historical,
        selected: current.is_some_and(|c| c.observation == o.observation) && !o.historical,
    }
}

#[cfg(test)]
#[path = "shared_xiangqi_tests.rs"]
mod tests;
