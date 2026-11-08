use super::*;
fn config() -> Config {
    // Only configuration/replay tests; no child is launched with this executable.
    template(std::env::current_exe().unwrap().to_str().unwrap()).unwrap()
}
#[test]
fn schedule_is_seeded_complete_paired_and_multiple_baseline() {
    let mut c = config();
    c.rounds = 2;
    let a = c.schedule();
    assert_eq!(a, c.schedule());
    assert_eq!(a.len(), 56);
    for pair in a.as_chunks::<2>().0 {
        assert!(!pair[0].swapped);
        assert!(pair[1].swapped);
        let mut reversed = pair[0].clone();
        reversed.swapped = true;
        assert_eq!(reversed, pair[1]);
    }
    for cycle in 0..2 {
        for opening in 0..7 {
            for baseline in 1..3 {
                assert_eq!(
                    a.iter()
                        .filter(|p| p.cycle == cycle
                            && p.opening == opening
                            && p.baseline == baseline)
                        .count(),
                    2
                );
            }
        }
    }
    c.seed += 1;
    assert_ne!(a, c.schedule());
}
#[test]
fn strict_effective_limits_and_bounded_aggregate_work() {
    let c = config();
    for field in ["model", "options", "args", "search_seed"] {
        let mut value = serde_json::to_value(&c).unwrap();
        value["players"][0][field] = serde_json::json!("unsupported");
        assert!(Config::from_json(&value.to_string()).is_err(), "{field}");
    }
    let mut bad = c.clone();
    bad.players[0].limits[6].work.depth = 3;
    assert!(bad.validate().is_err());
    let mut bad = c.clone();
    bad.players[0].limits[2].work.nodes = 513;
    assert!(bad.validate().is_err());
    let mut bad = c.clone();
    bad.max_steps = 512;
    assert!(bad.validate().is_err());
    let mut bad = c.clone();
    bad.players[0].limits.remove(0);
    assert!(bad.validate().is_err());
}
#[test]
fn grouped_roots_cannot_leak_across_dataset_splits() {
    let mut c = config();
    let mut same = c.openings[0].clone();
    same.id = "duplicate".into();
    same.group = "different-declaration".into();
    same.split = Split::Train;
    c.openings.push(same);
    assert!(c.validate().is_err());
    c.openings.pop();
    c.openings[1].group = c.openings[0].group.clone();
    c.openings[1].split = Split::Train;
    assert!(c.validate().is_err());
    // Native descendants retain the same initial lineage even when current
    // position, commands, counters and outcome differ.
    let root = &c.openings[2].root;
    let mut game = match &root.record {
        mshogi_core::native::CanonicalRecord::Xiangqi(r) => r.to_game().unwrap(),
        _ => unreachable!(),
    };
    let action = game.position().legal_actions().unwrap()[0];
    game.submit(mshogi_core::xiangqi::GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: mshogi_core::xiangqi::GameCommand::Play { action },
    })
    .unwrap();
    let later = fresh_root(mshogi_core::native::CanonicalRecord::Xiangqi(Box::new(
        mshogi_core::xiangqi::GameRecord::from_game(&game).unwrap(),
    )))
    .unwrap();
    assert_eq!(lineage_key(root).unwrap(), lineage_key(&later).unwrap());
}
#[test]
fn cancellation_has_full_schedule_no_scores_or_generated_training_labels() {
    let c = config();
    let r = run_with_stop(&c, &mut || true).unwrap();
    assert_eq!(r.games.len(), 28);
    assert!(r.games.iter().all(|g| g.steps.is_empty()
        && g.stop == "cancelled"
        && g.score_first_twice.is_none()
        && g.cleanup.is_empty()));
    assert_eq!(r.data_jsonl().unwrap(), "");
    assert!(r.summaries().unwrap().iter().all(|s| s.unscored_pairs == 1
        && s.unscored_games == 2
        && s.complete_played_pairs == 0
        && s.pentanomial == [0; 5]));
    let encoded = r.to_json().unwrap();
    Report::from_json(&encoded).unwrap();
    let mut wrong = r.clone();
    wrong.games[0].score_first_twice = Some(1);
    assert!(wrong.validate().is_err());
    let mut wrong = r.clone();
    wrong.games[0].pairing.swapped = true;
    assert!(wrong.validate().is_err());
    let mut wrong = r.clone();
    wrong.config.seed += 1;
    assert!(wrong.validate().is_err());
    let mut wrong = r.clone();
    wrong.identities[0].model_identity = Some("imaginary-model".into());
    assert!(wrong.validate().is_err());
    let mut wrong = r.clone();
    wrong.games.pop();
    assert!(wrong.validate().is_err());
}
fn chess_move_result(pv: &[&str]) -> mshogi_engine::ExecutionResult {
    let choices: Vec<_> = pv
        .iter()
        .map(|s| mshogi_engine::EngineChoice::Move {
            action: s.parse().unwrap(),
        })
        .collect();
    let mshogi_engine::EngineChoice::Move { action } = choices[0] else {
        unreachable!()
    };
    mshogi_engine::ExecutionResult::Chess {
        result: mshogi_engine::ChessResult::Move {
            action,
            telemetry: mshogi_engine::Telemetry {
                score: Some(0),
                completed_depth: pv.len() as u16,
                nodes: 100,
                principal_variation: choices,
                status: mshogi_engine::StopReason::Complete,
            },
        },
    }
}
#[test]
fn chess_corrupt_pv_incomplete_and_scores_are_not_accepted() {
    let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
    for result in [
        chess_move_result(&["e2e4", "e2e5"]),
        mshogi_engine::ExecutionResult::Chess {
            result: mshogi_engine::ChessResult::Incomplete {
                telemetry: mshogi_engine::Telemetry {
                    score: Some(0),
                    completed_depth: 1,
                    nodes: 1,
                    principal_variation: vec![],
                    status: mshogi_engine::StopReason::Complete,
                },
            },
        },
    ] {
        let mut state = native::NativeState::from_root(&root).unwrap();
        assert!(
            admit_bounded(
                &mut state,
                &result,
                WorkLimits {
                    depth: 2,
                    nodes: 512
                }
            )
            .is_err()
        );
        assert_eq!(state.root().unwrap(), root);
    }
    let mut result = chess_move_result(&["e2e4"]);
    if let mshogi_engine::ExecutionResult::Chess {
        result: mshogi_engine::ChessResult::Move { telemetry, .. },
    } = &mut result
    {
        telemetry.score = Some(i32::MAX);
    }
    let mut state = native::NativeState::from_root(&root).unwrap();
    assert!(
        admit_bounded(
            &mut state,
            &result,
            WorkLimits {
                depth: 1,
                nodes: 512
            }
        )
        .is_err()
    );
}
#[test]
fn capacity_crossing_refuses_transition_atomically() {
    let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
    let mut state = native::NativeState::from_root(&root).unwrap();
    // Controlled smaller capacity uses the exact production refusal path,
    // without thousands of repeated native MatchObserve admissions.
    let capacity = root.chess_match.as_ref().unwrap().len();
    let e = admit_with_match_capacity(
        &mut state,
        &chess_move_result(&["e2e4"]),
        WorkLimits {
            depth: 1,
            nodes: 512,
        },
        capacity,
    )
    .unwrap_err();
    assert!(
        e.contains("capacity") || e.contains("transition evidence"),
        "{e}"
    );
    assert_eq!(state.root().unwrap(), root);
}
#[test]
fn observation_decoder_keeps_duplicates_and_wire_size_visible() {
    let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
    let context = ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 1,
        session_version: 1,
        process_generation: 1,
        game: root.record,
        limits: WorkLimits {
            depth: 1,
            nodes: 512,
        },
    };
    let step = Step {
        player: 0,
        elapsed_ns: 0,
        request: context.clone(),
        observation: Some(ExecutionObservation::new(
            context,
            chess_move_result(&["e2e4"]),
        )),
        failure: None,
        applied: true,
    };
    let text = serde_json::to_string(&step).unwrap();
    serde_json::from_str::<Step>(&text).unwrap();
    let duplicate = text.replacen(
        "\"schema_version\":1",
        "\"schema_version\":1,\"schema_version\":1",
        1,
    );
    assert!(serde_json::from_str::<Step>(&duplicate).is_err());
    let oversized = text.replacen(
        "\"observation\":{",
        &format!(
            "\"observation\":{{{}",
            " ".repeat(mshogi_engine::MAX_RESPONSE_BYTES)
        ),
        1,
    );
    assert!(serde_json::from_str::<Step>(&oversized).is_err());
}
#[test]
fn shogi_and_xiangqi_reject_illegal_later_pv_commands_atomically() {
    use mshogi_engine::*;
    let root = standard_root(RulesProfile::ShogiCsa36V1).unwrap();
    let first = ShogiChoice::Move {
        action: "7g7f".parse().unwrap(),
    };
    let invalid = ShogiChoice::Move {
        action: "7f7e".parse().unwrap(),
    };
    let result = ExecutionResult::Shogi {
        result: ShogiResult::Choice {
            choice: first,
            telemetry: ShogiTelemetry {
                evaluator_id: mshogi_search::shogi::EVALUATOR_ID.into(),
                search_id: mshogi_search::shogi::SEARCH_ID.into(),
                perspective: mshogi_core::shogi::Side::Sente,
                score: Some(0),
                completed_depth: 2,
                nodes: 100,
                principal_variation: vec![first, invalid],
                status: StopReason::Complete,
            },
        },
    };
    let mut state = native::NativeState::from_root(&root).unwrap();
    assert!(
        admit_bounded(
            &mut state,
            &result,
            WorkLimits {
                depth: 2,
                nodes: 512
            }
        )
        .is_err()
    );
    assert_eq!(state.root().unwrap(), root);
    let root = standard_root(RulesProfile::XiangqiWxf2018V1).unwrap();
    let first = XiangqiChoice::Move {
        action: "a0a1".parse().unwrap(),
    };
    let invalid = XiangqiChoice::Move {
        action: "a1a2".parse().unwrap(),
    };
    let no_win_assessment = match &root.record {
        mshogi_core::native::CanonicalRecord::Xiangqi(r) => r.no_win_assessment().clone(),
        _ => unreachable!(),
    };
    let result = ExecutionResult::Xiangqi {
        result: XiangqiResult::Choice {
            choice: first,
            telemetry: XiangqiTelemetry {
                evaluator_id: mshogi_search::xiangqi::EVALUATOR_ID.into(),
                search_id: mshogi_search::xiangqi::SEARCH_ID.into(),
                perspective: mshogi_core::xiangqi::Side::Red,
                score: Some(0),
                completed_depth: 2,
                nodes: 100,
                principal_variation: vec![first, invalid],
                status: XiangqiStop::Complete,
                no_win_assessment,
            },
        },
    };
    let mut state = native::NativeState::from_root(&root).unwrap();
    assert!(
        admit_bounded(
            &mut state,
            &result,
            WorkLimits {
                depth: 2,
                nodes: 512
            }
        )
        .is_err()
    );
    assert_eq!(state.root().unwrap(), root);
}

#[test]
fn terminal_claim_allowance_does_not_admit_extra_board_depth() {
    let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
    let mut state = native::NativeState::from_root(&root).unwrap();
    let mut result = chess_move_result(&["e2e4", "e7e5"]);
    if let mshogi_engine::ExecutionResult::Chess {
        result: mshogi_engine::ChessResult::Move { telemetry, .. },
    } = &mut result
    {
        telemetry.completed_depth = 1;
    }
    assert!(
        admit_bounded(
            &mut state,
            &result,
            WorkLimits {
                depth: 1,
                nodes: 512
            }
        )
        .is_err()
    );
    assert_eq!(state.root().unwrap(), root);
}
