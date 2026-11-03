//! Real Chu consumers. Fixture expectations remain in the independently reviewed corpus.
use super::*;
use crate::chu_fixture_roots as f;
use mshogi_core::chu as c;
use std::{fs, time::Duration};
fn controller() -> DesktopController {
    DesktopController::new("/unused/chu-engine".into()).unwrap()
}
fn settle(c: &mut DesktopController, receipt: AdmissionReceipt) -> Result<SharedSnapshot, String> {
    let until = Instant::now() + Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if let Some(r) = receipt.completion() {
            return r.map(|_| s);
        }
        assert!(Instant::now() < until, "native admission deadline");
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<SharedSnapshot, String> {
    let game = c.shared_snapshot().game;
    let (_, r) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: c.shared_token(),
        game,
        operation,
    })?;
    settle(c, r)
}
fn replace(c: &mut DesktopController, g: &c::Game, human: c::Side) {
    send(
        c,
        SessionOperation::Replace {
            record: CanonicalRecord::chu(c::GameRecord::from_game(g).unwrap()),
            human: NativeSide::Chu(human),
        },
    )
    .unwrap();
}
fn native(c: &DesktopController) -> c::GameRecord {
    c::GameRecord::from_game(&c.session.chu().unwrap().game).unwrap()
}
fn play(c: &mut DesktopController, a: c::Action) -> Result<SharedSnapshot, String> {
    let actor = c.session.chu().unwrap().game.position().side_to_move();
    send(
        c,
        SessionOperation::Chu {
            actor,
            command: c::GameCommand::Play { action: a },
        },
    )
}
fn fixture_game(id: &str) -> c::Game {
    f::root(&f::fixture(id)["given"]["root"])
}
fn wait_file(c: &mut DesktopController, id: u64) -> SharedSnapshot {
    let end = Instant::now() + Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if s.io.id == id && matches!(s.io.phase, IoPhase::Complete | IoPhase::Failed) {
            return s;
        }
        assert!(Instant::now() < end);
        std::thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn real_compound_effects_royals_promotion_outcomes_and_exact_prefixes() {
    let mut ctl = controller();
    for id in [
        "CHU-LION-C2-1",
        "CHU-RETURN-Ln-enemy",
        "CHU-RETURN-Ln-empty",
        "CHU-ROYAL-BOTH-COMPOUND",
    ] {
        let fixture = f::fixture(id);
        let g = if id == "CHU-RETURN-Ln-enemy" {
            f::contract_game()
        } else {
            f::root(&fixture["given"]["root"])
        };
        replace(&mut ctl, &g, g.position().side_to_move());
        let action = f::action(&fixture["given"]["commands"][0]);
        let s = ctl.shared_snapshot();
        assert_eq!(s.board.files.len(), 12);
        assert_eq!(s.board.ranks.len(), 12);
        assert!(s.hands.is_empty());
        assert!(
            !s.capabilities.engine_turn
                && !s.capabilities.move_analysis
                && !s.capabilities.drops
                && !s.capabilities.chess_full
        );
        assert!(
            s.choices
                .iter()
                .any(|ch| ch.action == serde_json::to_string(&action).unwrap())
        );
        let s = play(&mut ctl, action.clone()).unwrap();
        let effects = ctl.session.chu().unwrap().game.events()[0]
            .effects
            .as_ref()
            .unwrap();
        let cue = s.moves.last_action.as_ref().unwrap();
        assert_eq!(
            cue.steps,
            action
                .route
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        );
        assert_eq!(cue.action, serde_json::to_string(&action).unwrap());
        assert_eq!(cue.captures.as_ref().unwrap().len(), effects.captures.len());
        for (presented, actual) in cue.captures.as_ref().unwrap().iter().zip(&effects.captures) {
            assert_eq!(presented.square, actual.square.to_string());
        }
        let live = native(&ctl);
        let s = send(&mut ctl, SessionOperation::Inspect { event_index: 0 }).unwrap();
        assert!(s.historical);
        assert_eq!(s.event_index, 0);
        assert!(s.moves.last_action.is_none());
        assert_eq!(native(&ctl), live);
        assert_eq!(s.chu.unwrap().selected.board_ply, 0);
        assert!(send(&mut ctl, SessionOperation::Resign {}).is_err());
        let s = send(&mut ctl, SessionOperation::ReturnLive {}).unwrap();
        assert!(!s.historical);
        assert_eq!(s.event_index, 1);
        if id == "CHU-ROYAL-BOTH-COMPOUND" {
            assert!(s.status.concluded);
            assert!(s.status.label.contains("final royal capture"));
            assert_eq!(s.status.score_half_points, Some([2, 0]));
        }
        let s = send(&mut ctl, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(s.board_ply, 0);
        assert_eq!(
            ctl.session
                .chu()
                .unwrap()
                .document
                .retired_branches()
                .count(),
            1
        );
        assert_eq!(
            s.chu.unwrap().live.royals.len(),
            g.position()
                .pieces()
                .filter(|(_, p)| matches!(p.face(), c::Face::King | c::Face::Prince))
                .count()
        );
    }
    let fixture = f::fixture("CHU-PROMOTE-DE-YES");
    replace(
        &mut ctl,
        &f::root(&fixture["given"]["root"]),
        c::Side::Sente,
    );
    let a = f::action(&fixture["given"]["action"]);
    let s = ctl.shared_snapshot();
    assert_eq!(
        s.choices
            .iter()
            .filter(|v| v.from == a.from.to_string() && v.steps == vec!["6d"])
            .count(),
        2
    );
    let s = play(&mut ctl, a).unwrap();
    let history = s.chu.unwrap().selected;
    assert!(
        history
            .identities
            .iter()
            .any(|i| i.piece.base == c::Base::Elephant
                && i.piece.promoted
                && i.face == c::Face::Prince)
    );
    assert_eq!(history.royals.len(), 3);
    replace(&mut ctl, &fixture_game("CHU-MATE"), c::Side::Sente);
    let s = ctl.shared_snapshot();
    assert!(s.status.concluded);
    assert_eq!(s.status.label, "Gote wins by mate");
    assert!(s.choices.is_empty());
}
#[test]
fn commands_keep_actor_admission_native_and_board_ply_distinct_from_events() {
    let mut ctl = controller();
    replace(&mut ctl, &c::Game::standard(), c::Side::Gote);
    // Non-turn terminal/offer actors are admitted by Game, not a shared turn filter.
    send(
        &mut ctl,
        SessionOperation::Chu {
            actor: c::Side::Gote,
            command: c::GameCommand::OfferDraw,
        },
    )
    .unwrap();
    let s = send(
        &mut ctl,
        SessionOperation::Chu {
            actor: c::Side::Sente,
            command: c::GameCommand::DeclineDraw,
        },
    )
    .unwrap();
    assert_eq!(s.board_ply, 0);
    assert_eq!(s.event_index, 2);
    assert!(s.moves.entries.is_empty());
    let before = native(&ctl);
    assert!(
        send(
            &mut ctl,
            SessionOperation::Chu {
                actor: c::Side::Gote,
                command: c::GameCommand::AcceptDraw
            }
        )
        .is_err()
    );
    assert_eq!(native(&ctl), before);
    send(
        &mut ctl,
        SessionOperation::Chu {
            actor: c::Side::Sente,
            command: c::GameCommand::OfferDraw,
        },
    )
    .unwrap();
    let s = send(
        &mut ctl,
        SessionOperation::Chu {
            actor: c::Side::Gote,
            command: c::GameCommand::AcceptDraw,
        },
    )
    .unwrap();
    assert_eq!(s.status.label, "Draw by agreement");
    send(&mut ctl, SessionOperation::Undo {}).unwrap();
    assert!(native(&ctl).conclusion().is_none());
    let s = send(&mut ctl, SessionOperation::Resign {}).unwrap();
    assert_eq!(s.status.label, "Sente wins by resignation");
}
#[test]
fn atomic_file_reopen_history_branch_and_normal_product_gate() {
    let dir = std::env::temp_dir().join(format!("msh78-session-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("native-chu.mshogi");
    let mut ctl = controller();
    let fixture = f::fixture("CHU-LION-C2-1");
    replace(
        &mut ctl,
        &f::root(&fixture["given"]["root"]),
        c::Side::Sente,
    );
    play(&mut ctl, f::action(&fixture["given"]["commands"][0])).unwrap();
    send(&mut ctl, SessionOperation::TakeBack {}).unwrap();
    send(
        &mut ctl,
        SessionOperation::Chu {
            actor: c::Side::Gote,
            command: c::GameCommand::OfferDraw,
        },
    )
    .unwrap();
    send(&mut ctl, SessionOperation::Inspect { event_index: 0 }).unwrap();
    let s = send(
        &mut ctl,
        SessionOperation::ChuPreferences {
            settings: crate::chu_document::ChuSettings {
                orientation: c::Side::Gote,
            },
        },
    )
    .unwrap();
    assert!(s.historical);
    assert_eq!(s.event_index, 0);
    assert_eq!(
        ctl.session.chu().unwrap().document.settings().orientation,
        c::Side::Gote
    );
    let old = ctl.shared_snapshot();
    let record = native(&ctl);
    let id = ctl.queue_shared_save(path.clone()).unwrap();
    assert_eq!(wait_file(&mut ctl, id).io.phase, IoPhase::Complete);
    let bytes = fs::read(&path).unwrap();
    let id = ctl
        .queue_product_open(path.clone(), ctl.shared_open_token())
        .unwrap();
    let s = wait_file(&mut ctl, id);
    assert_eq!(s.io.phase, IoPhase::Failed);
    assert_eq!(s.game, GameKind::Chu);
    assert_eq!(native(&ctl), record);
    let id = ctl
        .queue_shared_open(path.clone(), ctl.shared_open_token())
        .unwrap();
    let s = wait_file(&mut ctl, id);
    assert_eq!(s.io.phase, IoPhase::Complete);
    assert!(s.token.document > old.token.document);
    assert!(s.token.version > old.token.version);
    assert!(s.paused);
    assert!(s.historical);
    assert_eq!(s.event_index, 0);
    assert_eq!(native(&ctl), record);
    assert!(ctl.session.chu().unwrap().pending.is_none());
    let s = send(&mut ctl, SessionOperation::ReturnLive {}).unwrap();
    assert_eq!(s.event_index, 1);
    assert!(
        send(&mut ctl, SessionOperation::Resume {})
            .unwrap_err()
            .contains("unsupported")
    );
    // Failed replacement/file operations preserve the exact accepted document.
    let exact = ctl.session.chu().unwrap().document.bytes().to_vec();
    fs::write(&path, b"{bad").unwrap();
    let id = ctl
        .queue_shared_open(path.clone(), ctl.shared_open_token())
        .unwrap();
    assert_eq!(wait_file(&mut ctl, id).io.phase, IoPhase::Failed);
    assert_eq!(ctl.session.chu().unwrap().document.bytes(), exact);
    fs::write(&path, &bytes).unwrap();
    send(
        &mut ctl,
        SessionOperation::ChuPreferences {
            settings: crate::chu_document::ChuSettings {
                orientation: c::Side::Gote,
            },
        },
    )
    .unwrap();
    assert!(ctl.shared_snapshot().document.dirty);
    let failed = dir.join("absent").join("save.mshogi");
    let id = ctl.queue_shared_save(failed).unwrap();
    assert_eq!(wait_file(&mut ctl, id).io.phase, IoPhase::Failed);
    assert!(ctl.shared_snapshot().document.dirty);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert!(
        ctl.shared_new_game(
            ctl.shared_open_token(),
            GameKind::Chu,
            NativeSide::Chu(c::Side::Sente)
        )
        .is_err()
    );
    ctl.pause_shared_for_close();
    let closed = ctl.session.chu().unwrap().document.clone();
    crate::chu_document::ValidatedChuDocument::decode(closed.bytes(), &mut || false).unwrap();
    assert!(ctl.close_requires_save());
    ctl.pause_shared_for_close();
    assert_eq!(closed.bytes(), ctl.session.chu().unwrap().document.bytes());
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn before_fourth_refusal_and_same_diagram_different_history_survive_shared_boundary() {
    let fixture = f::fixture("CHU-REP-QUIET");
    let mut g = f::root(&fixture["given"]["root"]);
    let mut ctl = controller();
    replace(&mut ctl, &g, c::Side::Sente);
    // Drive the independently expected complete route until the native obligation refuses.
    let commands = fixture["given"]["commands"].as_array().unwrap();
    for cmd in commands {
        let a = f::action(cmd);
        let actor = g.position().side_to_move();
        let before = native(&ctl);
        let expected = g.submit(c::GameRequest {
            expected: g.context(),
            actor,
            command: c::GameCommand::Play { action: a.clone() },
        });
        let actual = play(&mut ctl, a);
        assert_eq!(actual.is_ok(), expected.is_ok());
        if expected.is_err() {
            assert_eq!(native(&ctl), before);
            assert!(native(&ctl).conclusion().is_none());
            break;
        }
    }
    assert_eq!(native(&ctl).known_board_actions(), 10);
    assert!(
        !ctl.shared_snapshot()
            .chu
            .unwrap()
            .live
            .obligations
            .is_empty()
    );
    let live = native(&ctl);
    let fresh = c::Game::from_root(
        c::Position::from_diagram(
            &g.position().pieces().collect::<Vec<_>>(),
            g.position().side_to_move(),
            c::RootMetadata::diagram(g.snapshot().board_ply as u64),
        )
        .unwrap(),
    )
    .unwrap();
    assert_ne!(live, c::GameRecord::from_game(&fresh).unwrap());
}

#[test]
fn individual_royal_threat_is_not_all_royals_or_chess_self_check() {
    let mut ctl = controller();
    let f = f::fixture("CHU-MATE-PRINCE-RESPONSE");
    replace(&mut ctl, &f::root(&f["given"]["root"]), c::Side::Sente);
    let s = play(&mut ctl, f::action(&f["given"]["response"])).unwrap();
    let native = s.chu.unwrap().selected;
    let sente: Vec<_> = native
        .royals
        .iter()
        .filter(|r| r.identity.piece.side == c::Side::Sente)
        .collect();
    assert_eq!(sente.len(), 2);
    assert!(sente.iter().all(|r| r.local_capture_threat));
    assert_eq!(native.capture_all_royals_threat[0], Some(false));
    assert!(native.conclusion.is_none());
    assert!(native.threat_diagnostics.is_empty());
}

#[test]
fn actual_native_inspection_is_read_only_and_bound_to_exact_historical_selection() {
    let mut ctl = controller();
    let f = f::fixture("CHU-LION-C2-1");
    replace(&mut ctl, &f::root(&f["given"]["root"]), c::Side::Sente);
    let initial = ctl.shared_token();
    let before = native(&ctl);
    let view = ctl
        .shared_inspection(MovementInspectionQuery {
            expected: initial,
            origin: "1b".into(),
        })
        .unwrap();
    assert!(view.read_only);
    assert_eq!(view.game, GameKind::Chu);
    assert_eq!(view.side.id, NativeSide::Chu(c::Side::Gote));
    assert!(view.choices.iter().all(|v| v.from == "1b"));
    assert_eq!(native(&ctl), before);
    play(&mut ctl, f::action(&f["given"]["commands"][0])).unwrap();
    assert!(
        ctl.shared_inspection(MovementInspectionQuery {
            expected: initial,
            origin: "1b".into()
        })
        .is_err()
    );
    assert!(
        ctl.shared_inspection(MovementInspectionQuery {
            expected: ctl.shared_token(),
            origin: "1b".into()
        })
        .unwrap()
        .side
        .id == NativeSide::Chu(c::Side::Sente)
    );
    send(&mut ctl, SessionOperation::Inspect { event_index: 0 }).unwrap();
    let live = native(&ctl);
    let view = ctl
        .shared_inspection(MovementInspectionQuery {
            expected: ctl.shared_token(),
            origin: "1b".into(),
        })
        .unwrap();
    assert_eq!(view.side.id, NativeSide::Chu(c::Side::Gote));
    assert_eq!(native(&ctl), live);
}

#[test]
fn maximum_declared_counter_keeps_exact_native_presentation_and_unknown_history() {
    let mut ctl = controller();
    let base = f::root(&f::fixture("CHU-REP-QUIET")["given"]["root"]);
    let imported = c::Game::from_root(
        c::Position::from_diagram(
            &base.position().pieces().collect::<Vec<_>>(),
            c::Side::Sente,
            c::RootMetadata::imported("Exact counter fixture".into(), u64::MAX).unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    replace(&mut ctl, &imported, c::Side::Sente);
    let s = ctl.shared_snapshot();
    let presented = s.chu.unwrap();
    assert_eq!(presented.declared_root_plies, "18446744073709551615");
    assert!(presented.earlier_history_unknown);
    assert!(!presented.historical_reachability_proven);
    assert_eq!(presented.selected.board_ply, 0);
    assert_eq!(
        presented.selected.declared_total_plies,
        "18446744073709551615"
    );
    assert_eq!(presented.root.source(), Some("Exact counter fixture"));
    let before = native(&ctl);
    assert!(
        play(
            &mut ctl,
            c::Action {
                from: "1l".parse().unwrap(),
                route: vec!["1k".parse().unwrap()],
                promote: false
            }
        )
        .is_err()
    );
    let refusal = ctl.shared_snapshot();
    let fact = refusal.capacity.resource.as_ref().unwrap().facts()[0].clone();
    assert_eq!(fact.category, ResourceCategory::Unknown);
    assert_eq!((fact.count, fact.limit, fact.unit), (None, None, None));
    assert_eq!(native(&ctl), before);
    assert!(!refusal.status.concluded);
    let d = ctl.session.chu().unwrap().document.clone();
    let reopened =
        crate::chu_document::ValidatedChuDocument::decode(d.bytes(), &mut || false).unwrap();
    assert_eq!(
        reopened
            .record()
            .to_game()
            .unwrap()
            .position()
            .root()
            .declared_completed_plies(),
        u64::MAX
    );
}

#[test]
fn technical_move_rows_number_known_history_without_rounding_declared_counters() {
    let mut ctl = controller();
    let base = f::root(&f::fixture("CHU-REP-QUIET")["given"]["root"]);
    let root = c::Game::from_root(
        c::Position::from_diagram(
            &base.position().pieces().collect::<Vec<_>>(),
            c::Side::Sente,
            c::RootMetadata::diagram(u64::MAX - 1),
        )
        .unwrap(),
    )
    .unwrap();
    replace(&mut ctl, &root, c::Side::Sente);
    let s = play(
        &mut ctl,
        c::Action {
            from: "1l".parse().unwrap(),
            route: vec!["1k".parse().unwrap()],
            promote: false,
        },
    )
    .unwrap();
    assert_eq!(s.moves.entries[0].number, 1);
    assert!(s.moves.root.numbering.contains("18446744073709551614"));
    let p = s.chu.unwrap();
    assert_eq!(p.declared_root_plies, "18446744073709551614");
    assert_eq!(p.live.declared_total_plies, "18446744073709551615");
}

#[test]
fn known_root_pair_numbering_preserves_first_gote_action_order() {
    let mut ctl = controller();
    let base = f::root(&f::fixture("CHU-REP-QUIET")["given"]["root"]);
    let root = c::Game::from_root(
        c::Position::from_diagram(
            &base.position().pieces().collect::<Vec<_>>(),
            c::Side::Gote,
            c::RootMetadata::diagram(17),
        )
        .unwrap(),
    )
    .unwrap();
    replace(&mut ctl, &root, c::Side::Gote);
    play(
        &mut ctl,
        c::Action {
            from: "12a".parse().unwrap(),
            route: vec!["12b".parse().unwrap()],
            promote: false,
        },
    )
    .unwrap();
    let s = play(
        &mut ctl,
        c::Action {
            from: "1l".parse().unwrap(),
            route: vec!["1k".parse().unwrap()],
            promote: false,
        },
    )
    .unwrap();
    assert_eq!(
        s.moves
            .entries
            .iter()
            .map(|e| (e.number, e.column))
            .collect::<Vec<_>>(),
        vec![(1, 1), (2, 0)]
    );
}

#[test]
fn ambiguous_native_counter_capacity_keeps_all_ending_and_close_paths_saveable() {
    use crate::chu_document::{ChuControl, ValidatedChuDocument};
    let base = f::root(&f::fixture("CHU-REP-QUIET")["given"]["root"]);
    let g = c::Game::from_root(
        c::Position::from_diagram(
            &base.position().pieces().collect::<Vec<_>>(),
            c::Side::Sente,
            c::RootMetadata::diagram(u64::MAX),
        )
        .unwrap(),
    )
    .unwrap();
    let d =
        ValidatedChuDocument::new(c::GameRecord::from_game(&g).unwrap(), c::Side::Sente).unwrap();
    let bytes = d.bytes().to_vec();
    let e = d
        .with_control(
            ChuControl::Native {
                actor: c::Side::Sente,
                command: c::GameCommand::Play {
                    action: c::Action {
                        from: "1l".parse().unwrap(),
                        route: vec!["1k".parse().unwrap()],
                        promote: false,
                    },
                },
            },
            &mut || false,
        )
        .unwrap_err();
    assert_eq!(
        e.resource.unwrap().facts()[0].category,
        ResourceCategory::Unknown
    );
    assert_eq!(d.bytes(), bytes);
    let d = d
        .with_control(
            ChuControl::Native {
                actor: c::Side::Gote,
                command: c::GameCommand::Stop {
                    reason: c::StopReason::ResourceLimit,
                },
            },
            &mut || false,
        )
        .unwrap();
    let d = d
        .with_control(
            ChuControl::Native {
                actor: c::Side::Sente,
                command: c::GameCommand::Resign,
            },
            &mut || false,
        )
        .unwrap();
    ValidatedChuDocument::decode(d.with_reserved_close(1).unwrap().bytes(), &mut || false).unwrap();
}
