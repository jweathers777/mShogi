//! Real native line-server observations through the immutable document boundary.
//! These tests call serve_once in-process; actual subprocess tests are separate.
use super::*;
use mshogi_core::native::NativeSide;
use mshogi_engine::{
    CHU_PLAY_LIMITS, ChuResult, ExecutionContext, ExecutionRequest, ExecutionResponse,
    ExecutionTarget, PROTOCOL_VERSION,
};
use std::{io::Cursor, time::Instant};

fn standard() -> ValidatedChuDocument {
    ValidatedChuDocument::new(
        c::GameRecord::from_game(&c::Game::standard()).unwrap(),
        c::Side::Sente,
    )
    .unwrap()
}
fn context(record: &c::GameRecord, id: u64) -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: id,
        session_version: id,
        process_generation: id,
        game: CanonicalRecord::chu(record.clone()),
        limits: CHU_PLAY_LIMITS,
    }
}
fn execute(ctx: ExecutionContext) -> ExecutionObservation {
    let request = ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ctx,
    };
    let mut out = Vec::new();
    mshogi_engine::serve_once(
        Cursor::new(format!("{}\n", serde_json::to_string(&request).unwrap())),
        &mut out,
    )
    .unwrap();
    let response: ExecutionResponse = serde_json::from_slice(&out).unwrap();
    assert!(response.matches(&request));
    response.observation
}
fn tactical() -> ValidatedChuDocument {
    // Central Lion can capture pawn then rook in one complete turn. Both
    // opponent units disappear, producing the independently expected native
    // narrow material ending; the exact route remains engine-selected.
    let p = c::Position::from_diagram(
        &[
            ("12l", c::Side::Sente, "K"),
            ("1a", c::Side::Gote, "K"),
            ("6f", c::Side::Sente, "Ln"),
            ("6e", c::Side::Gote, "P"),
            ("5e", c::Side::Gote, "R"),
        ]
        .map(|(s, side, p)| (s.parse().unwrap(), c::Piece::from_code(side, p).unwrap())),
        c::Side::Sente,
        c::RootMetadata::diagram(71),
    )
    .unwrap();
    ValidatedChuDocument::new(
        c::GameRecord::from_game(&c::Game::from_root(p).unwrap()).unwrap(),
        c::Side::Gote,
    )
    .unwrap()
}
fn control(
    d: &ValidatedChuDocument,
    actor: c::Side,
    command: c::GameCommand,
) -> ValidatedChuDocument {
    d.with_control(ChuControl::Native { actor, command }, &mut || false)
        .unwrap()
}
fn decode(d: &ValidatedChuDocument) -> ValidatedChuDocument {
    ValidatedChuDocument::decode(d.bytes(), &mut || false).unwrap()
}
#[test]
fn actual_engine_compound_choice_is_journaled_before_exact_native_action_and_roundtrips() {
    let d = tactical();
    let before = d.record().clone();
    let observation = execute(context(d.record(), 1));
    let ExecutionResult::Chu {
        result: ChuResult::Choice { action, telemetry },
    } = &observation.result
    else {
        panic!("{observation:?}")
    };
    assert_eq!(action.route.len(), 2);
    assert_eq!(telemetry.principal_variation.first(), Some(action));
    assert_eq!(telemetry.score, Some(999_999));
    let expected = action.clone();
    let d = d.with_observation(observation, &mut || false).unwrap();
    assert_eq!(d.observations().len(), 1);
    assert!(matches!(
        &d.controls()[0],
        ChuControl::Observation { index: 0 }
    ));
    assert!(
        matches!(&d.controls()[1],ChuControl::Native{actor:c::Side::Sente,command:c::GameCommand::Play{action}} if *action==expected)
    );
    let game = d.record().to_game().unwrap();
    let event = &game.events()[0];
    assert_eq!(
        event.command,
        c::GameCommand::Play {
            action: expected.clone()
        }
    );
    let effects = event.effects.as_ref().unwrap();
    assert_eq!(effects.captures.len(), 2);
    assert_eq!(
        effects
            .captures
            .iter()
            .map(|c| c.square)
            .collect::<Vec<_>>(),
        expected.route
    );
    assert_eq!(
        game.conclusion(),
        Some(c::Conclusion::Material {
            winner: c::Side::Sente
        })
    );
    assert_eq!(before.event_count(), 0);
    let reopened = decode(&d);
    assert_eq!(reopened.record(), d.record());
    assert_eq!(reopened.observations(), d.observations());
}
#[test]
fn historical_analysis_uses_exact_prefix_without_mutation_or_future_and_survives_take_back() {
    let d = standard();
    let observation = execute(context(d.record(), 1));
    let d = d.with_observation(observation, &mut || false).unwrap();
    let game = d.record().to_game().unwrap();
    let prefix = game.record_prefix(0).unwrap();
    let mut ctx = context(&prefix, 2);
    ctx.purpose = ExecutionPurpose::MoveAnalysis {
        target: ExecutionTarget {
            selection_id: 9,
            event_index: 0,
            board_ply: 0,
            session_version: 2,
            label: "standard before first action".into(),
            turn: NativeSide::Chu(c::Side::Sente),
        },
    };
    let observation = execute(ctx);
    let before = d.record().clone();
    let d = d
        .with_selection(Some(0), &mut || false)
        .unwrap()
        .with_observation(observation.clone(), &mut || false)
        .unwrap();
    assert_eq!(d.record(), &before);
    assert_eq!(d.selected_event(), Some(0));
    assert_eq!(d.observations()[1], observation);
    assert_eq!(
        native(&d.observations()[1].context.game)
            .unwrap()
            .event_count(),
        0
    );
    let reopened = decode(&d);
    assert_eq!(reopened.selected_event(), Some(0));
    assert_eq!(reopened.record(), &before);
    let rewound = d
        .with_control(
            ChuControl::Rewind {
                event_index: 0,
                reason: RewindKind::TakeBack,
            },
            &mut || false,
        )
        .unwrap();
    assert_eq!(rewound.record().event_count(), 0);
    assert_eq!(rewound.observations().len(), 2);
    let retired: Vec<_> = rewound.retired_branches().collect();
    assert_eq!(retired.len(), 1);
    assert_eq!(retired[0].1, &before);
    assert_eq!(decode(&rewound).observations(), rewound.observations());
}
#[test]
fn forged_actual_observation_score_pv_limits_context_and_journal_are_rejected_atomically() {
    let d = tactical();
    let observation = execute(context(d.record(), 3));
    let before = d.bytes().to_vec();
    for case in 0..6 {
        let mut forged = observation.clone();
        match case {
            0 => {
                let ExecutionResult::Chu {
                    result: ChuResult::Choice { telemetry, .. },
                } = &mut forged.result
                else {
                    panic!()
                };
                telemetry.score = Some(123);
            }
            1 => {
                let ExecutionResult::Chu {
                    result: ChuResult::Choice { telemetry, .. },
                } = &mut forged.result
                else {
                    panic!()
                };
                telemetry.principal_variation[0].route.pop();
            }
            2 => forged.context.limits.nodes = 0,
            3 => forged.context.request_id = 0,
            4 => {
                let game = c::Game::standard();
                forged.context.game =
                    CanonicalRecord::chu(c::GameRecord::from_game(&game).unwrap());
            }
            _ => {
                forged.context.purpose = ExecutionPurpose::MoveAnalysis {
                    target: ExecutionTarget {
                        selection_id: 1,
                        event_index: 1,
                        board_ply: 0,
                        session_version: 3,
                        label: "future nonexistent event".into(),
                        turn: NativeSide::Chu(c::Side::Sente),
                    },
                }
            }
        }
        assert!(
            d.with_observation(forged, &mut || false).is_err(),
            "forged case{case}"
        );
        assert_eq!(d.bytes(), before);
    }
    let accepted = d.with_observation(observation, &mut || false).unwrap();
    let mut omitted = accepted.payload.clone();
    omitted.controls.pop();
    omitted.current = omitted.initial.clone();
    assert!(validate(omitted, &mut || false).is_err());
    let mut reordered = accepted.payload.clone();
    reordered.controls.swap(0, 1);
    assert!(validate(reordered, &mut || false).is_err());
    let mut changed = accepted.payload.clone();
    let ChuControl::Native {
        command: c::GameCommand::Play { action },
        ..
    } = &mut changed.controls[1]
    else {
        panic!()
    };
    action.route.reverse();
    assert!(validate(changed, &mut || false).is_err());
    assert_eq!(d.bytes(), before);
}
#[test]
fn actual_zero_work_observations_256_to_257_keep_current_and_reserved_endings_saveable() {
    let baseline = standard();
    let mut payload = baseline.payload.clone();
    for id in 1..=255 {
        let mut ctx = context(baseline.record(), id);
        ctx.limits.nodes = 0;
        let observation = execute(ctx);
        assert!(
            matches!(&observation.result,ExecutionResult::Chu{result:ChuResult::Incomplete{telemetry}} if telemetry.score.is_none() && telemetry.nodes==0 && telemetry.principal_variation.is_empty())
        );
        payload.controls.push(ChuControl::Observation {
            index: payload.observations.len(),
        });
        payload.observations.push(observation);
    }
    payload.retired_execution_floor = 255;
    let d = validate(payload, &mut || false).unwrap();
    let mut ctx = context(d.record(), 256);
    ctx.limits.nodes = 0;
    let d = d.with_observation(execute(ctx), &mut || false).unwrap();
    let before = d.bytes().to_vec();
    assert_eq!(d.observations().len(), 256);
    assert_eq!(d.record(), baseline.record());
    let mut ctx = context(d.record(), 257);
    ctx.limits.nodes = 0;
    let error = d.with_observation(execute(ctx), &mut || false).unwrap_err();
    let fact = &error.resource.as_ref().unwrap().facts()[0];
    assert_eq!(fact.category, ResourceCategory::OrdinaryObservations);
    assert_eq!(fact.count, Some(257));
    assert_eq!(d.bytes(), before);
    assert_eq!(decode(&d).observations().len(), 256);
    let ended = control(
        &d,
        c::Side::Sente,
        c::GameCommand::Stop {
            reason: c::StopReason::Cancelled,
        },
    );
    let ended = control(&ended, c::Side::Gote, c::GameCommand::Resign);
    let ended = ended.with_reserved_close(257).unwrap();
    let restored = decode(&ended);
    assert_eq!(restored.observations().len(), 256);
    assert_eq!(
        restored.record().conclusion(),
        Some(c::Conclusion::Resignation {
            winner: c::Side::Sente
        })
    );
    println!(
        "CHU_ACTUAL_OBSERVATION_BOUNDARY {}",
        serde_json::to_string(d.usage()).unwrap()
    );
}
#[test]
fn real_engine_standard_progression_measures_targets_pv_and_retired_branch_capacity() {
    let started = Instant::now();
    let mut d = standard();
    for ply in 1..=32 {
        let observation = execute(context(d.record(), ply));
        assert!(
            matches!(
                &observation.result,
                ExecutionResult::Chu {
                    result: ChuResult::Choice { .. }
                }
            ),
            "representative standard progression at{ply}: {observation:?}"
        );
        d = d.with_observation(observation, &mut || false).unwrap();
        assert_eq!(d.record().known_board_actions(), ply as usize);
        if [12, 32].contains(&ply) {
            let before = d.record().clone();
            let selected = d
                .with_selection(Some(ply as usize / 2), &mut || false)
                .unwrap();
            let begin = Instant::now();
            let reopened = decode(&selected);
            assert_eq!(reopened.record(), &before);
            println!(
                "CHU_ENGINE_DOCUMENT_MEASUREMENT {}",
                serde_json::json!({"board_actions":ply,"usage":selected.usage(),"decode_ms":begin.elapsed().as_millis(),"elapsed_ms":started.elapsed().as_millis()})
            );
        }
    }
    let before = d.record().clone();
    let d = d
        .with_control(
            ChuControl::Rewind {
                event_index: 24,
                reason: RewindKind::TakeBack,
            },
            &mut || false,
        )
        .unwrap();
    assert_eq!(d.observations().len(), 32);
    assert_eq!(d.record().known_board_actions(), 24);
    assert_eq!(d.retired_branches().next().unwrap().1, &before);
    let restored = decode(&d);
    assert_eq!(restored.record(), d.record());
    assert_eq!(restored.observations(), d.observations());
    println!(
        "CHU_ENGINE_DOCUMENT_TAKE_BACK {}",
        serde_json::to_string(d.usage()).unwrap()
    );
}
