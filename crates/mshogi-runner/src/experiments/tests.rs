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
    bad.players[0].limits[6].work.depth = 65;
    assert!(bad.validate().is_err());
    let mut bad = c.clone();
    bad.players[0].limits[2].work.nodes = mshogi_search::xiangqi::MAX_NODES + 1;
    assert!(bad.validate().is_err());
    let mut bad = c.clone();
    bad.wall_budget_ms = u64::MAX;
    bad.max_steps = usize::MAX;
    bad.validate().unwrap();
    bad.wall_budget_ms = 0;
    assert!(bad.validate().is_err());
    let mut bad = c.clone();
    bad.players[0].limits.remove(0);
    assert!(bad.validate().is_err());
    let mut bad = c;
    bad.players[0].limits[0].work.time_ms = Some(1_000);
    assert!(
        bad.validate().is_err(),
        "timed work cannot claim deterministic native reruns"
    );
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
    let checked = Report::from_json(&encoded).unwrap();
    assert_eq!(checked.to_json().unwrap(), encoded);
    assert_eq!(
        serde_json::to_value(r.summaries().unwrap()).unwrap(),
        serde_json::to_value(checked.summaries().unwrap()).unwrap()
    );
    assert_eq!(r.data_jsonl().unwrap(), checked.data_jsonl().unwrap());
    let imported = ValidatedReport::from_json(&encoded).unwrap();
    assert_eq!(imported.to_json().unwrap(), encoded);
    let mut editable = imported.into_report();
    editable.games[0].score_first_twice = Some(1);
    assert!(editable.to_json().is_err());
    assert!(editable.summaries().is_err());
    assert!(editable.data_jsonl().is_err());
    let forged = serde_json::to_string(&editable).unwrap();
    assert!(ValidatedReport::from_json(&forged).is_err());
    assert!(editable.into_validated().is_err());
    // Editing an owned copy cannot change the evidence covered by the wrapper.
    assert_eq!(r.to_json().unwrap(), encoded);
    assert!(ValidatedReport::from_json("{\"schema\":1,\"schema\":1}").is_err());
    let mut wrong = r.as_report().clone();
    wrong.games[0].score_first_twice = Some(1);
    assert!(wrong.validate().is_err());
    let mut wrong = r.as_report().clone();
    wrong.games[0].pairing.swapped = true;
    assert!(wrong.validate().is_err());
    let mut wrong = r.as_report().clone();
    wrong.config.seed += 1;
    assert!(wrong.validate().is_err());
    let mut wrong = r.as_report().clone();
    wrong.identities[0].model_identity = Some("imaginary-model".into());
    assert!(wrong.validate().is_err());
    let mut wrong = r.as_report().clone();
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
                selective_depth: pv.len() as u16,
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
                    selective_depth: 1,
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
                    time_ms: None,
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
                time_ms: None,
                depth: 1,
                nodes: 512
            }
        )
        .is_err()
    );
}
#[test]
fn observation_decoder_keeps_duplicates_visible() {
    let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
    let context = ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 1,
        session_version: 1,
        game: root.record,
        limits: WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 512,
        },
    };
    let step = Step {
        clock: None,
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
                time_ms: None,
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
                time_ms: None,
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
        telemetry.selective_depth = 1;
    }
    assert!(
        admit_bounded(
            &mut state,
            &result,
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 512
            }
        )
        .is_err()
    );
    assert_eq!(state.root().unwrap(), root);
}

#[test]
fn fischer_allocation_charge_and_replay_refuse_tampering() {
    let clock = FischerClock {
        base_ms: 10_000,
        increment_ms: 100,
    };
    let work = WorkLimits {
        depth: 4,
        nodes: 512,
        time_ms: None,
    };
    // Normal share: 9950 / 30 + 75 = 406; first-request transport reserve
    // min(300, 406 / 2) = 203, then 50.
    assert_eq!(
        clock_work(work, Some(clock), 10_000, true).time_ms,
        Some(203)
    );
    assert_eq!(
        clock_work(work, Some(clock), 10_000, false).time_ms,
        Some(356)
    );
    assert_eq!(
        clock_receipt(clock, 10_000, 1_000_001, true).after_ms,
        10_098
    );
    assert_eq!(
        clock_receipt(clock, 10_000, 1_000_001, false).after_ms,
        9_998
    );
    let mut c = config();
    c.openings.truncate(1);
    c.players.truncate(2);
    c.clock = Some(clock);
    let mut report = run_with_stop(&c, &mut || true).unwrap().as_report().clone();
    let player = 0;
    let request = ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 1,
        session_version: 1,
        game: c.openings[0].root.record.clone(),
        limits: clock_work(
            c.players[player]
                .work(c.openings[0].root.record.profile())
                .unwrap(),
            Some(clock),
            10_000,
            true,
        ),
    };
    report.games[0].steps.push(Step {
        player,
        elapsed_ns: 1_000_001,
        request,
        clock: Some(clock_receipt(clock, 10_000, 1_000_001, false)),
        observation: None,
        failure: Some("cancelled".into()),
        applied: false,
    });
    report.validate().unwrap();
    let mut bad = report.clone();
    bad.games[0].steps[0].clock.as_mut().unwrap().after_ms += 1;
    assert!(bad.validate().is_err());
    let mut bad = report.clone();
    bad.games[0].steps[0].request.limits.time_ms = Some(999);
    assert!(bad.validate().is_err());
    let mut bad = report.clone();
    bad.games[0].steps[0].elapsed_ns += 1_000_000;
    assert!(bad.validate().is_err());
    let step = &mut report.games[0].steps[0];
    step.elapsed_ns = 10_000_000_000;
    step.clock = Some(clock_receipt(clock, 10_000, step.elapsed_ns, false));
    step.failure = Some("clock_limit".into());
    report.games[0].stop = "clock_limit".into();
    report.validate().unwrap();
    assert_eq!(report.games[0].score_first_twice, None);
    let step = &mut report.games[0].steps[0];
    step.observation = Some(ExecutionObservation::new(
        step.request.clone(),
        chess_move_result(&["e2e4"]),
    ));
    report.validate().unwrap();
    report.games[0].steps[0].applied = true;
    assert!(report.validate().is_err());
}

#[test]
fn successful_shogi_clock_steps_replay_both_players_and_colors() {
    use mshogi_engine::{ExecutionResult, ShogiChoice, ShogiResult, ShogiTelemetry, StopReason};
    let mut c = config();
    c.openings
        .retain(|o| o.root.record.profile() == RulesProfile::ShogiCsa36V1);
    c.players.truncate(2);
    c.max_steps = 4;
    c.clock = Some(FischerClock {
        base_ms: 10_000,
        increment_ms: 100,
    });
    let mut report = run_with_stop(&c, &mut || true).unwrap().into_report();
    let mut request_id = 0;
    for game in &mut report.games {
        let mut state = native::NativeState::from_root(&c.openings[0].root).unwrap();
        for (i, token) in ["7g7f", "3c3d", "2g2f", "8c8d"].iter().enumerate() {
            let first = i % 2 == 0;
            let player = usize::from(first == game.pairing.swapped);
            // Both engines spend 25.000001 ms on their first turn, then 50 ms.
            // Ceil charging and increment give 10074 then 10124 ms each.
            let (before_ms, elapsed_ns, after_ms) = if i < 2 {
                (10_000, 25_000_001, 10_074)
            } else {
                (10_074, 50_000_000, 10_124)
            };
            let limits = clock_work(
                c.players[player].work(RulesProfile::ShogiCsa36V1).unwrap(),
                c.clock,
                before_ms,
                i < 2,
            );
            request_id += 1;
            let context = ExecutionContext {
                purpose: ExecutionPurpose::EngineTurn {},
                request_id,
                session_version: request_id,
                game: state.root().unwrap().record,
                limits,
            };
            let choice = ShogiChoice::Move {
                action: token.parse().unwrap(),
            };
            let result = ExecutionResult::Shogi {
                result: ShogiResult::Choice {
                    choice,
                    telemetry: ShogiTelemetry {
                        evaluator_id: mshogi_search::shogi::EVALUATOR_ID.into(),
                        search_id: mshogi_search::shogi::SEARCH_ID.into(),
                        perspective: if first {
                            mshogi_core::shogi::Side::Sente
                        } else {
                            mshogi_core::shogi::Side::Gote
                        },
                        score: Some(0),
                        completed_depth: 1,
                        nodes: 1,
                        principal_variation: vec![choice],
                        status: StopReason::Complete,
                    },
                },
            };
            assert!(matches!(
                admit_bounded(&mut state, &result, limits).unwrap(),
                native::Admission::Applied
            ));
            game.steps.push(Step {
                player,
                elapsed_ns,
                clock: Some(ClockReceipt {
                    before_ms,
                    charged_ms: elapsed_ns.div_ceil(1_000_000),
                    after_ms,
                }),
                request: context.clone(),
                observation: Some(ExecutionObservation::new(context, result)),
                failure: None,
                applied: true,
            });
        }
        game.final_root = state.root().unwrap();
        game.stop = "step_limit".into();
    }
    report.validate().unwrap();
    assert!(report.games.iter().all(|g| g.score_first_twice.is_none()));
    // Time odds (MSH-204): replay uses each participant's own clock, so the
    // same steps under a doubled clock for player 0 no longer replay.
    let mut odds = report.clone();
    let shared = odds.config.clock.unwrap();
    odds.config.players[0].clock = Some(FischerClock {
        base_ms: shared.base_ms * 2,
        increment_ms: shared.increment_ms * 2,
    });
    odds.config_sha256 = digest(&serde_json::to_string(&odds.config).unwrap());
    odds.config.validate().unwrap();
    assert!(odds.validate().is_err());
    // A participant clock needs the shared clock.
    let mut c = odds.config.clone();
    c.clock = None;
    assert!(c.validate().is_err());
    assert_eq!(odds.config.player_clock(1).unwrap().base_ms, shared.base_ms);
    // Encoded steps omit the replay-determined native record (MSH-204); the
    // decoder restores it by native replay and the result is identical.
    let text = report.to_json().unwrap();
    let encoded: serde_json::Value = serde_json::from_str(&text).unwrap();
    for step in encoded["games"][0]["steps"].as_array().unwrap() {
        assert!(step["request"].get("game").is_none());
        assert!(step["observation"]["context"].get("game").is_none());
    }
    let decoded = Report::from_json(&text).unwrap();
    assert_eq!(decoded.to_json().unwrap(), text);
    for (a, b) in decoded.games.iter().zip(&report.games) {
        for (x, y) in a.steps.iter().zip(&b.steps) {
            assert_eq!(x.request, y.request);
        }
    }
    // A full (unelided) record is still accepted and checked against replay.
    let full = serde_json::to_string(&report).unwrap();
    assert!(Report::from_json(&full).is_ok());
    let mut tampered: serde_json::Value = serde_json::from_str(&full).unwrap();
    tampered["games"][0]["steps"][1]["request"]["game"] =
        tampered["games"][0]["steps"][0]["request"]["game"].clone();
    assert!(Report::from_json(&tampered.to_string()).is_err());
    // A record kept only in the observation is refused: a record is either
    // omitted in both places or present in both.
    let mut mixed = encoded.clone();
    mixed["games"][0]["steps"][0]["observation"]["context"]["game"] = serde_json::from_str::<
        serde_json::Value,
    >(&full)
    .unwrap()["games"][0]["steps"][0]["request"]["game"]
        .clone();
    assert!(Report::from_json(&mixed.to_string()).is_err());
    // An explicit null record in an elided observation is not the canonical form.
    let mut null = encoded.clone();
    null["games"][0]["steps"][0]["observation"]["context"]["game"] = serde_json::Value::Null;
    assert!(Report::from_json(&null.to_string()).is_err());
    // An elided step after a changed observation replays a different history.
    let mut changed = encoded.clone();
    changed["games"][0]["steps"][0]["applied"] = serde_json::json!(false);
    assert!(Report::from_json(&changed.to_string()).is_err());
    let mut bad = report.clone();
    bad.games[1].steps[2].clock.as_mut().unwrap().before_ms = 10_000;
    assert!(bad.validate().is_err());
    let mut bad = report;
    bad.games[0].steps[0].clock.as_mut().unwrap().after_ms -= 100;
    assert!(bad.validate().is_err());
}

#[test]
fn clock_control_refuses_watchdog_below_possible_allocation() {
    let mut c = config();
    c.clock = Some(FischerClock {
        base_ms: 600_000,
        increment_ms: 0,
    });
    // The largest share is (600_000 - 50) / 30 = 19_998.
    c.watchdog_ms = 19_997;
    assert!(c.validate().unwrap_err().to_string().contains("watchdog"));
    c.watchdog_ms = 19_998;
    c.validate().unwrap();
}

#[test]
fn native_report_decoder_accepts_evidence_above_old_32_mib_limit() {
    let mut c = config();
    c.openings.truncate(1);
    c.players.truncate(2);
    let report = run_with_stop(&c, &mut || true).unwrap();
    let mut text = report.to_json().unwrap();
    // Legal insignificant whitespace exercises the actual decoder boundary
    // without manufacturing thousands of moves or altering native history.
    text.extend(std::iter::repeat_n(' ', 33 * 1024 * 1024 - text.len()));
    let decoded = ValidatedReport::from_json(&text).unwrap();
    assert_eq!(decoded.config_sha256, report.config_sha256);
    assert_eq!(decoded.games.len(), 2);
    // Raising the evidence ceiling must not admit fabricated native results.
    let mut bad = decoded.into_report();
    bad.games[0].score_first_twice = Some(2);
    assert!(bad.validate().is_err());
}
