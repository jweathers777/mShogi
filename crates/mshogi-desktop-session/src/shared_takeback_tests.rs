//! MSH-113: real native histories and records; these are controller tests, not GUI evidence.
use super::*;
use mshogi_core::xiangqi as x;
use std::sync::atomic::{AtomicU64, Ordering};

fn controller() -> DesktopController {
    let mut c = DesktopController::new("/unused/msh113-native-engine".into()).unwrap();
    c.shared.manual_engine = true;
    c
}
fn game_kind(c: &DesktopController) -> GameKind {
    if c.shared.xiangqi.is_some() {
        GameKind::Xiangqi
    } else if c.shared.shogi.is_some() {
        GameKind::Shogi
    } else {
        GameKind::Chess
    }
}
fn settle(c: &mut DesktopController) {
    let deadline = Instant::now() + std::time::Duration::from_secs(30);
    while c.shared_snapshot().capacity.status == CapacityPhase::Checking {
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<SharedSnapshot, String> {
    let (_, receipt) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: c.shared_token(),
        game: game_kind(c),
        operation,
    })?;
    settle(c);
    receipt.completion().unwrap()?;
    Ok(c.shared_snapshot())
}
fn replace(c: &mut DesktopController, game: GameKind, second: bool) {
    let (record, human) = match game {
        GameKind::Shogi => (
            CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new())),
            NativeSide::Shogi(if second {
                s::Side::Gote
            } else {
                s::Side::Sente
            }),
        ),
        GameKind::Xiangqi => (
            CanonicalRecord::xiangqi(
                x::GameRecord::from_game(&x::Game::standard(x::ClaimPolicy::Explicit).unwrap())
                    .unwrap(),
            ),
            NativeSide::Xiangqi(if second { x::Side::Black } else { x::Side::Red }),
        ),
        _ => unreachable!(),
    };
    send(c, SessionOperation::Replace { record, human }).unwrap();
    send(c, SessionOperation::Resume {}).unwrap();
}
fn play(c: &mut DesktopController, action: &str) {
    let op = match game_kind(c) {
        GameKind::Shogi => SessionOperation::Play {
            action: action.parse().unwrap(),
        },
        GameKind::Xiangqi => SessionOperation::Xiangqi {
            command: x::GameCommand::Play {
                action: action.parse().unwrap(),
            },
        },
        _ => unreachable!(),
    };
    send(c, op).unwrap();
}
fn pending(c: &DesktopController) -> SharedRequest {
    if let Some(s) = &c.shared.shogi {
        s.pending.clone().unwrap()
    } else {
        c.shared.xiangqi.as_ref().unwrap().pending.clone().unwrap()
    }
}
fn execute(request: &SharedRequest) -> SharedResponse {
    let mut bytes = Vec::new();
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!("{}\n", serde_json::to_string(request).unwrap())),
        &mut bytes,
    )
    .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}
fn engine(c: &mut DesktopController) -> SharedResponse {
    let response = execute(&pending(c));
    assert!(c.accept_shared(response.clone()));
    settle(c);
    response
}
fn wait_file(c: &mut DesktopController, id: u64) {
    let deadline = Instant::now() + std::time::Duration::from_secs(30);
    loop {
        let s = c.shared_snapshot();
        assert!(Instant::now() < deadline);
        if s.io.id == id && matches!(s.io.phase, IoPhase::Complete | IoPhase::Failed) {
            assert_eq!(s.io.phase, IoPhase::Complete, "{:?}", s.io.message);
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn roundtrip(c: &mut DesktopController, label: &str) {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "msh113-{}-{}-{label}.mshogi",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let before = c.shared_snapshot();
    let id = c.queue_shared_save(path.clone()).unwrap();
    wait_file(c, id);
    assert!(!c.shared_snapshot().document.dirty);
    let id = c
        .queue_shared_open(path.clone(), c.shared_open_token())
        .unwrap();
    wait_file(c, id);
    let reopened = c.shared_snapshot();
    assert_eq!(reopened.context, before.context);
    assert_ne!(reopened.token, before.token);
    assert!(reopened.paused);
    assert!(reopened.execution_purpose.is_none());
    assert!(reopened.observations.iter().all(|o| o.historical));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn takeback_native_shogi_and_xiangqi_pairs_pending_repeated_and_fresh_contexts() {
    for game in [GameKind::Shogi, GameKind::Xiangqi] {
        let mut c = controller();
        replace(&mut c, game, false);
        let root = c.shared_snapshot();
        assert!(!root.capabilities.take_back);
        assert!(send(&mut c, SessionOperation::TakeBack {}).is_err());
        play(
            &mut c,
            if game == GameKind::Shogi {
                "7g7f"
            } else {
                "a3a4"
            },
        );
        let old = pending(&c);
        let late = execute(&old);
        let before = c.shared_snapshot();
        assert!(before.capabilities.take_back);
        let undone = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(undone.context, root.context);
        assert!(undone.paused);
        assert_eq!(undone.board_ply, 0);
        assert_ne!(undone.token.version, before.token.version);
        assert_ne!(undone.token.selection, before.token.selection);
        assert!(!c.accept_shared(late));
        assert!(
            c.shared_command(SharedDesktopCommand {
                expected: before.token,
                game,
                operation: SessionOperation::Resume {}
            })
            .is_err()
        );
        send(&mut c, SessionOperation::Resume {}).unwrap();
        play(
            &mut c,
            if game == GameKind::Shogi {
                "7g7f"
            } else {
                "a3a4"
            },
        );
        engine(&mut c);
        assert_eq!(c.shared_snapshot().board_ply, 2);
        let pair = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(pair.context, root.context);
        assert_eq!(pair.observations.len(), 1);
        assert!(pair.observations[0].historical);
        roundtrip(&mut c, "pair");
        assert!(!c.shared_snapshot().capabilities.take_back);
    }
}
#[test]
fn takeback_native_preserves_engine_opening_and_repeated_human_decisions() {
    for game in [GameKind::Shogi, GameKind::Xiangqi] {
        let mut c = controller();
        replace(&mut c, game, true);
        engine(&mut c);
        let opening = c.shared_snapshot();
        assert_eq!(opening.board_ply, 1);
        assert!(!opening.capabilities.take_back);
        for _ in 0..2 {
            let action = c.shared_snapshot().choices[0].action.clone();
            play(&mut c, &action);
            engine(&mut c);
        }
        assert_eq!(c.shared_snapshot().board_ply, 5);
        let first = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(first.board_ply, 3);
        let second = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(second.context, opening.context);
        assert_eq!(second.board_ply, 1);
        assert!(!second.capabilities.take_back);
        assert!(second.observations.iter().all(|o| o.historical));
        roundtrip(&mut c, "second");
    }
}
#[test]
fn takeback_native_terminal_only_human_turn_and_off_turn_resignations() {
    for game in [GameKind::Shogi, GameKind::Xiangqi] {
        let mut c = controller();
        replace(&mut c, game, false);
        let root = c.shared_snapshot().context;
        send(&mut c, SessionOperation::Resign {}).unwrap();
        assert!(c.shared_snapshot().status.concluded);
        assert!(c.shared_snapshot().capabilities.take_back);
        let undone = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(undone.context, root);
        assert!(!undone.status.concluded);
        send(&mut c, SessionOperation::Resume {}).unwrap();
        play(
            &mut c,
            if game == GameKind::Shogi {
                "7g7f"
            } else {
                "a3a4"
            },
        );
        send(&mut c, SessionOperation::Resign {}).unwrap();
        let undone = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(undone.context, root);
        assert_eq!(undone.board_ply, 0);
        roundtrip(&mut c, "resigned");
    }
}
#[test]
fn takeback_native_history_requires_return_to_live_and_disables_stale_inspection() {
    for game in [GameKind::Shogi, GameKind::Xiangqi] {
        let mut c = controller();
        replace(&mut c, game, false);
        play(
            &mut c,
            if game == GameKind::Shogi {
                "7g7f"
            } else {
                "a3a4"
            },
        );
        engine(&mut c);
        let before = c.shared_snapshot();
        let viewed = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        assert!(viewed.historical);
        assert!(!viewed.capabilities.take_back);
        assert!(send(&mut c, SessionOperation::TakeBack {}).is_err());
        send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert!(
            c.shared_command(SharedDesktopCommand {
                expected: before.token,
                game,
                operation: SessionOperation::Inspect { event_index: 0 }
            })
            .is_err()
        );
    }
}

fn x_corpus(text: &str, id: &str, maximum: usize) -> x::Game {
    let corpus: serde_json::Value = serde_json::from_str(text).unwrap();
    let f = corpus["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap();
    let mut game = x::Game::from_root(
        x::PositionRoot::from_fen(f["root_fen"].as_str().unwrap()).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    for action in f["moves"].as_array().unwrap().iter().take(maximum) {
        game.submit(x::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: x::GameCommand::Play {
                action: action.as_str().unwrap().parse().unwrap(),
            },
        })
        .unwrap();
    }
    game
}
#[test]
fn takeback_native_xiangqi_warning_fourth_and_claim_use_committed_command_boundaries() {
    for (id, expected_plies, warning) in [("F022-CC", 6, false), ("F023-CCC", 10, true)] {
        let game = x_corpus(
            include_str!("../../../tests/fixtures/xiangqi/history-cycles.json"),
            id,
            usize::MAX,
        );
        let mut c = controller();
        send(
            &mut c,
            SessionOperation::Replace {
                record: CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap()),
                human: NativeSide::Xiangqi(x::Side::Red),
            },
        )
        .unwrap();
        let restored = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(restored.board_ply, expected_plies);
        assert!(!restored.status.concluded);
        assert_eq!(
            restored
                .xiangqi
                .as_ref()
                .unwrap()
                .warnings
                .iter()
                .any(|w| w.active),
            warning
        );
        let expected = x_corpus(
            include_str!("../../../tests/fixtures/xiangqi/history-cycles.json"),
            id,
            usize::try_from(expected_plies).unwrap(),
        );
        assert_eq!(
            restored.context,
            CanonicalRecord::xiangqi(x::GameRecord::from_game(&expected).unwrap())
        );
        roundtrip(&mut c, id);
    }
    let mut game = x_corpus(
        include_str!("../../../tests/fixtures/xiangqi/natural-count.json"),
        "NATURAL-10-CHECKS",
        100,
    );
    let eligible = x::GameRecord::from_game(&game).unwrap();
    game.submit(x::GameRequest {
        expected: game.context(),
        actor: x::Side::Red,
        command: x::GameCommand::ClaimNaturalCount,
    })
    .unwrap();
    assert!(game.conclusion().is_some());
    let mut c = controller();
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap()),
            human: NativeSide::Xiangqi(x::Side::Red),
        },
    )
    .unwrap();
    let restored = send(&mut c, SessionOperation::TakeBack {}).unwrap();
    assert_eq!(restored.context, CanonicalRecord::xiangqi(eligible));
    assert_eq!(restored.board_ply, 100);
    assert!(!restored.status.concluded);
    assert!(
        restored
            .xiangqi
            .as_ref()
            .unwrap()
            .count_advice
            .iter()
            .any(|a| a.claimant == x::Side::Red && a.eligible)
    );
    roundtrip(&mut c, "natural-count");
}
#[test]
fn takeback_native_shogi_declaration_and_fourfold_restore_native_history() {
    let imported = |sfen| {
        s::Game::from_root(
            s::GameRoot::import(
                s::Position::from_sfen(sfen).unwrap(),
                s::RootCategory::ReducedMaterialDiagram,
                s::CounterSource::Supplied,
            )
            .unwrap(),
        )
    };
    let mut declared = imported("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1");
    let root = s::GameRecord::from_game(&declared);
    declared
        .submit(s::GameRequest {
            expected: declared.context(),
            actor: s::Side::Sente,
            command: s::GameCommand::DeclareEnteringKing,
        })
        .unwrap();
    let mut c = controller();
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&declared)),
            human: NativeSide::Shogi(s::Side::Sente),
        },
    )
    .unwrap();
    let restored = send(&mut c, SessionOperation::TakeBack {}).unwrap();
    assert_eq!(restored.context, CanonicalRecord::shogi(root));
    assert!(!restored.status.concluded);
    roundtrip(&mut c, "declaration");
    let mut repeated = imported("4k4/9/5R3/9/9/9/9/9/K8 b - 1");
    for action in ["4c3c", "5a4a", "3c4c", "4a5a"].repeat(3) {
        repeated
            .submit(s::GameRequest {
                expected: repeated.context(),
                actor: repeated.position().side_to_move(),
                command: s::GameCommand::Play(action.parse().unwrap()),
            })
            .unwrap();
    }
    assert!(repeated.conclusion().is_some());
    let expected = s::GameRecord::from_game(&repeated.prefix(10).unwrap());
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&repeated)),
            human: NativeSide::Shogi(s::Side::Sente),
        },
    )
    .unwrap();
    let restored = send(&mut c, SessionOperation::TakeBack {}).unwrap();
    assert_eq!(restored.board_ply, 10);
    assert!(!restored.status.concluded);
    assert_eq!(restored.context, CanonicalRecord::shogi(expected));
    roundtrip(&mut c, "fourfold");
}

#[test]
fn takeback_native_unavailable_engine_first_refusal_keeps_pending_work_atomic() {
    for game in [GameKind::Shogi, GameKind::Xiangqi] {
        let mut c = controller();
        replace(&mut c, game, true);
        let before = c.shared_snapshot();
        let request = pending(&c);
        assert!(!before.capabilities.take_back);
        assert!(send(&mut c, SessionOperation::TakeBack {}).is_err());
        let after = c.shared_snapshot();
        assert_eq!(after.token, before.token);
        assert_eq!(after.context, before.context);
        assert_eq!(after.paused, before.paused);
        assert_eq!(pending(&c), request);
        engine(&mut c);
        assert_eq!(c.shared_snapshot().board_ply, 1);
    }
}
