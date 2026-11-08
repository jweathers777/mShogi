//! Both immutable Tenjiku identities reach actual native execution with exact context guards.
use mshogi_core::{native::*, tenjiku as t};
use mshogi_engine::*;
use serde_json::{Value, json};
use std::io::Cursor;

fn record(profile: t::Profile) -> CanonicalRecord {
    let entries = [
        (16, 16, t::Side::Sente, t::Base::K),
        (15, 14, t::Side::Sente, t::Base::P),
        (1, 1, t::Side::Gote, t::Base::K),
        (2, 3, t::Side::Gote, t::Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (file, rank, side, base))| {
        (
            t::Square::new(file, rank).unwrap(),
            t::Piece::new(id as u16, side, base, false).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    let game = t::Game::from_root(
        t::Position::from_diagram(
            profile,
            &entries,
            t::Side::Sente,
            t::RootMetadata::imported("execution-tenjiku-fixture".into(), 120).unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    CanonicalRecord::tenjiku(t::GameRecord::from_game(&game).unwrap())
}

fn context(profile: t::Profile) -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 129,
        session_version: 7,
        process_generation: 3,
        game: record(profile),
        limits: WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 512,
        },
    }
}

fn target(context: &ExecutionContext) -> ExecutionTarget {
    ExecutionTarget {
        selection_id: 3,
        event_index: context.game.event_count() as u64,
        board_ply: context.game.known_board_actions() as u64,
        session_version: context.session_version,
        label: "Exact native Tenjiku root".into(),
        turn: NativeSide::Tenjiku(t::Side::Sente),
    }
}

fn request(context: ExecutionContext) -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context,
    }
}

fn run(context: &ExecutionContext) -> ExecutionResponse {
    let mut bytes = vec![];
    serve_once(
        Cursor::new(format!(
            "{}\n",
            serde_json::to_string(&request(context.clone())).unwrap()
        )),
        &mut bytes,
    )
    .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

fn assert_failure(response: &ExecutionResponse, kind: FailureKind) {
    let ExecutionResult::Failure { failure } = &response.observation.result else {
        panic!("unsupported execution published a native result: {response:?}")
    };
    assert_eq!(failure.kind, kind);
    let wire = serde_json::to_value(&response.observation.result).unwrap();
    assert_eq!(wire.as_object().unwrap().len(), 2);
    assert_eq!(wire["kind"], "failure");
    assert_eq!(wire["failure"].as_object().unwrap().len(), 2);
    assert!(wire.get("score").is_none());
    assert!(wire.get("action").is_none());
    assert!(wire.get("principal_variation").is_none());
}

#[test]
fn actual_internal_entry_point_searches_each_profile_and_rejects_chess_full() {
    assert_eq!(
        execution_capabilities(GameKind::Tenjiku),
        &[
            ExecutionOperation::EngineTurn,
            ExecutionOperation::MoveAnalysis
        ]
    );
    for profile in t::Profile::ALL {
        let mut c = context(profile);
        let selection = target(&c);
        for purpose in [
            ExecutionPurpose::EngineTurn {},
            ExecutionPurpose::MoveAnalysis {
                target: selection.clone(),
            },
            ExecutionPurpose::ChessFullReachability {
                target: selection.clone(),
                mate: ReachabilityTarget::Either,
                limits: FullLimits {
                    max_positions: 1,
                    max_transitions: 0,
                },
            },
        ] {
            c.limits = if matches!(purpose, ExecutionPurpose::ChessFullReachability { .. }) {
                FULL_SENTINEL
            } else {
                WorkLimits {
                    time_ms: None,
                    depth: 1,
                    nodes: 512,
                }
            };
            c.purpose = purpose;
            assert!(c.valid_limits());
            let response = run(&c);
            assert!(response.matches(&request(c.clone())));
            assert_eq!(response.observation.context, c);
            assert_eq!(
                response.observation.context.game.profile(),
                RulesProfile::from(profile)
            );
            if matches!(c.purpose, ExecutionPurpose::ChessFullReachability { .. }) {
                assert_failure(&response, FailureKind::UnsupportedMode);
            } else {
                let ExecutionResult::Tenjiku { result } = &response.observation.result else {
                    panic!("{response:?}")
                };
                let CanonicalRecord::Tenjiku(record) = &c.game else {
                    unreachable!()
                };
                result.validate(&record.to_game().unwrap()).unwrap();
                assert!(matches!(result, TenjikuResult::Choice { .. }));
            }
        }
    }
}

#[test]
fn complete_context_mismatch_cannot_accept_a_real_native_observation() {
    for profile in t::Profile::ALL {
        let mut c = context(profile);
        c.purpose = ExecutionPurpose::MoveAnalysis { target: target(&c) };
        let response = run(&c);
        for field in 0..10 {
            let mut wrong = c.clone();
            match field {
                0 => wrong.request_id += 1,
                1 => wrong.session_version += 1,
                2 => wrong.process_generation += 1,
                3 => wrong.limits.nodes += 1,
                4 => wrong.purpose = ExecutionPurpose::EngineTurn {},
                5 => {
                    wrong.game = record(match profile {
                        t::Profile::AdamsModern => t::Profile::MullerCompleted,
                        t::Profile::MullerCompleted => t::Profile::AdamsModern,
                    });
                }
                _ => {
                    let ExecutionPurpose::MoveAnalysis { target } = &mut wrong.purpose else {
                        unreachable!()
                    };
                    match field {
                        6 => target.selection_id += 1,
                        7 => target.label.push_str(" changed"),
                        8 => target.event_index += 1,
                        _ => target.board_ply += 1,
                    }
                }
            }
            assert!(!response.matches(&request(wrong)), "mismatch field {field}");
        }
    }
}

#[test]
fn game_qualified_turn_history_and_limits_are_checked_before_execution() {
    for profile in t::Profile::ALL {
        for invalid in 0..5 {
            let mut c = context(profile);
            let mut selection = target(&c);
            match invalid {
                0 => selection.turn = NativeSide::Shogi(mshogi_core::shogi::Side::Sente),
                1 => selection.turn = NativeSide::Tenjiku(t::Side::Gote),
                2 => selection.event_index = 1,
                3 => selection.board_ply = 120,
                _ => c.limits.depth = 0,
            }
            c.purpose = ExecutionPurpose::MoveAnalysis { target: selection };
            assert!(!c.valid_limits());
            assert_failure(&run(&c), FailureKind::InvalidRequest);
        }
    }
}

#[test]
fn raw_observation_parser_preserves_strict_native_context_and_cooperative_stop() {
    for profile in t::Profile::ALL {
        let mut c = context(profile);
        c.purpose = ExecutionPurpose::MoveAnalysis { target: target(&c) };
        let observation = run(&c).observation;
        let raw = serde_json::to_string(&observation).unwrap();
        let mut polls = 0;
        assert_eq!(
            ExecutionObservation::from_json_with_stop(&raw, &mut || {
                polls += 1;
                false
            })
            .unwrap(),
            observation,
        );
        assert!(polls > 20);
        for after in [1, 20, polls / 2, polls - 1] {
            let mut count = 0;
            assert_eq!(
                ExecutionObservation::from_json_with_stop(&raw, &mut || {
                    count += 1;
                    count >= after
                })
                .unwrap_err(),
                "Execution observation parsing stopped",
            );
            assert_eq!(count, after);
        }
        for key in [
            "\"selection_id\":3",
            "\"completed_depth\":1",
            "\"promoted\":false",
        ] {
            let duplicate = raw.replacen(key, &format!("{key},{key}"), 1);
            assert_ne!(duplicate, raw);
            assert!(ExecutionObservation::from_json_with_stop(&duplicate, &mut || false).is_err());
        }
        for (field, value) in [("schema_version", json!(2)), ("unrecognized", json!(true))] {
            let mut invalid: Value = serde_json::from_str(&raw).unwrap();
            invalid[field] = value;
            assert!(
                ExecutionObservation::from_json_with_stop(&invalid.to_string(), &mut || false)
                    .is_err()
            );
        }
        let mut invalid: Value = serde_json::from_str(&raw).unwrap();
        let other = match profile {
            t::Profile::AdamsModern => t::Profile::MullerCompleted,
            t::Profile::MullerCompleted => t::Profile::AdamsModern,
        };
        invalid["context"]["game"]["rules_profile"] = json!(other.id());
        invalid["context"]["game"]["profile_edition"] = json!(other.edition());
        assert!(
            ExecutionObservation::from_json_with_stop(&invalid.to_string(), &mut || false).is_err()
        );
        invalid = serde_json::from_str(&raw).unwrap();
        invalid["result"] = json!({"kind":"tenjiku","result":{"kind":"choice","score":0}});
        assert!(
            ExecutionObservation::from_json_with_stop(&invalid.to_string(), &mut || false).is_err()
        );
        assert!(observation.matches(&c));
    }
}
