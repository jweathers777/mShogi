use super::*;
use serde_json::{Value, json};
fn standard(profile: c::Profile) -> ValidatedTenjikuDocument {
    ValidatedTenjikuDocument::new(
        c::GameRecord::from_game(&c::Game::standard(profile)).unwrap(),
        c::Side::Sente,
    )
    .unwrap()
}
// Administrative capacity/ending semantics do not require a dense initial
// board. Keep real native roots while avoiding repeated dense-board legality
// work in exhaustive ledger tests; full64/128 histories are measured below.
fn administrative_root(profile: c::Profile) -> ValidatedTenjikuDocument {
    let entries = [
        (16, 16, c::Side::Sente, c::Base::K),
        (15, 14, c::Side::Sente, c::Base::P),
        (1, 1, c::Side::Gote, c::Base::K),
        (2, 3, c::Side::Gote, c::Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (file, rank, side, base))| {
        (
            c::Square::new(file, rank).unwrap(),
            c::Piece::new(id as u16, side, base, false).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    let game = c::Game::from_root(
        c::Position::from_diagram(
            profile,
            &entries,
            c::Side::Sente,
            c::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap();
    ValidatedTenjikuDocument::new(c::GameRecord::from_game(&game).unwrap(), c::Side::Sente).unwrap()
}
fn command(
    d: &ValidatedTenjikuDocument,
    actor: c::Side,
    command: c::GameCommand,
) -> ValidatedTenjikuDocument {
    d.with_control(TenjikuControl::Native { actor, command }, &mut || false)
        .unwrap()
}
fn decoded(d: &ValidatedTenjikuDocument) -> ValidatedTenjikuDocument {
    ValidatedTenjikuDocument::decode(d.bytes(), &mut || false).unwrap()
}
#[test]
fn both_profiles_preserve_controls_exact_selection_settings_and_native_conclusions() {
    for profile in c::Profile::ALL {
        let d = standard(profile);
        assert_eq!(d.profile(), RulesProfile::from(profile));
        let d = command(&d, c::Side::Sente, c::GameCommand::OfferDraw);
        let d = command(&d, c::Side::Gote, c::GameCommand::DeclineDraw);
        let d = d.with_selection(Some(1), &mut || false).unwrap();
        let d = d
            .with_control(
                TenjikuControl::Settings {
                    settings: TenjikuSettings {
                        orientation: c::Side::Gote,
                    },
                },
                &mut || false,
            )
            .unwrap();
        let r = decoded(&d);
        assert_eq!(r.profile(), d.profile());
        assert_eq!(r.record(), d.record());
        assert_eq!(r.selected_event(), Some(1));
        assert_eq!(r.settings(), d.settings());
        assert_eq!(
            r.record()
                .to_game()
                .unwrap()
                .record_prefix(1)
                .unwrap()
                .to_game()
                .unwrap()
                .snapshot()
                .draw_offer,
            Some(c::Side::Sente)
        );
        let d = command(&d, c::Side::Sente, c::GameCommand::OfferDraw);
        let d = command(&d, c::Side::Gote, c::GameCommand::AcceptDraw);
        assert_eq!(d.record().conclusion(), Some(c::Conclusion::Agreement));
        assert_eq!(decoded(&d).record(), d.record());
    }
}
#[test]
fn strict_duplicate_unknown_missing_derived_and_cross_profile_documents_refuse() {
    for profile in c::Profile::ALL {
        let d = command(
            &administrative_root(profile),
            c::Side::Sente,
            c::GameCommand::Resign,
        );
        let before = d.bytes().to_vec();
        let text = String::from_utf8(before.clone()).unwrap();
        assert!(
            ValidatedTenjikuDocument::decode(
                text.replacen("\"format\":", "\"format\":0,\"format\":", 1)
                    .as_bytes(),
                &mut || false
            )
            .is_err()
        );
        let original: Value = serde_json::from_slice(&before).unwrap();
        let other = if profile == c::Profile::AdamsModern {
            c::Profile::MullerCompleted
        } else {
            c::Profile::AdamsModern
        };
        for edit in 0..6 {
            let mut value = original.clone();
            match edit {
                0 => value["rules_profile"] = json!(RulesProfile::from(other)),
                1 => value["profile_edition"] = json!(other.edition()),
                2 => {
                    value["payload"]
                        .as_object_mut()
                        .unwrap()
                        .remove("selected_event");
                }
                3 => value["payload"]["controls"][0]["command"]["victims"] = json!([]),
                4 => {
                    value["payload"]["current"] = serde_json::to_value(CanonicalRecord::tenjiku(
                        c::GameRecord::from_game(&c::Game::standard(other)).unwrap(),
                    ))
                    .unwrap()
                }
                _ => value["payload"]["unknown"] = json!(true),
            }
            assert!(
                ValidatedTenjikuDocument::decode(&serde_json::to_vec(&value).unwrap(), &mut || {
                    false
                })
                .is_err(),
                "accepted mutation{edit}"
            );
        }
        assert_eq!(d.bytes(), before);
        assert_eq!(decoded(&d).record(), d.record());
    }
}
#[test]
fn native_burn_phase_order_and_retired_branch_roundtrip_both_profiles() {
    for profile in c::Profile::ALL {
        let sq = |f, r| c::Square::new(f, r).unwrap();
        let entries = [
            (
                sq(8, 10),
                c::Piece::new(0, c::Side::Sente, c::Base::FiD, false).unwrap(),
            ),
            (
                sq(16, 16),
                c::Piece::new(1, c::Side::Sente, c::Base::K, false).unwrap(),
            ),
            (
                sq(15, 14),
                c::Piece::new(2, c::Side::Sente, c::Base::P, false).unwrap(),
            ),
            (
                sq(9, 8),
                c::Piece::new(3, c::Side::Gote, c::Base::FiD, false).unwrap(),
            ),
            (
                sq(8, 7),
                c::Piece::new(4, c::Side::Gote, c::Base::P, false).unwrap(),
            ),
            (
                sq(1, 1),
                c::Piece::new(5, c::Side::Gote, c::Base::K, false).unwrap(),
            ),
            (
                sq(2, 3),
                c::Piece::new(6, c::Side::Gote, c::Base::P, false).unwrap(),
            ),
        ];
        let game = c::Game::from_root(
            c::Position::from_diagram(
                profile,
                &entries,
                c::Side::Sente,
                c::RootMetadata::diagram(0),
            )
            .unwrap(),
        )
        .unwrap();
        let d =
            ValidatedTenjikuDocument::new(c::GameRecord::from_game(&game).unwrap(), c::Side::Sente)
                .unwrap();
        let moved = command(
            &d,
            c::Side::Sente,
            c::GameCommand::Play {
                action: c::Action::single(sq(8, 10), sq(8, 8)),
            },
        );
        let game = moved.record().to_game().unwrap();
        assert!(game.position().piece(sq(8, 8)).is_none());
        assert_eq!(
            game.position().piece(sq(8, 7)).is_some(),
            profile == c::Profile::MullerCompleted
        );
        let effects = game.events()[0].effects.as_ref().unwrap();
        assert!(matches!(
            effects.phases.last(),
            Some(c::EffectPhase::MoverDies { .. })
        ));
        assert_eq!(decoded(&moved).record(), moved.record());
        let rewind = moved
            .with_control(
                TenjikuControl::Rewind {
                    event_index: 0,
                    reason: RewindKind::TakeBack,
                },
                &mut || false,
            )
            .unwrap();
        assert_eq!(rewind.record(), d.record());
        assert_eq!(rewind.retired_branches().next().unwrap().1, moved.record());
        assert_eq!(
            decoded(&rewind).retired_branches().next().unwrap().1,
            moved.record()
        );
    }
}
#[test]
fn all_eight_actor_endings_early_close_reopen_and_final_close_fit_actual_reserve() {
    for profile in c::Profile::ALL {
        for selected in [None, Some(0)] {
            let d = administrative_root(profile)
                .with_selection(selected, &mut || false)
                .unwrap();
            let maximum = d.usage().encoded_bytes + d.usage().reserved_bytes;
            let early = d.with_reserved_close(MAX_SAFE_ID - 65).unwrap();
            let reopened = decoded(&early);
            for a in [c::Side::Sente, c::Side::Gote] {
                let stopped = command(
                    &reopened,
                    a,
                    c::GameCommand::Stop {
                        reason: c::StopReason::EngineFailure,
                    },
                );
                for b in [c::Side::Sente, c::Side::Gote] {
                    let resigned = command(&stopped, b, c::GameCommand::Resign);
                    for z in [c::Side::Sente, c::Side::Gote] {
                        let x = command(
                            &resigned,
                            z,
                            c::GameCommand::Stop {
                                reason: c::StopReason::EngineFailure,
                            },
                        );
                        let x = x.with_reserved_close(MAX_SAFE_ID - 65).unwrap();
                        assert!(x.bytes().len() <= maximum);
                        assert_eq!(decoded(&x).record(), x.record());
                    }
                }
            }
        }
    }
}
#[test]
fn cancellation_is_typed_and_preserves_complete_accepted_candidate() {
    for profile in c::Profile::ALL {
        let d = administrative_root(profile);
        let before = d.bytes().to_vec();
        assert!(
            d.with_control(
                TenjikuControl::Settings {
                    settings: TenjikuSettings {
                        orientation: c::Side::Gote
                    }
                },
                &mut || true
            )
            .unwrap_err()
            .stopped
        );
        let mut calls = 0;
        let e = ValidatedTenjikuDocument::decode(&before, &mut || {
            calls += 1;
            calls >= 8
        })
        .unwrap_err();
        assert!(e.stopped);
        assert!(e.resource.is_none());
        assert_eq!(d.bytes(), before);
        assert_eq!(decoded(&d).record(), d.record());
    }
}
#[test]
fn control_capacity_and_work_refusal_preserve_actual_current_record_and_close() {
    let d = administrative_root(c::Profile::AdamsModern);
    let mut p = d.payload.clone();
    p.controls = vec![
        TenjikuControl::Settings {
            settings: d.settings().clone()
        };
        MAX_ARCHIVE_CONTROLS - 5
    ];
    let d = validate(p, &mut || false).unwrap();
    let before = d.bytes().to_vec();
    let e = d
        .with_control(
            TenjikuControl::Settings {
                settings: d.settings().clone(),
            },
            &mut || false,
        )
        .unwrap_err();
    assert!(e.resource.is_some());
    assert_eq!(d.bytes(), before);
    let closed = d.with_reserved_close(MAX_SAFE_ID - 65).unwrap();
    assert_eq!(decoded(&closed).record(), d.record());
    let mut total = MAX_TENJIKU_DOCUMENT_WORK;
    let error = charge(&mut total, 1).unwrap_err();
    assert!(error.resource.is_some());
    assert!(!error.stopped);
}
#[path = "execution_tests.rs"]
mod execution;

fn grown(profile: c::Profile, plies: usize, controls_only: bool) -> (c::Game, Vec<TenjikuControl>) {
    let mut game = c::Game::standard(profile);
    let mut controls = vec![];
    let mut seed = 0x87_2026u64;
    for index in 0..plies {
        let (actor, command) = if controls_only {
            if index % 2 == 0 {
                (c::Side::Sente, c::GameCommand::OfferDraw)
            } else {
                (c::Side::Gote, c::GameCommand::DeclineDraw)
            }
        } else {
            let mut choices = game.local_position().legal_actions();
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            choices.sort_by_key(|a| {
                let p = game.position().piece(a.from).unwrap();
                let to = a.to().unwrap();
                let forward = if p.side == c::Side::Sente {
                    to.rank() < a.from.rank()
                } else {
                    to.rank() > a.from.rank()
                };
                let mix =
                    (u64::from(a.from.index() as u16) * 257 + u64::from(to.index() as u16)) ^ seed;
                (
                    !forward,
                    a.promote,
                    p.is_royal(),
                    p.base != c::Base::P,
                    mix,
                    a.clone(),
                )
            });
            let mut accepted = None;
            for action in choices {
                if action.promote {
                    continue;
                }
                let (_, effect) = game.local_position().simulate(&action).unwrap();
                if effect.mover_after.is_none() || effect.captures().next().is_some() {
                    continue;
                }
                let cmd = c::GameCommand::Play { action };
                match game.submit(c::GameRequest {
                    expected: game.context(),
                    actor: effect.actor,
                    command: cmd.clone(),
                }) {
                    Ok(_) => {
                        accepted = Some((effect.actor, cmd));
                        break;
                    }
                    Err(c::GameError::Repetition) => continue,
                    Err(e) => panic!("native workload refused{e}"),
                }
            }
            let (actor, cmd) = accepted.expect("quiet native continuation");
            controls.push(TenjikuControl::Native {
                actor,
                command: cmd,
            });
            continue;
        };
        game.submit(c::GameRequest {
            expected: game.context(),
            actor,
            command: command.clone(),
        })
        .unwrap();
        controls.push(TenjikuControl::Native { actor, command });
    }
    (game, controls)
}
#[test]
#[ignore = "explicit release resource measurement; not an ordinary per-edit regression"]
fn measure_complete_native_histories_imports_branches_selection_and_control_boundary() {
    use std::time::Instant;
    for profile in c::Profile::ALL {
        for (count, controls_only) in [(64, false), (128, false), (1021, true)] {
            let root = standard(profile);
            let root_before = root.bytes().to_vec();
            let native_start = Instant::now();
            let (game, journal) = grown(profile, count, controls_only);
            let native_us = native_start.elapsed().as_micros();
            let mut payload = root.payload.clone();
            payload.current = CanonicalRecord::tenjiku(c::GameRecord::from_game(&game).unwrap());
            payload.controls = journal;
            let begin = Instant::now();
            let candidate = validate(payload, &mut || false);
            let validate_us = begin.elapsed().as_micros();
            if controls_only {
                let error = candidate.unwrap_err();
                assert_work_refusal(&error);
                let record = c::GameRecord::from_game(&game).unwrap();
                let begin = Instant::now();
                let import_error =
                    ValidatedTenjikuDocument::new(record.clone(), c::Side::Sente).unwrap_err();
                assert_work_refusal(&import_error);
                assert_eq!(record, c::GameRecord::from_game(&game).unwrap());
                assert_eq!(game.events().len(), count);
                assert!(game.conclusion().is_none());
                assert_eq!(root.bytes(), root_before);
                assert_eq!(decoded(&root).record(), root.record());
                println!(
                    "{}",
                    json!({"profile":profile.id(),"events":count,"controls_only":true,"native_record_work":game.replay_work(),"native_build_us":native_us,"candidate_accepted":false,"candidate_us":validate_us,"candidate_resource":error.resource,"import_accepted":false,"import_us":begin.elapsed().as_micros(),"import_resource":import_error.resource,"accepted_root_preserved":true,"native_history_preserved":true})
                );
                continue;
            }
            let doc = candidate.unwrap();
            let begin = Instant::now();
            let imported = ValidatedTenjikuDocument::new(doc.record().clone(), c::Side::Sente);
            let import_us = begin.elapsed().as_micros();
            assert!(
                imported.is_ok(),
                "representative complete native import refused"
            );
            let imported_observation = imported.as_ref().ok().map(|d| {
                let begin = Instant::now();
                let result = d.with_observation(
                    execution::execute(execution::context(d.record(), 1)),
                    &mut || false,
                );
                assert!(result.is_ok(), "representative failure observation refused");
                json!({"accepted":result.is_ok(), "elapsed_us":begin.elapsed().as_micros(),
                    "resource":result.as_ref().err().and_then(|e|e.resource.as_ref()),
                    "accepted_usage":result.as_ref().ok().map(|d|d.usage())})
            });
            let begin = Instant::now();
            let decoded = ValidatedTenjikuDocument::decode(doc.bytes(), &mut || false).unwrap();
            let decode_us = begin.elapsed().as_micros();
            let begin = Instant::now();
            let _runtime = decoded.record().to_game_with_stop(&mut || false).unwrap();
            let restore_us = begin.elapsed().as_micros();
            let mut branch = None;
            if count <= 128 {
                let begin = Instant::now();
                let rewound = doc
                    .with_control(
                        TenjikuControl::Rewind {
                            event_index: count * 3 / 4,
                            reason: RewindKind::TakeBack,
                        },
                        &mut || false,
                    )
                    .unwrap();
                let observed = rewound
                    .with_observation(
                        execution::execute(execution::context(rewound.record(), 1)),
                        &mut || false,
                    )
                    .unwrap();
                let selected = observed
                    .with_selection(Some(count / 2), &mut || false)
                    .unwrap();
                let mutation_us = begin.elapsed().as_micros();
                let begin = Instant::now();
                let d = ValidatedTenjikuDocument::decode(selected.bytes(), &mut || false).unwrap();
                let decode_branch_us = begin.elapsed().as_micros();
                let begin = Instant::now();
                let runtime = d.record().to_game_with_stop(&mut || false).unwrap();
                let prefix = runtime
                    .record_prefix_with_stop(count / 2, &mut || false)
                    .unwrap();
                let _view = prefix.to_game_with_stop(&mut || false).unwrap();
                let restore_branch_us = begin.elapsed().as_micros();
                branch = Some(
                    json!({"usage":selected.usage(),"mutation_us":mutation_us,"decode_us":decode_branch_us,"restore_us":restore_branch_us}),
                );
            }
            println!(
                "{}",
                json!({"profile":profile.id(),"events":count,"controls_only":controls_only,"native_build_us":native_us,"native_record_work":game.replay_work(),"document":doc.usage(),"validation_us":validate_us,"decode_us":decode_us,"runtime_restore_us":restore_us,"import_us":import_us,"import_accepted":imported.is_ok(),"import_usage":imported.as_ref().ok().map(|d|d.usage()),"imported_observation":imported_observation,"branch_observation_selection":branch})
            );
        }
    }
}

#[test]
#[ignore = "explicit release measured policy boundary and actual ending journey"]
fn measure_policy_boundary_and_actual_closing_path() {
    use std::time::{Duration, Instant};
    for profile in c::Profile::ALL {
        let (game, _) = grown(profile, 128, false);
        let base =
            ValidatedTenjikuDocument::new(c::GameRecord::from_game(&game).unwrap(), c::Side::Sente)
                .unwrap();
        let observation = execution::execute(execution::context(base.record(), 1));
        let original = validate(repeated_contexts(&base, &observation, 8), &mut || false).unwrap();
        let maximum_bytes = original.usage().encoded_bytes + original.usage().reserved_bytes;
        let begin = Instant::now();
        let restored = ValidatedTenjikuDocument::decode(original.bytes(), &mut || false).unwrap();
        let decode_us = begin.elapsed().as_micros();
        let begin = Instant::now();
        let _runtime = restored.record().to_game_with_stop(&mut || false).unwrap();
        let restore_us = begin.elapsed().as_micros();
        let begin = Instant::now();
        let mut ended = restored.with_reserved_close(MAX_SAFE_ID - 65).unwrap();
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
            ended = self::command(&ended, actor, command);
        }
        let ended = ended.with_reserved_close(MAX_SAFE_ID - 65).unwrap();
        let closing_us = begin.elapsed().as_micros();
        assert!(ended.bytes().len() <= maximum_bytes);
        assert_eq!(ended.record().event_count(), 131);
        let begin = Instant::now();
        let deadline = begin + Duration::from_secs(15);
        let checked =
            ValidatedTenjikuDocument::decode(ended.bytes(), &mut || Instant::now() >= deadline)
                .unwrap();
        let _runtime = checked
            .record()
            .to_game_with_stop(&mut || Instant::now() >= deadline)
            .unwrap();
        let ending_decode_us = begin.elapsed().as_micros();
        assert_eq!(checked.record(), ended.record());
        println!(
            "{}",
            json!({"policy_boundary_profile":profile.id(), "import_usage":original.usage(), "decode_us":decode_us, "restore_us":restore_us, "actual_closing_path_us":closing_us, "ending_usage":ended.usage(), "ending_decode_restore_us":ending_decode_us})
        );
    }
}

fn assert_work_refusal(error: &TenjikuDocumentError) {
    assert!(!error.stopped);
    assert!(
        error
            .resource
            .as_ref()
            .unwrap()
            .facts()
            .iter()
            .any(|fact| fact.category == ResourceCategory::ReplayWork
                && fact.limit == Some(MAX_TENJIKU_DOCUMENT_WORK))
    );
}
fn repeated_contexts(
    base: &ValidatedTenjikuDocument,
    observation: &ExecutionObservation,
    count: usize,
) -> Payload {
    let mut payload = base.payload.clone();
    // Synthetic nonce variants of one actual UnsupportedMode response retain
    // real complete native contexts; they are not separately executed requests.
    for index in 0..count {
        let mut o = observation.clone();
        let id = index as u64 + 1;
        o.context.request_id = id;
        o.context.process_generation = id;
        o.context.session_version = id;
        payload.observations.push(o);
        payload.controls.push(TenjikuControl::Observation { index });
    }
    payload.retired_execution_floor = count as u64;
    payload
}

pub(super) fn measurement_document(profile: c::Profile) -> ValidatedTenjikuDocument {
    let (game, _) = grown(profile, 128, false);
    let base =
        ValidatedTenjikuDocument::new(c::GameRecord::from_game(&game).unwrap(), c::Side::Sente)
            .unwrap();
    let observation = execution::execute(execution::context(base.record(), 1));
    validate(repeated_contexts(&base, &observation, 8), &mut || false).unwrap()
}

#[test]
#[ignore = "explicit release repeated-context work/deadline policy measurement"]
fn measure_repeated_action_contexts_with_deadline() {
    use std::time::{Duration, Instant};
    for profile in c::Profile::ALL {
        let (game, _) = grown(profile, 128, false);
        let base =
            ValidatedTenjikuDocument::new(c::GameRecord::from_game(&game).unwrap(), c::Side::Sente)
                .unwrap();
        let observation = execution::execute(execution::context(base.record(), 1));
        let base_before = base.bytes().to_vec();
        for count in [4, 8, 28] {
            let payload = repeated_contexts(&base, &observation, count);
            let begin = Instant::now();
            let candidate = validate(payload, &mut || false);
            let candidate_us = begin.elapsed().as_micros();
            if count == 28 {
                let error = candidate.unwrap_err();
                assert_work_refusal(&error);
                assert_eq!(base.record(), &c::GameRecord::from_game(&game).unwrap());
                assert_eq!(base.bytes(), base_before);
                assert_eq!(decoded(&base).record(), base.record());
                println!(
                    "{}",
                    json!({"repeated_context_profile":profile.id(),"contexts":count,"candidate_accepted":false,"candidate_us":candidate_us,"resource":error.resource,"accepted_base_preserved":true,"native_history_preserved":true})
                );
                continue;
            }
            let document = candidate.unwrap();
            let begin = Instant::now();
            let deadline = begin + Duration::from_secs(15);
            let result = ValidatedTenjikuDocument::decode(document.bytes(), &mut || {
                Instant::now() >= deadline
            });
            let (accepted, stopped, resource) = match result {
                Ok(d) => match d
                    .record()
                    .to_game_with_stop(&mut || Instant::now() >= deadline)
                {
                    Ok(_) => (true, false, None),
                    Err(e) => (false, e == c::RecordError::Stopped, None),
                },
                Err(e) => (false, e.stopped, e.resource),
            };
            let decode_restore_us = begin.elapsed().as_micros();
            if count <= 8 {
                assert!(accepted, "representative repeated contexts must reopen");
            } else {
                assert!(accepted || stopped || resource.is_some());
            }
            println!(
                "{}",
                json!({"repeated_context_profile":profile.id(),"contexts":count,"usage":document.usage(),"candidate_us":candidate_us,"decode_restore_us":decode_restore_us,"accepted":accepted,"stopped":stopped,"resource":resource})
            );
        }
    }
}

pub(super) fn measured_real_engine_document(profile: c::Profile) -> ValidatedTenjikuDocument {
    let (game, _) = grown(profile, 128, false);
    let base =
        ValidatedTenjikuDocument::new(c::GameRecord::from_game(&game).unwrap(), c::Side::Sente)
            .unwrap();
    let mut ctx = execution::context(base.record(), 1);
    ctx.limits = mshogi_engine::TENJIKU_PLAY_LIMITS;
    ctx.purpose = mshogi_engine::ExecutionPurpose::MoveAnalysis {
        target: mshogi_engine::ExecutionTarget {
            selection_id: 1,
            event_index: 128,
            board_ply: 128,
            session_version: 1,
            label: "Retained128, actual engine analysis".into(),
            turn: mshogi_core::native::NativeSide::Tenjiku(game.position().side_to_move()),
        },
    };
    let request = mshogi_engine::ExecutionRequest {
        protocol: mshogi_engine::PROTOCOL_VERSION,
        context: ctx,
    };
    let mut bytes = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!("{}\n", serde_json::to_string(&request).unwrap())),
        &mut bytes,
    )
    .unwrap();
    let response: mshogi_engine::ExecutionResponse = serde_json::from_slice(&bytes).unwrap();
    assert!(response.matches(&request));
    assert!(matches!(
        &response.observation.result,
        ExecutionResult::Tenjiku {
            result: mshogi_engine::TenjikuResult::Choice { .. }
        }
    ));
    // One actual search result with explicitly synthetic nonce variants measures
    // repeated-context/PV growth; it is not four separately executed searches.
    let mut last = base.clone();
    for count in 1..=8 {
        let mut payload = repeated_contexts(&base, &response.observation, count);
        for o in &mut payload.observations {
            if let ExecutionPurpose::MoveAnalysis { target } = &mut o.context.purpose {
                target.session_version = o.context.session_version;
                target.selection_id = o.context.request_id;
            }
        }
        let begin = std::time::Instant::now();
        match validate(payload, &mut || false) {
            Ok(d) => {
                println!(
                    "{}",
                    serde_json::json!({"real_engine_context_growth":profile.id(),"contexts":count,"validate_us":begin.elapsed().as_micros(),"usage":d.usage()})
                );
                last = d;
            }
            Err(e) => {
                assert!(e.resource.is_some(), "{e}");
                println!(
                    "{}",
                    serde_json::json!({"real_engine_context_refusal":profile.id(),"contexts":count,"validate_us":begin.elapsed().as_micros(),"diagnostic":e.to_string(),"resource":e.resource})
                );
                break;
            }
        }
    }
    assert!(!last.observations().is_empty());
    last
}
