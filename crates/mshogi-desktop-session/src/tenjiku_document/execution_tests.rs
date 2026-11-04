use super::*;
use mshogi_core::native::NativeSide;
use mshogi_engine::{
    ExecutionContext, ExecutionRequest, ExecutionResponse, ExecutionTarget, FailureKind,
    PROTOCOL_VERSION,
};
use std::io::Cursor;
pub(super) fn execute(context: ExecutionContext) -> ExecutionObservation {
    let request = ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context,
    };
    let mut output = vec![];
    mshogi_engine::serve_once(
        Cursor::new(format!("{}\n", serde_json::to_string(&request).unwrap())),
        &mut output,
    )
    .unwrap();
    let response: ExecutionResponse = serde_json::from_slice(&output).unwrap();
    assert!(response.matches(&request));
    assert!(matches!(
        &response.observation.result,
        ExecutionResult::Tenjiku {
            result: mshogi_engine::TenjikuResult::Incomplete { .. }
        }
    ));
    response.observation
}
pub(super) fn context(record: &c::GameRecord, id: u64) -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: id,
        session_version: id,
        process_generation: id,
        game: CanonicalRecord::tenjiku(record.clone()),
        limits: WorkLimits { depth: 1, nodes: 1 },
    }
}
#[test]
fn oversized_observation_native_history_preserves_typed_capacity_and_accepted_bytes() {
    for profile in c::Profile::ALL {
        let d = command(
            &administrative_root(profile),
            c::Side::Sente,
            c::GameCommand::OfferDraw,
        );
        let accepted = d.bytes().to_vec();
        let observation = execute(context(d.record(), 1));
        let mut raw = serde_json::to_value(&observation).unwrap();
        let event = raw["context"]["game"]["payload"]["events"][0].clone();
        raw["context"]["game"]["payload"]["events"] =
            serde_json::json!(vec![event; c::MAX_GAME_EVENTS + 1]);
        assert!(
            raw["context"]["game"].to_string().len()
                < mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES
        );
        assert!(matches!(
            ExecutionObservation::from_json_with_stop_typed(&raw.to_string(), &mut || false),
            Err(mshogi_engine::ObservationDecodeError::Native(
                mshogi_core::native::EnvelopeError::Capacity
            ))
        ));
        let mut wire: serde_json::Value = serde_json::from_slice(&accepted).unwrap();
        wire["payload"]["observations"] = serde_json::json!([raw]);
        let error =
            ValidatedTenjikuDocument::decode(&serde_json::to_vec(&wire).unwrap(), &mut || false)
                .unwrap_err();
        assert!(error.resource.is_some(), "{error}");
        assert!(!error.stopped);
        assert_eq!(d.bytes(), accepted);
        for stop_at in [1, 20] {
            let mut polls = 0;
            let text = serde_json::to_string(&observation).unwrap();
            assert!(matches!(
                ExecutionObservation::from_json_with_stop_typed(&text, &mut || {
                    polls += 1;
                    polls == stop_at
                }),
                Err(mshogi_engine::ObservationDecodeError::Stopped)
            ));
        }
    }
}
#[test]
fn real_incomplete_work_is_unscored_exact_historical_and_survives_rewind() {
    for profile in c::Profile::ALL {
        let d = administrative_root(profile);
        let d = command(&d, c::Side::Sente, c::GameCommand::OfferDraw);
        let prefix = d.record().to_game().unwrap().record_prefix(0).unwrap();
        let mut ctx = context(&prefix, 1);
        ctx.purpose = ExecutionPurpose::MoveAnalysis {
            target: ExecutionTarget {
                selection_id: 1,
                event_index: 0,
                board_ply: 0,
                session_version: 1,
                label: "Root before draw offer".into(),
                turn: NativeSide::Tenjiku(c::Side::Sente),
            },
        };
        let observation = execute(ctx);
        let saved = observation.clone();
        let d = d.with_observation(observation, &mut || false).unwrap();
        assert_eq!(d.record().event_count(), 1);
        assert_eq!(d.observations(), std::slice::from_ref(&saved));
        let d = d.with_selection(Some(0), &mut || false).unwrap();
        let d = d
            .with_control(
                TenjikuControl::Rewind {
                    event_index: 0,
                    reason: RewindKind::TakeBack,
                },
                &mut || false,
            )
            .unwrap();
        assert_eq!(d.retired_branches().count(), 1);
        assert_eq!(d.observations(), [saved]);
        let r = decoded(&d);
        assert_eq!(r.observations(), d.observations());
        assert_eq!(r.record(), d.record());
    }
}
#[test]
fn both_direction_cross_profile_contexts_invalid_targets_and_forged_results_refuse() {
    for profile in c::Profile::ALL {
        let d = administrative_root(profile);
        let before = d.bytes().to_vec();
        let actual = execute(context(d.record(), 1));
        let other = if profile == c::Profile::AdamsModern {
            c::Profile::MullerCompleted
        } else {
            c::Profile::AdamsModern
        };
        let wrong = execute(context(administrative_root(other).record(), 1));
        assert!(d.with_observation(wrong, &mut || false).is_err());
        let mut forged = actual.clone();
        forged.context.request_id = 0;
        assert!(d.with_observation(forged, &mut || false).is_err());
        let mut forged = actual.clone();
        forged.context.purpose = ExecutionPurpose::MoveAnalysis {
            target: ExecutionTarget {
                selection_id: 1,
                event_index: 1,
                board_ply: 0,
                session_version: 1,
                label: "Forged target beyond retained root".into(),
                turn: NativeSide::Tenjiku(c::Side::Sente),
            },
        };
        assert!(d.with_observation(forged, &mut || false).is_err());
        let mut forged = actual;
        forged.result = ExecutionResult::Chess {
            result: mshogi_engine::ChessResult::Failure {
                failure: mshogi_engine::EngineFailure {
                    kind: FailureKind::UnsupportedMode,
                    message: "Foreign result family deliberately forged for rejection".into(),
                },
            },
        };
        assert!(d.with_observation(forged, &mut || false).is_err());
        assert_eq!(d.bytes(), before);
    }
}
#[test]
fn incomplete_observation_capacity_256_to257_preserves_complete_document_and_all_endings() {
    for profile in c::Profile::ALL {
        let baseline = administrative_root(profile);
        let observation = execute(context(baseline.record(), 1));
        let mut payload = baseline.payload.clone();
        // Synthetic nonce variants of one real incomplete observation test
        // document capacity; this is not255 separately executed engine requests.
        for id in 1..=256 {
            let mut o = observation.clone();
            o.context.request_id = id;
            o.context.process_generation = id;
            o.context.session_version = id;
            payload.controls.push(TenjikuControl::Observation {
                index: payload.observations.len(),
            });
            payload.observations.push(o);
        }
        payload.retired_execution_floor = 256;
        let bytes = document::encode(baseline.profile(), &payload).unwrap();
        let d = ValidatedTenjikuDocument::decode(&bytes, &mut || false).unwrap();
        assert_eq!(d.observations().len(), 256);
        let before = d.bytes().to_vec();
        let error = d
            .with_observation(execute(context(d.record(), 257)), &mut || false)
            .unwrap_err();
        assert!(error.resource.is_some());
        assert_eq!(d.bytes(), before);
        // Exercise actual native endings, then strict-decode the complete final
        // document once. Incremental document endings are covered across every
        // actor combination separately; avoid repeatedly replaying 256 contexts.
        let mut ended = d.with_reserved_close(MAX_SAFE_ID - 65).unwrap().payload;
        let mut game = d.record().to_game().unwrap();
        for (actor, command) in [
            (
                c::Side::Gote,
                c::GameCommand::Stop {
                    reason: c::StopReason::EngineFailure,
                },
            ),
            (c::Side::Sente, c::GameCommand::Resign),
            (
                c::Side::Gote,
                c::GameCommand::Stop {
                    reason: c::StopReason::EngineFailure,
                },
            ),
        ] {
            game.submit(c::GameRequest {
                expected: game.context(),
                actor,
                command: command.clone(),
            })
            .unwrap();
            ended
                .controls
                .push(TenjikuControl::Native { actor, command });
        }
        ended.current = CanonicalRecord::tenjiku(c::GameRecord::from_game(&game).unwrap());
        ended.controls.push(TenjikuControl::Close {});
        let bytes = document::encode(d.profile(), &ended).unwrap();
        let restored = ValidatedTenjikuDocument::decode(&bytes, &mut || false).unwrap();
        assert_eq!(restored.observations(), d.observations());
        assert_eq!(restored.record().event_count(), 3);
        assert_eq!(restored.record(), &c::GameRecord::from_game(&game).unwrap());
    }
}

#[test]
fn real_completed_engine_action_requires_observation_then_exact_action_and_analysis_is_read_only() {
    for profile in c::Profile::ALL {
        let baseline = administrative_root(profile);
        let mut ctx = context(baseline.record(), 1);
        ctx.limits = mshogi_engine::TENJIKU_PLAY_LIMITS;
        let request = ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: ctx.clone(),
        };
        let mut output = vec![];
        mshogi_engine::serve_once(
            Cursor::new(format!("{}\n", serde_json::to_string(&request).unwrap())),
            &mut output,
        )
        .unwrap();
        let response: ExecutionResponse = serde_json::from_slice(&output).unwrap();
        assert!(response.matches(&request));
        let observation = response.observation;
        assert!(matches!(
            &observation.result,
            ExecutionResult::Tenjiku {
                result: mshogi_engine::TenjikuResult::Choice { .. }
            }
        ));
        let d = baseline
            .with_observation(observation.clone(), &mut || false)
            .unwrap();
        assert_eq!(d.record().event_count(), 1);
        assert!(matches!(
            &d.controls()[0],
            TenjikuControl::Observation { index: 0 }
        ));
        assert!(matches!(
            &d.controls()[1],
            TenjikuControl::Native {
                command: c::GameCommand::Play { .. },
                ..
            }
        ));
        assert_eq!(decoded(&d).record(), d.record());
        assert_eq!(decoded(&d).observations(), d.observations());
        for forgery in 0..4 {
            let mut payload = d.payload.clone();
            match forgery {
                0 => {
                    payload.controls.remove(1);
                }
                1 => payload.controls.swap(0, 1),
                2 => {
                    if let TenjikuControl::Native { actor, .. } = &mut payload.controls[1] {
                        *actor = actor.opponent()
                    }
                }
                _ => {
                    if let ExecutionResult::Tenjiku {
                        result: mshogi_engine::TenjikuResult::Choice { telemetry, .. },
                    } = &mut payload.observations[0].result
                    {
                        telemetry.score = Some(123)
                    }
                }
            }
            assert!(
                validate(payload, &mut || false).is_err(),
                "forgery {forgery}"
            );
        }
        let mut analysis = observation;
        analysis.context.purpose = ExecutionPurpose::MoveAnalysis {
            target: ExecutionTarget {
                selection_id: 1,
                event_index: 0,
                board_ply: 0,
                session_version: 1,
                label: "Imported root, read-only".into(),
                turn: NativeSide::Tenjiku(c::Side::Sente),
            },
        };
        let d = baseline.with_observation(analysis, &mut || false).unwrap();
        assert_eq!(d.record(), baseline.record());
        assert_eq!(d.controls().len(), 1);
        assert_eq!(decoded(&d).record(), baseline.record());
        assert_eq!(baseline.record().event_count(), 0);
    }
}
