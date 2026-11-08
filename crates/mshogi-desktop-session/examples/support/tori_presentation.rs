//! Actual DesktopController fixtures, not observations copied from expectations.
use mshogi_core::{
    native::{CanonicalRecord, NativeSide},
    tori as t,
};
use mshogi_desktop_session::{
    DesktopController, IoPhase, SessionOperation, SharedDesktopCommand, SharedSnapshot,
};
use std::{
    path::Path,
    time::{Duration, Instant},
};
#[path = "tori.rs"]
mod roots;
fn send(c: &mut DesktopController, operation: SessionOperation) -> SharedSnapshot {
    let game = c.shared_snapshot().game;
    let (_, r) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: c.shared_token(),
            game,
            operation,
        })
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let s = c.shared_snapshot();
        if let Some(result) = r.completion() {
            result.unwrap();
            return s;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn install(g: &t::Game) -> DesktopController {
    let mut c = DesktopController::new("/unused/fixture-tori-engine".into()).unwrap();
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::tori(t::GameRecord::from_game(g).unwrap()),
            human: NativeSide::Tori(t::Side::Sente),
        },
    );
    c
}
fn write(dir: &Path, name: &str, s: &SharedSnapshot) {
    std::fs::write(
        dir.join(format!("shared-tori-{name}.json")),
        serde_json::to_string_pretty(s).unwrap() + "\n",
    )
    .unwrap();
}
fn submit(c: &mut DesktopController, command: t::GameCommand) -> SharedSnapshot {
    let _snapshot = send(
        c,
        SessionOperation::Tori {
            actor: t::Side::Sente,
            command,
        },
    );
    #[cfg(feature = "desktop-verification")]
    {
        c.verify_tori_pause_play().unwrap();
        c.shared_snapshot()
    }
    #[cfg(not(feature = "desktop-verification"))]
    _snapshot
}
fn play(g: &mut t::Game, from: &str, to: &str, face: t::Face) {
    g.submit(t::GameRequest {
        expected: g.context(),
        actor: g.position().side_to_move(),
        command: t::GameCommand::Play {
            action: t::Action::Move {
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
                result_face: face,
            },
        },
    })
    .unwrap();
}
pub fn write_all(dir: &Path) {
    #[cfg(feature = "desktop-verification")]
    {
        let mut probe = DesktopController::new(std::env::current_exe().unwrap()).unwrap();
        send(
            &mut probe,
            SessionOperation::Replace {
                record: CanonicalRecord::tori(
                    t::GameRecord::from_game(&roots::contract_game()).unwrap(),
                ),
                human: NativeSide::Tori(t::Side::Sente),
            },
        );
        submit(
            &mut probe,
            t::GameCommand::Play {
                action: t::Action::Move {
                    from: "7c".parse().unwrap(),
                    to: "7b".parse().unwrap(),
                    result_face: t::Face::Eagle,
                },
            },
        );
        probe
            .verify_tori_execution(
                mshogi_engine::WorkLimits {
                    time_ms: None,
                    depth: 1,
                    nodes: 1,
                },
                false,
            )
            .unwrap();
        let pending = probe.verify_tori_pending_snapshot().unwrap();
        assert_eq!(pending.phase, mshogi_desktop_session::EnginePhase::Thinking);
        write(dir, "pending-unsupported", &pending);
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let snapshot = probe.shared_snapshot();
            // Historical filename retained for fixture compatibility: this is now
            // actual bounded incomplete native work, not UnsupportedMode.
            if snapshot.observations.len() == 1 {
                write(dir, "unsupported", &snapshot);
                break;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    #[cfg(feature = "desktop-verification")]
    {
        let mut probe = DesktopController::new(std::env::current_exe().unwrap()).unwrap();
        send(
            &mut probe,
            SessionOperation::Replace {
                record: CanonicalRecord::tori(
                    t::GameRecord::from_game(&roots::contract_game()).unwrap(),
                ),
                human: NativeSide::Tori(t::Side::Sente),
            },
        );
        probe
            .verify_tori_execution(
                mshogi_engine::WorkLimits {
                    time_ms: None,
                    depth: 1,
                    nodes: 1000,
                },
                true,
            )
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let snapshot = probe.shared_snapshot();
            if snapshot.analysis.is_some() {
                write(dir, "analysis-complete", &snapshot);
                break;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    #[cfg(feature = "desktop-verification")]
    for (name, file) in [("capacity-deadline", false), ("file-deadline", true)] {
        let mut c = install(&roots::contract_game());
        let before = c.shared_open_token();
        let snapshot = c.verify_tori_validation_deadline(file).unwrap();
        assert_eq!(c.shared_open_token(), before);
        write(dir, name, &snapshot);
    }
    let mut c = install(&t::Game::standard());
    write(dir, "standard", &c.shared_snapshot());
    let mut c = install(&roots::contract_game());
    write(dir, "contract", &c.shared_snapshot());
    let origins = c
        .shared_snapshot()
        .pieces
        .iter()
        .map(|p| p.square.clone())
        .chain(
            c.shared_snapshot()
                .hands
                .iter()
                .filter(|h| h.count > 0)
                .map(|h| h.origin.clone()),
        )
        .collect::<Vec<_>>();
    let inspections = origins
        .into_iter()
        .map(|origin| {
            c.shared_inspection(mshogi_desktop_session::MovementInspectionQuery {
                expected: c.shared_token(),
                origin,
            })
            .unwrap()
        })
        .collect::<Vec<_>>();
    std::fs::write(
        dir.join("shared-tori-inspections.json"),
        serde_json::to_string_pretty(&inspections).unwrap() + "\n",
    )
    .unwrap();
    let flipped = send(
        &mut c,
        SessionOperation::ToriPreferences {
            settings: mshogi_desktop_session::tori_document::ToriSettings {
                orientation: t::Side::Gote,
            },
        },
    );
    write(dir, "gote", &flipped);
    send(
        &mut c,
        SessionOperation::ToriPreferences {
            settings: mshogi_desktop_session::tori_document::ToriSettings {
                orientation: t::Side::Sente,
            },
        },
    );
    let action = t::Action::Move {
        from: "7c".parse().unwrap(),
        to: "7b".parse().unwrap(),
        result_face: t::Face::Eagle,
    };
    write(
        dir,
        "captured-promoted",
        &submit(&mut c, t::GameCommand::Play { action }),
    );
    write(
        dir,
        "takeback",
        &send(&mut c, SessionOperation::TakeBack {}),
    );
    write(
        dir,
        "dropped",
        &submit(
            &mut c,
            t::GameCommand::Play {
                action: t::Action::Drop {
                    face: t::Face::RightQuail,
                    to: "5e".parse().unwrap(),
                },
            },
        ),
    );
    write(
        dir,
        "selected-root",
        &send(&mut c, SessionOperation::Inspect { event_index: 0 }),
    );
    write(
        dir,
        "return-live",
        &send(&mut c, SessionOperation::ReturnLive {}),
    );
    write(
        dir,
        "stopped",
        &submit(
            &mut c,
            t::GameCommand::Stop {
                reason: t::StopReason::Cancelled,
            },
        ),
    );
    write(dir, "resigned", &submit(&mut c, t::GameCommand::Resign));
    // Independently authored TORI-H-COUNT eight-action history, actual native execution.
    let board = [("4g", t::Side::Sente), ("4a", t::Side::Gote)].map(|(sq, side)| {
        (
            sq.parse().unwrap(),
            t::Piece {
                side,
                face: t::Face::Phoenix,
            },
        )
    });
    let mut g = t::Game::from_root(
        t::Position::from_diagram(
            &board,
            [t::Hand::empty(), t::Hand::empty()],
            t::Side::Sente,
            t::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap();
    for _ in 0..2 {
        for (from, to) in [("4g", "5g"), ("4a", "5a"), ("5g", "4g"), ("5a", "4a")] {
            play(&mut g, from, to, t::Face::Phoenix);
        }
    }
    let mut c = install(&g);
    write(dir, "repetition", &c.shared_snapshot());
    write(
        dir,
        "repetition-prefix",
        &send(&mut c, SessionOperation::Inspect { event_index: 4 }),
    );
    // Actual failed oversized Open: typed native resource facts, current Tori preserved.
    let path = std::env::temp_dir().join(format!("msh132-oversized-{}.mshogi", std::process::id()));
    std::fs::write(&path, vec![b' '; 16 * 1024 * 1024 + 1]).unwrap();
    let id = c
        .queue_shared_open(path.clone(), c.shared_open_token())
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let s = c.shared_snapshot();
        if s.io.id == id && s.io.phase == IoPhase::Failed {
            write(dir, "resource", &s);
            break;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
    std::fs::remove_file(path).unwrap();
}
