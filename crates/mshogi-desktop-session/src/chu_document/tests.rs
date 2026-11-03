use super::*;
use mshogi_engine::{EngineFailure, ExecutionContext, FailureKind};
use serde_json::Value;
fn standard() -> ValidatedChuDocument {
    ValidatedChuDocument::new(
        c::GameRecord::from_game(&c::Game::standard()).unwrap(),
        c::Side::Sente,
    )
    .unwrap()
}
fn command(
    d: &ValidatedChuDocument,
    actor: c::Side,
    command: c::GameCommand,
) -> ValidatedChuDocument {
    d.with_control(ChuControl::Native { actor, command }, &mut || false)
        .unwrap()
}
fn failure(d: &ValidatedChuDocument, id: u64) -> ExecutionObservation {
    ExecutionObservation::new(ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {}, request_id: id, process_generation: id, session_version: id,
        game: CanonicalRecord::chu(d.record().clone()), limits: WorkLimits { depth: 1, nodes: 1 },
    }, ExecutionResult::Failure { failure: EngineFailure { kind: FailureKind::UnsupportedMode, message: "Constructed unscored test observation; real supervised behavior is tested separately.".into() } })
}
#[test]
fn chu_current_envelope_roundtrip_and_native_nonmove_semantics() {
    let d = standard();
    let d = command(&d, c::Side::Sente, c::GameCommand::OfferDraw);
    assert_eq!(d.record().known_board_actions(), 0);
    let d = command(&d, c::Side::Gote, c::GameCommand::DeclineDraw);
    let d = d.with_selection(Some(1), &mut || false).unwrap();
    let restored = ValidatedChuDocument::decode(d.bytes(), &mut || false).unwrap();
    assert_eq!(restored.record(), d.record());
    assert_eq!(restored.selected_event(), Some(1));
    assert!(restored.record().conclusion().is_none());
    assert_eq!(
        restored
            .record()
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
    assert_eq!(d.record().conclusion(), Some(c::Conclusion::AgreedDraw));
    ValidatedChuDocument::decode(
        d.with_reserved_close(MAX_SAFE_ID - 65).unwrap().bytes(),
        &mut || false,
    )
    .unwrap();
}
#[test]
fn chu_observation_256_to_257_refuses_atomically_without_losing_save_path() {
    let baseline = standard();
    let mut p = baseline.payload.clone();
    for id in 1..=255 {
        p.controls.push(ChuControl::Observation {
            index: p.observations.len(),
        });
        p.observations.push(failure(&baseline, id));
    }
    p.retired_execution_floor = 255;
    let d = validate(p, &mut || false).unwrap();
    let d = d.with_observation(failure(&d, 256), &mut || false).unwrap();
    let before = d.bytes().to_vec();
    let e = d
        .with_observation(failure(&d, 257), &mut || false)
        .unwrap_err();
    assert_eq!(
        e.resource.unwrap().facts()[0].category,
        ResourceCategory::OrdinaryObservations
    );
    assert_eq!(d.bytes(), before);
    let closed = d.with_reserved_close(256).unwrap();
    let reopened = ValidatedChuDocument::decode(closed.bytes(), &mut || false).unwrap();
    assert_eq!(reopened.observations().len(), 256);
    let d = command(
        &reopened,
        c::Side::Sente,
        c::GameCommand::Stop {
            reason: c::StopReason::Cancelled,
        },
    );
    let d = command(&d, c::Side::Gote, c::GameCommand::Resign);
    ValidatedChuDocument::decode(d.with_reserved_close(256).unwrap().bytes(), &mut || false)
        .unwrap();
}
#[test]
fn chu_duplicate_conflicting_and_forged_documents_reject() {
    let d = standard();
    let raw = std::str::from_utf8(d.bytes()).unwrap();
    let duplicate = raw.replacen(
        "\"mode\":\"untimed\"",
        "\"mode\":\"untimed\",\"mode\":\"untimed\"",
        1,
    );
    assert_ne!(duplicate, raw);
    assert!(ValidatedChuDocument::decode(duplicate.as_bytes(), &mut || false).is_err());
    let mut v: Value = serde_json::from_slice(d.bytes()).unwrap();
    v["game_kind"] = "shogi".into();
    assert!(ValidatedChuDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err());
    let mut v: Value = serde_json::from_slice(d.bytes()).unwrap();
    v["payload"]["current"]["payload"]["conclusion"] = serde_json::json!({"kind":"agreed_draw"});
    assert!(ValidatedChuDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err());
    assert!(ValidatedChuDocument::decode(d.bytes(), &mut || true).is_err());
    assert_eq!(d.record().event_count(), 0);
}
#[test]
fn chu_runtime_width_and_close_reserved_path_reopen() {
    let d = standard();
    let old_max = d.usage().encoded_bytes + d.usage().reserved_bytes;
    let d = d.with_runtime_floor(MAX_SAFE_ID - 65).unwrap();
    assert_eq!(d.usage().encoded_bytes + d.usage().reserved_bytes, old_max);
    let d = d.with_reserved_close(MAX_SAFE_ID - 65).unwrap();
    assert!(d.bytes().len() <= old_max);
    assert_eq!(
        d.with_reserved_close(MAX_SAFE_ID - 65).unwrap().bytes(),
        d.bytes()
    );
    let restored = ValidatedChuDocument::decode(d.bytes(), &mut || false).unwrap();
    assert!(restored.usage().encoded_bytes + restored.usage().reserved_bytes <= old_max);
}
#[test]
fn chu_record_work_fact_is_native_derived_and_not_wire_data() {
    let game = c::Game::standard();
    let r = c::GameRecord::from_game(&game).unwrap();
    assert_eq!(r.reconstruction_work(), game.replay_work());
    assert!(r.reconstruction_work() > 0);
    let parsed = c::GameRecord::from_json(r.to_json()).unwrap();
    assert_eq!(r.reconstruction_work(), parsed.reconstruction_work());
    assert!(!r.to_json().contains("reconstruction_work"));
}

#[test]
fn actual_ending_encodings_fit_reserve_for_null_and_short_selection_all_actors() {
    for selection in [None, Some(0)] {
        let d = standard().with_selection(selection, &mut || false).unwrap();
        let max_bytes = d.usage().encoded_bytes + d.usage().reserved_bytes;
        for stop_actor in [c::Side::Sente, c::Side::Gote] {
            for resign_actor in [c::Side::Sente, c::Side::Gote] {
                let d = d.with_reserved_close(MAX_SAFE_ID - 65).unwrap();
                let d = command(
                    &d,
                    stop_actor,
                    c::GameCommand::Stop {
                        reason: c::StopReason::EngineFailure,
                    },
                );
                let d = command(&d, resign_actor, c::GameCommand::Resign);
                let d = d.with_reserved_close(MAX_SAFE_ID - 65).unwrap();
                assert!(
                    d.bytes().len() <= max_bytes,
                    "{} > {max_bytes}",
                    d.bytes().len()
                );
                ValidatedChuDocument::decode(d.bytes(), &mut || false).unwrap();
            }
        }
    }
}
#[test]
fn control_and_byte_bounds_preserve_exact_saveable_candidate() {
    let d = standard();
    let mut p = d.payload.clone();
    // Ongoing documents reserve early Close, Stop, Resign and final Close.
    p.controls = vec![
        ChuControl::Settings {
            settings: p.settings.clone()
        };
        MAX_ARCHIVE_CONTROLS - 4
    ];
    let d = validate(p, &mut || false).unwrap();
    let e = d
        .with_control(
            ChuControl::Settings {
                settings: d.settings().clone(),
            },
            &mut || false,
        )
        .unwrap_err();
    assert_eq!(
        e.resource.unwrap().facts()[0].category,
        ResourceCategory::Controls
    );
    let d = command(
        &d,
        c::Side::Gote,
        c::GameCommand::Stop {
            reason: c::StopReason::ResourceLimit,
        },
    );
    let d = command(&d, c::Side::Sente, c::GameCommand::Resign);
    ValidatedChuDocument::decode(d.with_reserved_close(1).unwrap().bytes(), &mut || false).unwrap();
    let d = standard();
    let mut exact = d.bytes().to_vec();
    exact.resize(MAX_ARCHIVE_BYTES, b' ');
    ValidatedChuDocument::decode(&exact, &mut || false).unwrap();
    exact.push(b' ');
    assert_eq!(
        ValidatedChuDocument::decode(&exact, &mut || false)
            .unwrap_err()
            .resource
            .unwrap()
            .facts()[0]
            .category,
        ResourceCategory::EncodedDocumentBytes
    );
}

fn grow(seed: u64, plies: usize, standard_root: bool) -> (c::Game, Vec<ChuControl>) {
    let mut g = if standard_root {
        c::Game::standard()
    } else {
        crate::chu_fixture_roots::root(
            &crate::chu_fixture_roots::fixture("CHU-REP-QUIET")["given"]["root"],
        )
    };
    let mut controls = vec![];
    let mut rng = seed;
    for _ in 0..plies {
        let actor = g.position().side_to_move();
        let choices: Vec<_> = g
            .legal_actions()
            .into_iter()
            .filter(|a| {
                standard_root || {
                    let p = g.position().piece(a.from).unwrap();
                    p.base == c::Base::King
                        && a.to().is_some_and(|s| {
                            if actor == c::Side::Sente {
                                s.rank() >= 8
                            } else {
                                s.rank() <= 5
                            }
                        })
                        && g.position().piece(a.to().unwrap()).is_none()
                }
            })
            .collect();
        assert!(!choices.is_empty());
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let command = c::GameCommand::Play {
            action: choices[(rng as usize) % choices.len()].clone(),
        };
        match g.submit(c::GameRequest {
            expected: g.context(),
            actor,
            command: command.clone(),
        }) {
            Ok(_) => controls.push(ChuControl::Native { actor, command }),
            Err(c::GameError::Capacity(_)) => break,
            Err(e) => panic!("{e}"),
        }
    }
    (g, controls)
}
#[test]
fn complete_document_measurements_and_aggregate_work_admission() {
    for (name, plies, standard_root) in [
        ("standard32", 32, true),
        ("history96", 96, false),
        ("history-native-boundary", 1000, false),
    ] {
        let initial = if standard_root {
            c::Game::standard()
        } else {
            crate::chu_fixture_roots::root(
                &crate::chu_fixture_roots::fixture("CHU-REP-QUIET")["given"]["root"],
            )
        };
        let (g, controls) = grow(if standard_root { 17 } else { 3917 }, plies, standard_root);
        let record = c::GameRecord::from_game(&g).unwrap();
        let mut p =
            ValidatedChuDocument::new(c::GameRecord::from_game(&initial).unwrap(), c::Side::Sente)
                .unwrap()
                .payload;
        p.controls = controls;
        p.current = CanonicalRecord::chu(record.clone());
        let timer = std::time::Instant::now();
        let d = validate(p, &mut || false).unwrap();
        let d = if name == "history96" {
            let d = d.with_observation(failure(&d, 2), &mut || false).unwrap();
            d.with_control(
                ChuControl::Rewind {
                    event_index: 48,
                    reason: RewindKind::TakeBack,
                },
                &mut || false,
            )
            .unwrap()
            .with_selection(Some(24), &mut || false)
            .unwrap()
        } else {
            d
        };
        let t = std::time::Instant::now();
        let reopened = ValidatedChuDocument::decode(d.bytes(), &mut || false).unwrap();
        let decode_ms = t.elapsed().as_millis();
        let t = std::time::Instant::now();
        crate::shared::chu::ChuSession::restore(reopened, &mut || false).unwrap();
        let restore_ms = t.elapsed().as_millis();
        eprintln!(
            "MSH78 MEASURE {name}: native_events={} native_bytes={} native_work={} document={} decode_ms={decode_ms} restore_ms={restore_ms} total_ms={}",
            record.event_count(),
            record.to_json().len(),
            record.reconstruction_work(),
            serde_json::to_string(d.usage()).unwrap(),
            timer.elapsed().as_millis()
        );
        if plies < 1000 {
            assert!(
                decode_ms + restore_ms < 15_000,
                "mandatory representative journey"
            );
        }
        // Imported history repeats its initial record on the supported decode path.
        let imported = ValidatedChuDocument::new(record, c::Side::Sente);
        if name == "history-native-boundary" {
            assert_eq!(
                imported.unwrap_err().resource.unwrap().facts()[0].category,
                ResourceCategory::ReplayWork
            );
        }
    }
}

#[test]
fn retained_native_event_boundary_reserves_stop_and_resign_without_discarding_observations() {
    let mut g = crate::chu_fixture_roots::root(
        &crate::chu_fixture_roots::fixture("CHU-REP-QUIET")["given"]["root"],
    );
    for _ in 0..8 {
        for (actor, command) in [
            (c::Side::Sente, c::GameCommand::OfferDraw),
            (c::Side::Gote, c::GameCommand::DeclineDraw),
        ] {
            g.submit(c::GameRequest {
                expected: g.context(),
                actor,
                command,
            })
            .unwrap();
        }
    }
    let initial =
        ValidatedChuDocument::new(c::GameRecord::from_game(&g).unwrap(), c::Side::Sente).unwrap();
    let mut p = initial.payload.clone();
    for id in 1..=253 {
        p.controls.push(ChuControl::Observation {
            index: p.observations.len(),
        });
        p.observations.push(failure(&initial, id));
    }
    let prefix = g.record_prefix(14).unwrap();
    let mut o = failure(&initial, 254);
    o.context.game = CanonicalRecord::chu(prefix);
    o.context.purpose = ExecutionPurpose::MoveAnalysis {
        target: mshogi_engine::ExecutionTarget {
            selection_id: 254,
            event_index: 14,
            board_ply: 0,
            session_version: 254,
            label: "Exact 14-event prefix, zero board plies".into(),
            turn: mshogi_core::native::NativeSide::Chu(c::Side::Sente),
        },
    };
    p.controls.push(ChuControl::Observation {
        index: p.observations.len(),
    });
    p.observations.push(o);
    p.retired_execution_floor = 254;
    let d = validate(p, &mut || false).unwrap();
    assert_eq!(d.usage().retained_events, MAX_ARCHIVE_EVENTS - 2);
    let before = d.bytes().to_vec();
    let error = d
        .with_control(
            ChuControl::Native {
                actor: c::Side::Sente,
                command: c::GameCommand::OfferDraw,
            },
            &mut || false,
        )
        .unwrap_err();
    assert_eq!(
        error.resource.unwrap().facts()[0].category,
        ResourceCategory::NativeEvents
    );
    assert_eq!(d.bytes(), before);
    let d = command(
        &d,
        c::Side::Gote,
        c::GameCommand::Stop {
            reason: c::StopReason::Cancelled,
        },
    );
    let d = command(&d, c::Side::Sente, c::GameCommand::Resign);
    assert_eq!(d.usage().retained_events, MAX_ARCHIVE_EVENTS);
    let d = d.with_reserved_close(254).unwrap();
    let reopened = ValidatedChuDocument::decode(d.bytes(), &mut || false).unwrap();
    assert_eq!(reopened.observations().len(), 254);
}
