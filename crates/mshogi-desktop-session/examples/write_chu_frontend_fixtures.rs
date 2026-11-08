//! Actual native/controller Chu DTOs; separate development generator, never bundled.
#[path = "support/chu.rs"]
mod roots;
use mshogi_core::{
    chu as c,
    native::{CanonicalRecord, NativeSide},
};
use mshogi_desktop_session::{
    DesktopController, SessionOperation, SharedDesktopCommand, SharedSnapshot,
};
fn send(c: &mut DesktopController, operation: SessionOperation) -> SharedSnapshot {
    let game = c.shared_snapshot().game;
    let (_, r) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: c.shared_token(),
            game,
            operation,
        })
        .unwrap();
    let end = std::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if let Some(done) = r.completion() {
            done.unwrap();
            return s;
        }
        assert!(std::time::Instant::now() < end);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("internal-search") {
        use std::io::BufRead;
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();
        mshogi_engine::serve_once(std::io::Cursor::new(line), std::io::stdout().lock()).unwrap();
        return;
    }
    let dir = std::path::PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    std::fs::create_dir_all(&dir).unwrap();
    // Use the very same admitted controller snapshots as the live native adapter.
    for (name, id) in [
        ("routes", "CHU-RETURN-Ln-enemy"),
        ("captures", "CHU-LION-C2-1"),
        ("promotion", "CHU-PROMOTE-DE-YES"),
        ("royals", "CHU-ROYAL-BOTH-COMPOUND"),
        ("counter", "CHU-REP-QUIET"),
    ] {
        let f = roots::fixture(id);
        let g = if name == "counter" {
            let base = roots::root(&f["given"]["root"]);
            let mut g = c::Game::from_root(
                c::Position::from_diagram(
                    &base.position().pieces().collect::<Vec<_>>(),
                    c::Side::Sente,
                    c::RootMetadata::imported("Exact counter fixture".into(), u64::MAX - 1)
                        .unwrap(),
                )
                .unwrap(),
            )
            .unwrap();
            g.submit(c::GameRequest {
                expected: g.context(),
                actor: c::Side::Sente,
                command: c::GameCommand::Play {
                    action: c::Action {
                        from: "1l".parse().unwrap(),
                        route: vec!["1k".parse().unwrap()],
                        promote: false,
                    },
                },
            })
            .unwrap();
            g
        } else if name == "routes" {
            roots::contract_game()
        } else {
            roots::root(&f["given"]["root"])
        };
        let mut c = DesktopController::new(std::env::current_exe().unwrap()).unwrap();
        let s = send(
            &mut c,
            SessionOperation::Replace {
                record: CanonicalRecord::chu(c::GameRecord::from_game(&g).unwrap()),
                human: NativeSide::Chu(g.position().side_to_move()),
            },
        );
        let write = |name: &str, s: &SharedSnapshot| {
            std::fs::write(
                dir.join(format!("shared-chu-{name}.json")),
                serde_json::to_string_pretty(s).unwrap() + "\n",
            )
            .unwrap()
        };
        write(name, &s);
        if name == "routes" {
            let inspection = c
                .shared_inspection(mshogi_desktop_session::MovementInspectionQuery {
                    expected: c.shared_token(),
                    origin: "11a".into(),
                })
                .unwrap();
            std::fs::write(
                dir.join("shared-chu-inspection.json"),
                serde_json::to_string_pretty(&inspection).unwrap() + "\n",
            )
            .unwrap();
        }
        if name == "routes" {
            let before = c.shared_snapshot().context;
            for (file, nodes) in [("analysis-complete", 512), ("analysis-incomplete", 1)] {
                let count = c.shared_snapshot().observations.len();
                c.verify_chu_execution(
                    mshogi_engine::WorkLimits {
                        time_ms: None,
                        depth: 1,
                        nodes,
                    },
                    true,
                )
                .unwrap();
                let end = std::time::Instant::now() + std::time::Duration::from_secs(20);
                let snapshot = loop {
                    let snapshot = c.shared_snapshot();
                    if snapshot.observations.len() > count {
                        break snapshot;
                    }
                    assert!(
                        std::time::Instant::now() < end,
                        "actual fixture observation deadline"
                    );
                    std::thread::sleep(std::time::Duration::from_millis(1));
                };
                assert_eq!(snapshot.context, before, "analysis changed native game");
                write(file, &snapshot);
            }
        }
        if name == "captures" {
            let s = send(
                &mut c,
                SessionOperation::Chu {
                    actor: c::Side::Sente,
                    command: c::GameCommand::Play {
                        action: roots::action(&f["given"]["commands"][0]),
                    },
                },
            );
            write("captured", &s);
        }
    }
}
